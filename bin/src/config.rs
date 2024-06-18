use crate::error::RayhunterError;

use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigFile {
    qmdl_store_path: Option<String>,
    port: Option<u16>,
    readonly_mode: Option<bool>,
    ui_level: Option<u8>,
}

#[derive(Debug)]
pub struct Config {
    pub qmdl_store_path: String,
    pub port: u16,
    pub readonly_mode: bool,
    pub ui_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_store_path: "/data/rayhunter/qmdl".to_string(),
            port: 8080,
            readonly_mode: false,
            ui_level: 1,
        }
    }
}

pub fn parse_config<P>(path: P) -> Result<Config, RayhunterError> where P: AsRef<std::path::Path> {
    let mut config = Config::default();
    if let Ok(config_file) = std::fs::read_to_string(&path) {
        let parsed_config: ConfigFile = toml::from_str(&config_file)
            .map_err(RayhunterError::ConfigFileParsingError)?;
        if let Some(path) = parsed_config.qmdl_store_path { config.qmdl_store_path = path }
        if let Some(port) = parsed_config.port { config.port = port }
        if let Some(readonly_mode) = parsed_config.readonly_mode { config.readonly_mode = readonly_mode }
        if let Some(ui_level) = parsed_config.ui_level { config.ui_level = ui_level }
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
