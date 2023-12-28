use orca::gsmtap_parser::GsmtapParser;
use orca::pcap::GsmtapPcapWriter;
use orca::qmdl::{QmdlReader, QmdlReaderError};
use orca::diag_reader::DiagReader;

use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use std::fs::File;
use std::io::Write;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::task::{Poll, Context};
use futures_core::Stream;
use log::error;
use tokio::sync::mpsc;

// Streams a pcap file chunk-by-chunk to the client by reading the QMDL data
// written so far. This is done by spawning a blocking thread (a tokio thread
// capable of handling blocking operations) which streams chunks of pcap data to
// a channel that's piped to the client.
pub async fn serve_pcap(State(state): State<Arc<ServerState>>) -> Result<Response, (StatusCode, String)> {
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

pub struct ServerState {
    pub qmdl_bytes_written: Arc<RwLock<usize>>,
    pub qmdl_path: String,
}
