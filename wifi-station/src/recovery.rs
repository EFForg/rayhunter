use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Result, bail};
use log::{info, warn};
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::client::WifiClient;
use crate::{AP_IFACE, ERR_CREATE_STA, STA_IFACE, WifiState, WifiStatus, detect_bridge_iface};

async fn get_module_path() -> Result<String> {
    // /system/lib/modules/ is the Android convention (UZ801 etc.)
    let fixed = "/system/lib/modules/wlan.ko";
    if Path::new(fixed).exists() {
        return Ok(fixed.into());
    }

    let kver = get_kernel_version().await?;
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

async fn get_kernel_version() -> Result<String> {
    if let Ok(out) = Command::new("uname").arg("-r").output().await
        && out.status.success()
    {
        return Ok(String::from_utf8_lossy(&out.stdout).trim().to_string());
    }
    let ver = tokio::fs::read_to_string("/proc/version").await?;
    ver.split_whitespace()
        .nth(2)
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("could not parse kernel version from /proc/version"))
}

async fn create_sta_with_iw(iw_bin: &str) -> Result<()> {
    let out = Command::new(iw_bin)
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
        .await;
    if let Ok(ref o) = out
        && o.status.success()
    {
        return Ok(());
    }
    info!("direct managed creation failed, trying P2P_CLIENT workaround");
    let out = Command::new(iw_bin)
        .args([
            "dev",
            AP_IFACE,
            "interface",
            "add",
            STA_IFACE,
            "type",
            "__p2pcl",
        ])
        .output()
        .await?;
    if !out.status.success() {
        bail!(
            "{ERR_CREATE_STA} ({STA_IFACE}): {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let out = Command::new(iw_bin)
        .args(["dev", STA_IFACE, "set", "type", "managed"])
        .output()
        .await?;
    if !out.status.success() {
        bail!(
            "{ERR_CREATE_STA} ({STA_IFACE}: set type managed): {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

async fn teardown_and_reload_module() -> Result<()> {
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

    for _ in 0..30 {
        if Path::new(&format!("/sys/class/net/{AP_IFACE}")).exists() {
            return Ok(());
        }
        sleep(Duration::from_secs(1)).await;
    }
    bail!("{AP_IFACE} did not appear after insmod");
}

pub(crate) async fn reload_wifi_module(hostapd_conf: &str, iw_bin: &str) -> Result<()> {
    teardown_and_reload_module().await?;
    start_hostapd_and_bridge(hostapd_conf).await;
    create_sta_with_iw(iw_bin).await?;
    info!("WiFi module reloaded and AP restored");
    Ok(())
}

/// Reload the wifi module but create wlan1 BEFORE starting hostapd.
///
/// The UZ801's wcnss_wlan driver (kernel 3.10.28) cannot scan on wlan1 while
/// hostapd is active on wlan0 — the radio is exclusively locked to AP mode.
/// Once wpa_supplicant associates (gets past scanning), concurrent AP+STA data
/// flow works fine. So we create wlan1 right after insmod while the radio is
/// idle, let the caller connect wpa_supplicant, then start hostapd afterward.
///
/// Killing hostapd directly doesn't work because Android's netd daemon detects
/// the death and tears down the entire wifi stack (rmmod + insmod + hostapd
/// restart). Doing our own module reload preempts netd's lifecycle management.
pub(crate) async fn reload_wifi_module_sta_first(iw_bin: &str) -> Result<()> {
    teardown_and_reload_module().await?;
    create_sta_with_iw(iw_bin).await?;
    Ok(())
}

/// Bring up wlan0, add it to the bridge, and start hostapd.
///
/// netd may also start its own hostapd after detecting the module reload.
/// A duplicate hostapd is harmless — it fails to bind the interface and exits.
pub(crate) async fn start_hostapd_and_bridge(hostapd_conf: &str) {
    let _ = Command::new("ifconfig")
        .args([AP_IFACE, "up"])
        .output()
        .await;
    let _ = Command::new("brctl")
        .args(["addif", detect_bridge_iface(), AP_IFACE])
        .output()
        .await;

    if Path::new(hostapd_conf).exists() {
        let result = Command::new("hostapd")
            .args(["-B", hostapd_conf])
            .output()
            .await;
        match result {
            Ok(o) if o.status.success() => info!("hostapd restarted"),
            Ok(o) => warn!(
                "hostapd restart failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            ),
            Err(e) => warn!("failed to start hostapd: {e}"),
        }
    }
}

/// Returns true if TX counter starts advancing after any step.
pub(crate) async fn attempt_data_path_recovery(
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

    false
}
