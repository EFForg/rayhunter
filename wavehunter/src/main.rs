mod config;
mod error;
mod server;

use crate::config::{parse_config, parse_args};
use crate::server::{ServerState, serve_pcap};
use crate::error::WavehunterError;

use orca::diag_device::DiagDevice;
use orca::diag_reader::DiagReader;

use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use std::sync::Arc;

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
