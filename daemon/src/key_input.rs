use log::{error, info};
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::diag::DiagDeviceCtrlMessage;

#[derive(Debug)]
enum Event {
    KeyDown,
    KeyUp,
}

const INPUT_EVENT_SIZE: usize = 32;

pub fn run_key_input_thread(
    task_tracker: &TaskTracker,
    config: &config::Config,
    diag_tx: Sender<DiagDeviceCtrlMessage>,
    cancellation_token: CancellationToken,
) {
    if config.key_input_mode == 0 {
        return;
    }

    task_tracker.spawn(async move {
        // Open the input device
        let mut file = match File::open("/dev/input/event0").await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open /dev/input/event0: {e}");
                return;
            }
        };

        let mut buffer = [0u8; INPUT_EVENT_SIZE];
        let mut last_keyup: Option<Instant> = None;
        let mut last_event_time: Option<Instant> = None;

        loop {
            tokio::select! {
               _ = cancellation_token.cancelled() => {
                    info!("received key input shutdown");
                    return;
                }
                result = file.read_exact(&mut buffer) => {
                    if let Err(e) = result {
                        error!("failed to read key input: {e}");
                        return;
                    }
                }
            }

            let event = parse_event(buffer);

            let now = Instant::now();

            // On orbic it was observed that pressing the power button can trigger many successive
            // events. Drop events that are too close together.
            if let Some(last_time) = last_event_time
                && now.duration_since(last_time) < Duration::from_millis(50)
            {
                last_event_time = Some(now);
                continue;
            }
            last_event_time = Some(now);

            match event {
                Event::KeyUp => {
                    if let Some(last_keyup_instant) = last_keyup {
                        let elapsed = now.duration_since(last_keyup_instant);

                        if elapsed >= Duration::from_millis(100)
                            && elapsed <= Duration::from_millis(800)
                        {
                            if let Err(e) = diag_tx.send(DiagDeviceCtrlMessage::StopRecording).await
                            {
                                error!("Failed to send StopRecording: {e}");
                            }
                            if let Err(e) =
                                diag_tx.send(DiagDeviceCtrlMessage::StartRecording).await
                            {
                                error!("Failed to send StartRecording: {e}");
                            }
                            last_keyup = None;
                            continue;
                        }
                    }

                    last_keyup = Some(now);
                }
                Event::KeyDown => {}
            }
        }
    });
}

fn parse_event(input: [u8; INPUT_EVENT_SIZE]) -> Event {
    if input[12] == 0 {
        Event::KeyUp
    } else {
        Event::KeyDown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_event_keydown_m7350_v5() {
        let input = [
            0x57, 0x6c, 0x09, 0x00, 0x7c, 0xfb, 0x03, 0x00, 0x01, 0x00, 0x74, 0x00, 0x01, 0x00,
            0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(matches!(parse_event(input), Event::KeyDown));
    }

    #[test]
    fn test_parse_event_keyup_m7350_v5() {
        let input = [
            0x57, 0x6c, 0x09, 0x00, 0x1b, 0x15, 0x05, 0x00, 0x01, 0x00, 0x74, 0x00, 0x00, 0x00,
            0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(matches!(parse_event(input), Event::KeyUp));
    }
}
