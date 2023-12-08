mod hdlc;
mod diag;
mod diag_device;
mod log_codes;

use crate::diag_device::DiagDevice;

fn main() -> diag_device::DiagResult<()> {
    // this should eventually be removed for prod
    env_logger::init();

    let mut dev = DiagDevice::new()?;
    dev.config_logs()?;

    loop {
        for msg in dev.read_response()? {
            println!("msg: {:?}", msg);
        }
    }
}
