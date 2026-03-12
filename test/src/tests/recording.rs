use std::time::Duration;

use anyhow::ensure;
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run, run_slow};

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    let can_record = caps.recording;
    vec![
        Trial::test("recording::capture_produces_data", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                ctx()
                    .client
                    .with_recording(|name| async move {
                        let manifest = ctx().client.get_qmdl_manifest().await?;
                        let entry = manifest
                            .entries
                            .iter()
                            .find(|e| e.name == name)
                            .ok_or_else(|| anyhow::anyhow!("entry {name} not in manifest"))?;

                        ensure!(
                            entry.qmdl_size_bytes > 0,
                            "recording captured 0 bytes of QMDL data"
                        );
                        ensure!(!entry.start_time.is_empty(), "start_time missing");
                        Ok(())
                    })
                    .await
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("recording::start_sets_current_entry", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                let _ = ctx().client.stop_recording().await;
                tokio::time::sleep(Duration::from_millis(500)).await;

                ctx().client.start_recording().await?;
                let manifest = ctx().client.get_qmdl_manifest().await?;
                ensure!(
                    manifest.current_entry.is_some(),
                    "current_entry should be set while recording"
                );

                ctx().client.stop_recording().await?;
                tokio::time::sleep(Duration::from_millis(500)).await;

                let manifest = ctx().client.get_qmdl_manifest().await?;
                ensure!(
                    manifest.current_entry.is_none(),
                    "current_entry should be None after stop"
                );

                if let Some(entry) = manifest.entries.last() {
                    let _ = ctx().client.delete_recording(&entry.name).await;
                }
                Ok(())
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("recording::delete_single", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                let name = ctx().client.create_recording().await?;

                ctx().client.delete_recording(&name).await?;

                let after = ctx().client.get_qmdl_manifest().await?;
                ensure!(
                    !after.entries.iter().any(|e| e.name == name),
                    "entry {name} still present after delete"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("recording::delete_all", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                let _ = ctx().client.stop_recording().await;
                tokio::time::sleep(Duration::from_millis(500)).await;

                ctx().client.delete_all_recordings().await?;

                let manifest = ctx().client.get_qmdl_manifest().await?;
                ensure!(
                    manifest.entries.is_empty(),
                    "entries should be empty after delete_all, got {}",
                    manifest.entries.len()
                );
                Ok(())
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("recording::double_start_resilience", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                let _ = ctx().client.stop_recording().await;
                tokio::time::sleep(Duration::from_millis(500)).await;

                ctx().client.start_recording().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;

                let result = ctx().client.start_recording().await;
                ctx().client.get_config().await?;
                let _ = ctx().client.stop_recording().await;

                if let Err(e) = result {
                    eprintln!("  double-start returned error (acceptable): {e}");
                }

                let manifest = ctx().client.get_qmdl_manifest().await?;
                if let Some(entry) = manifest.entries.last() {
                    let _ = ctx().client.delete_recording(&entry.name).await;
                }
                Ok(())
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("recording::debug_mode_blocks_mutations", move || {
            run_slow(async {
                ensure!(http, "no HTTP access");
                let client = &ctx().client;
                let original = client.get_config().await?;

                let mut debug_on = original.clone();
                debug_on.debug_mode = true;
                client.set_config(&debug_on).await?;
                client.wait_for_ready(Duration::from_secs(30)).await?;

                let start_resp = client.start_recording_raw().await?;
                let stop_resp = client.stop_recording_raw().await?;
                let delete_resp = client.delete_recording_raw("anything").await?;
                let delete_all_resp = client.delete_all_recordings_raw().await?;

                // Restore before asserting so we don't leave debug_mode on
                let mut restore = client.get_config().await?;
                restore.debug_mode = false;
                if let Err(e) = client.set_config(&restore).await {
                    eprintln!("  WARNING: config restore failed: {e:#}");
                } else if let Err(e) = client.wait_for_ready(Duration::from_secs(30)).await {
                    eprintln!("  WARNING: daemon did not recover after restore: {e:#}");
                }

                ensure!(
                    start_resp.status() == 403,
                    "start_recording expected 403 in debug_mode, got {}",
                    start_resp.status()
                );
                ensure!(
                    stop_resp.status() == 403,
                    "stop_recording expected 403 in debug_mode, got {}",
                    stop_resp.status()
                );
                ensure!(
                    delete_resp.status() == 403,
                    "delete_recording expected 403 in debug_mode, got {}",
                    delete_resp.status()
                );
                ensure!(
                    delete_all_resp.status() == 403,
                    "delete_all_recordings expected 403 in debug_mode, got {}",
                    delete_all_resp.status()
                );
                Ok(())
            })
        }),
        Trial::test("recording::stop_while_stopped_is_idempotent", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let client = &ctx().client;
                let _ = client.stop_recording().await;
                tokio::time::sleep(Duration::from_millis(500)).await;

                client.stop_recording().await?;
                client.get_config().await?;
                Ok(())
            })
        }),
        Trial::test("recording::low_disk_returns_507", move || {
            run_slow(async {
                ensure!(http, "no HTTP access");
                let client = &ctx().client;
                let original = client.get_config().await?;

                let mut high_threshold = original.clone();
                high_threshold.min_space_to_start_recording_mb = 999_999;
                client.set_config(&high_threshold).await?;
                client.wait_for_ready(Duration::from_secs(30)).await?;

                let resp = client.start_recording_raw().await?;

                // Restore before asserting
                if let Err(e) = client.set_config(&original).await {
                    eprintln!("  WARNING: config restore failed: {e:#}");
                } else if let Err(e) = client.wait_for_ready(Duration::from_secs(30)).await {
                    eprintln!("  WARNING: daemon did not recover after restore: {e:#}");
                }

                ensure!(
                    resp.status() == 507,
                    "expected 507 Insufficient Storage, got {}",
                    resp.status()
                );
                Ok(())
            })
        }),
        Trial::test("recording::delete_nonexistent_returns_400", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx()
                    .client
                    .delete_recording_raw("nonexistent_name")
                    .await?;
                ensure!(resp.status() == 400, "expected 400, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("recording::delete_while_recording", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                let client = &ctx().client;

                let _ = client.stop_recording().await;
                tokio::time::sleep(Duration::from_millis(500)).await;

                client.start_recording().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;

                let manifest = client.get_qmdl_manifest().await?;
                let entry_name = manifest
                    .current_entry
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("no current_entry while recording"))?
                    .name
                    .clone();

                client.delete_recording(&entry_name).await?;

                let after = client.get_qmdl_manifest().await?;
                ensure!(
                    !after.entries.iter().any(|e| e.name == entry_name),
                    "entry {entry_name} still present after delete"
                );
                ensure!(
                    after.current_entry.is_none(),
                    "current_entry should be None after deleting active recording"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!can_record),
    ]
}
