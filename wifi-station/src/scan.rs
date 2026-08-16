use anyhow::Result;
use serde::Serialize;

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
    if !crate::link::is_up(iface).await {
        let _ = crate::link::set_up(iface, false).await;
        let _ =
            crate::netlink::set_interface_type(iface, wl_nl80211::Nl80211InterfaceType::Station)
                .await;
        let _ = crate::link::set_up(iface, true).await;
    }

    crate::netlink::scan(iface).await
}

pub(crate) fn resolve_security(
    has_rsn: bool,
    has_wpa: bool,
    has_sae: bool,
    has_psk: bool,
) -> String {
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

/// Parser for `iw dev <iface> scan` text output.
///
/// Scanning now goes through nl80211 directly ([`crate::netlink::scan`]); this
/// is kept only so the security-classification test corpus keeps exercising
/// [`resolve_security`] and [`push_or_update`], which the netlink path shares.
#[cfg(test)]
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

pub(crate) fn push_or_update(
    networks: &mut Vec<WifiNetwork>,
    ssid: String,
    signal: i32,
    security: &str,
) {
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
