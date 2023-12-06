mod hdlc;
mod diag;
mod diag_device;
mod log_codes;

use crate::diag_device::DiagDevice;

fn main() -> std::io::Result<()> {
    let mut dev = DiagDevice::new().unwrap();
    dev.config_logs().unwrap();

    loop {
        println!("waiting for message...");
        for msg in dev.read_response().unwrap() {
            println!("msg: {:?}", msg);
        }
    }
}
