use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub qmdl_store_path: String,
    pub port: u16,
    pub debug_mode: bool,
    pub device: String,
    pub ui_level: u8,
    pub colorblind_mode: bool,
    pub key_input_mode: u8,
    pub ntfy_url: Option<String>,
    pub enabled_notifications: Vec<String>,
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
        Self {
            qmdl_store_path: "/data/rayhunter/qmdl".into(),
            port: 8080,
            debug_mode: false,
            device: "orbic".into(),
            ui_level: 1,
            colorblind_mode: false,
            key_input_mode: 0,
            ntfy_url: None,
            enabled_notifications: vec!["Warning".into(), "LowBattery".into()],
            analyzers: AnalyzerConfig::default(),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AnalyzerConfig {
    pub diagnostic_analyzer: bool,
    pub connection_redirect_2g_downgrade: bool,
    pub lte_sib6_and_7_downgrade: bool,
    pub null_cipher: bool,
    pub nas_null_cipher: bool,
    pub incomplete_sib: bool,
    pub test_analyzer: bool,
    pub imsi_requested: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            diagnostic_analyzer: true,
            connection_redirect_2g_downgrade: true,
            lte_sib6_and_7_downgrade: true,
            null_cipher: true,
            nas_null_cipher: true,
            incomplete_sib: true,
            test_analyzer: false,
            imsi_requested: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SystemStats {
    pub disk_stats: DiskStats,
    pub memory_stats: MemoryStats,
    pub runtime_metadata: RuntimeMetadata,
    pub battery_status: Option<BatteryState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiskStats {
    pub partition: String,
    pub total_size: String,
    pub used_size: String,
    pub available_size: String,
    pub used_percent: String,
    pub mounted_on: String,
    pub available_bytes: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemoryStats {
    pub total: String,
    pub used: String,
    pub free: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeMetadata {
    pub rayhunter_version: String,
    pub system_os: String,
    pub arch: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatteryState {
    pub level: u8,
    pub is_plugged_in: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestStats {
    pub entries: Vec<ManifestEntry>,
    pub current_entry: Option<ManifestEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestEntry {
    pub name: String,
    pub start_time: String,
    pub last_message_time: Option<String>,
    pub qmdl_size_bytes: usize,
    pub rayhunter_version: Option<String>,
    pub system_os: Option<String>,
    pub arch: Option<String>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TimeResponse {
    pub system_time: String,
    pub adjusted_time: String,
    pub offset_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnalysisStatus {
    pub queued: Vec<String>,
    pub running: Option<String>,
    pub finished: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiStatus {
    pub state: String,
    pub ssid: Option<String>,
    pub ip: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal_dbm: i32,
    pub security: String,
}
