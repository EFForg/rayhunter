use crate::config;
use crate::display::DisplayState;

use log::info;
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;

pub fn update_ui(task_tracker: &TaskTracker, _config: &config::Config, mut _ui_shutdown_rx: oneshot::Receiver<()>, mut _ui_update_rx: Receiver<DisplayState>) -> JoinHandle<()> {
    task_tracker.spawn_blocking(|| {
        info!("Spawning dummy UI due to unsupported build.");
    })
}

