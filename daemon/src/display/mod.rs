use rayhunter::analysis::analyzer::EventType;
use serde::{Deserialize, Serialize};

#[cfg(any(
    feature = "device-orbic",
    feature = "device-tplink",
    feature = "device-wingtech"
))]
mod generic_framebuffer;

#[cfg(feature = "device-pinephone")]
pub mod headless;
#[cfg(feature = "device-orbic")]
pub mod orbic;
#[cfg(feature = "device-tmobile")]
pub mod tmobile;
#[cfg(feature = "device-tplink")]
pub mod tplink;
#[cfg(feature = "device-tplink")]
pub mod tplink_framebuffer;
#[cfg(feature = "device-tplink")]
pub mod tplink_onebit;
#[cfg(feature = "device-uz801")]
pub mod uz801;
#[cfg(feature = "device-wingtech")]
pub mod wingtech;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DisplayState {
    /// We're recording but no warning has been found yet.
    Recording,
    /// We're not recording.
    Paused,
    /// A non-informational event has been detected.
    ///
    /// Note that EventType::Informational is never sent through this. If it is, it's the same as
    /// Recording
    WarningDetected { event_type: EventType },
}
