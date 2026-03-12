use anyhow::{Result, bail};
use log::{info, warn};
use tokio::process::Command;

use wifi_station::detect_bridge_iface;

use crate::config::Config;

async fn run_iptables(args: &[&str]) -> Result<()> {
    let out = Command::new("iptables").args(args).output().await?;
    if !out.status.success() {
        bail!(
            "iptables {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}

pub async fn apply(config: &Config) {
    let _ = Command::new("iptables")
        .args(["-F", "OUTPUT"])
        .output()
        .await;

    if config.firewall_restrict_outbound {
        match setup_outbound_whitelist(&config.firewall_allowed_ports, &config.ntfy_url).await {
            Ok(()) => info!("outbound firewall active: allowing DHCP, DNS, HTTPS only"),
            Err(e) => warn!("firewall setup failed: {e}"),
        }
    }
}

async fn setup_outbound_whitelist(
    extra_ports: &Option<Vec<u16>>,
    ntfy_url: &Option<String>,
) -> Result<()> {
    run_iptables(&["-A", "OUTPUT", "-o", "lo", "-j", "ACCEPT"]).await?;
    run_iptables(&["-A", "OUTPUT", "-o", detect_bridge_iface(), "-j", "ACCEPT"]).await?;
    run_iptables(&[
        "-A",
        "OUTPUT",
        "-m",
        "state",
        "--state",
        "ESTABLISHED,RELATED",
        "-j",
        "ACCEPT",
    ])
    .await?;
    run_iptables(&[
        "-A", "OUTPUT", "-p", "udp", "--dport", "67:68", "-j", "ACCEPT",
    ])
    .await?;
    run_iptables(&["-A", "OUTPUT", "-p", "udp", "--dport", "53", "-j", "ACCEPT"]).await?;
    run_iptables(&["-A", "OUTPUT", "-p", "tcp", "--dport", "53", "-j", "ACCEPT"]).await?;
    run_iptables(&[
        "-A", "OUTPUT", "-p", "tcp", "--dport", "443", "-j", "ACCEPT",
    ])
    .await?;

    if let Some(url) = ntfy_url
        && let Ok(parsed) = url::Url::parse(url)
        && let Some(port) = parsed.port()
        && port != 443
    {
        let port_str = port.to_string();
        run_iptables(&[
            "-A", "OUTPUT", "-p", "tcp", "--dport", &port_str, "-j", "ACCEPT",
        ])
        .await?;
        info!("firewall: auto-allowed port {port} for ntfy");
    }

    if let Some(ports) = extra_ports {
        for port in ports {
            let port_str = port.to_string();
            run_iptables(&[
                "-A", "OUTPUT", "-p", "tcp", "--dport", &port_str, "-j", "ACCEPT",
            ])
            .await?;
        }
    }

    run_iptables(&["-A", "OUTPUT", "-j", "DROP"]).await?;

    let _ = tokio::fs::write("/proc/sys/net/bridge/bridge-nf-call-iptables", "0").await;

    Ok(())
}
