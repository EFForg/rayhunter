use log::info;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::display::{DisplayState, tplink_framebuffer, tplink_onebit};

use std::fs;

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    ui_shutdown_rx: oneshot::Receiver<()>,
    ui_update_rx: Receiver<DisplayState>,
) {
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    // Since this is a one-time check at startup, using sync is acceptable
    // The alternative would be to make the entire initialization async
    if fs::exists(tplink_onebit::OLED_PATH).unwrap_or_default() {
        info!("detected one-bit display");
        tplink_onebit::update_ui(task_tracker, config, ui_shutdown_rx, ui_update_rx)
    } else {
        info!("fallback to framebuffer");
        tplink_framebuffer::update_ui(task_tracker, config, ui_shutdown_rx, ui_update_rx)
    }
}
