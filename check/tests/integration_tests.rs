use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_cli_json_format_flag_exists() {
    // Get the path to the compiled binary
    let binary_path = env!("CARGO_BIN_EXE_rayhunter-check");

    // Run the command with --help to verify --format flag exists
    let output = Command::new(binary_path)
        .arg("--help")
        .output()
        .expect("Failed to execute rayhunter-check");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("Output format"));
    assert!(stdout.contains("text"));
    assert!(stdout.contains("json"));
}

#[test]
fn test_cli_format_flag_defaults_to_text() {
    // Verify the default value is 'text'
    let binary_path = env!("CARGO_BIN_EXE_rayhunter-check");

    let output = Command::new(binary_path)
        .arg("--help")
        .output()
        .expect("Failed to execute rayhunter-check");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("default: text") || stdout.contains("[default: text]"));
}

#[test]
fn test_cli_accepts_json_format() {
    // This test verifies the CLI accepts --format json without erroring
    // We use a non-existent path to avoid actually processing files
    let binary_path = env!("CARGO_BIN_EXE_rayhunter-check");
    let temp_dir = TempDir::new().unwrap();
    let non_existent_path = temp_dir.path().join("nonexistent");

    // Create an empty directory for testing
    std::fs::create_dir(&non_existent_path).unwrap();

    let output = Command::new(binary_path)
        .arg("-p")
        .arg(&non_existent_path)
        .arg("--format")
        .arg("json")
        .arg("--quiet")
        .output()
        .expect("Failed to execute rayhunter-check");

    // Should succeed (exit code 0) even with no files
    assert!(output.status.success());
}

#[test]
fn test_cli_rejects_invalid_format() {
    let binary_path = env!("CARGO_BIN_EXE_rayhunter-check");
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path)
        .arg("-p")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("invalid")
        .output()
        .expect("Failed to execute rayhunter-check");

    // Should fail with invalid format
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("invalid") || stderr.contains("format"));
}

