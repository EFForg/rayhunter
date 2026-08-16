use log::warn;

use crate::{DEFAULT_WPA_CONF_PATH, SecurityType, WifiConfig};

pub async fn update_wpa_conf(config: &WifiConfig) {
    let path = config
        .wpa_conf_path
        .as_deref()
        .unwrap_or(DEFAULT_WPA_CONF_PATH);
    update_wpa_conf_at(config, path).await;
}

pub(crate) async fn update_wpa_conf_at(config: &WifiConfig, path: &str) {
    let has_ssid = config
        .wifi_ssid
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());
    let has_password = config
        .wifi_password
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());

    if has_ssid && has_password {
        let security_type = config.security_type.unwrap_or_default();
        let conf = format_wpa_conf(
            config.wifi_ssid.as_ref().unwrap(),
            config.wifi_password.as_ref().unwrap(),
            config.ctrl_interface.as_deref(),
            security_type,
        );
        if let Err(e) = tokio::fs::write(path, conf).await {
            warn!("failed to write wpa_supplicant config: {e}");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).await;
        }
    } else if !has_ssid {
        let _ = tokio::fs::remove_file(path).await;
    } else {
        warn!("wifi_ssid set without wifi_password, skipping wpa_supplicant config");
    }
}

fn escape_wpa_value(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\n', '\r'], "")
}

/// Generate a wpa_supplicant configuration file from an SSID and password.
/// Escapes backslashes and double quotes, strips newlines from both fields.
/// `ctrl_interface` overrides the default `/var/run/wpa_supplicant` path.
pub fn format_wpa_conf(
    ssid: &str,
    password: &str,
    ctrl_interface: Option<&str>,
    security_type: SecurityType,
) -> String {
    let ssid = escape_wpa_value(ssid);
    let password = escape_wpa_value(password);
    let ctrl = ctrl_interface.unwrap_or("/var/run/wpa_supplicant");
    match security_type {
        SecurityType::WpaPsk => format!(
            "ctrl_interface={ctrl}\nnetwork={{\n    ssid=\"{ssid}\"\n    psk=\"{password}\"\n    key_mgmt=WPA-PSK\n}}\n"
        ),
        SecurityType::Sae => format!(
            "ctrl_interface={ctrl}\nnetwork={{\n    ssid=\"{ssid}\"\n    sae_password=\"{password}\"\n    key_mgmt=SAE\n    ieee80211w=2\n}}\n"
        ),
    }
}

/// Read the SSID and security type from a wpa_supplicant configuration file.
/// Returns None if the file doesn't exist or has no ssid line.
pub fn read_network_from_wpa_conf(path: &str) -> Option<(String, SecurityType)> {
    let content = std::fs::read_to_string(path).ok()?;
    let mut ssid = None;
    let mut security = SecurityType::WpaPsk;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(s) = trimmed
            .strip_prefix("ssid=\"")
            .and_then(|s| s.strip_suffix('"'))
        {
            ssid = Some(s.replace("\\\"", "\"").replace("\\\\", "\\"));
        }
        if trimmed == "key_mgmt=SAE" {
            security = SecurityType::Sae;
        }
    }
    Some((ssid?, security))
}

/// Read the SSID from a wpa_supplicant configuration file.
/// Returns None if the file doesn't exist or has no ssid line.
pub fn read_ssid_from_wpa_conf(path: &str) -> Option<String> {
    read_network_from_wpa_conf(path).map(|(ssid, _)| ssid)
}
