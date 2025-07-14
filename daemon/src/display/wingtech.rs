/// Display support for the Wingtech CT2MHS01 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
///   WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
///   WT_HARDWARE_VERSION=89323_1_20
use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer::{self, Dimensions, GenericFramebuffer};

use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

const FB_PATH: &str = "/dev/fb0";

#[derive(Copy, Clone, Default)]
struct Framebuffer;

impl GenericFramebuffer for Framebuffer {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            height: 128,
            width: 160,
        }
    }

    fn write_buffer(&mut self, buffer: &[(u8, u8, u8)]) {
        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (*r as u16 & 0b11111000) << 8;
            rgb565 |= (*g as u16 & 0b11111100) << 3;
            rgb565 |= (*b as u16) >> 3;
            raw_buffer.extend(rgb565.to_le_bytes());
        }

        std::fs::write(FB_PATH, &raw_buffer).unwrap();
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    ui_shutdown_rx: oneshot::Receiver<()>,
    ui_update_rx: Receiver<DisplayState>,
) {
    generic_framebuffer::update_ui(
        task_tracker,
        config,
        Framebuffer,
        ui_shutdown_rx,
        ui_update_rx,
    )
}
