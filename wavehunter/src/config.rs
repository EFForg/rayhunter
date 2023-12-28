use crate::error::WavehunterError;

use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct ConfigFile {
    qmdl_path: Option<String>,
    port: Option<u16>,
}

#[derive(Debug)]
pub struct Config {
    pub qmdl_path: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_path: "./wavehunter.qmdl".to_string(),
            port: 8080,
        }
    }
}

pub fn parse_config<P>(path: P) -> Result<Config, WavehunterError> where P: AsRef<std::path::Path> {
    let config_file = std::fs::read_to_string(&path)
        .map_err(|_| WavehunterError::MissingConfigFile(format!("{:?}", path.as_ref())))?;
    let parsed_config: ConfigFile = toml::from_str(&config_file)
        .map_err(WavehunterError::ConfigFileParsingError)?;
    let mut config = Config::default();
    parsed_config.qmdl_path.map(|path| config.qmdl_path = path);
    parsed_config.port.map(|path| config.port = path);
    Ok(config)
}

pub struct Args {
    pub config_path: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/config/file", args[0]);
        std::process::exit(1);
    }
    Args {
        config_path: args[1].clone(),
    }
}
