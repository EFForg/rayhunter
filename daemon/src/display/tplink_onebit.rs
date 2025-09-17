/// Display module for the TP-Link M7350 oled one-bit display.
///
/// https://github.com/m0veax/tplink_m7350/tree/main/oled
use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use std::time::Duration;

pub const OLED_PATH: &str = "/sys/class/display/oled/oled_buffer";

// those coordinates were mainly chosen for a spot that doesn't get regularly updated by the main
// oledd service. otherwise we'd have to write to the display more than once per second to prevent
// the icon from flickering.
const STATUS_X: u8 = 104;
const STATUS_Y: u8 = 40;
const STATUS_W: u8 = 16;
const STATUS_H: u8 = 16;

macro_rules! pixel {
    (x) => {
        0
    };
    (_) => {
        1
    };
}

macro_rules! pixelart {
    (x=$x:expr, y=$y:expr, width=$width:expr, height=$height:expr; $($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt)*) => {{
        // one bit per pixel + 4 bytes for header
        const BUF_SIZE: usize = ($width as usize * $height as usize) / 8 + 4;
        const BUF_BYTES: [u8; BUF_SIZE] = [
            $x,
            $y,
            $width,
            $height,
            $(
                (pixel!($a) << 7 | pixel!($b) << 6 | pixel!($c) << 5 | pixel!($d) << 4 | pixel!($e) << 3 | pixel!($f) << 2 | pixel!($g) << 1 | pixel!($h)),
            )*
        ];

        &BUF_BYTES
    }}
}

const STATUS_PAUSED: &[u8] = pixelart! {
    x=STATUS_X, y=STATUS_Y, width=STATUS_W, height=STATUS_H;
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
};

const STATUS_SMILING: &[u8] = pixelart! {
    x=STATUS_X, y=STATUS_Y, width=STATUS_W, height=STATUS_H;
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
};

const STATUS_WARNING: &[u8] = pixelart! {
    x=STATUS_X, y=STATUS_Y, width=STATUS_W, height=STATUS_H;
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
};

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    shutdown_token: CancellationToken,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    task_tracker.spawn(async move {
        let mut pixels = STATUS_SMILING;

        loop {
            if shutdown_token.is_cancelled() {
                info!("received UI shutdown");
                break;
            }

            match ui_update_rx.try_recv() {
                Ok(DisplayState::Paused) => pixels = STATUS_PAUSED,
                Ok(DisplayState::Recording) => pixels = STATUS_SMILING,
                Ok(DisplayState::WarningDetected { .. }) => pixels = STATUS_WARNING,
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(e) => {
                    error!("error receiving framebuffer update message: {e}");
                }
            };

            // we write the status every second because it may have been overwritten through menu
            // navigation.
            if display_level != 0
                && let Err(e) = tokio::fs::write(OLED_PATH, pixels).await
            {
                error!("failed to write to display: {e}");
            }

            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });
}

#[test]
fn test_pixelart_macro() {
    assert_eq!(
        STATUS_WARNING,
        [
            104, 40, 16, 16, 255, 255, 224, 7, 159, 249, 191, 253, 190, 125, 190, 125, 190, 125,
            190, 125, 190, 125, 191, 253, 190, 125, 190, 125, 191, 253, 159, 249, 224, 7, 255, 255
        ]
    );
}
