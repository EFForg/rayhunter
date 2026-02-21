use log::{info, warn};
use tokio::process::Command;

use crate::config::Config;

const FIREWALL_FLAG: &str = "/data/rayhunter/firewall-enabled";

pub async fn apply(config: &Config) {
    if config.block_ota_daemons {
        block_ota_daemons().await;
    }

    let _ = Command::new("iptables")
        .args(["-F", "OUTPUT"])
        .output()
        .await;

    if config.firewall_restrict_outbound {
        setup_outbound_whitelist(&config.firewall_allowed_ports, &config.ntfy_url).await;
        let _ = tokio::fs::write(FIREWALL_FLAG, "").await;
    } else {
        let _ = tokio::fs::remove_file(FIREWALL_FLAG).await;
    }
}

async fn block_ota_daemons() {
    let stub = "#!/bin/sh\nwhile true; do sleep 3600; done\n";
    if let Err(e) = tokio::fs::write("/tmp/daemon-stub", stub).await {
        warn!("failed to write daemon stub: {e}");
        return;
    }
    let _ = Command::new("chmod")
        .args(["755", "/tmp/daemon-stub"])
        .output()
        .await;

    for daemon in &["dmclient", "upgrade"] {
        let path = format!("/usr/bin/{daemon}");
        let _ = Command::new("mount")
            .args(["--bind", "/tmp/daemon-stub", &path])
            .output()
            .await;
        let _ = Command::new("pkill").args(["-9", daemon]).output().await;
    }
}

async fn setup_outbound_whitelist(extra_ports: &Option<Vec<u16>>, ntfy_url: &Option<String>) {
    let _ = Command::new("iptables")
        .args(["-A", "OUTPUT", "-o", "lo", "-j", "ACCEPT"])
        .output()
        .await;
    let _ = Command::new("iptables")
        .args(["-A", "OUTPUT", "-o", "bridge0", "-j", "ACCEPT"])
        .output()
        .await;

    let _ = Command::new("iptables")
        .args([
            "-A",
            "OUTPUT",
            "-m",
            "state",
            "--state",
            "ESTABLISHED,RELATED",
            "-j",
            "ACCEPT",
        ])
        .output()
        .await;

    let _ = Command::new("iptables")
        .args([
            "-A", "OUTPUT", "-p", "udp", "--dport", "67:68", "-j", "ACCEPT",
        ])
        .output()
        .await;
    let _ = Command::new("iptables")
        .args(["-A", "OUTPUT", "-p", "udp", "--dport", "53", "-j", "ACCEPT"])
        .output()
        .await;
    let _ = Command::new("iptables")
        .args(["-A", "OUTPUT", "-p", "tcp", "--dport", "53", "-j", "ACCEPT"])
        .output()
        .await;
    let _ = Command::new("iptables")
        .args([
            "-A", "OUTPUT", "-p", "tcp", "--dport", "443", "-j", "ACCEPT",
        ])
        .output()
        .await;

    if let Some(url) = ntfy_url
        && let Ok(parsed) = url::Url::parse(url)
        && let Some(port) = parsed.port()
        && port != 443
    {
        let _ = Command::new("iptables")
            .args([
                "-A",
                "OUTPUT",
                "-p",
                "tcp",
                "--dport",
                &port.to_string(),
                "-j",
                "ACCEPT",
            ])
            .output()
            .await;
        info!("firewall: auto-allowed port {port} for ntfy");
    }

    if let Some(ports) = extra_ports {
        for port in ports {
            let _ = Command::new("iptables")
                .args([
                    "-A",
                    "OUTPUT",
                    "-p",
                    "tcp",
                    "--dport",
                    &port.to_string(),
                    "-j",
                    "ACCEPT",
                ])
                .output()
                .await;
        }
    }

    let _ = Command::new("iptables")
        .args(["-A", "OUTPUT", "-j", "DROP"])
        .output()
        .await;

    let _ = tokio::fs::write("/proc/sys/net/bridge/bridge-nf-call-iptables", "0").await;

    info!("outbound firewall active: allowing DHCP, DNS, HTTPS only");
}
