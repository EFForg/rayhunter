use wavehunter::diag_device::{DiagDevice, DiagResult};
use wavehunter::diag_reader::DiagReader;
use wavehunter::gsmtap_parser::GsmtapParser;
use wavehunter::pcap::PcapFile;

use log::debug;
use std::time::SystemTime;
use std::os::unix::fs;

fn main() -> DiagResult<()> {
    env_logger::init();

    let systime: u64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => systime=n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    let base_path = "/data/wavehunter/";
    let pcap_path = format!("{}{}-wavehunter.pcap", base_path, systime);
    let pcap_link = format!("{}wavehunter.pcap", base_path);
    let debug_path = format!("{}{}-wavehunter.debug", base_path, systime);
    // we need to remove the file first
    //fs::symlink(pcap_path.clone(), pcap_link)?;


    let mut dev = DiagDevice::new()?;
    dev.enable_debug_mode(debug_path)?;
    dev.config_logs()?;

    println!("The orca is hunting for stingrays...");
    println!("Writing pcaps to {}", pcap_path);

    let mut gsmtap_parser = GsmtapParser::new();
    let mut pcap_file = PcapFile::new(pcap_path).unwrap();
    pcap_file.write_iface_header().unwrap();


    loop {
        for msg in dev.read_response()? {
            debug!("msg: {:?}", msg);
            if let Some((timestamp, gsmtap_msg)) = gsmtap_parser.recv_message(msg).unwrap() {
                debug!("gsmtap_msg: {:?}", gsmtap_msg);
                pcap_file.write_gsmtap_message(gsmtap_msg, timestamp).unwrap();
            }
        }
    }
}
