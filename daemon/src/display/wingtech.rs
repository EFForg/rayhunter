/// Display support for the Wingtech CT2MHS01 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
///   WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
///   WT_HARDWARE_VERSION=89323_1_20
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
