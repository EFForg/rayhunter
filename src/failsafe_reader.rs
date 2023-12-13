mod hdlc;
mod diag;
mod diag_device;
mod log_codes;

use crate::diag_device::{FailsafeFileReader, DiagInterface};

fn main() -> diag_device::DiagResult<()> {
    // this should eventually be removed for prod
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/failsafe/file", args[0]);
    }
    let mut failsafe_reader = FailsafeFileReader::new(&args[1])?;

    loop {
        for msg in failsafe_reader.read_response()? {
            println!("msg: {:?}", msg);
        }
    }
}
