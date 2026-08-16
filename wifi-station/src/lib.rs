//! Async WiFi station (STA) lifecycle manager for embedded Linux.
//!
//! `wifi-station` runs and supervises a `wlan1` STA interface alongside an
//! existing `wlan0` AP on dongles and MiFi-style devices, so a host can roam
//! onto upstream WiFi while continuing to serve its own clients.
//!
//! It targets the messy realities of small embedded systems: quirky vendor
//! drivers, busybox userspace, AP+STA coexistence locks, kernel module crashes,
//! and stalled data paths. Recovery is built in.
//!
//! # Mental model
//!
//! [`run_wifi_client`] spawns a long-lived Tokio task that creates the STA
//! interface, runs `wpa_supplicant` and `udhcpc`, installs policy routing so
//! STA traffic does not displace an existing default route (e.g. cellular
//! `rmnet`), and watches the link. When the data path stalls or the interface
//! disappears it walks a graduated recovery ladder — `wpa_cli reassociate`,
//! `wpa_supplicant` restart, interface cycle, and finally a full `wlan.ko`
//! `rmmod`/`insmod` — before giving up after a fixed cap.
//!
//! Live state is published through a [`WifiStatus`] held behind an
//! `Arc<RwLock<_>>` so the rest of the application can read it. Shutdown is
//! cooperative via a [`CancellationToken`].
//!
//! [`CancellationToken`]: tokio_util::sync::CancellationToken
//!
//! # Quick start
//!
//! ```no_run
//! use std::sync::Arc;
//! use tokio::sync::RwLock;
//! use tokio_util::sync::CancellationToken;
//! use tokio_util::task::TaskTracker;
//! use wifi_station::{WifiConfig, WifiStatus, run_wifi_client};
//!
//! # async fn run() {
//! let tasks = TaskTracker::new();
//! let shutdown = CancellationToken::new();
//! let status = Arc::new(RwLock::new(WifiStatus::default()));
//!
//! let config = WifiConfig { wifi_enabled: true, ..Default::default() };
//! run_wifi_client(&tasks, &config, shutdown.clone(), status.clone());
//!
//! // ...later, on shutdown:
//! shutdown.cancel();
//! tasks.close();
//! tasks.wait().await;
//! # }
//! ```
//!
//! Provisioning a network means writing the `wpa_supplicant` config file
//! before (or while) the supervisor is running; [`format_wpa_conf`] and
//! [`update_wpa_conf`] are safe writers that escape user-supplied SSIDs and
//! passwords. [`scan_wifi_networks`] performs a one-shot scan via `iw`.
//!
//! # Runtime requirements
//!
//! The host must provide `iw`, `wpa_supplicant`, `udhcpc`, `ip`, and
//! `killall` on `PATH` (or paths configured via [`WifiConfig`]).
//! Module-reload recovery additionally uses `rmmod`, `insmod`, `hostapd`,
//! `brctl`, and `ifconfig`. Defaults match a typical busybox + Android-derived
//! rootfs.
//!
//! # Cargo features
//!
//! - `utoipa` — derives `utoipa::ToSchema` on [`WifiStatus`], [`WifiState`],
//!   [`SecurityType`], and [`WifiNetwork`] for inclusion in OpenAPI specs.

use std::path::Path;

use serde::{Deserialize, Serialize};

mod client;
mod config;
mod diagnostics;
mod monitor;
mod recovery;
mod routing;
mod scan;

pub use config::{
    format_wpa_conf, read_network_from_wpa_conf, read_ssid_from_wpa_conf, update_wpa_conf,
};
pub use monitor::run_wifi_client;
pub use scan::{WifiNetwork, scan_wifi_networks};

#[cfg(test)]
pub(crate) use config::update_wpa_conf_at;
#[cfg(test)]
pub(crate) use routing::parse_default_route;
#[cfg(test)]
pub(crate) use scan::parse_iw_scan;

#[derive(Clone, Default)]
pub struct WifiConfig {
    pub wifi_enabled: bool,
    pub dns_servers: Option<Vec<String>>,
    pub wifi_ssid: Option<String>,
    pub wifi_password: Option<String>,
    pub security_type: Option<SecurityType>,
    pub wpa_supplicant_bin: Option<String>,
    pub hostapd_conf: Option<String>,
    pub ctrl_interface: Option<String>,
    pub udhcpc_hook_path: Option<String>,
    pub dhcp_lease_path: Option<String>,
    pub wpa_conf_path: Option<String>,
    pub iw_bin: Option<String>,
    pub udhcpc_bin: Option<String>,
    pub crash_log_dir: Option<String>,
    pub wakelock_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SecurityType {
    #[default]
    WpaPsk,
    Sae,
}

pub(crate) const DEFAULT_WPA_CONF_PATH: &str = "/etc/wpa_supplicant/wpa_sta.conf";
pub(crate) const DEFAULT_WPA_BIN: &str = "wpa_supplicant";
pub(crate) const DEFAULT_IW_BIN: &str = "iw";
pub(crate) const DEFAULT_UDHCPC_HOOK_PATH: &str = "/tmp/wifi-station-udhcpc-hook.sh";
pub(crate) const DEFAULT_DHCP_LEASE_PATH: &str = "/tmp/wifi-station-dhcp-lease";
pub(crate) const DEFAULT_CRASH_LOG_DIR: &str = "/tmp/wifi-station-crash-logs";
pub(crate) const DEFAULT_WAKELOCK_NAME: &str = "wifi-station";

pub(crate) const UDHCPC_HOOK_SCRIPT: &str = r#"#!/bin/sh
LEASE_FILE="{}"

case "$1" in
    bound|renew)
        ip addr flush dev "$interface"
        ip addr add "$ip/$mask" dev "$interface"
        echo "gateway=$router" > "$LEASE_FILE"
        echo "dns=$dns" >> "$LEASE_FILE"
        ;;
    deconfig)
        ip addr flush dev "$interface"
        rm -f "$LEASE_FILE"
        ;;
esac
"#;
pub(crate) const DEFAULT_DNS: &[&str] = &["9.9.9.9", "149.112.112.112"];
pub(crate) const MAX_RECOVERY_ATTEMPTS: u32 = 5;
pub(crate) const BASE_BACKOFF_SECS: u64 = 30;
pub(crate) const HOSTAPD_CONF: &str = "/data/misc/wifi/hostapd.conf";
pub(crate) const AP_IFACE: &str = "wlan0";
pub const STA_IFACE: &str = "wlan1";

/// Error prefix emitted when `iw interface add` can't create the STA iface.
/// Matched in monitor.rs to decide whether a module reload should recover.
pub(crate) const ERR_CREATE_STA: &str = "failed to create STA interface";

const BRIDGE_CANDIDATES: &[&str] = &["bridge0", "br0"];

pub fn detect_bridge_iface() -> &'static str {
    for name in BRIDGE_CANDIDATES {
        if Path::new(&format!("/sys/class/net/{name}")).exists() {
            return name;
        }
    }
    BRIDGE_CANDIDATES[0]
}

/// Possible wifi client states
#[derive(Clone, Copy, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum WifiState {
    #[default]
    Disabled,
    Connecting,
    Connected,
    Failed,
    Recovering,
    DataPathDead,
}

/// The status of the wifi client
#[derive(Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WifiStatus {
    /// The state of the wifi connection
    pub state: WifiState,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Connected SSID
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Connected IP
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// String containing error messages
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Count of packets transmitted
    pub tx_packets: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iw_scan_basic() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tTSF: 12345 usec
\tfreq: 2412
\tsignal: -45.00 dBm
\tSSID: MyNetwork
\tRSN:\t * Version: 1
BSS 11:22:33:44:55:66(on wlan1)
\tsignal: -72.00 dBm
\tSSID: OtherNet
\tWPA:\t * Version: 1
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 2);
        assert_eq!(networks[0].ssid, "MyNetwork");
        assert_eq!(networks[0].signal_dbm, -45);
        assert_eq!(networks[0].security, "WPA2");
        assert_eq!(networks[1].ssid, "OtherNet");
        assert_eq!(networks[1].signal_dbm, -72);
        assert_eq!(networks[1].security, "WPA");
    }

    #[test]
    fn test_parse_iw_scan_dedup_keeps_strongest() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -80.00 dBm
\tSSID: DupNet
\tRSN:\t * Version: 1
BSS 11:22:33:44:55:66(on wlan1)
\tsignal: -50.00 dBm
\tSSID: DupNet
\tRSN:\t * Version: 1
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].ssid, "DupNet");
        assert_eq!(networks[0].signal_dbm, -50);
    }

    #[test]
    fn test_parse_iw_scan_hidden_ssid_filtered() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -45.00 dBm
\tSSID:
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 0);
    }

    #[test]
    fn test_parse_iw_scan_open_network() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -60.00 dBm
\tSSID: OpenCafe
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].security, "Open");
    }

    #[test]
    fn test_parse_iw_scan_wpa3_only() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -45.00 dBm
\tSSID: WPA3Net
\tRSN:\t * Version: 1
\t\t * Group cipher: CCMP
\t\t * Pairwise ciphers: CCMP
\t\t * Authentication suites: SAE
\t\t * Capabilities: MFP-required
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].security, "WPA3");
    }

    #[test]
    fn test_parse_iw_scan_wpa3_transition() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -50.00 dBm
\tSSID: TransitionNet
\tRSN:\t * Version: 1
\t\t * Group cipher: CCMP
\t\t * Pairwise ciphers: CCMP
\t\t * Authentication suites: PSK SAE
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].security, "WPA3 (transition)");
    }

    #[test]
    fn test_parse_iw_scan_wpa2_explicit_psk() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -55.00 dBm
\tSSID: WPA2Net
\tRSN:\t * Version: 1
\t\t * Group cipher: CCMP
\t\t * Pairwise ciphers: CCMP
\t\t * Authentication suites: PSK
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].security, "WPA2");
    }

    #[test]
    fn test_parse_iw_scan_mixed_networks() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -40.00 dBm
\tSSID: SecureWPA3
\tRSN:\t * Version: 1
\t\t * Group cipher: CCMP
\t\t * Pairwise ciphers: CCMP
\t\t * Authentication suites: SAE
BSS 11:22:33:44:55:66(on wlan1)
\tsignal: -60.00 dBm
\tSSID: ClassicWPA2
\tRSN:\t * Version: 1
\t\t * Group cipher: CCMP
\t\t * Pairwise ciphers: CCMP
\t\t * Authentication suites: PSK
BSS 77:88:99:aa:bb:cc(on wlan1)
\tsignal: -70.00 dBm
\tSSID: OpenCafe
";
        let networks = parse_iw_scan(output);
        assert_eq!(networks.len(), 3);
        let wpa3 = networks.iter().find(|n| n.ssid == "SecureWPA3").unwrap();
        assert_eq!(wpa3.security, "WPA3");
        let wpa2 = networks.iter().find(|n| n.ssid == "ClassicWPA2").unwrap();
        assert_eq!(wpa2.security, "WPA2");
        let open = networks.iter().find(|n| n.ssid == "OpenCafe").unwrap();
        assert_eq!(open.security, "Open");
    }

    #[tokio::test]
    async fn test_update_wpa_conf_writes_and_removes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa_sta.conf");
        let path_str = path.to_str().unwrap();

        let mut config = WifiConfig {
            wifi_ssid: Some("TestNet".to_string()),
            wifi_password: Some("pass123".to_string()),
            ..Default::default()
        };

        update_wpa_conf_at(&config, path_str).await;

        let content = tokio::fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("ssid=\"TestNet\""));
        assert!(content.contains("psk=\"pass123\""));

        config.wifi_ssid = None;
        config.wifi_password = None;
        update_wpa_conf_at(&config, path_str).await;
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn test_update_wpa_conf_ssid_without_password_is_noop() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa_sta.conf");
        let path_str = path.to_str().unwrap();

        let config = WifiConfig {
            wifi_ssid: Some("TestNet".to_string()),
            wifi_password: None,
            ..Default::default()
        };

        update_wpa_conf_at(&config, path_str).await;
        assert!(!path.exists());
    }

    #[test]
    fn test_parse_default_route() {
        let (gw, dev) = parse_default_route("default via 192.168.1.1 dev bridge0").unwrap();
        assert_eq!(gw, "192.168.1.1");
        assert_eq!(dev, "bridge0");

        let (gw, dev) =
            parse_default_route("default via 10.0.0.1 dev rmnet_data0 metric 100").unwrap();
        assert_eq!(gw, "10.0.0.1");
        assert_eq!(dev, "rmnet_data0");

        assert!(parse_default_route("default dev bridge0 scope link").is_none());
        assert!(parse_default_route("").is_none());
    }

    #[test]
    fn test_format_wpa_conf_basic() {
        let conf = format_wpa_conf("MyNetwork", "mypassword", None, SecurityType::WpaPsk);
        assert!(conf.contains("ssid=\"MyNetwork\""));
        assert!(conf.contains("psk=\"mypassword\""));
        assert!(conf.contains("key_mgmt=WPA-PSK"));
        assert!(conf.starts_with("ctrl_interface=/var/run/wpa_supplicant\n"));
    }

    #[test]
    fn test_format_wpa_conf_escapes_quotes() {
        let conf = format_wpa_conf("My\"Net", "pass\"word", None, SecurityType::WpaPsk);
        assert!(conf.contains("ssid=\"My\\\"Net\""));
        assert!(conf.contains("psk=\"pass\\\"word\""));
    }

    #[test]
    fn test_format_wpa_conf_escapes_backslashes() {
        let conf = format_wpa_conf("Net\\work", "pass\\word", None, SecurityType::WpaPsk);
        assert!(conf.contains("ssid=\"Net\\\\work\""));
        assert!(conf.contains("psk=\"pass\\\\word\""));
    }

    #[test]
    fn test_format_wpa_conf_strips_newlines() {
        let conf = format_wpa_conf(
            "legit",
            "pass\n}\nnetwork={\n    ssid=\"evil\"",
            None,
            SecurityType::WpaPsk,
        );
        assert_eq!(
            conf.lines().count(),
            format_wpa_conf("legit", "clean", None, SecurityType::WpaPsk)
                .lines()
                .count(),
            "newlines in password must not inject extra config lines"
        );
    }

    #[test]
    fn test_format_wpa_conf_sae() {
        let conf = format_wpa_conf("SAENet", "saepass", None, SecurityType::Sae);
        assert!(conf.contains("key_mgmt=SAE"));
        assert!(conf.contains("ieee80211w=2"));
        assert!(conf.contains("sae_password=\"saepass\""));
        assert!(!conf.contains("psk="));
    }

    #[test]
    fn test_format_wpa_conf_sae_escapes() {
        let conf = format_wpa_conf("SAENet", "pass\"w\\ord", None, SecurityType::Sae);
        assert!(conf.contains("sae_password=\"pass\\\"w\\\\ord\""));
    }

    #[test]
    fn test_read_ssid_from_wpa_conf() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("TestSSID", "password123", None, SecurityType::WpaPsk);
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("TestSSID".to_string()));
    }

    #[test]
    fn test_read_ssid_roundtrips_special_chars() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("My\"Net\\work", "pass", None, SecurityType::WpaPsk);
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("My\"Net\\work".to_string()));
    }

    #[test]
    fn test_read_network_wpa_psk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("PskNet", "pskpass", None, SecurityType::WpaPsk);
        std::fs::write(&path, conf).unwrap();

        let (ssid, sec) = read_network_from_wpa_conf(path.to_str().unwrap()).unwrap();
        assert_eq!(ssid, "PskNet");
        assert_eq!(sec, SecurityType::WpaPsk);
    }

    #[test]
    fn test_read_network_sae() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("SaeNet", "saepass", None, SecurityType::Sae);
        std::fs::write(&path, conf).unwrap();

        let (ssid, sec) = read_network_from_wpa_conf(path.to_str().unwrap()).unwrap();
        assert_eq!(ssid, "SaeNet");
        assert_eq!(sec, SecurityType::Sae);
    }

    #[test]
    fn test_read_ssid_missing_file() {
        assert_eq!(read_ssid_from_wpa_conf("/nonexistent/path"), None);
    }
}
