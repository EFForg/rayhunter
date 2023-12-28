use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use futures_core::Stream;
use log::error;
use orca::diag_device::{DiagDevice, DiagDeviceError};
use orca::diag_reader::DiagReader;
use orca::gsmtap_parser::GsmtapParser;
use orca::pcap::GsmtapPcapWriter;
use orca::qmdl::{QmdlReader, QmdlReaderError};

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use axum::routing::get;
use std::fs::File;
use thiserror::Error;
use serde::Deserialize;
use std::io::Write;
use std::sync::Arc;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Poll, Context};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, RwLock};
use toml;

#[derive(Error, Debug)]
enum WavehunterError {
    #[error("Missing config file: {0}")]
    MissingConfigFile(String),
    #[error("Config file parsing error: {0}")]
    ConfigFileParsingError(#[from] toml::de::Error),
    #[error("Diag intialization error: {0}")]
    DiagInitError(DiagDeviceError),
    #[error("Diag read error: {0}")]
    DiagReadError(DiagDeviceError),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::io::Error),
}

#[derive(Deserialize)]
struct ConfigFile {
    qmdl_path: Option<String>,
    port: Option<u16>,
}

#[derive(Debug)]
struct Config {
    qmdl_path: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_path: "./wavehunter.qmdl".to_string(),
            port: 8080,
        }
    }
}

fn parse_config<P>(path: P) -> Result<Config, WavehunterError> where P: AsRef<std::path::Path> {
    let config_file = std::fs::read_to_string(&path)
        .map_err(|_| WavehunterError::MissingConfigFile(format!("{:?}", path.as_ref())))?;
    let parsed_config: ConfigFile = toml::from_str(&config_file)
        .map_err(WavehunterError::ConfigFileParsingError)?;
    let mut config = Config::default();
    parsed_config.qmdl_path.map(|path| config.qmdl_path = path);
    parsed_config.port.map(|path| config.port = path);
    Ok(config)
}

struct Args {
    config_path: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/config/file", args[0]);
        std::process::exit(1);
    }
    Args {
        config_path: args[1].clone(),
    }
}

fn run_diag_read_thread(mut dev: DiagDevice, bytes_read_lock: Arc<RwLock<usize>>) -> tokio::task::JoinHandle<Result<(), WavehunterError>> {
    tokio::task::spawn_blocking(move || {
        loop {
            // TODO: once we're actually doing analysis, we'll wanna use the messages
            // returned here. Until then, the DiagDevice has already written those messages
            // to the QMDL file, so we can just ignore them.
            let _messages = dev.read_response().map_err(WavehunterError::DiagReadError)?;

            // keep track of how many bytes were written to the QMDL file so we can read
            // a valid block of data from it in the HTTP server
            let mut bytes_read = bytes_read_lock.blocking_write();
            *bytes_read = dev.qmdl_writer.total_written;
        }
    })
}

// Streams a pcap file chunk-by-chunk to the client by reading the QMDL data
// written so far. This is done by spawning a blocking thread (a tokio thread
// capable of handling blocking operations) which streams chunks of pcap data to
// a channel that's piped to the client.
async fn serve_pcap(State(state): State<Arc<ServerState>>) -> Result<Response, (StatusCode, String)> {
    let qmdl_bytes_written = *state.qmdl_bytes_written.read().await;
    if qmdl_bytes_written == 0 {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "QMDL file is empty, try again in a bit!".to_string()
        ));
    }

    let (tx, rx) = mpsc::channel(1);
    let channel_reader = ChannelReader { rx };
    let channel_writer = ChannelWriter { tx };
    tokio::task::spawn_blocking(move || {
        // the QMDL reader should stop at the last successfully written data
        // chunk (qmdl_bytes_written)
        let qmdl_file = File::open(&state.qmdl_path).unwrap();
        let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_bytes_written));

        let mut gsmtap_parser = GsmtapParser::new();
        let mut pcap_writer = GsmtapPcapWriter::new(channel_writer).unwrap();
        pcap_writer.write_iface_header().unwrap();
        loop {
            match qmdl_reader.read_response() {
                Ok(messages) => {
                    for maybe_msg in messages {
                        match maybe_msg {
                            Ok(msg) => {
                                let maybe_gsmtap_msg = gsmtap_parser.recv_message(msg)
                                    .expect("error parsing gsmtap message");
                                if let Some((timestamp, gsmtap_msg)) = maybe_gsmtap_msg {
                                    pcap_writer.write_gsmtap_message(gsmtap_msg, timestamp)
                                        .expect("error writing pcap packet");
                                }
                            },
                            Err(e) => {
                                error!("error parsing message: {:?}", e);
                            },
                        }
                    }
                },
                // this is expected, and just means we've reached the end of the
                // safely written QMDL data
                Err(QmdlReaderError::MaxBytesReached(_)) => break,
                Err(e) => {
                    error!("error reading qmdl file: {:?}", e);
                    break;
                },
            }
        }
    });

    let headers = [(CONTENT_TYPE, "application/vnd.tcpdump.pcap")];
    let body = Body::from_stream(channel_reader);
    Ok((headers, body).into_response())
}

struct ChannelWriter {
    tx: mpsc::Sender<Vec<u8>>,
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.tx.blocking_send(buf.to_vec())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "channel closed"))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct ChannelReader {
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Stream for ChannelReader {
    type Item = Result<Vec<u8>, String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(msg)) => Poll::Ready(Some(Ok(msg))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct ServerState {
    qmdl_bytes_written: Arc<RwLock<usize>>,
    qmdl_path: String,
}

#[tokio::main]
async fn main() -> Result<(), WavehunterError> {
    env_logger::init();

    let args = parse_args();
    let config = parse_config(&args.config_path)?;

    let mut dev = DiagDevice::new(&config.qmdl_path)
        .map_err(WavehunterError::DiagInitError)?;
    dev.config_logs()
        .map_err(WavehunterError::DiagInitError)?;

    let qmdl_bytes_lock = Arc::new(RwLock::new(dev.qmdl_writer.total_written));
    // TODO: handle exiting gracefully
    let _read_thread_handle = run_diag_read_thread(dev, qmdl_bytes_lock.clone());

    println!("The orca is hunting for stingrays...");

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(&addr).await?;
    let state = Arc::new(ServerState {
        qmdl_bytes_written: qmdl_bytes_lock,
        qmdl_path: config.qmdl_path,
    });

    let app = Router::new()
        .route("/output.pcap", get(serve_pcap))
        .with_state(state);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
