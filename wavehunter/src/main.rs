mod config;
mod error;
mod pcap;
mod server;
mod stats;
mod qmdl_store;
mod diag;

use crate::config::{parse_config, parse_args};
use crate::diag::run_diag_read_thread;
use crate::qmdl_store::QmdlStore;
use crate::server::{ServerState, get_qmdl, serve_static};
use crate::pcap::get_pcap;
use crate::stats::get_system_stats;
use crate::error::WavehunterError;

use axum::response::Redirect;
use diag::{DiagDeviceCtrlMessage, start_recording, stop_recording};
use log::{info, error};
use orca::diag_device::DiagDevice;
use axum::routing::{get, post};
use axum::Router;
use orca::qmdl::QmdlWriter;
use stats::get_qmdl_manifest;
use tokio::sync::mpsc::{self, Sender};
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::{RwLock, oneshot};
use std::sync::Arc;

async fn run_server(
    task_tracker: &TaskTracker,
    config: &config::Config,
    qmdl_store_lock: Arc<RwLock<QmdlStore>>,
    server_shutdown_rx: oneshot::Receiver<()>,
    diag_device_sender: Sender<DiagDeviceCtrlMessage>
) -> JoinHandle<()> {
    let state = Arc::new(ServerState {
        qmdl_store_lock,
        diag_device_ctrl_sender: diag_device_sender,
        readonly_mode: config.readonly_mode,
    });

    let app = Router::new()
        .route("/api/pcap/*name", get(get_pcap))
        .route("/api/qmdl/*name", get(get_qmdl))
        .route("/api/system-stats", get(get_system_stats))
        .route("/api/qmdl-manifest", get(get_qmdl_manifest))
        .route("/api/start-recording", post(start_recording))
        .route("/api/stop-recording", post(stop_recording))
        .route("/", get(|| async { Redirect::permanent("/index.html") }))
        .route("/*path", get(serve_static))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    task_tracker.spawn(async move {
        info!("The orca is hunting for stingrays...");
        axum::serve(listener, app)
            .with_graceful_shutdown(server_shutdown_signal(server_shutdown_rx))
            .await.unwrap();
    })
}

async fn server_shutdown_signal(server_shutdown_rx: oneshot::Receiver<()>) {
    server_shutdown_rx.await.unwrap();
    info!("Server received shutdown signal, exiting...");
}

async fn init_qmdl_store(config: &config::Config) -> Result<QmdlStore, WavehunterError> {
    match (QmdlStore::exists(&config.qmdl_store_path).await?, config.readonly_mode) {
        (true, _) => Ok(QmdlStore::load(&config.qmdl_store_path).await?),
        (false, false) => Ok(QmdlStore::create(&config.qmdl_store_path).await?),
        (false, true) => Err(WavehunterError::NoStoreReadonlyMode(config.qmdl_store_path.clone())),
    }
}

fn run_ctrl_c_thread(
    task_tracker: &TaskTracker,
    diag_device_sender: Sender<DiagDeviceCtrlMessage>,
    server_shutdown_tx: oneshot::Sender<()>,
    qmdl_store_lock: Arc<RwLock<QmdlStore>>
) -> JoinHandle<Result<(), WavehunterError>> {
    task_tracker.spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                let mut qmdl_store = qmdl_store_lock.write().await;
                if qmdl_store.current_entry.is_some() {
                    info!("Closing current QMDL entry...");
                    qmdl_store.close_current_entry().await?;
                    info!("Done!");
                }

                server_shutdown_tx.send(())
                    .expect("couldn't send server shutdown signal");
                diag_device_sender.send(DiagDeviceCtrlMessage::Exit).await
                    .expect("couldn't send Exit message to diag thread");
            },
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
        Ok(())
    })
}

#[tokio::main]
async fn main() -> Result<(), WavehunterError> {
    env_logger::init();

    let args = parse_args();
    let config = parse_config(&args.config_path)?;

    let task_tracker = TaskTracker::new();

    let qmdl_store_lock = Arc::new(RwLock::new(init_qmdl_store(&config).await?));
    let (tx, rx) = mpsc::channel::<DiagDeviceCtrlMessage>(1);
    if !config.readonly_mode {
        let qmdl_file = qmdl_store_lock.write().await.new_entry().await?;
        let qmdl_writer = QmdlWriter::new(qmdl_file.into_std().await);
        let mut dev = DiagDevice::new(Some(qmdl_writer))
            .map_err(WavehunterError::DiagInitError)?;
        dev.config_logs()
            .map_err(WavehunterError::DiagInitError)?;

        run_diag_read_thread(&task_tracker, dev, rx, qmdl_store_lock.clone());
    }

    let (server_shutdown_tx, server_shutdown_rx) = oneshot::channel::<()>();
    run_ctrl_c_thread(&task_tracker, tx.clone(), server_shutdown_tx, qmdl_store_lock.clone());
    run_server(&task_tracker, &config, qmdl_store_lock.clone(), server_shutdown_rx, tx).await;

    task_tracker.close();
    task_tracker.wait().await;

    Ok(())
}
