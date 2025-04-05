use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;

use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer::{self, GenericFramebuffer, Dimensions};

use tokio::sync::oneshot;
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
    ) {
        let width = self.dimensions().width;
        let height = buffer.len() / width;
        let mut f = File::options().write(true).open(FB_PATH).unwrap();
        let mut arg = fb_fillrect {
            dx: 0,
            dy: 0,
            width,
            height,
            color: 0xffff, // not sure what this is
            rop: 0,
        };

        unsafe {
            let res = libc::ioctl(
                f.as_raw_fd(),
                0x4619,  // FBIORECT_DISPLAY
                &mut arg as *mut _,
                std::mem::size_of::<fb_fillrect>(),
            );

            if res < 0 {
                panic!("failed to send FBIORECT_DISPLAY ioctl, {}", res);
            }
        }

        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            // note: RGB -> RBG
            let mut rbg565: u16 = (*r as u16 & 0b11111000) << 8;
            rbg565 |= (*b as u16 & 0b11111100) << 3;
            rbg565 |= (*g as u16) >> 3;
            raw_buffer.extend(rbg565.to_le_bytes());
        }

        f.write_all(&raw_buffer).unwrap();

    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    ui_shutdown_rx: oneshot::Receiver<()>,
    ui_update_rx: Receiver<DisplayState>
) {
    generic_framebuffer::update_ui(
        task_tracker,
        config,
        Framebuffer,
        ui_shutdown_rx,
        ui_update_rx,
    )
}
