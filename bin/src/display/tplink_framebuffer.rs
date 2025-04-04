use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;

use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;
use tokio::sync::oneshot::error::TryRecvError;

use std::thread::sleep;

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


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color565 {
    Red    = 0b1111100000000000,
    Green  = 0b0000000000011111,
    Blue   = 0b0000011111100000,
    White  = 0b1111111111111111,
}

impl From<DisplayState> for Color565 {
    fn from(state: DisplayState) -> Self {
        match state {
            DisplayState::Paused => Color565::White,
            DisplayState::Recording => Color565::Green, 
            DisplayState::RecordingCBM => Color565::Blue, 
            DisplayState::WarningDetected => Color565::Red,
        }
    }
}


pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>
) {
    let mut display_color: Color565;
    if config.colorblind_mode {
        display_color = Color565::Blue;
    } else {
        display_color = Color565::Green;
    }

    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    task_tracker.spawn_blocking(move || {
        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                },
                Err(TryRecvError::Empty) => {},
                Err(e) => panic!("error receiving shutdown message: {e}")
            }

            match ui_update_rx.try_recv() {
                Ok(state) => display_color = state.into(),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                Err(e) => error!("error receiving framebuffer update message: {e}")
            }

            if display_level != 0 {
                let mut f = File::options().write(true).open(FB_PATH).unwrap();
                let mut arg = fb_fillrect {
                    dx: 0,
                    dy: 0,
                    width: 128,
                    height: 2,
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

                let mut buffer: Vec<u8> = Vec::new();
                let px_num = arg.width * arg.height;

                for _ in 0..px_num {
                    buffer.extend((display_color as u16).to_le_bytes());
                }
                f.write_all(&buffer).unwrap();
            };
            sleep(Duration::from_millis(1000));
        }
    });
}
