/// Display module for the TP-Link M7350 oled one-bit display.
///
/// https://github.com/m0veax/tplink_m7350/tree/main/oled
use crate::config;
use crate::display::DisplayState;

use log::{info, error};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;
use tokio::sync::oneshot::error::TryRecvError;

use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub const OLED_PATH: &str = "/sys/class/display/oled/oled_buffer";

// those coordinates were mainly chosen for a spot that doesn't get regularly updated by the main
// oledd service. otherwise we'd have to write to the display more than once per second to prevent
// the icon from flickering.
const STATUS_X: u8 = 104;
const STATUS_Y: u8 = 40;
const STATUS_W: u8 = 16;
const STATUS_H: u8 = 16;

const STATUS_HEADER: [u8; 4] = [STATUS_X, STATUS_Y, STATUS_W, STATUS_H];

macro_rules! pixel {
    (x) => { 0 };
    (_) => { 1 };
}

macro_rules! pixelart {
    ($($tt:tt)*) => {{
        // could be improved to be const expr or at least to compile to something that doesn't
        // allocate. but the macro is easier to write this way.
        let mut bytes = Vec::new();
        let mut i = 0;
        let mut byte = 0;
        $(
            byte |= pixel!($tt);
            if i == 7 {
                bytes.push(byte);
                i = 0;
                byte = 0;
            } else {
                i += 1;
                byte <<= 1;
            }
        )*

        // last byte is bogus, discard it to silence warnings
        let _ = byte;

        assert_eq!(i % 8, 0);
        bytes
    }}
}

fn paused() -> Vec<u8> {
    let mut command = STATUS_HEADER.to_vec();

    command.extend(pixelart! {
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
        _ _ _ x x x x x x x x x x _ _ _ 
        _ x x _ _ _ _ _ _ _ _ _ _ x x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ x _ _ _ _ x _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x x _ _ _ _ _ _ _ _ _ _ x x _ 
        _ _ _ x x x x x x x x x x _ _ _ 
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
    });
    command
}

fn smiling() -> Vec<u8> {
    let mut command = STATUS_HEADER.to_vec();

    command.extend(pixelart! {
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
        _ _ _ x x x x x x x x x x _ _ _ 
        _ x x _ _ _ _ _ _ _ _ _ _ x x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ x _ _ _ _ x _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ x _ _ _ _ x _ _ _ x _ 
        _ x _ _ _ x _ _ _ _ x _ _ _ x _ 
        _ x _ _ _ x x x x x x _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x _ _ _ _ _ _ _ _ _ _ _ _ x _ 
        _ x x _ _ _ _ _ _ _ _ _ _ x x _ 
        _ _ _ x x x x x x x x x x _ _ _ 
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
    });
    command
}

fn frowning() -> Vec<u8> {
    let mut command = STATUS_HEADER.to_vec();

    command.extend(
        pixelart! {
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
            _ _ _ x x x x x x x x x x _ _ _
            _ x x _ _ _ _ _ _ _ _ _ _ x x _
            _ x _ _ _ _ _ _ _ _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ _ _ _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ x x _ _ _ _ _ x _
            _ x _ _ _ _ _ _ _ _ _ _ _ _ x _
            _ x x _ _ _ _ _ _ _ _ _ _ x x _
            _ _ _ x x x x x x x x x x _ _ _ 
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
        }
    );
    command
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }


    task_tracker.spawn_blocking(move || {
        let mut pixels = smiling();

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
                Ok(DisplayState::Paused) => pixels = paused(),
                Ok(DisplayState::Recording) => pixels = smiling(),
                Ok(DisplayState::WarningDetected) => pixels = frowning(),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                Err(e) => {
                    error!("error receiving framebuffer update message: {e}");
                }
            };

            // we write the status every second because it may have been overwritten through menu
            // navigation.
            if display_level != 0 {
                if let Err(e) = fs::write(OLED_PATH, &pixels) {
                    error!("failed to write to display: {e}");
                }
            }

            sleep(Duration::from_millis(1000));
        }
    });
}

#[test]
fn test_pixels() {
    let pixels = frowning();
    assert_eq!(pixels, [104, 40, 16, 16, 255, 255, 224, 7, 159, 249, 191, 253, 191, 253, 187, 221, 191, 253, 191, 253, 184, 29, 187, 221, 187, 221, 191, 253, 191, 253, 159, 249, 224, 7, 255, 255]);
}
