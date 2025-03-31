mod generic_framebuffer;

#[cfg(feature = "tplink")]
mod tplink;
#[cfg(feature = "tplink")]
mod tplink_framebuffer;
#[cfg(feature = "tplink")]
mod tplink_onebit;

#[cfg(feature = "tplink")]
pub use tplink::update_ui;

#[cfg(feature = "orbic")]
mod orbic;
#[cfg(feature = "orbic")]
pub use orbic::update_ui;

pub enum DisplayState {
    Recording,
    Paused,
    WarningDetected,
    RecordingCBM,
}

#[cfg(all(feature = "orbic", feature = "tplink"))]
compile_error!("cannot compile for many devices at once");

#[cfg(not(any(feature = "orbic", feature = "tplink")))]
compile_error!("cannot compile for no device at all");
