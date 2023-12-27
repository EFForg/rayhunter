use std::fs::File;

use wavehunter::qmdl::QmdlReader;
use wavehunter::diag_reader::DiagReader;
use wavehunter::gsmtap_parser::GsmtapParser;
use wavehunter::pcap::GsmtapPcapWriter;

use log::{debug, error};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        error!("Usage: {} /path/to/qmdl/file", args[0]);
        std::process::exit(1);
    }

    let qmdl_file = File::open(&args[1]).unwrap();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, None);

    let mut gsmtap_parser = GsmtapParser::new();
    let pcap_file = std::fs::File::options()
        .create(true)
        .write(true)
        .open("./wavehunter.pcap")
        .expect("error opening pcap file");
    let mut pcap_writer = GsmtapPcapWriter::new(pcap_file).unwrap();
    pcap_writer.write_iface_header().unwrap();

    loop {
        for maybe_msg in qmdl_reader.read_response().expect("error reading qmdl file") {
            match maybe_msg {
                Ok(msg) => {
                    debug!("msg: {:?}", msg);
                    let maybe_gsmtap_msg = gsmtap_parser.recv_message(msg).expect("error parsing gsmtap message");
                    if let Some((timestamp, gsmtap_msg)) = maybe_gsmtap_msg {
                        debug!("gsmtap_msg: {:?}", gsmtap_msg);
                        pcap_writer.write_gsmtap_message(gsmtap_msg, timestamp)
                            .expect("error writing pcap packet");
                    }
                },
                Err(e) => {
                    dbg!("error parsing message: {:?}", e);
                },
            }
        }
    }
}
