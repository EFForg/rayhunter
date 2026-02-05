use log::warn;
use serde::{Deserialize, Serialize};

use rayhunter::Device;
use rayhunter::analysis::analyzer::AnalyzerConfig;

use crate::error::RayhunterError;
use crate::notifications::NotificationType;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub qmdl_store_path: String,
    pub port: u16,
    pub debug_mode: bool,
    pub device: Device,
    pub ui_level: u8,
    pub colorblind_mode: bool,
    pub key_input_mode: u8,
    pub ntfy_url: Option<String>,
    pub enabled_notifications: Vec<NotificationType>,
    pub analyzers: AnalyzerConfig,
    /// Enable HTTPS on https_port (generates self-signed cert on first use)
    pub https_enabled: bool,
    /// HTTPS port (only active when https_enabled = true)
    #[serde(default = "default_https_port")]
    pub https_port: u16,
    /// Custom hostnames/IPs to include in TLS certificate SANs.
    /// If empty, uses device-specific defaults. Can include IPs or DNS names.
    #[serde(default)]
    pub tls_hosts: Vec<String>,
}

fn default_https_port() -> u16 {
    8443
}

impl Default for Config {
    fn default() -> Self {
        Config {
            qmdl_store_path: "/data/rayhunter/qmdl".to_string(),
            port: 8080,
            debug_mode: false,
            device: Device::Orbic,
            ui_level: 1,
            colorblind_mode: false,
            key_input_mode: 0,
            analyzers: AnalyzerConfig::default(),
            ntfy_url: None,
            enabled_notifications: vec![NotificationType::Warning, NotificationType::LowBattery],
            https_enabled: false,
            https_port: default_https_port(),
            tls_hosts: Vec::new(),
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
        warn!("unable to read config file, using default config");
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
