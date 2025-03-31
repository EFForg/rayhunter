use crate::config;
use crate::display::DisplayState;

use log::info;
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;

pub fn update_ui(_task_tracker: &TaskTracker, _config: &config::Config, mut _ui_shutdown_rx: oneshot::Receiver<()>, mut _ui_update_rx: Receiver<DisplayState>) {
    info!("Spawning dummy UI due to unsupported build.");
}
