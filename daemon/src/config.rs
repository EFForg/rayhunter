use log::warn;
use serde::{Deserialize, Serialize};

use rayhunter::Device;
use rayhunter::analysis::analyzer::AnalyzerConfig;

use crate::error::RayhunterError;
use crate::notifications::NotificationType;
use crate::wifi::WPA_CONF_PATH;

/// The structure of a valid rayhunter configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct Config {
    /// Path to store QMDL files
    pub qmdl_store_path: String,
    /// Listening port
    pub port: u16,
    /// Debug mode
    pub debug_mode: bool,
    /// Internal device name
    pub device: Device,
    /// UI level
    pub ui_level: u8,
    /// Colorblind mode
    pub colorblind_mode: bool,
    /// Key input mode
    pub key_input_mode: u8,
    /// ntfy.sh URL
    pub ntfy_url: Option<String>,
    /// Vector containing the types of enabled notifications
    pub enabled_notifications: Vec<NotificationType>,
    /// Vector containing the list of enabled analyzers
    pub analyzers: AnalyzerConfig,
    pub min_space_to_start_recording_mb: u64,
    pub min_space_to_continue_recording_mb: u64,
    pub wifi_ssid: Option<String>,
    pub wifi_password: Option<String>,
    pub wifi_enabled: bool,
    pub block_ota_daemons: bool,
    pub dns_servers: Option<Vec<String>>,
    pub firewall_restrict_outbound: bool,
    pub firewall_allowed_ports: Option<Vec<u16>>,
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
            min_space_to_start_recording_mb: 1,
            min_space_to_continue_recording_mb: 1,
            wifi_ssid: None,
            wifi_password: None,
            wifi_enabled: false,
            block_ota_daemons: false,
            dns_servers: None,
            firewall_restrict_outbound: true,
            firewall_allowed_ports: None,
        }
    }
}

pub async fn parse_config<P>(path: P) -> Result<Config, RayhunterError>
where
    P: AsRef<std::path::Path>,
{
    let mut config = if let Ok(config_file) = tokio::fs::read_to_string(&path).await {
        toml::from_str(&config_file).map_err(RayhunterError::ConfigFileParsingError)?
    } else {
        warn!("unable to read config file, using default config");
        Config::default()
    };

    config.wifi_ssid = rayhunter::read_ssid_from_wpa_conf(WPA_CONF_PATH);
    config.wifi_password = None;

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
