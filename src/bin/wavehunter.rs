use wavehunter::diag_device::{DiagDevice, DiagResult};
use wavehunter::diag_reader::DiagReader;

fn main() -> DiagResult<()> {
    // this should eventually be removed for prod
    env_logger::init();

    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open("/dev/diag")?;
    let mut dev = DiagDevice::new(&file)?;
    dev.enable_debug_mode("/data/wavehunter-debug")?;
    dev.config_logs()?;

    loop {
        for msg in dev.read_response()? {
            println!("msg: {:?}", msg);
        }
    }
}
