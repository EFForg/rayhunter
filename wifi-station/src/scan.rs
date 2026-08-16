use anyhow::Result;
use serde::Serialize;
use tokio::process::Command;

/// A struct defining a wifi network
#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WifiNetwork {
    /// The SSID of the access point
    pub ssid: String,
    /// Signal strength in dBm
    pub signal_dbm: i32,
    /// Encryption type(s) available
    pub security: String,
}

pub async fn scan_wifi_networks(iface: &str) -> Result<Vec<WifiNetwork>> {
    let link_out = Command::new("ip")
        .args(["link", "show", iface])
        .output()
        .await?;
    let link_stdout = String::from_utf8_lossy(&link_out.stdout);
    let already_up = link_stdout.contains("state UP");

    if !already_up {
        let _ = Command::new("ip")
            .args(["link", "set", iface, "down"])
            .output()
            .await;
        let _ = Command::new("iw")
            .args(["dev", iface, "set", "type", "managed"])
            .output()
            .await;
        let _ = Command::new("ip")
            .args(["link", "set", iface, "up"])
            .output()
            .await;
    }

    let out = Command::new("iw")
        .args(["dev", iface, "scan"])
        .output()
        .await?;
    Ok(parse_iw_scan(&String::from_utf8_lossy(&out.stdout)))
}

fn resolve_security(has_rsn: bool, has_wpa: bool, has_sae: bool, has_psk: bool) -> String {
    if has_sae && has_psk {
        "WPA3 (transition)".to_string()
    } else if has_sae {
        "WPA3".to_string()
    } else if has_rsn {
        "WPA2".to_string()
    } else if has_wpa {
        "WPA".to_string()
    } else {
        "Open".to_string()
    }
}

pub(crate) fn parse_iw_scan(output: &str) -> Vec<WifiNetwork> {
    let mut networks: Vec<WifiNetwork> = Vec::new();
    let mut current_ssid: Option<String> = None;
    let mut current_signal: i32 = -100;
    let mut has_rsn = false;
    let mut has_wpa = false;
    let mut in_rsn_block = false;
    let mut has_sae = false;
    let mut has_psk = false;

    for line in output.lines() {
        let trimmed = line.trim();
        if line.starts_with("BSS ") {
            if let Some(ssid) = current_ssid.take()
                && !ssid.is_empty()
            {
                let security = resolve_security(has_rsn, has_wpa, has_sae, has_psk);
                push_or_update(&mut networks, ssid, current_signal, &security);
            }
            current_signal = -100;
            has_rsn = false;
            has_wpa = false;
            in_rsn_block = false;
            has_sae = false;
            has_psk = false;
        } else if trimmed.starts_with("RSN:") {
            has_rsn = true;
            in_rsn_block = true;
        } else if trimmed.starts_with("WPA:") {
            has_wpa = true;
            in_rsn_block = false;
        } else if in_rsn_block {
            if trimmed.starts_with("*") {
                if let Some(suites) = trimmed.strip_prefix("* Authentication suites:") {
                    for suite in suites.split_whitespace() {
                        match suite {
                            "SAE" => has_sae = true,
                            "PSK" => has_psk = true,
                            _ => {}
                        }
                    }
                }
            } else {
                in_rsn_block = false;
                if let Some(ssid) = trimmed.strip_prefix("SSID: ") {
                    current_ssid = Some(ssid.to_string());
                } else if let Some(sig) = trimmed.strip_prefix("signal: ")
                    && let Some(dbm) = sig.split_whitespace().next()
                {
                    current_signal = dbm.parse::<f32>().unwrap_or(-100.0) as i32;
                }
            }
        } else if let Some(ssid) = trimmed.strip_prefix("SSID: ") {
            current_ssid = Some(ssid.to_string());
        } else if let Some(sig) = trimmed.strip_prefix("signal: ")
            && let Some(dbm) = sig.split_whitespace().next()
        {
            current_signal = dbm.parse::<f32>().unwrap_or(-100.0) as i32;
        }
    }

    if let Some(ssid) = current_ssid
        && !ssid.is_empty()
    {
        let security = resolve_security(has_rsn, has_wpa, has_sae, has_psk);
        push_or_update(&mut networks, ssid, current_signal, &security);
    }

    networks.sort_by(|a, b| b.signal_dbm.cmp(&a.signal_dbm));
    networks
}

fn push_or_update(networks: &mut Vec<WifiNetwork>, ssid: String, signal: i32, security: &str) {
    if let Some(existing) = networks.iter_mut().find(|n| n.ssid == ssid) {
        if signal > existing.signal_dbm {
            existing.signal_dbm = signal;
        }
    } else {
        networks.push(WifiNetwork {
            ssid,
            signal_dbm: signal,
            security: security.to_string(),
        });
    }
}
