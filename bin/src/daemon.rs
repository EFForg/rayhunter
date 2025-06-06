mod analysis;
mod config;
mod diag;
mod display;
mod dummy_analyzer;
mod error;
mod key_input;
mod pcap;
mod qmdl_store;
mod server;
mod stats;

use crate::config::{parse_args, parse_config};
use crate::diag::run_diag_read_thread;
use crate::error::RayhunterError;
use crate::pcap::get_pcap;
use crate::qmdl_store::RecordingStore;
use crate::server::{get_qmdl, serve_static, ServerState};
use crate::stats::get_system_stats;

use analysis::{
    get_analysis_status, run_analysis_thread, start_analysis, AnalysisCtrlMessage, AnalysisStatus,
};
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use diag::{
    delete_all_recordings, delete_recording, get_analysis_report, start_recording, stop_recording,
    DiagDeviceCtrlMessage,
};
use log::{error, info};
use qmdl_store::RecordingStoreError;
use rayhunter::diag_device::DiagDevice;
use stats::get_qmdl_manifest;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::{oneshot, RwLock};
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;

type AppRouter = Router<Arc<ServerState>>;

fn get_router() -> AppRouter {
    Router::new()
        .route("/api/pcap/{name}", get(get_pcap))
        .route("/api/qmdl/{name}", get(get_qmdl))
        .route("/api/system-stats", get(get_system_stats))
        .route("/api/qmdl-manifest", get(get_qmdl_manifest))
        .route("/api/start-recording", post(start_recording))
        .route("/api/stop-recording", post(stop_recording))
        .route("/api/delete-recording/{name}", post(delete_recording))
        .route("/api/delete-all-recordings", post(delete_all_recordings))
        .route("/api/analysis-report/{name}", get(get_analysis_report))
        .route("/api/analysis", get(get_analysis_status))
        .route("/api/analysis/{name}", post(start_analysis))
        .route("/", get(|| async { Redirect::permanent("/index.html") }))
        .route("/{*path}", get(serve_static))
}

// Runs the axum server, taking all the elements needed to build up our
// ServerState and a oneshot Receiver that'll fire when it's time to shutdown
// (i.e. user hit ctrl+c)
async fn run_server(
    task_tracker: &TaskTracker,
    config: &config::Config,
    state: Arc<ServerState>,
    server_shutdown_rx: oneshot::Receiver<()>,
) -> JoinHandle<()> {
    info!("spinning up server");
    let app = get_router().with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    task_tracker.spawn(async move {
        info!("The orca is hunting for stingrays...");
        axum::serve(listener, app)
            .with_graceful_shutdown(server_shutdown_signal(server_shutdown_rx))
            .await
            .unwrap();
    })
}

async fn server_shutdown_signal(server_shutdown_rx: oneshot::Receiver<()>) {
    server_shutdown_rx.await.unwrap();
    info!("Server received shutdown signal, exiting...");
}

// Loads a RecordingStore if one exists, and if not, only create one if we're
// not in debug mode. If we fail to parse the manifest AND we're not in debug
// mode, try to recover by making a new (empty) manifest in the same directory.
async fn init_qmdl_store(config: &config::Config) -> Result<RecordingStore, RayhunterError> {
    let store_exists = RecordingStore::exists(&config.qmdl_store_path).await?;
    if config.debug_mode {
        if store_exists {
            Ok(RecordingStore::load(&config.qmdl_store_path).await?)
        } else {
            Err(RayhunterError::NoStoreDebugMode(
                config.qmdl_store_path.clone(),
            ))
        }
    } else if store_exists {
        match RecordingStore::load(&config.qmdl_store_path).await {
            Ok(store) => Ok(store),
            Err(RecordingStoreError::ParseManifestError(err)) => {
                error!("failed to parse QMDL manifest: {}", err);
                info!("creating new empty manifest...");
                let mut recording_store = RecordingStore::create(&config.qmdl_store_path).await?;
                info!("parsing existing qmdl files into recording store...");
                let path = Path::new(&config.qmdl_store_path);
                let mut entries = fs::read_dir(path).await?;

                // We might want to sort these newest to oldest so we don't have entries in manifest.toml in random order
                while let Some(entry) = entries.next_entry().await? {
                    let file_name = entry.file_name();
                    let file_name_str = match file_name.to_str() {
                        Some(s) => s,
                        None => continue, // skip non-UTF-8 names
                    };

                    if file_name_str.ends_with(".qmdl") {
                        let name = file_name_str.trim_end_matches(".qmdl");
                        info!("making entry for {}", name);
                        recording_store
                            .new_entry_from_existing(name.to_string())
                            .await?;
                    }
                }

                Ok(recording_store)
            }
            Err(err) => Err(err.into()),
        }
    } else {
        Ok(RecordingStore::create(&config.qmdl_store_path).await?)
    }
}

// Start a thread that'll track when user hits ctrl+c. When that happens,
// trigger various cleanup tasks, including sending signals to other threads to
// shutdown
fn run_ctrl_c_thread(
    task_tracker: &TaskTracker,
    diag_device_sender: Sender<DiagDeviceCtrlMessage>,
    server_shutdown_tx: oneshot::Sender<()>,
    maybe_ui_shutdown_tx: Option<oneshot::Sender<()>>,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    analysis_tx: Sender<AnalysisCtrlMessage>,
) -> JoinHandle<Result<(), RayhunterError>> {
    task_tracker.spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                let mut qmdl_store = qmdl_store_lock.write().await;
                if qmdl_store.current_entry.is_some() {
                    info!("Closing current QMDL entry...");
                    qmdl_store.close_current_entry().await?;
                    info!("Done!");
                }

                server_shutdown_tx
                    .send(())
                    .expect("couldn't send server shutdown signal");
                info!("sending UI shutdown");
                if let Some(ui_shutdown_tx) = maybe_ui_shutdown_tx {
                    ui_shutdown_tx
                        .send(())
                        .expect("couldn't send ui shutdown signal");
                }
                diag_device_sender
                    .send(DiagDeviceCtrlMessage::Exit)
                    .await
                    .expect("couldn't send Exit message to diag thread");
                analysis_tx
                    .send(AnalysisCtrlMessage::Exit)
                    .await
                    .expect("couldn't send Exit message to analysis thread");
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
        Ok(())
    })
}

#[tokio::main]
async fn main() -> Result<(), RayhunterError> {
    env_logger::init();

    let args = parse_args();
    let config = parse_config(&args.config_path)?;

    // TaskTrackers give us an interface to spawn tokio threads, and then
    // eventually await all of them ending
    let task_tracker = TaskTracker::new();
    println!("R A Y H U N T E R 🐳");

    let store = init_qmdl_store(&config).await?;
    let analysis_status = AnalysisStatus::new(&store);
    let qmdl_store_lock = Arc::new(RwLock::new(store));
    let (diag_tx, diag_rx) = mpsc::channel::<DiagDeviceCtrlMessage>(1);
    let (ui_update_tx, ui_update_rx) = mpsc::channel::<display::DisplayState>(1);
    let (analysis_tx, analysis_rx) = mpsc::channel::<AnalysisCtrlMessage>(5);
    let mut maybe_ui_shutdown_tx = None;
    if !config.debug_mode {
        let (ui_shutdown_tx, ui_shutdown_rx) = oneshot::channel();
        maybe_ui_shutdown_tx = Some(ui_shutdown_tx);
        let mut dev = DiagDevice::new()
            .await
            .map_err(RayhunterError::DiagInitError)?;
        dev.config_logs()
            .await
            .map_err(RayhunterError::DiagInitError)?;

        info!("Starting Diag Thread");
        run_diag_read_thread(
            &task_tracker,
            dev,
            diag_rx,
            ui_update_tx.clone(),
            qmdl_store_lock.clone(),
            analysis_tx.clone(),
            config.enable_dummy_analyzer,
        );
        info!("Starting UI");
        display::update_ui(&task_tracker, &config, ui_shutdown_rx, ui_update_rx);

        info!("Starting Key Input service");
        key_input::run_key_input_thread(&task_tracker, &config, diag_tx.clone());
    }
    let (server_shutdown_tx, server_shutdown_rx) = oneshot::channel::<()>();
    info!("create shutdown thread");
    let analysis_status_lock = Arc::new(RwLock::new(analysis_status));
    run_analysis_thread(
        &task_tracker,
        analysis_rx,
        qmdl_store_lock.clone(),
        analysis_status_lock.clone(),
        config.enable_dummy_analyzer,
    );
    run_ctrl_c_thread(
        &task_tracker,
        diag_tx.clone(),
        server_shutdown_tx,
        maybe_ui_shutdown_tx,
        qmdl_store_lock.clone(),
        analysis_tx.clone(),
    );
    let state = Arc::new(ServerState {
        qmdl_store_lock: qmdl_store_lock.clone(),
        diag_device_ctrl_sender: diag_tx,
        ui_update_sender: ui_update_tx,
        debug_mode: config.debug_mode,
        analysis_status_lock,
        analysis_sender: analysis_tx,
    });
    run_server(&task_tracker, &config, state, server_shutdown_rx).await;

    task_tracker.close();
    task_tracker.wait().await;

    info!("see you space cowboy...");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_router() {
        // assert that creating the router does not panic from invalid route patterns.
        let _ = get_router();
    }
}
