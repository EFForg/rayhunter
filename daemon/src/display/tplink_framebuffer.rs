use async_trait::async_trait;
use std::os::fd::AsRawFd;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;

use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer::{self, Dimensions, GenericFramebuffer};

use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;

const FB_PATH: &str = "/dev/fb0";

struct Framebuffer;

#[repr(C)]
struct fb_fillrect {
    dx: u32,
    dy: u32,
    width: u32,
    height: u32,
    color: u32,
    rop: u32,
}

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
        // for how to write to the buffer, consult M7350v5_en_gpl/bootable/recovery/recovery_color_oled.c
        let dimensions = self.dimensions();
        let width = dimensions.width;
        let height = buffer.len() as u32 / width;
        let mut f = OpenOptions::new().write(true).open(FB_PATH).await.unwrap();
        let mut arg = fb_fillrect {
            dx: 0,
            dy: 0,
            width,
            height,
            color: 0xffff, // not sure what this is
            rop: 0,
        };

        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (r as u16 & 0b11111000) << 8;
            rgb565 |= (g as u16 & 0b11111100) << 3;
            rgb565 |= (b as u16) >> 3;
            // note: big-endian!
            raw_buffer.extend(rgb565.to_be_bytes());
        }

        f.write_all(&raw_buffer).await.unwrap();

        // ioctl is a synchronous operation, but it's fast enough that it shouldn't block
        unsafe {
            let res = libc::ioctl(
                f.as_raw_fd(),
                0x4619, // FBIORECT_DISPLAY
                &mut arg as *mut _,
                std::mem::size_of::<fb_fillrect>(),
            );

            if res < 0 {
                panic!("failed to send FBIORECT_DISPLAY ioctl, {res}");
            }
        }
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
