use std::time::Duration;

use anyhow::{Result, ensure};
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run, run_slow};
use crate::shell::{DEVICE_CONFIG_PATH, DEVICE_WPA_CONF_PATH, ShellConnection};

async fn with_config_restore<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let client = &ctx().client;
    let original = client.get_config().await?;

    let result = f().await;

    // wifi_password is None (redacted by GET), so this won't undo
    // wpa_sta.conf changes — only the TOML config is restored.
    if let Err(e) = client.set_config(&original).await {
        eprintln!("  WARNING: config restore POST failed: {e:#}");
    } else if let Err(e) = client.wait_for_ready(Duration::from_secs(30)).await {
        eprintln!("  WARNING: daemon did not recover after restore: {e:#}");
    }

    result
}

async fn with_full_restore<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let client = &ctx().client;
    let shell = ctx()
        .shell
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("requires --shell"))?;
    let original = client.get_config().await?;
    let saved_wpa = shell.read_file(DEVICE_WPA_CONF_PATH).await?;

    let result = f().await;

    // Restore wpa_sta.conf first (before config POST triggers restart)
    match &saved_wpa {
        Some(conf) => {
            if let Err(e) = shell.write_file(DEVICE_WPA_CONF_PATH, conf).await {
                eprintln!("  WARNING: wpa_sta.conf restore failed: {e:#}");
            }
        }
        None => {
            let _ = shell.remove_file(DEVICE_WPA_CONF_PATH).await;
        }
    }

    if let Err(e) = client.set_config(&original).await {
        eprintln!("  WARNING: config restore POST failed: {e:#}");
    } else if let Err(e) = client.wait_for_ready(Duration::from_secs(30)).await {
        eprintln!("  WARNING: daemon did not recover after restore: {e:#}");
    }

    result
}

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    let shell = caps.shell;
    vec![
        Trial::test("security::get_config_never_returns_password", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let config = ctx().client.get_config().await?;
                ensure!(
                    config.wifi_password.is_none(),
                    "wifi_password present in GET /api/config"
                );

                let raw = ctx().client.get_config_raw().await?;
                ensure!(
                    !raw.contains("wifi_password"),
                    "raw GET /api/config JSON contains 'wifi_password' key"
                );
                Ok(())
            })
        }),
        Trial::test("security::password_not_echoed_after_post", move || {
            run_slow(async {
                ensure!(http, "no HTTP access");
                with_config_restore(|| async {
                    let mut config = ctx().client.get_config().await?;
                    config.wifi_password = Some("hunter2_canary".into());

                    ctx().client.set_config(&config).await?;
                    ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                    let raw = ctx().client.get_config_raw().await?;
                    ensure!(
                        !raw.contains("hunter2_canary"),
                        "password value appeared in GET /api/config after POST"
                    );
                    ensure!(
                        !raw.contains("wifi_password"),
                        "wifi_password key present in GET /api/config after POST"
                    );
                    Ok(())
                })
                .await
            })
        }),
        Trial::test("security::log_does_not_contain_password", move || {
            run_slow(async {
                ensure!(http, "no HTTP access");
                with_config_restore(|| async {
                    let mut config = ctx().client.get_config().await?;
                    config.wifi_password = Some("log_leak_canary_99".into());

                    ctx().client.set_config(&config).await?;
                    ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                    let log = ctx().client.get_log().await?;
                    ensure!(
                        !log.contains("log_leak_canary_99"),
                        "password canary value found in daemon log output"
                    );
                    Ok(())
                })
                .await
            })
        }),
        Trial::test("security::password_not_in_config_toml", move || {
            run_slow(async {
                ensure!(http && shell, "requires HTTP + shell");
                with_full_restore(|| async {
                    let shell_conn = ctx().shell.as_ref().unwrap();

                    let mut config = ctx().client.get_config().await?;
                    config.wifi_password = Some("toml_leak_canary_42".into());
                    ctx().client.set_config(&config).await?;
                    ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                    let toml = shell_conn
                        .read_file(DEVICE_CONFIG_PATH)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("{DEVICE_CONFIG_PATH} not found"))?;

                    ensure!(
                        !toml.contains("toml_leak_canary_42"),
                        "password canary found in {DEVICE_CONFIG_PATH}"
                    );
                    ensure!(
                        !toml.contains("wifi_password"),
                        "wifi_password key present in {DEVICE_CONFIG_PATH}"
                    );
                    ensure!(
                        !toml.contains("wifi_ssid"),
                        "wifi_ssid key present in {DEVICE_CONFIG_PATH}"
                    );
                    Ok(())
                })
                .await
            })
        })
        .with_ignored_flag(!shell),
    ]
}
