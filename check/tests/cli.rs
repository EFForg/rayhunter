//! End-to-end tests that run the built rayhunter-check binary, covering the
//! bits the unit tests can't: per-file error handling / exit code, and reading
//! legacy pcap. Inputs are built in a tempdir so nothing gets committed.

use std::process::Command;

use tempfile::TempDir;

fn check() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rayhunter-check"))
}

/// A dir with one good input and one broken one should still print the good
/// report and exit nonzero, not bail on the whole run.
#[test]
fn continues_past_a_bad_file_and_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    // empty qmdl parses as zero messages -> a report with no rows
    std::fs::write(dir.path().join("good.qmdl"), b"").unwrap();
    // not a pcap/pcapng magic, so the reader rejects it
    std::fs::write(dir.path().join("bad.pcap"), b"\x7fELF").unwrap();

    let output = check()
        .arg("--path")
        .arg(dir.path())
        .args(["--format", "json", "--quiet"])
        .output()
        .unwrap();

    // one input failed, so exit is nonzero...
    assert!(!output.status.success());
    // ...but the good one's report still lands on stdout as a json array

    let reports: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let reports = reports.as_array().expect("stdout should be a JSON array");
    assert_eq!(reports.len(), 1);
    assert!(reports[0]["path"].as_str().unwrap().ends_with("good.qmdl"));
}

/// Legacy pcap (not just pcapng) is accepted, detected by magic number rather
/// than by file extension.
#[test]
fn accepts_legacy_pcap_format() {
    let dir = TempDir::new().unwrap();
    // minimal libpcap global header (little-endian, microsecond, LINKTYPE_RAW),
    // no packets
    let mut pcap = Vec::new();
    pcap.extend_from_slice(&0xa1b2c3d4u32.to_le_bytes()); // magic
    pcap.extend_from_slice(&2u16.to_le_bytes()); // version major
    pcap.extend_from_slice(&4u16.to_le_bytes()); // version minor
    pcap.extend_from_slice(&0i32.to_le_bytes()); // thiszone
    pcap.extend_from_slice(&0u32.to_le_bytes()); // sigfigs
    pcap.extend_from_slice(&65535u32.to_le_bytes()); // snaplen
    pcap.extend_from_slice(&101u32.to_le_bytes()); // network (LINKTYPE_RAW)
    let path = dir.path().join("legacy.pcap");
    std::fs::write(&path, pcap).unwrap();

    let output = check()
        .arg("--path")
        .arg(&path)
        .args(["--format", "json", "--quiet"])
        .output()
        .unwrap();

    // accepted (not rejected as an unknown format): clean exit, and it produces
    // a report (no rows, since there are no packets)
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reports: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let reports = reports.as_array().expect("stdout should be a JSON array");
    assert_eq!(reports.len(), 1);
    assert!(
        reports[0]["path"]
            .as_str()
            .unwrap()
            .ends_with("legacy.pcap")
    );
}
