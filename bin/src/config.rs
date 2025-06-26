use serde::{Deserialize, Serialize};

use rayhunter::analysis::analyzer::AnalyzerConfig;

use crate::error::RayhunterError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub qmdl_store_path: String,
    pub port: u16,
    pub debug_mode: bool,
    pub ui_level: u8,
    pub enable_dummy_analyzer: bool,
    pub colorblind_mode: bool,
    pub key_input_mode: u8,
    pub ntfy_topic: Option<String>,
    pub analyzers: AnalyzerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_store_path: "/data/rayhunter/qmdl".to_string(),
            port: 8080,
            debug_mode: false,
            ui_level: 1,
            enable_dummy_analyzer: false,
            colorblind_mode: false,
            key_input_mode: 0,
            analyzers: AnalyzerConfig::default(),
            ntfy_topic: None,
        }
    }
}

pub async fn parse_config<P>(path: P) -> Result<Config, RayhunterError>
where
    P: AsRef<std::path::Path>,
{
    if let Ok(config_file) = tokio::fs::read_to_string(&path).await {
        Ok(toml::from_str(&config_file).map_err(RayhunterError::ConfigFileParsingError)?)
    } else {
        Ok(Config::default())
    }
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
