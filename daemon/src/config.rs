use log::warn;
use serde::{Deserialize, Serialize};

use rayhunter::Device;
use rayhunter::analysis::analyzer::AnalyzerConfig;

use crate::error::RayhunterError;
use crate::notifications::NotificationType;

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
    /// Minimum disk space required to start a recording
    pub min_space_to_start_recording_mb: u64,
    /// Minimum disk space required to continue a recording
    pub min_space_to_continue_recording_mb: u64,
    /// GPS mode: 0=Disabled, 1=Fixed coordinates, 2=API endpoint
    pub gps_mode: u8,
    /// Fixed latitude used when gps_mode=1
    pub gps_fixed_latitude: Option<f64>,
    /// Fixed longitude used when gps_mode=1
    pub gps_fixed_longitude: Option<f64>,
    /// Wifi client SSID
    pub wifi_ssid: Option<String>,
    /// Wifi client password
    pub wifi_password: Option<String>,
    /// Wifi security type (wpa_psk or sae)
    pub wifi_security: Option<wifi_station::SecurityType>,
    /// Wifi client mode
    pub wifi_enabled: bool,
    /// Vector containing wifi client DNS servers
    pub dns_servers: Option<Vec<String>>,
    /// Wifi client firewall mode
    pub firewall_restrict_outbound: bool,
    /// Vector containing additional wifi client firewall ports to open
    pub firewall_allowed_ports: Option<Vec<u16>>,
    /// Optional WebDAV upload configuration. When unset, no upload worker runs.
    pub webdav: Option<WebdavConfig>,
}

/// Configuration for uploading finished QMDL recordings to a WebDAV server.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct WebdavConfig {
    /// WebDAV server base URL, e.g. "https://example.com/remote.php/files/untitaker/my-subfolder/"
    pub url: String,
    /// Optional username for HTTP Basic auth
    pub username: Option<String>,
    /// Optional password for HTTP Basic auth
    pub password: Option<String>,
    /// Timeout (in seconds) for each upload request
    pub upload_timeout_secs: u64,
    /// How often (in seconds) the worker scans for entries to upload
    pub poll_interval_secs: u64,
    /// Minimum age (in seconds) an entry must have before it becomes eligible for upload
    pub min_age_secs: i64,
    /// Delete the file locally after a successful upload
    pub delete_on_upload: bool,
}

impl Default for WebdavConfig {
    fn default() -> Self {
        WebdavConfig {
            url: String::new(),
            username: None,
            password: None,
            upload_timeout_secs: 300,
            poll_interval_secs: 3600,
            min_age_secs: 86400,
            delete_on_upload: false,
        }
    }
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
            gps_mode: 0,
            gps_fixed_latitude: None,
            gps_fixed_longitude: None,
            wifi_ssid: None,
            wifi_password: None,
            wifi_security: None,
            wifi_enabled: false,
            dns_servers: None,
            firewall_restrict_outbound: true,
            firewall_allowed_ports: None,
            webdav: None,
        }
    }
}

impl Config {
    pub fn wifi_config(&self) -> wifi_station::WifiConfig {
        let (wpa_bin, hostapd_conf, ctrl_interface) = match self.device {
            Device::Tmobile | Device::Wingtech => (
                Some("/usr/sbin/wpa_supplicant".into()),
                Some("/data/configs/hostapd.conf".into()),
                None,
            ),
            Device::Uz801 => (
                Some("/system/bin/wpa_supplicant".into()),
                Some("/data/misc/wifi/hostapd.conf".into()),
                Some("/data/misc/wifi/sockets".into()),
            ),
            _ => (None, None, None),
        };
        wifi_station::WifiConfig {
            wifi_enabled: self.wifi_enabled,
            dns_servers: self.dns_servers.clone(),
            wifi_ssid: self.wifi_ssid.clone(),
            wifi_password: self.wifi_password.clone(),
            security_type: self.wifi_security,
            wpa_supplicant_bin: wpa_bin.or_else(|| resolve_bin("wpa_supplicant")),
            hostapd_conf,
            ctrl_interface,
            udhcpc_hook_path: Some("/data/rayhunter/udhcpc-hook.sh".into()),
            dhcp_lease_path: Some("/data/rayhunter/dhcp_lease".into()),
            wpa_conf_path: Some("/data/rayhunter/wpa_sta.conf".into()),
            iw_bin: resolve_bin("iw"),
            udhcpc_bin: resolve_bin("udhcpc"),
            crash_log_dir: Some("/data/rayhunter/crash-logs".into()),
            wakelock_name: Some("rayhunter".into()),
        }
    }
}

fn resolve_bin(name: &str) -> Option<String> {
    let local = format!("/data/rayhunter/bin/{name}");
    if std::path::Path::new(&local).exists() {
        return Some(local);
    }
    None
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

    if let Some((ssid, security)) =
        wifi_station::read_network_from_wpa_conf("/data/rayhunter/wpa_sta.conf")
    {
        config.wifi_ssid = Some(ssid);
        config.wifi_security = Some(security);
    } else {
        config.wifi_ssid = None;
        config.wifi_security = None;
    }
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
