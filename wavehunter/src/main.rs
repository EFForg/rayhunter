mod config;
mod error;
mod pcap;
mod server;
mod stats;

use crate::config::{parse_config, parse_args};
use crate::server::{ServerState, get_qmdl, serve_static};
use crate::pcap::get_pcap;
use crate::stats::{get_system_stats, get_diag_stats};
use crate::error::WavehunterError;

use axum::response::Redirect;
use orca::diag_device::DiagDevice;
use orca::diag_reader::DiagReader;
use axum::routing::get;
use axum::Router;
use tokio::fs::File;
use log::debug;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use std::sync::Arc;

fn run_diag_read_thread(mut dev: DiagDevice, bytes_read_lock: Arc<RwLock<usize>>) -> JoinHandle<Result<(), WavehunterError>> {
    tokio::task::spawn_blocking(move || {
        loop {
            // TODO: once we're actually doing analysis, we'll wanna use the messages
            // returned here. Until then, the DiagDevice has already written those messages
            // to the QMDL file, so we can just ignore them.
            debug!("reading response from diag device...");
            let _messages = dev.read_response().map_err(WavehunterError::DiagReadError)?;
            debug!("got diag response ({} messages)", _messages.len());

            // keep track of how many bytes were written to the QMDL file so we can read
            // a valid block of data from it in the HTTP server
            debug!("total QMDL bytes written: {}, updating state...", dev.qmdl_writer.total_written);
            let mut bytes_read = bytes_read_lock.blocking_write();
            *bytes_read = dev.qmdl_writer.total_written;
            debug!("done!");
        }
    })
}

async fn run_server(config: &config::Config, qmdl_bytes_written: Arc<RwLock<usize>>) -> Result<(), WavehunterError> {
    let state = Arc::new(ServerState {
        qmdl_bytes_written,
        qmdl_path: config.qmdl_path.clone(),
    });

    let app = Router::new()
        .route("/api/pcap/latest.pcap", get(get_pcap))
        .route("/api/qmdl/latest.qmdl", get(get_qmdl))
        .route("/api/system-stats", get(get_system_stats))
        .route("/api/diag-stats", get(get_diag_stats))
        .route("/", get(|| async { Redirect::permanent("/index.html") }))
        .route("/*path", get(serve_static))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), WavehunterError> {
    env_logger::init();

    let args = parse_args();
    let config = parse_config(&args.config_path)?;

    let qmdl_bytes_lock: Arc<RwLock<usize>>;
    if !config.debug_mode {
        let mut dev = DiagDevice::new(&config.qmdl_path)
            .map_err(WavehunterError::DiagInitError)?;
        dev.config_logs()
            .map_err(WavehunterError::DiagInitError)?;
        qmdl_bytes_lock = Arc::new(RwLock::new(dev.qmdl_writer.total_written));

        // TODO: handle exiting gracefully
        let _read_thread_handle = run_diag_read_thread(dev, qmdl_bytes_lock.clone());
    } else {
        let qmdl_file = File::open(&config.qmdl_path).await.expect("couldn't open QMDL file");
        let qmdl_file_size = qmdl_file.metadata().await.expect("couldn't get QMDL file metadata")
            .len() as usize;
        qmdl_bytes_lock = Arc::new(RwLock::new(qmdl_file_size));
    }

    println!("The orca is hunting for stingrays...");
    run_server(&config, qmdl_bytes_lock).await
}
