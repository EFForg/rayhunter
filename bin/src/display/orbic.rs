use crate::config;
use crate::display::framebuffer::{self, Framebuffer};
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;
use tokio::sync::oneshot::error::TryRecvError;

use std::thread::sleep;
use std::time::Duration;

use include_dir::{include_dir, Dir};

pub fn update_ui(task_tracker: &TaskTracker,  config: &config::Config, mut ui_shutdown_rx: oneshot::Receiver<()>, mut ui_update_rx: Receiver<DisplayState>) {
    static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/images/");
    let mut display_color: framebuffer::Color565;
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    if config.colorblind_mode {
        display_color = framebuffer::Color565::Blue;
    } else {
        display_color = framebuffer::Color565::Green;
    }

    task_tracker.spawn_blocking(move || {
        let mut fb: Framebuffer = Framebuffer::new();
        // this feels wrong, is there a more rusty way to do this?
        let mut img: Option<&[u8]> = None;
        if display_level == 2 {
            img = Some(IMAGE_DIR.get_file("orca.gif").expect("failed to read orca.gif").contents());
        } else if display_level == 3 {
            img = Some(IMAGE_DIR.get_file("eff.png").expect("failed to read eff.png").contents());
        }
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
                    Ok(state) => {
                        display_color = state.into();
                    },
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                    Err(e) => error!("error receiving framebuffer update message: {e}")
            }

            match display_level  {
                2 => {
                    fb.draw_gif(img.unwrap());
                },
                3 => {
                    fb.draw_img(img.unwrap())
                },
                128 => {
                    fb.draw_line(framebuffer::Color565::Cyan, 128);
                    fb.draw_line(framebuffer::Color565::Pink, 102);
                    fb.draw_line(framebuffer::Color565::White, 76);
                    fb.draw_line(framebuffer::Color565::Pink, 50);
                    fb.draw_line(framebuffer::Color565::Cyan, 25);
                },
                _ => { // this branch id for ui_level 1, which is also the default if an
                       // unknown value is used
                    fb.draw_line(display_color, 2);
                },
            };
            sleep(Duration::from_millis(1000));
        }
    });
}
