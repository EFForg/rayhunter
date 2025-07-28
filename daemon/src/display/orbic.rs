use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer;

use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use super::generic_framebuffer::FramebufferDevice;

const FB_PATH: &str = "/dev/fb0";

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    ui_shutdown_rx: oneshot::Receiver<()>,
    ui_update_rx: Receiver<DisplayState>,
) {
    generic_framebuffer::update_ui(
        task_tracker,
        config,
        FramebufferDevice::new(FB_PATH, None, None),
        ui_shutdown_rx,
        ui_update_rx,
    )
}
