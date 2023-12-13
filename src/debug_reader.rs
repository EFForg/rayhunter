mod hdlc;
mod diag;
mod diag_device;
mod diag_reader;
mod debug_file;
mod log_codes;

use crate::debug_file::DebugFileReader;
use crate::diag_reader::DiagReader;

fn main() -> diag_device::DiagResult<()> {
    // this should eventually be removed for prod
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/debug/file", args[0]);
        std::process::exit(1);
    }
    let mut debug_reader = DebugFileReader::new(&args[1])?;

    loop {
        for msg in debug_reader.read_response()? {
            println!("msg: {:?}", msg);
        }
    }
}
