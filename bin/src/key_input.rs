use log::error;
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
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
) {
    if config.key_input_mode == 0 {
        return;
    }

    task_tracker.spawn(async move {
        // Open the input device
        let mut file = match File::open("/dev/input/event0").await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open /dev/input/event0: {}", e);
                return;
            }
        };

        let mut buffer = [0u8; INPUT_EVENT_SIZE];
        let mut last_keyup: Option<Instant> = None;

        loop {
            if let Err(e) = file.read_exact(&mut buffer).await {
                error!("failed to read key input: {}", e);
                return;
            }

            let event = parse_event(buffer);

            match event {
                Event::KeyUp => {
                    if last_keyup.is_some()
                        && last_keyup.unwrap().elapsed() < Duration::from_millis(500)
                    {
                        if let Err(e) = diag_tx.send(DiagDeviceCtrlMessage::StopRecording).await {
                            error!("Failed to send StopRecording: {}", e);
                        }
                        if let Err(e) = diag_tx.send(DiagDeviceCtrlMessage::StartRecording).await {
                            error!("Failed to send StartRecording: {}", e);
                        }
                        last_keyup = None;
                    } else {
                        last_keyup = Some(Instant::now());
                    }
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
            0x57, 0x6c, 0x09, 0x00, 0x7c, 0xfb, 0x03, 0x00,
            0x01, 0x00, 0x74, 0x00, 0x01, 0x00, 0x00, 0x00,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(matches!(parse_event(input), Event::KeyDown));
    }

    #[test]
    fn test_parse_event_keyup_m7350_v5() {
        let input = [
            0x57, 0x6c, 0x09, 0x00, 0x1b, 0x15, 0x05, 0x00,
            0x01, 0x00, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(matches!(parse_event(input), Event::KeyUp));
    }
}
