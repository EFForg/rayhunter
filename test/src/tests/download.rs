use anyhow::ensure;
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run, run_slow};

const HDLC_TERMINATOR: u8 = 0x7e;
const PCAPNG_MAGIC: [u8; 4] = [0x0a, 0x0d, 0x0d, 0x0a];
const ZIP_MAGIC: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let http = caps.http;
    let can_record = caps.recording;
    vec![
        Trial::test("download::qmdl_pcap_and_zip", move || {
            run_slow(async {
                ensure!(http && can_record, "requires HTTP + recording capability");
                ctx()
                    .client
                    .with_recording(|name| async move {
                        let client = &ctx().client;

                        let qmdl = client.get_qmdl(&name).await?;
                        ensure!(!qmdl.is_empty(), "QMDL download returned empty body");
                        ensure!(
                            qmdl.contains(&HDLC_TERMINATOR),
                            "QMDL has no HDLC terminators (0x7e) — {} bytes, no frames",
                            qmdl.len()
                        );

                        let pcap = client.get_pcap(&name).await?;
                        ensure!(!pcap.is_empty(), "PCAP download returned empty body");
                        ensure!(
                            pcap.len() >= 4 && pcap[..4] == PCAPNG_MAGIC,
                            "PCAP missing PCAPng magic (expected 0a0d0d0a, got {:02x?})",
                            &pcap[..pcap.len().min(4)]
                        );

                        let zip = client.get_zip(&name).await?;
                        ensure!(!zip.is_empty(), "ZIP download returned empty body");
                        ensure!(
                            zip.len() >= 4 && zip[..4] == ZIP_MAGIC,
                            "not a valid ZIP (expected PK header, got {:02x?})",
                            &zip[..zip.len().min(4)]
                        );
                        ensure!(
                            find_in_bytes(&zip, b".qmdl"),
                            "ZIP does not contain a .qmdl file"
                        );
                        ensure!(
                            find_in_bytes(&zip, b".pcapng"),
                            "ZIP does not contain a .pcapng file"
                        );
                        Ok(())
                    })
                    .await
            })
        })
        .with_ignored_flag(!can_record),
        Trial::test("download::nonexistent_qmdl_returns_404", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx().client.get_qmdl_raw("nonexistent_name").await?;
                ensure!(resp.status() == 404, "expected 404, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("download::nonexistent_pcap_returns_404", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx().client.get_pcap_raw("nonexistent_name").await?;
                ensure!(resp.status() == 404, "expected 404, got {}", resp.status());
                Ok(())
            })
        }),
        Trial::test("download::nonexistent_zip_returns_404", move || {
            run(async {
                ensure!(http, "no HTTP access");
                let resp = ctx().client.get_zip_raw("nonexistent_name").await?;
                ensure!(resp.status() == 404, "expected 404, got {}", resp.status());
                Ok(())
            })
        }),
    ]
}

fn find_in_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
