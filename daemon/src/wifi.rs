use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use log::{error, info, warn};
use serde::Serialize;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::config::Config;

pub const WPA_CONF_PATH: &str = "/data/rayhunter/wpa_sta.conf";

const WPA_BIN: &str = "/data/rayhunter/bin/wpa_supplicant";
const DEFAULT_DNS: &[&str] = &["8.8.8.8", "1.1.1.1"];

#[derive(Clone, Serialize, Default)]
pub struct WifiStatus {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct WifiClient {
    iface: String,
    wpa_child: Option<Child>,
    dhcp_child: Option<Child>,
    rt_table: u32,
    dns_servers: Vec<String>,
    saved_resolv: Option<String>,
    saved_default_route: Option<String>,
}

impl WifiClient {
    fn new(dns_servers: Vec<String>) -> Self {
        WifiClient {
            iface: "wlan1".to_string(),
            wpa_child: None,
            dhcp_child: None,
            rt_table: 100,
            dns_servers,
            saved_resolv: None,
            saved_default_route: None,
        }
    }

    async fn start(&mut self) -> Result<()> {
        self.wait_for_interface().await?;
        self.set_managed_mode().await?;
        self.start_wpa_supplicant().await?;
        self.start_dhcp().await?;
        self.setup_routing().await?;
        self.allow_inbound().await;
        Ok(())
    }

    async fn stop(&mut self) {
        if let Some(mut child) = self.wpa_child.take() {
            let _ = child.kill().await;
        }
        if let Some(mut child) = self.dhcp_child.take() {
            let _ = child.kill().await;
        }
        self.remove_inbound().await;
        self.cleanup_routing().await;
        self.interface_down().await;

        if let Some(resolv) = self.saved_resolv.take() {
            let _ = tokio::fs::write("/etc/resolv.conf", resolv).await;
        }
        if let Some(route) = self.saved_default_route.take() {
            let args: Vec<&str> = route.split_whitespace().collect();
            let mut cmd_args = vec!["route", "add"];
            cmd_args.extend(&args);
            let _ = Command::new("ip").args(&cmd_args).output().await;
        }
    }

    async fn wait_for_interface(&self) -> Result<()> {
        for _ in 0..30 {
            if Path::new(&format!("/sys/class/net/{}", self.iface)).exists() {
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
        }
        bail!("{} not found after 30s", self.iface);
    }

    async fn set_managed_mode(&self) -> Result<()> {
        let out = Command::new("iw")
            .args(["dev", &self.iface, "set", "type", "managed"])
            .output()
            .await?;
        if !out.status.success() {
            bail!(
                "iw set type managed failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        let out = Command::new("ip")
            .args(["link", "set", &self.iface, "up"])
            .output()
            .await?;
        if !out.status.success() {
            bail!(
                "ip link set up failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        Ok(())
    }

    async fn start_wpa_supplicant(&mut self) -> Result<()> {
        use std::process::Stdio;
        let child = Command::new(WPA_BIN)
            .args(["-i", &self.iface, "-Dnl80211", "-c", WPA_CONF_PATH])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.wpa_child = Some(child);
        sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    async fn start_dhcp(&mut self) -> Result<()> {
        use std::process::Stdio;
        let child = Command::new("udhcpc")
            .args([
                "-i",
                &self.iface,
                "-s",
                "/etc/udhcpc.d/50default",
                "-t",
                "10",
                "-A",
                "3",
                "-f",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.dhcp_child = Some(child);

        for _ in 0..15 {
            sleep(Duration::from_secs(1)).await;
            if self.get_interface_ip().await.is_ok() {
                return Ok(());
            }
        }
        bail!("DHCP did not assign an address within 15s");
    }

    async fn setup_routing(&mut self) -> Result<()> {
        if self.saved_resolv.is_none() {
            self.saved_resolv = tokio::fs::read_to_string("/etc/resolv.conf").await.ok();
        }
        if self.saved_default_route.is_none() {
            let out = Command::new("ip")
                .args(["route", "show", "default"])
                .output()
                .await;
            if let Ok(o) = out {
                let stdout = String::from_utf8_lossy(&o.stdout);
                self.saved_default_route = stdout.lines().next().map(|s| s.to_string());
            }
        }

        self.cleanup_routing().await;

        let ip = self
            .get_interface_ip()
            .await
            .context("failed to get IP after DHCP")?;
        let subnet = self
            .get_interface_subnet()
            .await
            .context("failed to get subnet after DHCP")?;
        let gateway = self
            .get_interface_gateway()
            .await
            .context("failed to get gateway after DHCP")?;

        let _ = Command::new("ip")
            .args(["route", "del", "default", "dev", "bridge0"])
            .output()
            .await;
        let _ = Command::new("ip")
            .args([
                "route",
                "replace",
                "default",
                "via",
                &gateway,
                "dev",
                &self.iface,
                "metric",
                "10",
            ])
            .output()
            .await;

        let table = self.rt_table.to_string();
        let _ = Command::new("ip")
            .args(["rule", "add", "from", &ip, "table", &table])
            .output()
            .await;
        let _ = Command::new("ip")
            .args([
                "route",
                "add",
                &subnet,
                "dev",
                &self.iface,
                "src",
                &ip,
                "table",
                &table,
            ])
            .output()
            .await;
        let _ = Command::new("ip")
            .args([
                "route",
                "add",
                "default",
                "via",
                &gateway,
                "dev",
                &self.iface,
                "table",
                &table,
            ])
            .output()
            .await;

        let resolv = self
            .dns_servers
            .iter()
            .map(|s| format!("nameserver {s}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        tokio::fs::write("/etc/resolv.conf", resolv).await?;
        Ok(())
    }

    async fn get_interface_ip(&self) -> Result<String> {
        let out = Command::new("ip")
            .args(["addr", "show", &self.iface])
            .output()
            .await?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        stdout
            .lines()
            .find_map(|line| {
                let trimmed = line.trim();
                trimmed
                    .strip_prefix("inet ")?
                    .split('/')
                    .next()
                    .map(|s| s.to_string())
            })
            .context("no inet address on interface")
    }

    async fn get_interface_subnet(&self) -> Result<String> {
        let out = Command::new("ip")
            .args(["route", "show", "dev", &self.iface])
            .output()
            .await?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        stdout
            .lines()
            .find_map(|line| {
                if line.contains("proto kernel") {
                    line.split_whitespace().next().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .context("no kernel route for interface")
    }

    async fn get_interface_gateway(&self) -> Result<String> {
        // First try an explicit default route on this interface
        let out = Command::new("ip")
            .args(["route", "show", "dev", &self.iface, "default"])
            .output()
            .await?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Some(gw) = stdout.lines().find_map(|line| {
            let mut parts = line.split_whitespace();
            while let Some(word) = parts.next() {
                if word == "via" {
                    return parts.next().map(|s| s.to_string());
                }
            }
            None
        }) {
            return Ok(gw);
        }

        // When subnets overlap (e.g. bridge0 and wlan1 both on 192.168.1.0/24),
        // udhcpc may not add an explicit default route for wlan1. Fall back to
        // inferring the gateway as .1 from the kernel subnet route.
        let ip = self.get_interface_ip().await?;
        if let Some(last_dot) = ip.rfind('.') {
            return Ok(format!("{}.1", &ip[..last_dot]));
        }

        bail!("no default gateway for interface")
    }

    async fn cleanup_routing(&self) {
        let table = self.rt_table.to_string();
        let _ = Command::new("ip")
            .args(["rule", "del", "table", &table])
            .output()
            .await;
        let _ = Command::new("ip")
            .args(["route", "flush", "table", &table])
            .output()
            .await;
    }

    async fn allow_inbound(&self) {
        let _ = Command::new("iptables")
            .args(["-D", "INPUT", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
        let _ = Command::new("iptables")
            .args(["-D", "FORWARD", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
        let _ = Command::new("iptables")
            .args(["-I", "INPUT", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
        let _ = Command::new("iptables")
            .args(["-I", "FORWARD", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
    }

    async fn remove_inbound(&self) {
        let _ = Command::new("iptables")
            .args(["-D", "INPUT", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
        let _ = Command::new("iptables")
            .args(["-D", "FORWARD", "-i", &self.iface, "-j", "ACCEPT"])
            .output()
            .await;
    }

    async fn interface_down(&self) {
        let _ = Command::new("ip")
            .args(["link", "set", &self.iface, "down"])
            .output()
            .await;
    }
}

pub fn run_wifi_client(
    task_tracker: &TaskTracker,
    config: &Config,
    shutdown_token: CancellationToken,
    wifi_status: Arc<RwLock<WifiStatus>>,
) {
    if !config.wifi_enabled || !Path::new(WPA_CONF_PATH).exists() {
        return;
    }

    let dns_servers = config
        .dns_servers
        .clone()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_DNS.iter().map(|s| s.to_string()).collect());

    let ssid = rayhunter::read_ssid_from_wpa_conf(WPA_CONF_PATH);

    task_tracker.spawn(async move {
        {
            let mut status = wifi_status.write().await;
            status.state = "connecting".to_string();
            status.ssid = ssid.clone();
        }

        let mut client = WifiClient::new(dns_servers);
        match client.start().await {
            Ok(()) => {
                let ip = client.get_interface_ip().await.ok();
                let mut status = wifi_status.write().await;
                status.state = "connected".to_string();
                status.ssid = ssid.clone();
                status.ip = ip;
                status.error = None;
                info!("WiFi client connected");
            }
            Err(e) => {
                client.stop().await;
                let mut status = wifi_status.write().await;
                status.state = "failed".to_string();
                status.error = Some(format!("{e}"));
                error!("WiFi client failed to start: {e}");
                return;
            }
        }

        loop {
            tokio::select! {
                _ = shutdown_token.cancelled() => {
                    client.stop().await;
                    let mut status = wifi_status.write().await;
                    status.state = "disabled".to_string();
                    status.ip = None;
                    status.error = None;
                    info!("WiFi client stopped");
                    return;
                }
                _ = sleep(Duration::from_secs(30)) => {
                    if let Some(ref mut child) = client.dhcp_child
                        && let Ok(Some(_)) = child.try_wait()
                    {
                        warn!("udhcpc exited, restarting DHCP");
                        if let Err(e) = client.start_dhcp().await {
                            warn!("DHCP restart failed: {e}");
                        } else {
                            let _ = client.setup_routing().await;
                            let mut status = wifi_status.write().await;
                            status.ip = client.get_interface_ip().await.ok();
                        }
                    }
                }
            }
        }
    });
}

pub async fn update_wpa_conf(config: &Config) {
    let has_ssid = config
        .wifi_ssid
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());
    let has_password = config
        .wifi_password
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());

    if has_ssid && has_password {
        let conf = rayhunter::format_wpa_conf(
            config.wifi_ssid.as_ref().unwrap(),
            config.wifi_password.as_ref().unwrap(),
        );
        if let Err(e) = tokio::fs::write(WPA_CONF_PATH, conf).await {
            warn!("failed to write wpa_supplicant config: {e}");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ =
                tokio::fs::set_permissions(WPA_CONF_PATH, std::fs::Permissions::from_mode(0o600))
                    .await;
        }
    } else if !has_ssid {
        let _ = tokio::fs::remove_file(WPA_CONF_PATH).await;
    }
}

#[derive(Serialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal_dbm: i32,
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
    parse_iw_scan(&String::from_utf8_lossy(&out.stdout))
}

fn parse_iw_scan(output: &str) -> Result<Vec<WifiNetwork>> {
    let mut networks: Vec<WifiNetwork> = Vec::new();
    let mut current_ssid: Option<String> = None;
    let mut current_signal: i32 = -100;
    let mut current_security = String::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if line.starts_with("BSS ") {
            if let Some(ssid) = current_ssid.take()
                && !ssid.is_empty()
            {
                push_or_update(&mut networks, ssid, current_signal, &current_security);
            }
            current_signal = -100;
            current_security = String::new();
        } else if let Some(ssid) = trimmed.strip_prefix("SSID: ") {
            current_ssid = Some(ssid.to_string());
        } else if let Some(sig) = trimmed.strip_prefix("signal: ") {
            if let Some(dbm) = sig.split_whitespace().next() {
                current_signal = dbm.parse::<f32>().unwrap_or(-100.0) as i32;
            }
        } else if trimmed.starts_with("RSN:") {
            current_security = "WPA2".to_string();
        } else if trimmed.starts_with("WPA:") && current_security.is_empty() {
            current_security = "WPA".to_string();
        }
    }

    if let Some(ssid) = current_ssid
        && !ssid.is_empty()
    {
        push_or_update(&mut networks, ssid, current_signal, &current_security);
    }

    networks.sort_by(|a, b| b.signal_dbm.cmp(&a.signal_dbm));
    Ok(networks)
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
            security: if security.is_empty() {
                "Open".to_string()
            } else {
                security.to_string()
            },
        });
    }
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
        let networks = parse_iw_scan(output).unwrap();
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
        let networks = parse_iw_scan(output).unwrap();
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
        let networks = parse_iw_scan(output).unwrap();
        assert_eq!(networks.len(), 0);
    }

    #[test]
    fn test_parse_iw_scan_open_network() {
        let output = "\
BSS aa:bb:cc:dd:ee:ff(on wlan1)
\tsignal: -60.00 dBm
\tSSID: OpenCafe
";
        let networks = parse_iw_scan(output).unwrap();
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].security, "Open");
    }

    #[tokio::test]
    async fn test_update_wpa_conf_writes_and_removes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa_sta.conf");

        let mut config = Config::default();
        config.wifi_ssid = Some("TestNet".to_string());
        config.wifi_password = Some("pass123".to_string());

        tokio::fs::write(&path, "").await.unwrap();

        let conf = rayhunter::format_wpa_conf(
            config.wifi_ssid.as_ref().unwrap(),
            config.wifi_password.as_ref().unwrap(),
        );
        tokio::fs::write(&path, &conf).await.unwrap();

        let content = tokio::fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("ssid=\"TestNet\""));
        assert!(content.contains("psk=\"pass123\""));

        tokio::fs::remove_file(&path).await.unwrap();
        assert!(!path.exists());
    }
}
