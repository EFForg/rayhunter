use rayhunter::{
    diag::{CRC_CCITT, DataType, HdlcEncapsulatedMessage, MessagesContainer},
    hdlc::hdlc_encapsulate,
    qmdl::QmdlWriter,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

// Helper to create a minimal test QMDL file
async fn create_test_qmdl(path: &PathBuf) {
    let file = tokio::fs::File::create(path).await.unwrap();
    let mut writer = QmdlWriter::new(file);
    // Create a simple test message
    let data = hdlc_encapsulate(&[0x10u8; 10], &CRC_CCITT);
    let message = HdlcEncapsulatedMessage {
        len: data.len() as u32,
        data,
    };

    let container = MessagesContainer {
        data_type: DataType::UserSpace,
        num_messages: 1,
        messages: vec![message],
    };

    writer.write_container(&container).await.unwrap();
}

fn get_check_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../target/debug/rayhunter-check");
    path
}

#[tokio::test]
async fn test_ndjson_output_to_stdout() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args(["-p", qmdl_path.to_str().unwrap(), "-r", "ndjson"])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // NDJSON output should be on stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "stdout should contain NDJSON output");

    // First line should be metadata
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty(), "should have at least metadata line");

    // Verify it's valid JSON
    let first_line: serde_json::Value = serde_json::from_str(lines[0])
        .expect("First line should be valid JSON");
    assert!(first_line.get("analyzers").is_some(), "First line should contain analyzers metadata");
}

#[tokio::test]
async fn test_ndjson_output_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir).unwrap();

    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args([
            "-p", qmdl_path.to_str().unwrap(),
            "-r", "ndjson",
            "-o", output_dir.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // NDJSON file should be created
    let ndjson_path = output_dir.join("test.ndjson");
    assert!(ndjson_path.exists(), "NDJSON file should be created");

    // Verify file contents
    let contents = fs::read_to_string(ndjson_path).unwrap();
    assert!(!contents.is_empty(), "NDJSON file should not be empty");

    // Verify first line is metadata
    let lines: Vec<&str> = contents.lines().collect();
    let first_line: serde_json::Value = serde_json::from_str(lines[0])
        .expect("First line should be valid JSON");
    assert!(first_line.get("analyzers").is_some());
}

#[tokio::test]
async fn test_no_files_created_without_output_flag() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args(["-p", qmdl_path.to_str().unwrap(), "-r", "ndjson"])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // No .ndjson file should be created next to the input file
    let ndjson_path = temp_dir.path().join("test.ndjson");
    assert!(!ndjson_path.exists(), "NDJSON file should not be created without --output");
}

#[tokio::test]
async fn test_pcapify_requires_output() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args(["-p", qmdl_path.to_str().unwrap(), "-P"])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(!output.status.success(), "Should fail when --pcapify is used without --output");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("pcapify requires --output"),
            "Error message should mention --output requirement");
}

#[tokio::test]
async fn test_pcapify_creates_pcap_file() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir).unwrap();

    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args([
            "-p", qmdl_path.to_str().unwrap(),
            "-P",
            "-o", output_dir.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // PCAP file should be created
    let pcap_path = output_dir.join("test.pcapng");
    assert!(pcap_path.exists(), "PCAP file should be created with --pcapify");
}

#[tokio::test]
async fn test_show_skipped_flag() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    create_test_qmdl(&qmdl_path).await;

    // Run with --show-skipped
    let output_with = Command::new(get_check_binary())
        .args([
            "-p", qmdl_path.to_str().unwrap(),
            "-r", "ndjson",
            "--show-skipped"
        ])
        .output()
        .expect("Failed to execute rayhunter-check");

    // Run without --show-skipped
    let output_without = Command::new(get_check_binary())
        .args([
            "-p", qmdl_path.to_str().unwrap(),
            "-r", "ndjson"
        ])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output_with.status.success());
    assert!(output_without.status.success());

    // Both should produce output (at minimum, metadata line)
    let stdout_with = String::from_utf8_lossy(&output_with.stdout);
    let stdout_without = String::from_utf8_lossy(&output_without.stdout);

    assert!(!stdout_with.is_empty());
    assert!(!stdout_without.is_empty());
}

#[tokio::test]
async fn test_log_format_default() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args(["-p", qmdl_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // Log format outputs to stderr (all logs go to stderr)
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Analyzers:") || stderr.contains("Beginning analysis"),
            "Should contain log output on stderr");

    // Stdout should be empty for log format
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty() || stdout.trim().is_empty(),
            "stdout should be empty for log format");
}

#[tokio::test]
async fn test_output_directory_created() {
    let temp_dir = TempDir::new().unwrap();
    let qmdl_path = temp_dir.path().join("test.qmdl");
    let output_dir = temp_dir.path().join("nonexistent/nested/output");

    create_test_qmdl(&qmdl_path).await;

    let output = Command::new(get_check_binary())
        .args([
            "-p", qmdl_path.to_str().unwrap(),
            "-r", "ndjson",
            "-o", output_dir.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute rayhunter-check");

    assert!(output.status.success());

    // Output directory should be created
    assert!(output_dir.exists(), "Output directory should be created automatically");

    // NDJSON file should be created in the directory
    let ndjson_path = output_dir.join("test.ndjson");
    assert!(ndjson_path.exists(), "NDJSON file should be created in new directory");
}
