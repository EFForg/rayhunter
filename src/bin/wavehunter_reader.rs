use wavehunter::debug_file::DebugFileReader;
use wavehunter::diag_reader::DiagReader;
use wavehunter::diag_device::{DiagResult, DiagDeviceError};
use wavehunter::gsmtap_parser::GsmtapParser;
use wavehunter::pcap::PcapFile;

use log::{debug, error};

fn main() -> DiagResult<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        error!("Usage: {} /path/to/debug/file", args[0]);
        std::process::exit(1);
    }
    let mut debug_reader = DebugFileReader::new(&args[1])?;

    let mut gsmtap_parser = GsmtapParser::new();
    let mut pcap_file = PcapFile::new("./wavehunter.pcap").unwrap();
    pcap_file.write_iface_header().unwrap();

    loop {
        match debug_reader.read_response() {
            Ok(msgs) => {
                for msg in msgs {
                    debug!("msg: {:?}", msg);
                    if let Some((timestamp, gsmtap_msg)) = gsmtap_parser.recv_message(msg).unwrap() {
                        debug!("gsmtap_msg: {:?}", gsmtap_msg);
                        pcap_file.write_gsmtap_message(gsmtap_msg, timestamp).unwrap();
                    }
                }
            },
            Err(DiagDeviceError::IO(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Reached end of debug file, exiting...");
                std::process::exit(0);
            },
            Err(err) => panic!("Error reading debug file {}", err),
        }
    }
}
