use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use log::{error, info, warn};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::client::{TX_STALL_THRESHOLD, WifiClient};
use crate::config::read_ssid_from_wpa_conf;
use crate::diagnostics::{WakelockGuard, save_wifi_diagnostics};
use crate::recovery::{
    attempt_data_path_recovery, reload_wifi_module, reload_wifi_module_sta_first,
    start_hostapd_and_bridge,
};
use crate::{
    BASE_BACKOFF_SECS, DEFAULT_DNS, DEFAULT_WAKELOCK_NAME, DEFAULT_WPA_CONF_PATH, ERR_CREATE_STA,
    MAX_RECOVERY_ATTEMPTS, STA_IFACE, WifiConfig, WifiState, WifiStatus,
};

pub fn run_wifi_client(
    task_tracker: &TaskTracker,
    config: &WifiConfig,
    shutdown_token: CancellationToken,
    wifi_status: Arc<RwLock<WifiStatus>>,
) {
    let wpa_conf_path = config
        .wpa_conf_path
        .as_deref()
        .unwrap_or(DEFAULT_WPA_CONF_PATH);
    if !config.wifi_enabled || !Path::new(wpa_conf_path).exists() {
        return;
    }

    let dns_servers = config
        .dns_servers
        .clone()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_DNS.iter().map(|s| s.to_string()).collect());

    let ssid = read_ssid_from_wpa_conf(wpa_conf_path);
    let config = config.clone();

    task_tracker.spawn(async move {
        {
            let mut status = wifi_status.write().await;
            status.state = WifiState::Connecting;
            status.ssid = ssid.clone();
        }

        let wakelock_name = config
            .wakelock_name
            .as_deref()
            .unwrap_or(DEFAULT_WAKELOCK_NAME);
        let _wakelock = WakelockGuard::acquire(wakelock_name).await;

        let mut client = WifiClient::new(dns_servers, &config);
        let mut attempt = 0u32;
        let mut tried_module_reload = false;
        loop {
            match client.start().await {
                Ok(()) => {
                    if tried_module_reload {
                        start_hostapd_and_bridge(&client.hostapd_conf).await;
                    }
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
                    break;
                }
                Err(e) => {
                    client.stop().await;
                    attempt += 1;

                    // Reload covers two cases: UZ801 scan -EIO (radio locked to AP), and
                    // `iw interface add` rejected because hostapd is actively serving clients.
                    // In both we need to tear hostapd down, clear the radio, create wlan1, then
                    // restore hostapd once the STA has associated.
                    let err_str = format!("{e}");
                    let needs_module_reload =
                        err_str.contains("-EIO") || err_str.contains(ERR_CREATE_STA);
                    if needs_module_reload && !tried_module_reload {
                        info!(
                            "wifi start blocked (likely AP busy), reloading module and creating STA first"
                        );
                        match reload_wifi_module_sta_first(&client.iw_bin).await {
                            Ok(()) => {
                                tried_module_reload = true;
                                attempt = 0;
                                continue;
                            }
                            Err(reload_err) => {
                                error!("module reload failed: {reload_err}");
                            }
                        }
                    }

                    if attempt >= 3 {
                        if tried_module_reload {
                            start_hostapd_and_bridge(&client.hostapd_conf).await;
                        }
                        let mut status = wifi_status.write().await;
                        status.state = WifiState::Failed;
                        status.error = Some(format!("{e}"));
                        error!("WiFi client failed to start after {attempt} attempts: {e}");
                        return;
                    }
                    warn!(
                        "WiFi client start attempt {attempt} failed: {e}, retrying in {BASE_BACKOFF_SECS}s"
                    );
                    tokio::select! {
                        _ = shutdown_token.cancelled() => {
                            info!("WiFi shutdown during startup retry");
                            return;
                        }
                        _ = sleep(Duration::from_secs(BASE_BACKOFF_SECS)) => {}
                    }
                }
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
                            && let Err(e) = save_wifi_diagnostics(&client.crash_log_dir, "interface disappeared").await
                        {
                            warn!("failed to save wifi diagnostics: {e}");
                        }

                        client.stop().await;

                        if let Err(e) = reload_wifi_module(&client.hostapd_conf, &client.iw_bin).await {
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
                            if let Err(e) = save_wifi_diagnostics(&client.crash_log_dir, "TX+RX data path stall").await {
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
                                if let Err(e) = reload_wifi_module(&client.hostapd_conf, &client.iw_bin).await {
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
