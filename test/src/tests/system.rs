use anyhow::{bail, ensure};
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run};

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    vec![
        Trial::test("system::stats_has_disk_and_memory", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let stats = ctx().client.get_system_stats().await?;
                ensure!(
                    !stats.disk_stats.total_size.is_empty(),
                    "disk total_size empty"
                );
                ensure!(
                    !stats.disk_stats.used_percent.is_empty(),
                    "disk used_percent empty"
                );
                ensure!(!stats.memory_stats.total.is_empty(), "memory total empty");
                ensure!(!stats.memory_stats.free.is_empty(), "memory free empty");
                Ok(())
            })
        }),
        Trial::test("system::stats_has_runtime_metadata", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let stats = ctx().client.get_system_stats().await?;
                let meta = &stats.runtime_metadata;
                ensure!(!meta.rayhunter_version.is_empty(), "version empty");
                ensure!(!meta.system_os.is_empty(), "system_os empty");
                ensure!(!meta.arch.is_empty(), "arch empty");
                Ok(())
            })
        }),
        Trial::test("system::time_offset_roundtrip", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let before = ctx().client.get_time().await?;
                let original_offset = before.offset_seconds;

                let test_offset = original_offset + 3600;
                ctx().client.set_time_offset(test_offset).await?;

                let after = ctx().client.get_time().await?;
                ensure!(
                    after.offset_seconds == test_offset,
                    "offset not applied: expected {test_offset}, got {}",
                    after.offset_seconds
                );
                ensure!(
                    after.adjusted_time != after.system_time,
                    "adjusted_time equals system_time despite non-zero offset"
                );

                ctx().client.set_time_offset(original_offset).await?;

                let restored = ctx().client.get_time().await?;
                ensure!(
                    restored.offset_seconds == original_offset,
                    "offset not restored: expected {original_offset}, got {}",
                    restored.offset_seconds
                );
                Ok(())
            })
        }),
        Trial::test("system::invalid_time_offset_body_returns_422", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx().client.post_time_offset_raw("not valid json").await?;
                ensure!(resp.status() == 422, "expected 422, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("system::log_returns_text", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let log = ctx().client.get_log().await?;
                if log.is_empty() {
                    bail!("log endpoint returned empty body");
                }
                Ok(())
            })
        }),
    ]
}
