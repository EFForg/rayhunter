use std::path::Path;

use crate::{
    battery::{BatteryState, is_plugged_in_from_file},
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
            Some('1') => Ok(10),
            Some('2') => Ok(25),
            Some('3') => Ok(50),
            Some('4') => Ok(75),
            Some('5') => Ok(100),
            _ => Err(RayhunterError::BatteryLevelParseError),
        }?,
        is_plugged_in: is_plugged_in_from_file(Path::new(PLUGGED_IN_STATE_FILE)).await?,
    })
}
