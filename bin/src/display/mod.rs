#[cfg(feature = "device-orbic-rc400l")]
mod framebuffer;

#[cfg(feature = "device-tplink-m7350")]
mod tplink;
#[cfg(feature = "device-tplink-m7350")]
pub use tplink::update_ui;

#[cfg(feature = "device-orbic-rc400l")]
mod orbic;
#[cfg(feature = "device-orbic-rc400l")]
pub use orbic::update_ui;

pub enum DisplayState {
    Recording,
    Paused,
    WarningDetected,
    RecordingCBM,
}

#[cfg(all(feature = "device-orbic-rc400l", feature = "device-tplink-m7350"))]
compile_error!("cannot compile for many devices at once");

#[cfg(not(any(feature = "device-orbic-rc400l", feature = "device-tplink-m7350")))]
compile_error!("cannot compile for no device at all");
