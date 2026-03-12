use std::time::Duration;

use anyhow::ensure;
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run, run_slow};

async fn wait_for_analysis(name: &str) -> anyhow::Result<()> {
    let start = tokio::time::Instant::now();
    let timeout = Duration::from_secs(90);
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let status = ctx().client.get_analysis().await?;
        if status.finished.contains(&name.to_string()) {
            return Ok(());
        }
        let queued = status.queued.contains(&name.to_string());
        let running = status.running.as_ref() == Some(&name.to_string());
        ensure!(
            queued || running,
            "recording {name} disappeared from analysis status"
        );
        if start.elapsed() > timeout {
            let state = if running { "running" } else { "queued" };
            anyhow::bail!(
                "analysis of {name} stuck in '{state}' for {}s",
                timeout.as_secs()
            );
        }
    }
}

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    let can_record = caps.recording;
    vec![
        Trial::test("analysis::status_deserializes", move || {
            run_slow(async {
                ensure!(http, "no HTTP access");
                let _status = ctx().client.get_analysis().await?;
                Ok(())
            })
        }),
        Trial::test("analysis::queue_and_complete", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                ctx()
                    .client
                    .with_recording(|name| async move {
                        let status = ctx().client.start_analysis(&name).await?;
                        ensure!(
                            status.queued.contains(&name) || status.running.as_ref() == Some(&name),
                            "recording {name} not queued or running after start_analysis"
                        );
                        wait_for_analysis(&name).await
                    })
                    .await
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("analysis::report_is_valid_ndjson", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                ctx()
                    .client
                    .with_recording(|name| async move {
                        ctx().client.start_analysis(&name).await?;
                        wait_for_analysis(&name).await?;

                        let report = ctx().client.get_analysis_report(&name).await?;
                        ensure!(!report.is_empty(), "analysis report is empty");

                        for (i, line) in report.lines().enumerate() {
                            if line.trim().is_empty() {
                                continue;
                            }
                            let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
                            ensure!(
                                parsed.is_ok(),
                                "line {} is not valid JSON: {:?}\n  content: {}",
                                i + 1,
                                parsed.unwrap_err(),
                                &line[..line.len().min(200)]
                            );
                        }
                        Ok(())
                    })
                    .await
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test(
            "analysis::live_report_without_recording_returns_503",
            move || {
                run(async {
                    ensure!(http, "no HTTP access");
                    let client = &ctx().client;
                    let _ = client.stop_recording().await;
                    tokio::time::sleep(Duration::from_millis(500)).await;

                    let resp = client.get_analysis_report_raw("live").await?;
                    ensure!(
                        resp.status() == 503,
                        "expected 503 when no recording active, got {}",
                        resp.status()
                    );
                    Ok(())
                })
            },
        ),
        Trial::test("analysis::nonexistent_report_returns_404", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx()
                    .client
                    .get_analysis_report_raw("nonexistent_name")
                    .await?;
                ensure!(resp.status() == 404, "expected 404, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("analysis::nonexistent_name_queues_silently", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let name = "definitely_not_a_real_recording";
                let status = ctx().client.start_analysis(name).await?;
                ensure!(
                    status.queued.contains(&name.to_string())
                        || status.running.as_ref() == Some(&name.to_string())
                        || status.finished.contains(&name.to_string()),
                    "nonexistent name not found in any analysis state"
                );
                Ok(())
            })
        }),
    ]
}
