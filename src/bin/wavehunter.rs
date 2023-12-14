use wavehunter::diag_device::{DiagDevice, DiagResult};
use wavehunter::diag_reader::DiagReader;
use wavehunter::gsmtap_parser::GsmtapParser;
use wavehunter::pcap::PcapFile;

use log::debug;

fn main() -> DiagResult<()> {
    env_logger::init();

    let mut dev = DiagDevice::new()?;
    dev.enable_debug_mode("/data/wavehunter-debug")?;
    dev.config_logs()?;

    let mut gsmtap_parser = GsmtapParser::new();
    let mut pcap_file = PcapFile::new("/data/wavehunter.pcap").unwrap();
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
