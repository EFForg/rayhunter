use std::path::Path;

use crate::{
    battery::{BatteryLevel, BatteryState, is_plugged_in_from_file},
    error::RayhunterError,
};

const BATTERY_LEVEL_FILE: &str = "/sys/kernel/chg_info/level";
const PLUGGED_IN_STATE_FILE: &str = "/sys/kernel/chg_info/chg_en";

pub async fn get_battery_state() -> Result<BatteryState, RayhunterError> {
    Ok(BatteryState {
        level: match tokio::fs::read_to_string(&BATTERY_LEVEL_FILE)
            .await
            .map_err(RayhunterError::TokioError)?
            .chars()
            .next()
        {
            Some('1') => Ok(BatteryLevel::VeryLow),
            Some('2') => Ok(BatteryLevel::Low),
            Some('3') => Ok(BatteryLevel::Medium),
            Some('4') => Ok(BatteryLevel::High),
            Some('5') => Ok(BatteryLevel::Full),
            _ => Err(RayhunterError::BatteryLevelParseError),
        }?,
        is_plugged_in: is_plugged_in_from_file(Path::new(PLUGGED_IN_STATE_FILE)).await?,
    })
}
