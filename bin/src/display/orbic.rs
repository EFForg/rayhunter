use image::{codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage};
use std::time::Duration;
use std::io::Cursor;

use crate::config;
use crate::display::DisplayState;
use crate::generic_framebuffer::{self, GenericFramebuffer, Dimensions, Color};

use log::{error, info};
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;
use tokio::sync::oneshot::error::TryRecvError;

use std::thread::sleep;

use include_dir::{include_dir, Dir};

const FB_PATH: &str = "/dev/fb0";

#[derive(Copy, Clone, Default)]
struct Framebuffer;

impl GenericFramebuffer for Framebuffer {
    fn dimensions(&self) -> Dimensions {
        // TODO actually poll for this, maybe w/ fbset?
        Dimensions {
            height: 128,
            width: 128,
        }
    }

    fn write_buffer(
        &mut self,
        buffer: &[(u8, u8, u8)],
        width: u32,
        height: u32
    ) {
        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (r as u16 & 0b11111000) << 8;
            rgb565 |= (g as u16 & 0b11111100) << 3;
            rgb565 |= (b as u16) >> 3;
            raw_buffer.extend(rgb565.to_le_bytes());
        }

        std::fs::write(FB_PATH, &raw_buffer).unwrap();
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>
) {
    generic_framebuffer::update_ui(
        task_tracker,
        config,
        Framebuffer,
        ui_shutdown_rx,
        ui_update_rx,
    )
}
