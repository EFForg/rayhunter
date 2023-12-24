use wavehunter::qmdl::QmdlFileReader;
use wavehunter::diag_reader::DiagReader;
use wavehunter::gsmtap_parser::GsmtapParser;
use wavehunter::pcap::PcapFile;

use log::{debug, error};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        error!("Usage: {} /path/to/qmdl/file", args[0]);
        std::process::exit(1);
    }
    let mut qmdl_reader = QmdlFileReader::new(&args[1]).unwrap();

    let mut gsmtap_parser = GsmtapParser::new();
    let mut pcap_file = PcapFile::new("./wavehunter.pcap").unwrap();
    pcap_file.write_iface_header().unwrap();

    loop {
        match qmdl_reader.read_response() {
            Ok(msgs) => {
                for msg in msgs {
                    debug!("msg: {:?}", msg);
                    if let Some((timestamp, gsmtap_msg)) = gsmtap_parser.recv_message(msg).unwrap() {
                        debug!("gsmtap_msg: {:?}", gsmtap_msg);
                        pcap_file.write_gsmtap_message(gsmtap_msg, timestamp).unwrap();
                    }
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Reached end of QMDL file, exiting...");
                std::process::exit(0);
            },
            Err(err) => panic!("Error reading QMDL file {}", err),
        }
    }
}
