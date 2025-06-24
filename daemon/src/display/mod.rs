mod generic_framebuffer;

pub mod orbic;
pub mod headless;
pub mod tmobile;
pub mod tplink;
pub mod tplink_framebuffer;
pub mod tplink_onebit;
pub mod wingtech;

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayState {
    Recording,
    Paused,
    WarningDetected,
}
