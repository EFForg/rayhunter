use anyhow::Result;
use log::{info, warn};
use tokio::process::Command;

use crate::STA_IFACE;

pub(crate) struct WakelockGuard {
    name: Vec<u8>,
}

impl WakelockGuard {
    pub(crate) async fn acquire(name: &str) -> Self {
        let name_bytes = name.as_bytes().to_vec();
        match tokio::fs::write("/sys/power/wake_lock", &name_bytes).await {
            Ok(()) => info!("acquired kernel wakelock"),
            Err(e) => warn!("failed to acquire wakelock: {e}"),
        }
        WakelockGuard { name: name_bytes }
    }
}

impl Drop for WakelockGuard {
    fn drop(&mut self) {
        match std::fs::write("/sys/power/wake_unlock", &self.name) {
            Ok(()) => info!("released kernel wakelock"),
            Err(e) => warn!("failed to release wakelock: {e}"),
        }
    }
}

pub(crate) async fn save_wifi_diagnostics(crash_log_dir: &str, reason: &str) -> Result<()> {
    tokio::fs::create_dir_all(crash_log_dir).await?;

    if let Ok(mut entries) = tokio::fs::read_dir(crash_log_dir).await {
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
    let path = format!("{crash_log_dir}/wifi-diag-{timestamp}.log");

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
