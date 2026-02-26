use std::net::IpAddr;
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
const DEFAULT_DNS: &[&str] = &["9.9.9.9", "149.112.112.112"];
const CRASH_LOG_DIR: &str = "/data/rayhunter/crash-logs";
const MAX_RECOVERY_ATTEMPTS: u32 = 5;
const BASE_BACKOFF_SECS: u64 = 30;
const HOSTAPD_CONF: &str = "/data/misc/wifi/hostapd.conf";
const AP_IFACE: &str = "wlan0";
const BRIDGE_IFACE: &str = "bridge0";
pub const STA_IFACE: &str = "wlan1";

#[derive(Clone, Copy, PartialEq, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WifiState {
    #[default]
    Disabled,
    Connecting,
    Connected,
    Failed,
    Recovering,
}

#[derive(Clone, Serialize, Default)]
pub struct WifiStatus {
    pub state: WifiState,
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
            iface: STA_IFACE.to_string(),
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
            .filter(|s| s.parse::<IpAddr>().is_ok())
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
            let gw = format!("{}.1", &ip[..last_dot]);
            warn!("no explicit gateway for {}, assuming {gw}", self.iface);
            return Ok(gw);
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

    fn interface_exists(&self) -> bool {
        Path::new(&format!("/sys/class/net/{}", self.iface)).exists()
    }
}

async fn save_crash_diagnostics() -> Result<()> {
    tokio::fs::create_dir_all(CRASH_LOG_DIR).await?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let path = format!("{CRASH_LOG_DIR}/wifi-crash-{timestamp}.log");

    let dmesg = Command::new("dmesg").output().await;
    let modules = tokio::fs::read_to_string("/proc/modules").await;
    let ip_addr = Command::new("ip").args(["addr"]).output().await;
    let ps = Command::new("ps").output().await;

    let mut report = String::with_capacity(64 * 1024);
    report.push_str(&format!("WiFi module crash detected at {timestamp}\n\n"));

    report.push_str("=== dmesg ===\n");
    match &dmesg {
        Ok(output) => report.push_str(&String::from_utf8_lossy(&output.stdout)),
        Err(e) => report.push_str(&format!("(failed: {e})\n")),
    }

    report.push_str("\n=== /proc/modules ===\n");
    match &modules {
        Ok(content) => report.push_str(content),
        Err(e) => report.push_str(&format!("(failed: {e})\n")),
    }

    report.push_str("\n=== ip addr ===\n");
    match &ip_addr {
        Ok(output) => report.push_str(&String::from_utf8_lossy(&output.stdout)),
        Err(e) => report.push_str(&format!("(failed: {e})\n")),
    }

    report.push_str("\n=== ps ===\n");
    match &ps {
        Ok(output) => report.push_str(&String::from_utf8_lossy(&output.stdout)),
        Err(e) => report.push_str(&format!("(failed: {e})\n")),
    }

    tokio::fs::write(&path, report).await?;
    info!("saved crash diagnostics to {path}");
    Ok(())
}

async fn get_module_path() -> Result<String> {
    let out = Command::new("uname").arg("-r").output().await?;
    let kver = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let path = format!("/lib/modules/{kver}/extra/wlan.ko");
    if Path::new(&path).exists() {
        return Ok(path);
    }
    let alt = format!("/usr/lib/modules/{kver}/extra/wlan.ko");
    if Path::new(&alt).exists() {
        return Ok(alt);
    }
    bail!("wlan.ko not found for kernel {kver}");
}

async fn reload_wifi_module() -> Result<()> {
    let module_path = get_module_path().await?;

    let _ = Command::new("killall").arg("hostapd").output().await;

    let rmmod = Command::new("rmmod").arg("wlan").output().await?;
    if !rmmod.status.success() {
        warn!(
            "rmmod wlan (may already be unloaded): {}",
            String::from_utf8_lossy(&rmmod.stderr).trim()
        );
    }

    sleep(Duration::from_secs(2)).await;

    let insmod = Command::new("insmod").arg(&module_path).output().await?;
    if !insmod.status.success() {
        bail!(
            "insmod failed: {}",
            String::from_utf8_lossy(&insmod.stderr).trim()
        );
    }

    sleep(Duration::from_secs(3)).await;

    if !Path::new(&format!("/sys/class/net/{AP_IFACE}")).exists() {
        bail!("{AP_IFACE} did not appear after insmod");
    }

    let _ = Command::new("ifconfig")
        .args([AP_IFACE, "up"])
        .output()
        .await;
    let _ = Command::new("brctl")
        .args(["addif", BRIDGE_IFACE, AP_IFACE])
        .output()
        .await;

    if Path::new(HOSTAPD_CONF).exists() {
        let hostapd = Command::new("hostapd")
            .args(["-B", HOSTAPD_CONF])
            .output()
            .await?;
        if !hostapd.status.success() {
            warn!(
                "hostapd restart failed: {}",
                String::from_utf8_lossy(&hostapd.stderr).trim()
            );
        }
    }

    let add_sta = Command::new("iw")
        .args([
            "dev",
            AP_IFACE,
            "interface",
            "add",
            STA_IFACE,
            "type",
            "managed",
        ])
        .output()
        .await?;
    if !add_sta.status.success() {
        bail!(
            "failed to create {STA_IFACE}: {}",
            String::from_utf8_lossy(&add_sta.stderr).trim()
        );
    }

    info!("WiFi module reloaded and AP restored");
    Ok(())
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
            status.state = WifiState::Connecting;
            status.ssid = ssid.clone();
        }

        let mut client = WifiClient::new(dns_servers);
        match client.start().await {
            Ok(()) => {
                let ip = client.get_interface_ip().await.ok();
                let mut status = wifi_status.write().await;
                status.state = WifiState::Connected;
                status.ssid = ssid.clone();
                status.ip = ip;
                status.error = None;
                info!("WiFi client connected");
            }
            Err(e) => {
                client.stop().await;
                let mut status = wifi_status.write().await;
                status.state = WifiState::Failed;
                status.error = Some(format!("{e}"));
                error!("WiFi client failed to start: {e}");
                return;
            }
        }

        let mut recovery_attempts: u32 = 0;
        let mut backoff_secs: u64 = BASE_BACKOFF_SECS;

        loop {
            tokio::select! {
                _ = shutdown_token.cancelled() => {
                    client.stop().await;
                    let mut status = wifi_status.write().await;
                    status.state = WifiState::Disabled;
                    status.ip = None;
                    status.error = None;
                    info!("WiFi client stopped");
                    return;
                }
                _ = sleep(Duration::from_secs(backoff_secs)) => {
                    if !client.interface_exists() {
                        if recovery_attempts >= MAX_RECOVERY_ATTEMPTS {
                            error!(
                                "WiFi module recovery failed after {MAX_RECOVERY_ATTEMPTS} attempts, giving up"
                            );
                            client.stop().await;
                            let mut status = wifi_status.write().await;
                            status.state = WifiState::Failed;
                            status.error = Some(format!(
                                "module crash recovery failed after {MAX_RECOVERY_ATTEMPTS} attempts"
                            ));
                            return;
                        }

                        recovery_attempts += 1;
                        warn!(
                            "{STA_IFACE} interface disappeared, attempting recovery ({recovery_attempts}/{MAX_RECOVERY_ATTEMPTS})"
                        );

                        {
                            let mut status = wifi_status.write().await;
                            status.state = WifiState::Recovering;
                            status.ip = None;
                            status.error = None;
                        }

                        if recovery_attempts == 1
                            && let Err(e) = save_crash_diagnostics().await
                        {
                            warn!("failed to save crash diagnostics: {e}");
                        }

                        client.stop().await;

                        if let Err(e) = reload_wifi_module().await {
                            error!("module reload failed: {e}");
                            let mut status = wifi_status.write().await;
                            status.state = WifiState::Recovering;
                            status.error = Some(format!("{e}"));
                            backoff_secs = (backoff_secs * 2).min(240);
                            continue;
                        }

                        match client.start().await {
                            Ok(()) => {
                                let ip = client.get_interface_ip().await.ok();
                                let mut status = wifi_status.write().await;
                                status.state = WifiState::Connected;
                                status.ip = ip;
                                status.error = None;
                                info!(
                                    "WiFi client recovered after {recovery_attempts} attempt(s)"
                                );
                                recovery_attempts = 0;
                                backoff_secs = BASE_BACKOFF_SECS;
                            }
                            Err(e) => {
                                error!("WiFi client restart after recovery failed: {e}");
                                client.stop().await;
                                let mut status = wifi_status.write().await;
                                status.state = WifiState::Recovering;
                                status.error = Some(format!("{e}"));
                                backoff_secs = (backoff_secs * 2).min(240);
                            }
                        }
                        continue;
                    }

                    if let Some(ref mut child) = client.wpa_child
                        && let Ok(Some(_)) = child.try_wait()
                    {
                        warn!("wpa_supplicant exited, restarting");
                        client.wpa_child = None;
                        if let Err(e) = client.start_wpa_supplicant().await {
                            warn!("wpa_supplicant restart failed: {e}");
                        }
                    }

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

                    if recovery_attempts > 0 {
                        recovery_attempts = 0;
                        backoff_secs = BASE_BACKOFF_SECS;
                    }
                }
            }
        }
    });
}

pub async fn update_wpa_conf(config: &Config) {
    update_wpa_conf_at(config, WPA_CONF_PATH).await;
}

async fn update_wpa_conf_at(config: &Config, path: &str) {
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
    Ok(parse_iw_scan(&String::from_utf8_lossy(&out.stdout)))
}

fn parse_iw_scan(output: &str) -> Vec<WifiNetwork> {
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

    #[tokio::test]
    async fn test_update_wpa_conf_writes_and_removes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa_sta.conf");
        let path_str = path.to_str().unwrap();

        let mut config = Config::default();
        config.wifi_ssid = Some("TestNet".to_string());
        config.wifi_password = Some("pass123".to_string());

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

        let mut config = Config::default();
        config.wifi_ssid = Some("TestNet".to_string());
        config.wifi_password = None;

        update_wpa_conf_at(&config, path_str).await;
        assert!(!path.exists());
    }
}
