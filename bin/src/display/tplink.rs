use log::info;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::display::{DisplayState, framebuffer, tplink_onebit};

use std::fs;
use std::ffi::c_int;

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

    if fs::exists(tplink_onebit::OLED_PATH).unwrap_or_default() {
        info!("detected one-bit display");
        tplink_onebit::update_ui(
            task_tracker,
            config,
            ui_shutdown_rx,
            ui_update_rx
        )
    } else {
        info!("fallback to framebuffer");
        framebuffer::update_ui(
            task_tracker,
            config,
            ui_shutdown_rx,
            ui_update_rx
        )
    }
}

pub fn run_tplink_ioctl(fd: c_int, dx: u32, dy: u32, width: u32, height: u32) {
    let mut arg = fb_fillrect {
        dx,
        dy,
        width,
        height,
        color: 0xffff,
        rop: 0,
    };

    unsafe {
        let res = libc::ioctl(
            fd,
            0x4619,
            &mut arg as *mut _,
            std::mem::size_of::<fb_fillrect>(),
        );

        if res < 0 {
            panic!("failed to send FBIORECT_DISPLAY ioctl, {}", res);
        }
    }
}


#[repr(C)]
struct fb_fillrect {
    dx: u32,
    dy: u32,
    width: u32,
    height: u32,
    color: u32,
    rop: u32,
}
