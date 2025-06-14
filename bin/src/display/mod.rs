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

#[cfg(feature = "wingtech")]
mod wingtech;
#[cfg(feature = "wingtech")]
pub use wingtech::update_ui;

pub enum DisplayState {
    Recording,
    Paused,
    WarningDetected,
}
