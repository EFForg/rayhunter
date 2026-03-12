use std::time::Duration;

use anyhow::{Result, ensure};
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::client::RayhunterClient;
use crate::context::{ctx, run, run_slow};
use crate::shell::{AdbShell, DEVICE_CONFIG_PATH, DEVICE_WPA_CONF_PATH, ShellConnection};
use crate::types::Config;

// Restores on all exit paths including HTTP-unreachable scenarios via ADB fallback.
struct WifiGuard {
    original_config: Config,
    saved_config_toml: Option<String>,
    saved_wpa_conf: Option<String>,
}

impl WifiGuard {
    async fn save(client: &RayhunterClient, shell: &AdbShell) -> Result<Self> {
        let original_config = client.get_config().await?;
        let saved_config_toml = shell.read_file(DEVICE_CONFIG_PATH).await?;
        let saved_wpa_conf = shell.read_file(DEVICE_WPA_CONF_PATH).await?;
        Ok(Self {
            original_config,
            saved_config_toml,
            saved_wpa_conf,
        })
    }

    async fn restore(&self, client: &RayhunterClient, shell: &AdbShell) -> Result<()> {
        if client.set_config(&self.original_config).await.is_ok()
            && client.wait_for_ready(Duration::from_secs(30)).await.is_ok()
        {
            return Ok(());
        }

        eprintln!("  HTTP unreachable during restore, falling back to ADB");
        self.restore_via_shell(shell).await
    }

    async fn restore_via_shell(&self, shell: &AdbShell) -> Result<()> {
        if let Some(toml) = &self.saved_config_toml {
            shell.write_file(DEVICE_CONFIG_PATH, toml).await?;
        }
        match &self.saved_wpa_conf {
            Some(conf) => shell.write_file(DEVICE_WPA_CONF_PATH, conf).await?,
            None => shell.remove_file(DEVICE_WPA_CONF_PATH).await?,
        }

        let _ = shell.run_command("killall rayhunter-daemon").await;
        tokio::time::sleep(Duration::from_secs(5)).await;

        let client = &ctx().client;
        client.wait_for_ready(Duration::from_secs(30)).await?;
        Ok(())
    }
}

async fn with_wifi_guard<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let shell = ctx()
        .shell
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("wifi state tests require --shell"))?;
    let client = &ctx().client;
    let guard = WifiGuard::save(client, shell).await?;

    let result = f().await;

    if let Err(e) = guard.restore(client, shell).await {
        eprintln!("  WARNING: restore failed: {e:#}");
    }

    result
}

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    let wifi = caps.wifi_enabled;
    let shell = caps.shell;

    vec![
        Trial::test("wifi::status_shape", move || {
            run(async {
                ensure!(http && wifi, "requires HTTP + wifi_enabled");
                let status = ctx().client.get_wifi_status().await?;
                ensure!(
                    [
                        "disabled",
                        "connecting",
                        "connected",
                        "failed",
                        "recovering"
                    ]
                    .contains(&status.state.as_str()),
                    "unexpected wifi state: {}",
                    status.state
                );
                if status.state == "connected" {
                    ensure!(status.ssid.is_some(), "connected but ssid is None");
                }
                Ok(())
            })
        })
        .with_ignored_flag(!wifi),
        Trial::test("wifi::scan_returns_networks", move || {
            run_slow(async {
                ensure!(http && wifi, "requires HTTP + wifi_enabled");
                let networks = ctx().client.scan_wifi().await?;
                // A real device should see at least one network, but we
                // can't guarantee it — just verify the response parses
                for net in &networks {
                    ensure!(!net.ssid.is_empty(), "network with empty SSID");
                    ensure!(
                        !net.security.is_empty(),
                        "network with empty security field"
                    );
                }
                Ok(())
            })
        })
        .with_ignored_flag(!wifi),
        Trial::test("wifi::scan_rate_limit_429", move || {
            run_slow(async {
                ensure!(http && wifi, "requires HTTP + wifi_enabled");
                let client = &ctx().client;

                // The rate limit uses Mutex::try_lock, so both requests must
                // be in flight simultaneously. Retry a few times since timing
                // is not guaranteed.
                for attempt in 0..5 {
                    if attempt > 0 {
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                    let (r1, r2) = tokio::join!(client.scan_wifi_raw(), client.scan_wifi_raw(),);
                    let s1 = r1?.status();
                    let s2 = r2?.status();

                    let got_429 = s1 == reqwest::StatusCode::TOO_MANY_REQUESTS
                        || s2 == reqwest::StatusCode::TOO_MANY_REQUESTS;
                    let got_200 = s1.is_success() || s2.is_success();

                    if got_429 && got_200 {
                        return Ok(());
                    }
                }
                anyhow::bail!("concurrent scans never produced a 429 after 5 attempts");
            })
        })
        .with_ignored_flag(!wifi),
        Trial::test("wifi::disable_enable_roundtrip", move || {
            run_slow(async {
                ensure!(http && wifi && shell, "requires HTTP + wifi + shell");
                with_wifi_guard(|| async {
                    let client = &ctx().client;

                    let mut config = client.get_config().await?;
                    ensure!(config.wifi_enabled, "wifi not enabled at start");

                    config.wifi_enabled = false;
                    client.set_config(&config).await?;
                    client.wait_for_ready(Duration::from_secs(30)).await?;

                    let after_disable = client.get_config().await?;
                    ensure!(
                        !after_disable.wifi_enabled,
                        "wifi_enabled still true after disable"
                    );

                    let status = client.get_wifi_status().await?;
                    ensure!(
                        status.state == "disabled",
                        "expected wifi state 'disabled', got '{}'",
                        status.state
                    );

                    // Re-enable — guard.restore will also do this, but verify
                    // the explicit path works
                    let mut config = after_disable;
                    config.wifi_enabled = true;
                    client.set_config(&config).await?;
                    client.wait_for_ready(Duration::from_secs(30)).await?;

                    let after_enable = client.get_config().await?;
                    ensure!(
                        after_enable.wifi_enabled,
                        "wifi_enabled not restored to true"
                    );
                    Ok(())
                })
                .await
            })
        })
        .with_ignored_flag(!(wifi && shell)),
        Trial::test("wifi::wrong_ssid_produces_error", move || {
            run_slow(async {
                ensure!(http && wifi && shell, "requires HTTP + wifi + shell");
                with_wifi_guard(|| async {
                    let client = &ctx().client;
                    let mut config = client.get_config().await?;

                    config.wifi_ssid = Some("__rayhunter_nonexistent_test_net__".into());
                    config.wifi_password = Some("doesntmatter".into());
                    config.wifi_enabled = true;
                    client.set_config(&config).await?;
                    client.wait_for_ready(Duration::from_secs(30)).await?;

                    tokio::time::sleep(Duration::from_secs(10)).await;

                    let status = client.get_wifi_status().await?;
                    ensure!(
                        status.state != "connected",
                        "connected to nonexistent network — something is wrong"
                    );
                    Ok(())
                })
                .await
            })
        })
        .with_ignored_flag(!(wifi && shell)),
        Trial::test("wifi::ssid_without_password_rejected", move || {
            run_slow(async {
                ensure!(http && wifi && shell, "requires HTTP + wifi + shell");
                with_wifi_guard(|| async {
                    let client = &ctx().client;
                    let shell = ctx().shell.as_ref().unwrap();

                    let wpa_before = shell.read_file(DEVICE_WPA_CONF_PATH).await?;

                    let mut config = client.get_config().await?;
                    config.wifi_ssid = Some("TestNetNoPassword".into());
                    config.wifi_password = None;
                    config.wifi_enabled = true;
                    client.set_config(&config).await?;
                    client.wait_for_ready(Duration::from_secs(30)).await?;

                    // wpa_sta.conf should not have been overwritten with
                    // the passwordless config (update_wpa_conf is a no-op
                    // when ssid is set without password)
                    let wpa_after = shell.read_file(DEVICE_WPA_CONF_PATH).await?;
                    ensure!(
                        wpa_before == wpa_after,
                        "wpa_sta.conf changed when ssid was set without password"
                    );
                    Ok(())
                })
                .await
            })
        })
        .with_ignored_flag(!(wifi && shell)),
    ]
}
