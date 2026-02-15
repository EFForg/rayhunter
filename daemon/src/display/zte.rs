/// Display module for ZTE MF920V, light LEDs on the front of the device.
///
/// We use the network LED (bottom right LED that shows bars), as this one has full RGB colors.
///
/// DisplayState::Recording => Network LED is solid green (or blue in colorblind mode).
/// DisplayState::Paused => Network LED is solid white (all colors on).
/// DisplayState::WarningDetected => Network LED is solid red.
use log::{error, info};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

macro_rules! led {
    ($l:expr) => {{ format!("/sys/class/leds/led:{}/brightness", $l) }};
}

async fn led_on(path: String) {
    tokio::fs::write(&path, "255").await.ok();
}

async fn led_off(path: String) {
    tokio::fs::write(&path, "0").await.ok();
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    shutdown_token: CancellationToken,
    mut ui_update_rx: mpsc::Receiver<DisplayState>,
) {
    let mut invisible: bool = false;
    if config.ui_level == 0 {
        info!("Invisible mode, not spawning UI.");
        invisible = true;
    }
    let colorblind_mode = config.colorblind_mode;
    task_tracker.spawn(async move {
        let mut state = DisplayState::Recording;
        let mut last_state = DisplayState::Paused;
        let mut last_update = std::time::Instant::now();

        loop {
            if shutdown_token.is_cancelled() {
                info!("received UI shutdown");
                break;
            }
            match ui_update_rx.try_recv() {
                Ok(new_state) => state = new_state,
                Err(mpsc::error::TryRecvError::Empty) => {}
                Err(e) => error!("error receiving ui update message: {e}"),
            };

            // Update LEDs if state changed or if 5 seconds have passed since last update
            let now = std::time::Instant::now();
            let should_update = !invisible
                && (state != last_state
                    || now.duration_since(last_update) >= Duration::from_secs(5));

            if should_update {
                match state {
                    DisplayState::Paused => {
                        // White = all colors on
                        led_on(led!("net_blue")).await;
                        led_on(led!("net_red")).await;
                        led_on(led!("net_green")).await;
                    }
                    DisplayState::Recording => {
                        led_off(led!("net_red")).await;
                        if colorblind_mode {
                            led_off(led!("net_green")).await;
                            led_on(led!("net_blue")).await;
                        } else {
                            led_off(led!("net_blue")).await;
                            led_on(led!("net_green")).await;
                        }
                    }
                    DisplayState::WarningDetected { .. } => {
                        led_off(led!("net_green")).await;
                        led_off(led!("net_blue")).await;
                        led_on(led!("net_red")).await;
                    }
                }
                last_state = state;
                last_update = now;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}
