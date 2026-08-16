use std::net::IpAddr;

use anyhow::{Context, Result, bail};
use log::{info, warn};
use tokio::process::Command;

use crate::STA_IFACE;
use crate::client::WifiClient;

pub(crate) async fn run_ip(args: &[&str]) {
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
pub(crate) fn parse_default_route(line: &str) -> Option<(String, String)> {
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
pub(crate) async fn demote_cellular_default() {
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

pub(crate) async fn restore_cellular_default() {
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

pub(crate) async fn read_lease_field(lease_path: &str, field: &str) -> Option<String> {
    let content = tokio::fs::read_to_string(lease_path).await.ok()?;
    let prefix = format!("{field}=");
    content.lines().find_map(|line| {
        line.strip_prefix(&prefix)
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
    })
}

impl WifiClient {
    pub(crate) async fn setup_routing(&mut self) -> Result<()> {
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
        if let Some(dhcp_dns) = read_lease_field(&self.dhcp_lease_path, "dns").await {
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
        if let Err(e) = tokio::fs::write("/etc/resolv.conf", &resolv).await {
            warn!("/etc/resolv.conf not writable ({e}), bind-mounting from /tmp");
            tokio::fs::write("/tmp/resolv.conf", &resolv).await?;
            let status = Command::new("mount")
                .args(["-o", "bind", "/tmp/resolv.conf", "/etc/resolv.conf"])
                .status()
                .await;
            if !matches!(status, Ok(s) if s.success()) {
                warn!("bind mount failed, DNS may not work via /etc/resolv.conf");
            }
        }
        Ok(())
    }

    pub(crate) async fn get_interface_ip(&self) -> Result<String> {
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

        if let Some(gw) = read_lease_field(&self.dhcp_lease_path, "gateway").await {
            info!("using DHCP-provided gateway {gw} from lease file");
            return Ok(gw);
        }

        bail!("no default gateway for interface")
    }

    pub(crate) async fn cleanup_routing(&self) {
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
        let _ = tokio::fs::remove_file(&self.dhcp_lease_path).await;
    }

    pub(crate) async fn allow_inbound(&self) {
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

    pub(crate) async fn remove_inbound(&self) {
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
}
