use std::os::fd::AsRawFd;

use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer;

use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use super::generic_framebuffer::{FbInner, FramebufferDevice};

const FB_PATH: &str = "/dev/fb0";

#[repr(C)]
struct fb_fillrect {
    dx: u32,
    dy: u32,
    width: u32,
    height: u32,
    color: u32,
    rop: u32,
}

fn update_display(fb: &mut FbInner, buffer: &[(u8, u8, u8)]) {
    let width = fb.dims.width;
    let height = buffer.len() as u32 / width;
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
            fb.fd.as_raw_fd(),
            0x4619, // FBIORECT_DISPLAY
            &mut arg as *mut _,
            std::mem::size_of::<fb_fillrect>(),
        );

        if res < 0 {
            panic!("failed to send FBIORECT_DISPLAY ioctl, {res}");
        }
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
        FramebufferDevice::new(FB_PATH, None, Some(Box::new(update_display))),
        ui_shutdown_rx,
        ui_update_rx,
    )
}
