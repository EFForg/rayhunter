use std::path::Path;

use crate::{
    battery::{BatteryState, get_level_from_percentage_file},
    error::RayhunterError,
};

// Confirmed via /sys/class/power_supply/ on-device: a standard Linux
// power_supply interface for the bq27520 fuel-gauge chip, reporting a plain
// 0-100 capacity percentage directly (no coarse level mapping needed, unlike
// the Orbic).
const BATTERY_CAPACITY_FILE: &str = "/sys/class/power_supply/bq27520-battery/capacity";
const BATTERY_STATUS_FILE: &str = "/sys/class/power_supply/bq27520-battery/status";

pub async fn get_battery_state() -> Result<BatteryState, RayhunterError> {
    let status = tokio::fs::read_to_string(BATTERY_STATUS_FILE)
        .await
        .map_err(RayhunterError::TokioError)?;

    Ok(BatteryState {
        level: get_level_from_percentage_file(Path::new(BATTERY_CAPACITY_FILE)).await?,
        // Standard power_supply status values: "Charging", "Discharging",
        // "Not charging", "Full", "Unknown". Only "Discharging" means
        // running on battery alone.
        is_plugged_in: status.trim() != "Discharging",
    })
}
