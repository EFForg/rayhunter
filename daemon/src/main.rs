mod analysis;
mod battery;
mod config;
mod diag;
mod display;
mod error;
mod key_input;
mod notifications;
mod pcap;
mod qmdl_store;
mod server;
mod stats;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::battery::run_battery_notification_worker;
use crate::config::{parse_args, parse_config};
use crate::diag::run_diag_read_thread;
use crate::error::RayhunterError;
use crate::notifications::{NotificationService, run_notification_worker};
use crate::pcap::get_pcap;
use crate::qmdl_store::RecordingStore;
use crate::server::{
    ServerState, debug_set_display_state, get_config, get_qmdl, get_time, get_zip, serve_static,
    set_config, set_time_offset, test_notification,
};
use crate::stats::{get_qmdl_manifest, get_system_stats};

use analysis::{
    AnalysisCtrlMessage, AnalysisStatus, get_analysis_status, run_analysis_thread, start_analysis,
};
use axum::Router;
use axum::response::Redirect;
use axum::routing::{get, post};
use diag::{
    DiagDeviceCtrlMessage, delete_all_recordings, delete_recording, get_analysis_report,
    start_recording, stop_recording,
};
use log::{error, info};
use qmdl_store::RecordingStoreError;
use rayhunter::Device;
use rayhunter::diag_device::DiagDevice;
use stats::get_log;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, Sender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

type AppRouter = Router<Arc<ServerState>>;

fn get_router() -> AppRouter {
    Router::new()
        .route("/api/pcap/{name}", get(get_pcap))
        .route("/api/qmdl/{name}", get(get_qmdl))
        .route("/api/zip/{name}", get(get_zip))
        .route("/api/system-stats", get(get_system_stats))
        .route("/api/qmdl-manifest", get(get_qmdl_manifest))
        .route("/api/log", get(get_log))
        .route("/api/start-recording", post(start_recording))
        .route("/api/stop-recording", post(stop_recording))
        .route("/api/delete-recording/{name}", post(delete_recording))
        .route("/api/delete-all-recordings", post(delete_all_recordings))
        .route("/api/analysis-report/{name}", get(get_analysis_report))
        .route("/api/analysis", get(get_analysis_status))
        .route("/api/analysis/{name}", post(start_analysis))
        .route("/api/config", get(get_config))
        .route("/api/config", post(set_config))
        .route("/api/test-notification", post(test_notification))
        .route("/api/time", get(get_time))
        .route("/api/time-offset", post(set_time_offset))
        .route("/api/debug/display-state", post(debug_set_display_state))
        .route("/", get(|| async { Redirect::permanent("/index.html") }))
        .route("/{*path}", get(serve_static))
}

// Runs the axum server, taking all the elements needed to build up our
// ServerState and a oneshot Receiver that'll fire when it's time to shutdown
// (i.e. user hit ctrl+c)
async fn run_server(
    task_tracker: &TaskTracker,
    state: Arc<ServerState>,
    shutdown_token: CancellationToken,
) -> JoinHandle<()> {
    info!("spinning up server");
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    let app = get_router().with_state(state);

    task_tracker.spawn(async move {
        info!("The orca is hunting for stingrays...");
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_token.cancelled_owned())
            .await
            .unwrap();
    })
}

// Loads a RecordingStore if one exists, and if not, only create one if we're
// not in debug mode. If we fail to parse the manifest AND we're not in debug
// mode, try to recover the manifest from the existing QMDL files
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
                error!("failed to parse QMDL manifest: {err}");
                info!("recovering manifest from existing QMDL files...");
                Ok(RecordingStore::recover(&config.qmdl_store_path).await?)
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
fn run_shutdown_thread(
    task_tracker: &TaskTracker,
    diag_device_sender: Sender<DiagDeviceCtrlMessage>,
    shutdown_token: CancellationToken,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    analysis_tx: Sender<AnalysisCtrlMessage>,
) -> JoinHandle<Result<(), RayhunterError>> {
    info!("create shutdown thread");

    task_tracker.spawn(async move {
        select! {
            res = tokio::signal::ctrl_c() => {
                if let Err(err) = res {
                    error!("Unable to listen for shutdown signal: {err}");
                }
            }
            _ = shutdown_token.cancelled() => {}
        }

        let mut qmdl_store = qmdl_store_lock.write().await;
        if qmdl_store.current_entry.is_some() {
            info!("Closing current QMDL entry...");
            qmdl_store.close_current_entry().await?;
            info!("Done!");
        }

        shutdown_token.cancel();
        diag_device_sender
            .send(DiagDeviceCtrlMessage::Exit)
            .await
            .expect("couldn't send Exit message to diag thread");
        analysis_tx
            .send(AnalysisCtrlMessage::Exit)
            .await
            .expect("couldn't send Exit message to analysis thread");
        Ok(())
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), RayhunterError> {
    rayhunter::init_logging(log::LevelFilter::Info);

    #[cfg(feature = "rustcrypto-tls")]
    {
        rustls_rustcrypto::provider()
            .install_default()
            .expect("Couldn't install rustcrypto provider");
    }

    let args = parse_args();

    loop {
        let config = parse_config(&args.config_path).await?;
        if !run_with_config(&args, config).await? {
            return Ok(());
        }
    }
}

async fn run_with_config(
    args: &config::Args,
    config: config::Config,
) -> Result<bool, RayhunterError> {
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
    let restart_token = CancellationToken::new();
    let shutdown_token = restart_token.child_token();
    // Ensure shutdown_token is cancelled when this function exits for any
    // reason (e.g. diag device init failure), so all spawned tasks get
    // signaled to stop.
    let _shutdown_guard = shutdown_token.clone().drop_guard();

    let notification_service = NotificationService::new(config.ntfy_url.clone());

    if !config.debug_mode {
        info!("Using configuration for device: {0:?}", config.device);
        let mut dev = DiagDevice::new(&config.device)
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
            diag_tx.clone(),
            ui_update_tx.clone(),
            qmdl_store_lock.clone(),
            analysis_tx.clone(),
            config.analyzers.clone(),
            notification_service.new_handler(),
        );
        info!("Starting UI");

        let update_ui = match &config.device {
            Device::Orbic => display::orbic::update_ui,
            Device::Tplink => display::tplink::update_ui,
            Device::Tmobile => display::tmobile::update_ui,
            Device::Wingtech => display::wingtech::update_ui,
            Device::Pinephone => display::headless::update_ui,
            Device::Uz801 => display::uz801::update_ui,
        };
        update_ui(&task_tracker, &config, shutdown_token.clone(), ui_update_rx);

        info!("Starting Key Input service");
        key_input::run_key_input_thread(
            &task_tracker,
            &config,
            diag_tx.clone(),
            shutdown_token.clone(),
        );
    }

    let analysis_status_lock = Arc::new(RwLock::new(analysis_status));
    run_analysis_thread(
        &task_tracker,
        analysis_rx,
        qmdl_store_lock.clone(),
        analysis_status_lock.clone(),
        config.analyzers.clone(),
    );

    run_shutdown_thread(
        &task_tracker,
        diag_tx.clone(),
        shutdown_token.clone(),
        qmdl_store_lock.clone(),
        analysis_tx.clone(),
    );

    run_battery_notification_worker(
        &task_tracker,
        config.device.clone(),
        notification_service.new_handler(),
        shutdown_token.clone(),
    );

    run_notification_worker(
        &task_tracker,
        notification_service,
        config.enabled_notifications.clone(),
    );

    let state = Arc::new(ServerState {
        config_path: args.config_path.clone(),
        config,
        qmdl_store_lock: qmdl_store_lock.clone(),
        diag_device_ctrl_sender: diag_tx,
        analysis_status_lock,
        analysis_sender: analysis_tx,
        daemon_restart_token: restart_token.clone(),
        ui_update_sender: Some(ui_update_tx),
    });
    run_server(&task_tracker, state, shutdown_token.clone()).await;

    task_tracker.close();
    task_tracker.wait().await;

    info!("see you space cowboy...");
    Ok(restart_token.is_cancelled())
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
