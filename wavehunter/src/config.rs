use crate::error::WavehunterError;

use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigFile {
    qmdl_store_path: Option<String>,
    port: Option<u16>,
    readonly_mode: Option<bool>,
}

#[derive(Debug)]
pub struct Config {
    pub qmdl_store_path: String,
    pub port: u16,
    pub readonly_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_store_path: "/data/rayhunter/qmdl".to_string(),
            port: 8080,
            readonly_mode: false,
        }
    }
}

pub fn parse_config<P>(path: P) -> Result<Config, WavehunterError> where P: AsRef<std::path::Path> {
    let mut config = Config::default();
    if let Ok(config_file) = std::fs::read_to_string(&path) {
        let parsed_config: ConfigFile = toml::from_str(&config_file)
            .map_err(WavehunterError::ConfigFileParsingError)?;
        if let Some(path) = parsed_config.qmdl_store_path { config.qmdl_store_path = path }
        if let Some(port) = parsed_config.port { config.port = port }
        if let Some(readonly_mode) = parsed_config.readonly_mode { config.readonly_mode = readonly_mode }
    }
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
