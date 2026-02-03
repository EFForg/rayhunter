//! Global clock offset for adjusting timestamps.
//!
//! This module provides a global clock offset that can be used to adjust
//! timestamps when the device's system clock is incorrect. The offset is
//! stored in memory and is not persisted across restarts.

use chrono::{DateTime, Local, TimeDelta};
use std::sync::RwLock;

static CLOCK_OFFSET: RwLock<TimeDelta> = RwLock::new(TimeDelta::zero());

/// Get the current clock offset.
fn get_offset() -> TimeDelta {
    *CLOCK_OFFSET.read().unwrap()
}

/// Set the clock offset.
pub fn set_offset(offset: TimeDelta) {
    *CLOCK_OFFSET.write().unwrap() = offset;
}

/// Get the current adjusted time (system time + offset).
pub fn get_adjusted_now() -> DateTime<Local> {
    Local::now() + get_offset()
}
