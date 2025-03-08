mod analysis;
mod config;
mod error;
mod pcap;
mod server;
mod stats;
mod qmdl_store;
mod diag;
mod framebuffer;
mod dummy_analyzer;
pub mod telemetry;

use crate::config::{parse_config, parse_args};
use crate::diag::run_diag_read_thread;
use crate::qmdl_store::RecordingStore;
use crate::server::{ServerState, get_qmdl, serve_static};
use crate::pcap::get_pcap;
use crate::stats::get_system_stats;
use crate::error::RayhunterError;
use crate::framebuffer::Framebuffer;

use analysis::{get_analysis_status, run_analysis_thread, start_analysis, AnalysisCtrlMessage, AnalysisStatus};
use axum::response::Redirect;
use diag::{get_analysis_report, start_recording, stop_recording, DiagDeviceCtrlMessage};
use log::{info, error};
use rayhunter::diag_device::DiagDevice;
use axum::routing::{get, post};
use axum::Router;
use stats::get_qmdl_manifest;
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::sync::oneshot::error::TryRecvError;
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{RwLock, oneshot};
use std::sync::Arc;
use include_dir::{include_dir, Dir};

// Runs the axum server, taking all the elements needed to build up our
// ServerState and a oneshot Receiver that'll fire when it's time to shutdown
// (i.e. user hit ctrl+c)
async fn run_server(
    task_tracker: &TaskTracker,
    config: &config::Config,
    config_path: String,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    server_shutdown_rx: oneshot::Receiver<()>,
    ui_update_tx: Sender<framebuffer::DisplayState>,
    diag_device_sender: Sender<DiagDeviceCtrlMessage>,
    analysis_sender: Sender<AnalysisCtrlMessage>,
    analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    telemetry_tx: Sender<telemetry::TelemetryMessage>,
    telemetry_device_id: String,
) -> JoinHandle<()> {
    info!("spinning up server");

    let state = Arc::new(ServerState {
        qmdl_store_lock,
        diag_device_ctrl_sender: diag_device_sender,
        ui_update_sender: ui_update_tx,
        debug_mode: config.debug_mode,
        analysis_status_lock,
        analysis_sender,
        colorblind_mode: config.colorblind_mode,
        config: config.clone(),
        config_path: config_path.clone(),
        telemetry_sender: telemetry_tx.clone(),
        telemetry_device_id,
        telemetry_enabled: config.telemetry_enabled,
    });

    let app = Router::new()
        .route("/api/pcap/*name", get(get_pcap))
        .route("/api/qmdl/*name", get(get_qmdl))
        .route("/api/system-stats", get(get_system_stats))
        .route("/api/qmdl-manifest", get(get_qmdl_manifest))
        .route("/api/start-recording", post(start_recording))
        .route("/api/stop-recording", post(stop_recording))
        .route("/api/analysis-report/*name", get(get_analysis_report))
        .route("/api/analysis", get(get_analysis_status))
        .route("/api/analysis/*name", post(start_analysis))
        .route("/api/telemetry", get(telemetry::get_telemetry_status))
        .route("/api/telemetry", post(telemetry::update_telemetry_settings))
        .route("/", get(|| async { Redirect::permanent("/index.html") }))
        .route("/*path", get(serve_static))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
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

// Loads a QmdlStore if one exists, and if not, only create one if we're not in
// debug mode.
async fn init_qmdl_store(config: &config::Config) -> Result<RecordingStore, RayhunterError> {
    match (RecordingStore::exists(&config.qmdl_store_path).await?, config.debug_mode) {
        (true, _) => Ok(RecordingStore::load(&config.qmdl_store_path).await?),
        (false, false) => Ok(RecordingStore::create(&config.qmdl_store_path).await?),
        (false, true) => Err(RayhunterError::NoStoreDebugMode(config.qmdl_store_path.clone())),
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
    telemetry_tx: Sender<telemetry::TelemetryMessage>,
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

                // Send exit signal to telemetry
                telemetry_tx.send(telemetry::TelemetryMessage::Exit).await
                .expect("couldn't send Exit message to telemetry thread");

                server_shutdown_tx.send(())
                    .expect("couldn't send server shutdown signal");
                info!("sending UI shutdown");
                if let Some(ui_shutdown_tx) = maybe_ui_shutdown_tx {
                    ui_shutdown_tx.send(())
                        .expect("couldn't send ui shutdown signal");
                }
                diag_device_sender.send(DiagDeviceCtrlMessage::Exit).await
                    .expect("couldn't send Exit message to diag thread");
                analysis_tx.send(AnalysisCtrlMessage::Exit).await
                    .expect("couldn't send Exit message to analysis thread");
            },
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
        Ok(())
    })
}

fn update_ui(task_tracker: &TaskTracker,  config: &config::Config, mut ui_shutdown_rx: oneshot::Receiver<()>, mut ui_update_rx: Receiver<framebuffer::DisplayState>) -> JoinHandle<()> {
    static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/images/");
    let mut display_color: framebuffer::Color565;
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    if config.colorblind_mode {
        display_color = framebuffer::Color565::Blue;
    } else {
        display_color = framebuffer::Color565::Green;
    }

    task_tracker.spawn_blocking(move || {
        let mut fb: Framebuffer = Framebuffer::new();
        // this feels wrong, is there a more rusty way to do this?
        let mut img: Option<&[u8]> = None;
        if display_level == 2 {
            img = Some(IMAGE_DIR.get_file("orca.gif").expect("failed to read orca.gif").contents());
        } else if display_level == 3 {
            img = Some(IMAGE_DIR.get_file("eff.png").expect("failed to read eff.png").contents());
        }
        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                },
                Err(TryRecvError::Empty) => {},
                Err(e) => panic!("error receiving shutdown message: {e}")
            }
            match ui_update_rx.try_recv() {
                    Ok(state) => {
                        display_color = state.into();
                    },
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                    Err(e) => error!("error receiving framebuffer update message: {e}")
            }

            match display_level  {
                2 => {
                    fb.draw_gif(img.unwrap());
                },
                3 => {
                    fb.draw_img(img.unwrap())
                },
                128 => {
                    fb.draw_line(framebuffer::Color565::Cyan, 128);
                    fb.draw_line(framebuffer::Color565::Pink, 102);
                    fb.draw_line(framebuffer::Color565::White, 76);
                    fb.draw_line(framebuffer::Color565::Pink, 50);
                    fb.draw_line(framebuffer::Color565::Cyan, 25);
                },
                1 | _ => {
                    fb.draw_line(display_color, 2);
                },
            };
            sleep(Duration::from_millis(1000));
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), RayhunterError> {
    env_logger::init();

    let args = parse_args();
    let config = parse_config(&args.config_path)?;
    let config_path = args.config_path.clone(); // for telemetry

    // TaskTrackers give us an interface to spawn tokio threads, and then
    // eventually await all of them ending
    let task_tracker = TaskTracker::new();
    println!("R A Y H U N T E R 🐳");

    // Telemetry setup
    let (telemetry_tx, telemetry_rx) = mpsc::channel::<telemetry::TelemetryMessage>(100);
    let telemetry_manager = telemetry::TelemetryManager::new(config.clone());
    let telemetry_device_id = telemetry_manager.get_device_id().clone();

    let qmdl_store_lock = Arc::new(RwLock::new(init_qmdl_store(&config).await?));
    let (tx, rx) = mpsc::channel::<DiagDeviceCtrlMessage>(1);
    let (ui_update_tx, ui_update_rx) = mpsc::channel::<framebuffer::DisplayState>(1);
    let (analysis_tx, analysis_rx) = mpsc::channel::<AnalysisCtrlMessage>(5);
    let mut maybe_ui_shutdown_tx = None;
    if !config.debug_mode {
        let (ui_shutdown_tx, ui_shutdown_rx) = oneshot::channel();
        maybe_ui_shutdown_tx = Some(ui_shutdown_tx);
        let mut dev = DiagDevice::new().await
            .map_err(RayhunterError::DiagInitError)?;
        dev.config_logs().await
            .map_err(RayhunterError::DiagInitError)?;

        info!("Starting Diag Thread");
        run_diag_read_thread(&task_tracker, dev, rx, ui_update_tx.clone(), 
            qmdl_store_lock.clone(), config.enable_dummy_analyzer, Some(telemetry_tx.clone()));
        info!("Starting UI");
        update_ui(&task_tracker, &config, ui_shutdown_rx, ui_update_rx);
    }

    // Start telemetry thread if enabled
    if config.telemetry_enabled {
        telemetry_manager.run_telemetry_thread(&task_tracker, qmdl_store_lock.clone(), telemetry_rx);
    }

    let (server_shutdown_tx, server_shutdown_rx) = oneshot::channel::<()>();
    info!("create shutdown thread");
    let analysis_status_lock = Arc::new(RwLock::new(AnalysisStatus::default()));
    run_analysis_thread(&task_tracker, analysis_rx, qmdl_store_lock.clone(), analysis_status_lock.clone(), config.enable_dummy_analyzer);
    run_ctrl_c_thread(&task_tracker, tx.clone(), server_shutdown_tx, maybe_ui_shutdown_tx, qmdl_store_lock.clone(), analysis_tx.clone(), telemetry_tx.clone());
    run_server(&task_tracker, &config, config_path, qmdl_store_lock.clone(), server_shutdown_rx, ui_update_tx, tx, analysis_tx, analysis_status_lock, telemetry_tx, telemetry_device_id).await;

    task_tracker.close();
    task_tracker.wait().await;

    info!("see you space cowboy...");
    Ok(())
}
