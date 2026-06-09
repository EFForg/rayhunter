use log::info;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::display::DisplayState;

pub fn update_ui(
    task_tracker: &TaskTracker,
    _config: &config::Config,
    shutdown_token: CancellationToken,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    info!("Headless mode, not spawning UI.");
    task_tracker.spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown_token.cancelled() => break,
                _ = ui_update_rx.recv() => {}
            }
        }
    });
}
