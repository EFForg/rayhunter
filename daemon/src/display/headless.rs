use log::info;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::display::DisplayState;

pub fn update_ui(
    _task_tracker: &TaskTracker,
    _config: &config::Config,
    _ui_shutdown_rx: oneshot::Receiver<()>,
    _ui_update_rx: Receiver<DisplayState>,
) {
    info!("Headless mode, not spawning UI.");
}
