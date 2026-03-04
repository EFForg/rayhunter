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

#[derive(Clone, Default)]
pub struct WifiConfig {
    pub wifi_enabled: bool,
    pub dns_servers: Option<Vec<String>>,
    pub wifi_ssid: Option<String>,
    pub wifi_password: Option<String>,
}

pub const WPA_CONF_PATH: &str = "/data/rayhunter/wpa_sta.conf";

const WPA_BIN: &str = "/data/rayhunter/bin/wpa_supplicant";
const UDHCPC_HOOK: &str = "/data/rayhunter/udhcpc-hook.sh";
const DHCP_LEASE_FILE: &str = "/data/rayhunter/dhcp_lease";
const DEFAULT_DNS: &[&str] = &["9.9.9.9", "149.112.112.112"];
const CRASH_LOG_DIR: &str = "/data/rayhunter/crash-logs";
const MAX_RECOVERY_ATTEMPTS: u32 = 5;
const BASE_BACKOFF_SECS: u64 = 30;
const HOSTAPD_CONF: &str = "/data/misc/wifi/hostapd.conf";
const WAKELOCK_NAME: &[u8] = b"rayhunter";
const AP_IFACE: &str = "wlan0";
const BRIDGE_IFACE: &str = "bridge0";
pub const STA_IFACE: &str = "wlan1";

#[derive(Clone, Copy, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum WifiState {
    #[default]
    Disabled,
    Connecting,
    Connected,
    Failed,
    Recovering,
    DataPathDead,
}

#[derive(Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WifiStatus {
    pub state: WifiState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_packets: Option<u64>,
}

const TX_STALL_THRESHOLD: u32 = 3;

struct WifiClient {
    iface: String,
    wpa_child: Option<Child>,
    dhcp_child: Option<Child>,
    rt_table: u32,
    dns_servers: Vec<String>,
    saved_resolv: Option<String>,
    last_tx_packets: Option<u64>,
    last_rx_packets: Option<u64>,
    tx_stall_count: u32,
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
            last_tx_packets: None,
            last_rx_packets: None,
            tx_stall_count: 0,
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

        restore_cellular_default().await;

        if let Some(resolv) = self.saved_resolv.take() {
            let _ = tokio::fs::write("/etc/resolv.conf", resolv).await;
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
        self.wait_for_association().await
    }

    async fn wait_for_association(&self) -> Result<()> {
        let operstate_path = format!("/sys/class/net/{}/operstate", self.iface);
        for i in 0..30 {
            if let Ok(state) = tokio::fs::read_to_string(&operstate_path).await
                && state.trim() == "up"
            {
                info!("wpa_supplicant associated after {}s", i + 1);
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
        }
        bail!("wpa_supplicant did not associate within 30s");
    }

    async fn start_dhcp(&mut self) -> Result<()> {
        use std::process::Stdio;
        let _ = tokio::fs::remove_file(DHCP_LEASE_FILE).await;
        let child = Command::new("udhcpc")
            .args([
                "-i",
                &self.iface,
                "-s",
                UDHCPC_HOOK,
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

        for _ in 0..30 {
            sleep(Duration::from_secs(1)).await;
            if tokio::fs::metadata(DHCP_LEASE_FILE).await.is_ok() {
                return Ok(());
            }
        }
        bail!("DHCP did not assign an address within 30s");
    }

    async fn setup_routing(&mut self) -> Result<()> {
        if self.saved_resolv.is_none() {
            self.saved_resolv = tokio::fs::read_to_string("/etc/resolv.conf").await.ok();
        }

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

        self.cleanup_routing().await;

        demote_cellular_default().await;
        let out = Command::new("ip")
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
        if let Ok(o) = &out
            && !o.status.success()
        {
            warn!(
                "failed to add WiFi default route: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            );
        }

        let table = self.rt_table.to_string();
        run_ip(&["rule", "add", "from", &ip, "table", &table]).await;
        run_ip(&[
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
        .await;
        run_ip(&[
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
        .await;

        let gw_host = format!("{gateway}/32");
        run_ip(&["route", "replace", &gw_host, "dev", &self.iface]).await;
        run_ip(&[
            "route",
            "replace",
            &gw_host,
            "dev",
            &self.iface,
            "table",
            &table,
        ])
        .await;

        let arp_path = format!("/proc/sys/net/ipv4/conf/{}/arp_filter", self.iface);
        let _ = tokio::fs::write(&arp_path, "1").await;

        let mut dns: Vec<String> = Vec::new();
        if let Some(dhcp_dns) = read_lease_field("dns").await {
            dns.extend(
                dhcp_dns
                    .split_whitespace()
                    .filter(|s| s.parse::<IpAddr>().is_ok())
                    .map(|s| s.to_string()),
            );
        }
        if dns.is_empty() {
            dns.extend(
                self.dns_servers
                    .iter()
                    .filter(|s| s.parse::<IpAddr>().is_ok())
                    .cloned(),
            );
        }
        let resolv = dns
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

        if let Some(gw) = read_lease_field("gateway").await {
            info!("using DHCP-provided gateway {gw} from lease file");
            return Ok(gw);
        }

        bail!("no default gateway for interface")
    }

    async fn cleanup_routing(&self) {
        let table = self.rt_table.to_string();
        loop {
            let out = Command::new("ip")
                .args(["rule", "del", "table", &table])
                .output()
                .await;
            match out {
                Ok(o) if o.status.success() => continue,
                _ => break,
            }
        }
        let _ = Command::new("ip")
            .args(["route", "flush", "table", &table])
            .output()
            .await;
        let _ = Command::new("ip")
            .args(["route", "del", "default", "dev", &self.iface])
            .output()
            .await;
        let _ = tokio::fs::remove_file(DHCP_LEASE_FILE).await;
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
            .args(["-D", "FORWARD", "-o", &self.iface, "-j", "ACCEPT"])
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
        let _ = Command::new("iptables")
            .args(["-I", "FORWARD", "-o", &self.iface, "-j", "ACCEPT"])
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
        let _ = Command::new("iptables")
            .args(["-D", "FORWARD", "-o", &self.iface, "-j", "ACCEPT"])
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

    async fn read_tx_packets(&self) -> Option<u64> {
        let path = format!("/sys/class/net/{}/statistics/tx_packets", self.iface);
        tokio::fs::read_to_string(&path)
            .await
            .ok()?
            .trim()
            .parse()
            .ok()
    }

    async fn read_rx_packets(&self) -> Option<u64> {
        let path = format!("/sys/class/net/{}/statistics/rx_packets", self.iface);
        tokio::fs::read_to_string(&path)
            .await
            .ok()?
            .trim()
            .parse()
            .ok()
    }

    async fn check_tx_advancing(&self) -> bool {
        let first = self.read_tx_packets().await;
        sleep(Duration::from_secs(5)).await;
        let second = self.read_tx_packets().await;
        match (first, second) {
            (Some(a), Some(b)) => b > a,
            _ => false,
        }
    }
}

async fn run_ip(args: &[&str]) {
    let out = Command::new("ip").args(args).output().await;
    match out {
        Ok(o) if !o.status.success() => {
            warn!(
                "ip {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&o.stderr).trim()
            );
        }
        Err(e) => warn!("ip {} exec error: {e}", args.join(" ")),
        _ => {}
    }
}

/// Parse the gateway and device from an `ip route show default` line.
fn parse_default_route(line: &str) -> Option<(String, String)> {
    let mut parts = line.split_whitespace();
    let mut gw = None;
    let mut dev = None;
    while let Some(word) = parts.next() {
        match word {
            "via" => gw = parts.next().map(|s| s.to_string()),
            "dev" => dev = parts.next().map(|s| s.to_string()),
            _ => {}
        }
    }
    Some((gw?, dev?))
}

/// Demote cellular default route to metric 1000 so WiFi takes priority.
async fn demote_cellular_default() {
    let out = Command::new("ip")
        .args(["route", "show", "default"])
        .output()
        .await;
    let Ok(o) = out else { return };
    let stdout = String::from_utf8_lossy(&o.stdout);
    for line in stdout.lines() {
        if let Some((gw, dev)) = parse_default_route(line) {
            if dev == STA_IFACE {
                continue;
            }
            let _ = Command::new("ip")
                .args(["route", "del", "default", "via", &gw, "dev", &dev])
                .output()
                .await;
            let _ = Command::new("ip")
                .args([
                    "route", "add", "default", "via", &gw, "dev", &dev, "metric", "1000",
                ])
                .output()
                .await;
        }
    }
}

/// Restore demoted cellular default route to its original metric.
struct WakelockGuard;

impl WakelockGuard {
    async fn acquire() -> Self {
        match tokio::fs::write("/sys/power/wake_lock", WAKELOCK_NAME).await {
            Ok(()) => info!("acquired kernel wakelock"),
            Err(e) => warn!("failed to acquire wakelock: {e}"),
        }
        WakelockGuard
    }
}

impl Drop for WakelockGuard {
    fn drop(&mut self) {
        match std::fs::write("/sys/power/wake_unlock", WAKELOCK_NAME) {
            Ok(()) => info!("released kernel wakelock"),
            Err(e) => warn!("failed to release wakelock: {e}"),
        }
    }
}

async fn restore_cellular_default() {
    let out = Command::new("ip")
        .args(["route", "show", "default"])
        .output()
        .await;
    let Ok(o) = out else { return };
    let stdout = String::from_utf8_lossy(&o.stdout);
    for line in stdout.lines() {
        if line.contains("metric 1000")
            && let Some((gw, dev)) = parse_default_route(line)
        {
            let _ = Command::new("ip")
                .args([
                    "route", "del", "default", "via", &gw, "dev", &dev, "metric", "1000",
                ])
                .output()
                .await;
            let _ = Command::new("ip")
                .args(["route", "add", "default", "via", &gw, "dev", &dev])
                .output()
                .await;
        }
    }
}

async fn read_lease_field(field: &str) -> Option<String> {
    let content = tokio::fs::read_to_string(DHCP_LEASE_FILE).await.ok()?;
    let prefix = format!("{field}=");
    content.lines().find_map(|line| {
        line.strip_prefix(&prefix)
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
    })
}

async fn save_wifi_diagnostics(reason: &str) -> Result<()> {
    tokio::fs::create_dir_all(CRASH_LOG_DIR).await?;

    if let Ok(mut entries) = tokio::fs::read_dir(CRASH_LOG_DIR).await {
        let mut files = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("wifi-diag-") || name.starts_with("wifi-crash-") {
                files.push(entry.path());
            }
        }
        if files.len() >= 10 {
            files.sort();
            for old in &files[..files.len() - 9] {
                let _ = tokio::fs::remove_file(old).await;
            }
        }
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let path = format!("{CRASH_LOG_DIR}/wifi-diag-{timestamp}.log");

    let iface = STA_IFACE;
    let (
        dmesg,
        iw_link,
        iw_station,
        proc_net_dev,
        wpa_status,
        proc_arp,
        ip_route,
        brctl,
        iptables,
        modules,
        ip_addr,
        ps,
    ) = tokio::join!(
        Command::new("dmesg").output(),
        Command::new("iw").args(["dev", iface, "link"]).output(),
        Command::new("iw")
            .args(["dev", iface, "station", "dump"])
            .output(),
        tokio::fs::read_to_string("/proc/net/dev"),
        Command::new("wpa_cli")
            .args(["-i", iface, "status"])
            .output(),
        tokio::fs::read_to_string("/proc/net/arp"),
        Command::new("ip")
            .args(["route", "show", "table", "all"])
            .output(),
        Command::new("brctl").args(["show"]).output(),
        Command::new("iptables").args(["-L", "-v", "-n"]).output(),
        tokio::fs::read_to_string("/proc/modules"),
        Command::new("ip").args(["addr"]).output(),
        Command::new("ps").output(),
    );

    let operstate = tokio::fs::read_to_string(format!("/sys/class/net/{iface}/operstate")).await;
    let sysfs_stats = [
        "tx_packets",
        "tx_errors",
        "tx_dropped",
        "rx_packets",
        "rx_errors",
        "rx_dropped",
    ];
    let mut sysfs_report = String::new();
    for stat in &sysfs_stats {
        let val =
            tokio::fs::read_to_string(format!("/sys/class/net/{iface}/statistics/{stat}")).await;
        sysfs_report.push_str(&format!(
            "  {stat}: {}\n",
            match &val {
                Ok(v) => v.trim().to_string(),
                Err(e) => format!("(failed: {e})"),
            }
        ));
    }

    let mut report = String::with_capacity(128 * 1024);
    report.push_str(&format!(
        "WiFi diagnostics: {reason}\nTimestamp: {timestamp}\n\n"
    ));

    fn append_cmd(
        report: &mut String,
        label: &str,
        result: &Result<std::process::Output, std::io::Error>,
    ) {
        report.push_str(&format!("=== {label} ===\n"));
        match result {
            Ok(o) => report.push_str(&String::from_utf8_lossy(&o.stdout)),
            Err(e) => report.push_str(&format!("(failed: {e})\n")),
        }
        report.push('\n');
    }

    fn append_file(report: &mut String, label: &str, result: &Result<String, std::io::Error>) {
        report.push_str(&format!("=== {label} ===\n"));
        match result {
            Ok(s) => report.push_str(s),
            Err(e) => report.push_str(&format!("(failed: {e})\n")),
        }
        report.push('\n');
    }

    append_cmd(&mut report, "dmesg", &dmesg);
    append_cmd(&mut report, &format!("iw dev {iface} link"), &iw_link);
    append_cmd(
        &mut report,
        &format!("iw dev {iface} station dump"),
        &iw_station,
    );
    append_file(&mut report, "/proc/net/dev", &proc_net_dev);

    report.push_str(&format!("=== {iface} sysfs ===\n"));
    report.push_str(&format!(
        "  operstate: {}\n",
        match &operstate {
            Ok(v) => v.trim().to_string(),
            Err(e) => format!("(failed: {e})"),
        }
    ));
    report.push_str(&sysfs_report);
    report.push('\n');

    append_cmd(
        &mut report,
        &format!("wpa_cli -i {iface} status"),
        &wpa_status,
    );
    append_file(&mut report, "/proc/net/arp", &proc_arp);
    append_cmd(&mut report, "ip route show table all", &ip_route);
    append_cmd(&mut report, "brctl show", &brctl);
    append_cmd(&mut report, "iptables -L -v -n", &iptables);
    append_file(&mut report, "/proc/modules", &modules);
    append_cmd(&mut report, "ip addr", &ip_addr);
    append_cmd(&mut report, "ps", &ps);

    tokio::fs::write(&path, report).await?;
    info!("saved wifi diagnostics to {path}");
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

/// Returns true if TX counter starts advancing after any step.
async fn attempt_data_path_recovery(
    client: &mut WifiClient,
    wifi_status: &Arc<RwLock<WifiStatus>>,
    shutdown_token: &CancellationToken,
) -> bool {
    info!("data path recovery step 1: wpa_cli reassociate");
    let _ = Command::new("wpa_cli")
        .args(["-i", STA_IFACE, "reassociate"])
        .output()
        .await;
    tokio::select! {
        _ = shutdown_token.cancelled() => return false,
        _ = sleep(Duration::from_secs(10)) => {}
    }
    if client.check_tx_advancing().await {
        let mut status = wifi_status.write().await;
        status.state = WifiState::Connected;
        status.error = None;
        return true;
    }

    info!("data path recovery step 2: restart wpa_supplicant");
    if let Some(ref mut child) = client.wpa_child {
        let _ = child.kill().await;
        let _ = child.wait().await;
    }
    client.wpa_child = None;
    if let Err(e) = client.start_wpa_supplicant().await {
        warn!("wpa_supplicant restart failed in recovery: {e}");
    } else {
        tokio::select! {
            _ = shutdown_token.cancelled() => return false,
            _ = sleep(Duration::from_secs(10)) => {}
        }
        if client.check_tx_advancing().await {
            let mut status = wifi_status.write().await;
            status.state = WifiState::Connected;
            status.error = None;
            return true;
        }
    }

    if shutdown_token.is_cancelled() {
        return false;
    }

    info!("data path recovery step 3: interface cycle");
    client.stop().await;
    let _ = Command::new("ip")
        .args(["link", "set", STA_IFACE, "down"])
        .output()
        .await;
    tokio::select! {
        _ = shutdown_token.cancelled() => return false,
        _ = sleep(Duration::from_secs(2)) => {}
    }
    let _ = Command::new("ip")
        .args(["link", "set", STA_IFACE, "up"])
        .output()
        .await;
    tokio::select! {
        _ = shutdown_token.cancelled() => return false,
        _ = sleep(Duration::from_secs(2)) => {}
    }
    if let Err(e) = client.start().await {
        warn!("full restart failed in recovery step 3: {e}");
        return false;
    }
    tokio::select! {
        _ = shutdown_token.cancelled() => return false,
        _ = sleep(Duration::from_secs(10)) => {}
    }
    if client.check_tx_advancing().await {
        let mut status = wifi_status.write().await;
        status.state = WifiState::Connected;
        status.ip = client.get_interface_ip().await.ok();
        status.error = None;
        return true;
    }

    // Module reload handled by caller
    false
}

pub fn run_wifi_client(
    task_tracker: &TaskTracker,
    config: &WifiConfig,
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

    let ssid = read_ssid_from_wpa_conf(WPA_CONF_PATH);

    task_tracker.spawn(async move {
        {
            let mut status = wifi_status.write().await;
            status.state = WifiState::Connecting;
            status.ssid = ssid.clone();
        }

        let _wakelock = WakelockGuard::acquire().await;

        let mut client = WifiClient::new(dns_servers);
        match client.start().await {
            Ok(()) => {
                let ip = client.get_interface_ip().await.ok();
                client.last_tx_packets = client.read_tx_packets().await;
                client.last_rx_packets = client.read_rx_packets().await;
                let mut status = wifi_status.write().await;
                status.state = WifiState::Connected;
                status.ssid = ssid.clone();
                status.ip = ip;
                status.tx_packets = client.last_tx_packets;
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
                            && let Err(e) = save_wifi_diagnostics("interface disappeared").await
                        {
                            warn!("failed to save wifi diagnostics: {e}");
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

                    // Only flag a stall when BOTH TX and RX
                    // are frozen. RX always advances on a healthy link (beacons,
                    // broadcast ARP, etc.), so TX-only stall = idle device.
                    let tx_now = client.read_tx_packets().await;
                    let rx_now = client.read_rx_packets().await;
                    {
                        let mut status = wifi_status.write().await;
                        status.tx_packets = tx_now;
                    }
                    let tx_stalled = matches!((tx_now, client.last_tx_packets), (Some(a), Some(b)) if a == b);
                    let rx_stalled = matches!((rx_now, client.last_rx_packets), (Some(a), Some(b)) if a == b);
                    if tx_stalled && rx_stalled {
                        client.tx_stall_count += 1;
                        warn!(
                            "data path stall: tx={} rx={} unchanged for {} polls",
                            tx_now.unwrap_or(0),
                            rx_now.unwrap_or(0),
                            client.tx_stall_count
                        );
                        if client.tx_stall_count >= TX_STALL_THRESHOLD {
                            warn!("stall count reached {TX_STALL_THRESHOLD}, attempting data path recovery");
                            {
                                let mut status = wifi_status.write().await;
                                status.state = WifiState::DataPathDead;
                            }
                            if let Err(e) = save_wifi_diagnostics("TX+RX data path stall").await {
                                warn!("failed to save wifi diagnostics: {e}");
                            }
                            if attempt_data_path_recovery(&mut client, &wifi_status, &shutdown_token).await {
                                info!("data path recovery succeeded");
                                client.tx_stall_count = 0;
                                client.last_tx_packets = client.read_tx_packets().await;
                                client.last_rx_packets = client.read_rx_packets().await;
                            } else {
                                error!("data path recovery failed, falling through to module reload");
                                client.tx_stall_count = 0;
                                client.last_tx_packets = None;
                                client.last_rx_packets = None;
                                recovery_attempts += 1;
                                if recovery_attempts >= MAX_RECOVERY_ATTEMPTS {
                                    error!("module recovery failed after {MAX_RECOVERY_ATTEMPTS} attempts, giving up");
                                    client.stop().await;
                                                        let mut status = wifi_status.write().await;
                                    status.state = WifiState::Failed;
                                    status.error = Some(format!(
                                        "data path recovery failed after {MAX_RECOVERY_ATTEMPTS} attempts"
                                    ));
                                    return;
                                }
                                warn!("module reload attempt {recovery_attempts}/{MAX_RECOVERY_ATTEMPTS}");
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
                                        info!("WiFi client recovered via module reload");
                                    }
                                    Err(e) => {
                                        error!("WiFi restart after module reload failed: {e}");
                                        client.stop().await;
                                        let mut status = wifi_status.write().await;
                                        status.state = WifiState::Failed;
                                        status.error = Some(format!("{e}"));
                                        backoff_secs = (backoff_secs * 2).min(240);
                                    }
                                }
                            }
                            continue;
                        }
                    } else {
                        if client.tx_stall_count > 0 {
                            info!("data path advancing again (was stalled for {} polls)", client.tx_stall_count);
                        }
                        client.tx_stall_count = 0;
                    }
                    client.last_tx_packets = tx_now;
                    client.last_rx_packets = rx_now;

                    if recovery_attempts > 0 {
                        recovery_attempts = 0;
                        backoff_secs = BASE_BACKOFF_SECS;
                    }
                }
            }
        }
    });
}

pub async fn update_wpa_conf(config: &WifiConfig) {
    update_wpa_conf_at(config, WPA_CONF_PATH).await;
}

async fn update_wpa_conf_at(config: &WifiConfig, path: &str) {
    let has_ssid = config
        .wifi_ssid
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());
    let has_password = config
        .wifi_password
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty());

    if has_ssid && has_password {
        let conf = format_wpa_conf(
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

fn escape_wpa_value(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\n', '\r'], "")
}

/// Generate a wpa_supplicant configuration file from an SSID and password.
/// Escapes backslashes and double quotes, strips newlines from both fields.
pub fn format_wpa_conf(ssid: &str, password: &str) -> String {
    let ssid = escape_wpa_value(ssid);
    let password = escape_wpa_value(password);
    format!(
        "ctrl_interface=/var/run/wpa_supplicant\nnetwork={{\n    ssid=\"{ssid}\"\n    psk=\"{password}\"\n    key_mgmt=WPA-PSK\n}}\n"
    )
}

/// Read the SSID from a wpa_supplicant configuration file.
/// Returns None if the file doesn't exist or has no ssid line.
pub fn read_ssid_from_wpa_conf(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    content.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("ssid=\"")
            .and_then(|s| s.strip_suffix('"'))
            .map(|s| s.replace("\\\"", "\"").replace("\\\\", "\\"))
    })
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

        let mut config = WifiConfig::default();
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

        let mut config = WifiConfig::default();
        config.wifi_ssid = Some("TestNet".to_string());
        config.wifi_password = None;

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
        let conf = format_wpa_conf("MyNetwork", "mypassword");
        assert!(conf.contains("ssid=\"MyNetwork\""));
        assert!(conf.contains("psk=\"mypassword\""));
        assert!(conf.contains("key_mgmt=WPA-PSK"));
        assert!(conf.starts_with("ctrl_interface=/var/run/wpa_supplicant\n"));
    }

    #[test]
    fn test_format_wpa_conf_escapes_quotes() {
        let conf = format_wpa_conf("My\"Net", "pass\"word");
        assert!(conf.contains("ssid=\"My\\\"Net\""));
        assert!(conf.contains("psk=\"pass\\\"word\""));
    }

    #[test]
    fn test_format_wpa_conf_escapes_backslashes() {
        let conf = format_wpa_conf("Net\\work", "pass\\word");
        assert!(conf.contains("ssid=\"Net\\\\work\""));
        assert!(conf.contains("psk=\"pass\\\\word\""));
    }

    #[test]
    fn test_format_wpa_conf_strips_newlines() {
        let conf = format_wpa_conf("legit", "pass\n}\nnetwork={\n    ssid=\"evil\"");
        assert_eq!(
            conf.lines().count(),
            format_wpa_conf("legit", "clean").lines().count(),
            "newlines in password must not inject extra config lines"
        );
    }

    #[test]
    fn test_read_ssid_from_wpa_conf() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("TestSSID", "password123");
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("TestSSID".to_string()));
    }

    #[test]
    fn test_read_ssid_roundtrips_special_chars() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("My\"Net\\work", "pass");
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("My\"Net\\work".to_string()));
    }

    #[test]
    fn test_read_ssid_missing_file() {
        assert_eq!(read_ssid_from_wpa_conf("/nonexistent/path"), None);
    }
}
