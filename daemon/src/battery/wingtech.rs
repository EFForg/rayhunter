use std::path::Path;

use crate::{
    battery::{BatteryState, get_level_from_percentage_file, is_plugged_in_from_file},
    error::RayhunterError,
};

const BATTERY_LEVEL_FILE: &str =
    "/sys/devices/78b7000.i2c/i2c-3/3-0063/power_supply/cw2017-bat/capacity";
const PLUGGED_IN_STATE_FILE: &str = "/sys/devices/8a00000.ssusb/power_supply/usb/online";

pub async fn get_battery_state() -> Result<BatteryState, RayhunterError> {
    Ok(BatteryState {
        level: get_level_from_percentage_file(Path::new(BATTERY_LEVEL_FILE)).await?,
        is_plugged_in: is_plugged_in_from_file(Path::new(PLUGGED_IN_STATE_FILE)).await?,
    })
}
