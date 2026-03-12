use crate::types::Config;

const WIFI_CAPABLE_DEVICES: &[&str] = &["orbic", "moxee"];

pub struct Capabilities {
    pub http: bool,
    pub shell: bool,
    pub wifi_enabled: bool,
    pub wifi_capable: bool,
    pub recording: bool,
}

impl Capabilities {
    pub fn from_config(config: &Config, shell_available: bool) -> Self {
        let wifi_capable = WIFI_CAPABLE_DEVICES.contains(&config.device.as_str());
        Self {
            http: true,
            shell: shell_available,
            wifi_enabled: config.wifi_enabled && wifi_capable,
            wifi_capable,
            recording: !config.debug_mode,
        }
    }
}
