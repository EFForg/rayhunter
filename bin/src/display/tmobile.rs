/// Display module for Tmobile TMOHS1, blink LEDs on the front of the device.
/// DisplayState::Recording => Signal LED slowly blinks blue.
/// DisplayState::Paused => WiFi LED blinks white.
/// DisplayState::WarningDetected => Signal LED slowly blinks red.
use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use std::fs::write;
use std::thread::sleep;
use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

macro_rules! led {
    ($l:expr) => {{
        format!("/sys/class/leds/led:{}/blink", $l)
    }};
}

fn start_blinking(path: String) {
    write(&path, "1").ok();
}

fn stop_blinking(path: String) {
    write(&path, "0").ok();
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: mpsc::Receiver<DisplayState>,
) {
    let mut invisible: bool = false;
    if config.ui_level == 0 {
        info!("Invisible mode, not spawning UI.");
        invisible = true;
    }
    task_tracker.spawn_blocking(move || {
        let mut state = DisplayState::Recording;
        let mut last_state = DisplayState::Paused;

        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                }
                Err(oneshot::error::TryRecvError::Empty) => {}
                Err(e) => panic!("error receiving shutdown message: {e}"),
            }
            match ui_update_rx.try_recv() {
                Ok(new_state) => state = new_state,
                Err(mpsc::error::TryRecvError::Empty) => {}
                Err(e) => error!("error receiving ui update message: {e}"),
            };
            if invisible || state == last_state {
                sleep(Duration::from_secs(1));
                continue;
            }
            match state {
                DisplayState::Paused => {
                    stop_blinking(led!("signal_blue"));
                    stop_blinking(led!("signal_red"));
                    start_blinking(led!("wlan_white"));
                }
                DisplayState::Recording => {
                    stop_blinking(led!("wlan_white"));
                    stop_blinking(led!("signal_red"));
                    start_blinking(led!("signal_blue"));
                }
                DisplayState::WarningDetected => {
                    stop_blinking(led!("wlan_white"));
                    stop_blinking(led!("signal_blue"));
                    start_blinking(led!("signal_red"));
                }
            }
            last_state = state;
            sleep(Duration::from_secs(1));
        }
    });
}
