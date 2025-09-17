/// Display module for Uz801, light LEDs on the front of the device.
/// DisplayState::Recording => Green LED is solid.
/// DisplayState::Paused => Signal LED is solid blue (wifi LED).
/// DisplayState::WarningDetected => Signal LED is solid red.
use log::{error, info};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

macro_rules! led {
    ($l:expr) => {{ format!("/sys/class/leds/{}/brightness", $l) }};
}

async fn led_on(path: String) {
    tokio::fs::write(&path, "1").await.ok();
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
                        led_off(led!("red")).await;
                        led_off(led!("green")).await;
                        led_on(led!("wifi")).await;
                    }
                    DisplayState::Recording => {
                        led_off(led!("red")).await;
                        led_off(led!("wifi")).await;
                        led_on(led!("green")).await;
                    }
                    DisplayState::WarningDetected { .. } => {
                        led_off(led!("green")).await;
                        led_off(led!("wifi")).await;
                        led_on(led!("red")).await;
                    }
                }
                last_state = state;
                last_update = now;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}
