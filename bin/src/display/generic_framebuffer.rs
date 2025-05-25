use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;
use tokio_util::task::TaskTracker;

use std::thread::sleep;

#[derive(Copy, Clone)]
pub struct Dimensions {
    pub width: u32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Cyan,
    Yellow,
    Pink,
}

impl Color {
    fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => (0xff, 0, 0),
            Color::Green => (0, 0xff, 0),
            Color::Blue => (0, 0, 0xff),
            Color::White => (0xff, 0xff, 0xff),
            Color::Black => (0, 0, 0),
            Color::Cyan => (0, 0xff, 0xff),
            Color::Yellow => (0xff, 0xff, 0),
            Color::Pink => (0xfe, 0x24, 0xff),
        }
    }
}

impl Color {
    fn from_state(state: DisplayState, colorblind_mode: bool) -> Self {
        match state {
            DisplayState::Paused => Color::White,
            DisplayState::Recording => {
                if colorblind_mode {
                    Color::Blue
                } else {
                    Color::Green
                }
            }
            DisplayState::WarningDetected => Color::Red,
        }
    }
}

pub trait GenericFramebuffer: Send + 'static {
    fn dimensions(&self) -> Dimensions;

    fn write_buffer(
        &mut self,
        buffer: &[(u8, u8, u8)], // rgb, row-wise, left-to-right, top-to-bottom
    );

    fn draw_line(&mut self, color: Color, height: u32) {
        let width = self.dimensions().width;
        let px_num = height * width;
        let mut buffer = Vec::new();
        for _ in 0..px_num {
            buffer.push(color.rgb());
        }

        self.write_buffer(&buffer);
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut fb: impl GenericFramebuffer,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    let colorblind_mode = config.colorblind_mode;
    let mut display_color = Color::from_state(DisplayState::Recording, colorblind_mode);

    task_tracker.spawn_blocking(move || {
        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => panic!("error receiving shutdown message: {e}"),
            }
            match ui_update_rx.try_recv() {
                Ok(state) => {
                    display_color = Color::from_state(state, colorblind_mode);
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(e) => error!("error receiving framebuffer update message: {e}"),
            }

            match display_level {
                128 => {
                    fb.draw_line(Color::Cyan, 128);
                    fb.draw_line(Color::Pink, 102);
                    fb.draw_line(Color::White, 76);
                    fb.draw_line(Color::Pink, 50);
                    fb.draw_line(Color::Cyan, 25);
                }
                _ => {
                    // this branch id for ui_level 1, which is also the default if an
                    // unknown value is used
                    fb.draw_line(display_color, 2);
                }
            };
            sleep(Duration::from_millis(1000));
        }
    });
}
