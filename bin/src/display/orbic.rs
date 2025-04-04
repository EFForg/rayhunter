use crate::config;
use crate::display::framebuffer;
use crate::display::DisplayState;

use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;

pub fn update_ui(task_tracker: &TaskTracker,  config: &config::Config, ui_shutdown_rx: oneshot::Receiver<()>, ui_update_rx: Receiver<DisplayState>) {
    framebuffer::update_ui(
        task_tracker,
        config,
        ui_shutdown_rx,
        ui_update_rx,
    )
}
