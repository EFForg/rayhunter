use std::time::Duration;

use anyhow::{bail, ensure};
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run};

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    vec![
        Trial::test("config::get_returns_valid_json", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let config = ctx().client.get_config().await?;
                ensure!(
                    !config.qmdl_store_path.is_empty(),
                    "qmdl_store_path is empty"
                );
                ensure!(config.port > 0, "port is zero");
                ensure!(
                    [
                        "orbic",
                        "tplink",
                        "tmobile",
                        "wingtech",
                        "pinephone",
                        "uz801",
                        "moxee"
                    ]
                    .contains(&config.device.as_str()),
                    "unknown device type: {}",
                    config.device
                );
                Ok(())
            })
        }),
        Trial::test("config::set_and_restore_preserves_fields", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let original = ctx().client.get_config().await?;
                let original_level = original.ui_level;
                let original_colorblind = original.colorblind_mode;

                let new_level = if original_level == 1 { 2 } else { 1 };
                let mut modified = original.clone();
                modified.ui_level = new_level;
                modified.colorblind_mode = !original_colorblind;

                ctx().client.set_config(&modified).await?;
                ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                let after = ctx().client.get_config().await?;
                ensure!(
                    after.ui_level == new_level,
                    "ui_level not updated: expected {new_level}, got {}",
                    after.ui_level
                );
                ensure!(
                    after.colorblind_mode != original_colorblind,
                    "colorblind_mode not updated"
                );
                ensure!(
                    after.port == original.port,
                    "port changed unexpectedly: {} -> {}",
                    original.port,
                    after.port
                );
                ensure!(
                    after.device == original.device,
                    "device changed unexpectedly: {} -> {}",
                    original.device,
                    after.device
                );

                let mut restore = after;
                restore.ui_level = original_level;
                restore.colorblind_mode = original_colorblind;
                ctx().client.set_config(&restore).await?;
                ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                let restored = ctx().client.get_config().await?;
                ensure!(
                    restored.ui_level == original_level,
                    "ui_level not restored: expected {original_level}, got {}",
                    restored.ui_level
                );
                Ok(())
            })
        }),
        Trial::test("config::set_triggers_restart", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let original = ctx().client.get_config().await?;

                ctx().client.set_config(&original).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;

                match ctx().client.wait_for_ready(Duration::from_secs(30)).await {
                    Ok(()) => Ok(()),
                    Err(e) => bail!("daemon did not recover after config POST: {e}"),
                }
            })
        }),
        Trial::test("config::invalid_json_returns_422", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx().client.post_config_raw("not valid json").await?;
                ensure!(resp.status() == 422, "expected 422, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("config::wifi_ssid_stripped_from_toml_on_post", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let mut config = ctx().client.get_config().await?;
                config.wifi_ssid = Some("test_canary_ssid".into());
                ctx().client.set_config(&config).await?;
                ctx().client.wait_for_ready(Duration::from_secs(30)).await?;

                // Daemon reads from TOML on restart, which had ssid stripped
                let after = ctx().client.get_config().await?;
                ensure!(
                    after.wifi_ssid.is_none(),
                    "wifi_ssid survived restart — TOML should not contain it, got {:?}",
                    after.wifi_ssid
                );

                Ok(())
            })
        }),
    ]
}
