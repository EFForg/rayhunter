use std::path::Path;
use std::time::Duration;

use log::{info, warn};
use rayhunter::Device;
use serde::Serialize;
use tokio::select;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    error::RayhunterError,
    notifications::{Notification, NotificationType},
};

#[cfg(feature = "device-orbic")]
pub mod orbic;
#[cfg(feature = "device-tmobile")]
pub mod tmobile;
#[cfg(feature = "device-tplink")]
pub mod tplink;
#[cfg(feature = "device-wingtech")]
pub mod wingtech;

const LOW_BATTERY_LEVEL: u8 = 10;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub struct BatteryState {
    level: u8,
    is_plugged_in: bool,
}

#[allow(dead_code)]
async fn is_plugged_in_from_file(path: &Path) -> Result<bool, RayhunterError> {
    match tokio::fs::read_to_string(path)
        .await
        .map_err(RayhunterError::TokioError)?
        .chars()
        .next()
    {
        Some('0') => Ok(false),
        Some('1') => Ok(true),
        _ => Err(RayhunterError::BatteryPluggedInStatusParseError),
    }
}

#[allow(dead_code)]
async fn get_level_from_percentage_file(path: &Path) -> Result<u8, RayhunterError> {
    tokio::fs::read_to_string(path)
        .await
        .map_err(RayhunterError::TokioError)?
        .trim_end()
        .parse()
        .or(Err(RayhunterError::BatteryLevelParseError))
}

pub async fn get_battery_status(device: &Device) -> Result<BatteryState, RayhunterError> {
    match device {
        #[cfg(feature = "device-orbic")]
        Device::Orbic => Ok(orbic::get_battery_state().await?),
        #[cfg(feature = "device-wingtech")]
        Device::Wingtech => Ok(wingtech::get_battery_state().await?),
        #[cfg(feature = "device-tmobile")]
        Device::Tmobile => Ok(tmobile::get_battery_state().await?),
        #[cfg(feature = "device-tplink")]
        Device::Tplink => Ok(tplink::get_battery_state().await?),
        _ => Err(RayhunterError::FunctionNotSupportedForDeviceError),
    }
}

pub fn run_battery_notification_worker(
    task_tracker: &TaskTracker,
    device: Device,
    notification_channel: tokio::sync::mpsc::Sender<Notification>,
    shutdown_token: CancellationToken,
) {
    task_tracker.spawn(async move {
        // Don't send a notification initially if the device starts at a low battery level.
        let mut triggered = match get_battery_status(&device).await {
            Err(RayhunterError::FunctionNotSupportedForDeviceError) => {
                info!("Battery status not supported for this device, disabling battery notifications");
                return;
            }
            Err(e) => {
                warn!("Failed to get battery status: {e}");
                true
            }
            Ok(status) => status.level <= LOW_BATTERY_LEVEL,
        };

        loop {
            select! {
                _ = shutdown_token.cancelled() => break,
                _ = tokio::time::sleep(Duration::from_secs(15)) => {}
            }

            let status = match get_battery_status(&device).await {
                Err(RayhunterError::FunctionNotSupportedForDeviceError) => {
                    info!("Battery status not supported for this device, disabling battery notifications");
                    break;
                }
                Err(e) => {
                    warn!("Failed to get battery status: {e}");
                    continue;
                }
                Ok(status) => status,
            };

            // To avoid flapping, if the notification has already been triggered
            // wait until the device has been plugged in and the battery level
            // is high enough to re-enable notifications.
            if triggered && status.is_plugged_in && status.level > LOW_BATTERY_LEVEL {
                triggered = false;
                continue;
            }
            if !triggered && !status.is_plugged_in && status.level <= LOW_BATTERY_LEVEL {
                notification_channel
                    .send(Notification::new(
                        NotificationType::LowBattery,
                        "Rayhunter's battery is low".to_string(),
                        None,
                    ))
                    .await
                    .expect("Failed to send to notification channel");
                triggered = true;
            }
        }
    });
}
