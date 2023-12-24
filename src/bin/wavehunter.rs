use wavehunter::diag_device::{DiagDevice, DiagDeviceError};
use wavehunter::diag_reader::DiagReader;
use wavehunter::gsmtap_parser::{GsmtapParser, GsmtapParserError};
use wavehunter::pcap::{PcapFile, PcapFileError};

use log::debug;
use thiserror::Error;
use serde::Deserialize;
use toml;

#[derive(Error, Debug)]
enum WavehunterError {
    #[error("Missing config file: {0}")]
    MissingConfigFile(String),
    #[error("Config file parsing error: {0}")]
    ConfigFileParsingError(#[from] toml::de::Error),
    #[error("Pcap file initialization error: {0}")]
    PcapFileInitError(PcapFileError),
    #[error("Pcap file write error: {0}")]
    PcapFileWriteError(PcapFileError),
    #[error("Diag intialization error: {0}")]
    DiagInitError(DiagDeviceError),
    #[error("Diag read error: {0}")]
    DiagReadError(DiagDeviceError),
    #[error("GSMTAP parsing error: {0}")]
    GsmtapParsingError(GsmtapParserError),
}

#[derive(Deserialize)]
struct ConfigFile {
    qmdl_path: Option<String>,
    pcap_path: Option<String>,
}

#[derive(Debug)]
struct Config {
    qmdl_path: String,
    pcap_path: String,
}

fn parse_config<P>(path: P) -> Result<Config, WavehunterError> where P: AsRef<std::path::Path> {
    let config_file = std::fs::read_to_string(&path)
        .map_err(|_| WavehunterError::MissingConfigFile(format!("{:?}", path.as_ref())))?;
    let parsed_config: ConfigFile = toml::from_str(&config_file)
        .map_err(WavehunterError::ConfigFileParsingError)?;
    Ok(Config {
        qmdl_path: parsed_config.qmdl_path.unwrap_or("./wavehunter.qmdl".to_string()),
        pcap_path: parsed_config.pcap_path.unwrap_or("./wavehunter.pcap".to_string()),
    })
}

fn main() -> Result<(), WavehunterError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/config/file", args[0]);
        std::process::exit(1);
    }

    let config = parse_config(&args[1])?;

    let mut dev = DiagDevice::new(&config.qmdl_path)
        .map_err(WavehunterError::DiagInitError)?;
    dev.config_logs()
        .map_err(WavehunterError::DiagInitError)?;

    println!("The orca is hunting for stingrays...");

    let mut gsmtap_parser = GsmtapParser::new();
    // We are going to want to add a timestamp to this pcap file eventually
    let mut pcap_file = PcapFile::new(&config.pcap_path)
        .map_err(WavehunterError::PcapFileInitError)?;
    pcap_file.write_iface_header()
        .map_err(WavehunterError::PcapFileWriteError)?;

    loop {
        for msg in dev.read_response().map_err(WavehunterError::DiagReadError)? {
            debug!("msg: {:?}", msg);
            if let Some((timestamp, gsmtap_msg)) = gsmtap_parser.recv_message(msg).map_err(WavehunterError::GsmtapParsingError)? {
                debug!("gsmtap_msg: {:?}", gsmtap_msg);
                pcap_file.write_gsmtap_message(gsmtap_msg, timestamp)
                    .map_err(WavehunterError::PcapFileWriteError)?;
            }
        }
    }
}
