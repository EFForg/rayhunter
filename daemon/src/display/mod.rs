use rayhunter::analysis::analyzer::EventType;
use serde::{Deserialize, Serialize};

mod generic_framebuffer;

pub mod headless;
pub mod orbic;
pub mod tmobile;
pub mod tplink;
pub mod tplink_framebuffer;
pub mod tplink_onebit;
pub mod uz801;
pub mod wingtech;
pub mod zte;

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
