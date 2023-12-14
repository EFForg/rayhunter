use wavehunter::debug_file::DebugFileReader;
use wavehunter::diag_reader::DiagReader;
use wavehunter::diag_device::DiagResult;

fn main() -> DiagResult<()> {
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
