use std::path::Path;

use rayhunter::Device;
use serde::Serialize;

use crate::error::RayhunterError;

pub mod orbic;
pub mod tmobile;
pub mod wingtech;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum BatteryLevel {
    VeryLow,
    Low,
    Medium,
    High,
    Full,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub struct BatteryState {
    level: BatteryLevel,
    is_plugged_in: bool,
}

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

async fn get_level_from_percentage_file(path: &Path) -> Result<BatteryLevel, RayhunterError> {
    match tokio::fs::read_to_string(path)
        .await
        .map_err(RayhunterError::TokioError)?
        .trim_end()
        .parse()
    {
        Ok(0..=10) => Ok(BatteryLevel::VeryLow),
        Ok(11..=25) => Ok(BatteryLevel::Low),
        Ok(26..=50) => Ok(BatteryLevel::Medium),
        Ok(51..=75) => Ok(BatteryLevel::High),
        Ok(76..=100) => Ok(BatteryLevel::Full),
        _ => Err(RayhunterError::BatteryLevelParseError),
    }
}

pub async fn get_battery_status(device: &Device) -> Result<BatteryState, RayhunterError> {
    Ok(match device {
        Device::Orbic => orbic::get_battery_state().await?,
        Device::Wingtech => wingtech::get_battery_state().await?,
        Device::Tmobile => tmobile::get_battery_state().await?,
        _ => return Err(RayhunterError::FunctionNotSupportedForDeviceError),
    })
}
