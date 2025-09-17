use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer::{self, Dimensions, GenericFramebuffer};
use async_trait::async_trait;

use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

const FB_PATH: &str = "/dev/fb0";

#[derive(Copy, Clone, Default)]
struct Framebuffer;

#[async_trait]
impl GenericFramebuffer for Framebuffer {
    fn dimensions(&self) -> Dimensions {
        // TODO actually poll for this, maybe w/ fbset?
        Dimensions {
            height: 128,
            width: 128,
        }
    }

    async fn write_buffer(&mut self, buffer: Vec<(u8, u8, u8)>) {
        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (r as u16 & 0b11111000) << 8;
            rgb565 |= (g as u16 & 0b11111100) << 3;
            rgb565 |= (b as u16) >> 3;
            raw_buffer.extend(rgb565.to_le_bytes());
        }

        tokio::fs::write(FB_PATH, &raw_buffer).await.unwrap();
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    shutdown_token: CancellationToken,
    ui_update_rx: Receiver<DisplayState>,
) {
    generic_framebuffer::update_ui(
        task_tracker,
        config,
        Framebuffer,
        shutdown_token,
        ui_update_rx,
    )
}
