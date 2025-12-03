use std::sync::Arc;

use crate::battery::get_battery_status;
use crate::error::RayhunterError;
use crate::server::ServerState;
use crate::{battery::BatteryState, qmdl_store::ManifestEntry};

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Duration;
use log::error;
use rayhunter::{Device, util::RuntimeMetadata};
use serde::Serialize;
use tokio::process::Command;

#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub disk_stats: DiskStats,
    pub memory_stats: MemoryStats,
    pub runtime_metadata: RuntimeMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<BatteryState>,
}

impl SystemStats {
    pub async fn new(qmdl_path: &str, device: &Device) -> Result<Self, String> {
        Ok(Self {
            disk_stats: DiskStats::new(qmdl_path, device).await?,
            memory_stats: MemoryStats::new(device).await?,
            runtime_metadata: RuntimeMetadata::new(),
            battery_status: match get_battery_status(device).await {
                Ok(status) => Some(status),
                Err(RayhunterError::FunctionNotSupportedForDeviceError) => None,
                Err(err) => {
                    log::error!("Failed to get battery status: {err}");
                    None
                }
            },
        })
    }
}

#[derive(Debug, Serialize)]
pub struct DiskStats {
    partition: String,
    total_size: String,
    used_size: String,
    available_size: String,
    used_percent: String,
    mounted_on: String,
}

impl DiskStats {
    // runs "df -h <qmdl_path>" to get storage statistics for the partition containing
    // the QMDL file.
    pub async fn new(qmdl_path: &str, device: &Device) -> Result<Self, String> {
        // Uz801 needs to be told to use the busybox df specifically
        let mut df_cmd: Command;
        if matches!(device, Device::Uz801) {
            df_cmd = Command::new("busybox");
            df_cmd.arg("df");
        } else {
            df_cmd = Command::new("df");
        }
        df_cmd.arg("-h");
        df_cmd.arg(qmdl_path);
        let stdout = get_cmd_output(df_cmd).await?;

        // Handle standard df -h format
        let mut parts = stdout.split_whitespace().skip(7);
        Ok(Self {
            partition: parts.next().ok_or("error parsing df output")?.to_string(),
            total_size: parts.next().ok_or("error parsing df output")?.to_string(),
            used_size: parts.next().ok_or("error parsing df output")?.to_string(),
            available_size: parts.next().ok_or("error parsing df output")?.to_string(),
            used_percent: parts.next().ok_or("error parsing df output")?.to_string(),
            mounted_on: parts.next().ok_or("error parsing df output")?.to_string(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MemoryStats {
    total: String,
    used: String,
    free: String,
}

// runs the given command and returns its stdout as a string
async fn get_cmd_output(mut cmd: Command) -> Result<String, String> {
    let cmd_str = format!("{:?}", &cmd);
    let output = cmd
        .output()
        .await
        .map_err(|e| format!("error running command {}: {}", &cmd_str, e))?;
    if !output.status.success() {
        return Err(format!(
            "command {} failed with exit code {}",
            &cmd_str,
            output.status.code().unwrap()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

impl MemoryStats {
    // runs "free -k" and parses the output to retrieve memory stats for most devices,
    pub async fn new(device: &Device) -> Result<Self, String> {
        // Use busybox for Uz801
        let mut free_cmd: Command;
        if matches!(device, Device::Uz801) {
            free_cmd = Command::new("busybox");
            free_cmd.arg("free");
        } else {
            free_cmd = Command::new("free");
        }
        free_cmd.arg("-k");
        let stdout = get_cmd_output(free_cmd).await?;
        let mut numbers = stdout
            .split_whitespace()
            .flat_map(|part| part.parse::<usize>());
        Ok(Self {
            total: humanize_kb(numbers.next().ok_or("error parsing free output")?),
            used: humanize_kb(numbers.next().ok_or("error parsing free output")?),
            free: humanize_kb(numbers.next().ok_or("error parsing free output")?),
        })
    }
}

// turns a number of kilobytes (like 28293) into a human-readable string (like "28.3M")
fn humanize_kb(kb: usize) -> String {
    if kb < 1000 {
        return format!("{kb}K");
    }
    format!("{:.1}M", kb as f64 / 1024.0)
}

pub async fn get_system_stats(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<SystemStats>, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    match SystemStats::new(qmdl_store.path.to_str().unwrap(), &state.config.device).await {
        Ok(stats) => Ok(Json(stats)),
        Err(err) => {
            error!("error getting system stats: {err}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error getting system stats".to_string(),
            ))
        }
    }
}

#[derive(Serialize)]
pub struct ManifestStats {
    pub entries: Vec<ManifestEntry>,
    pub current_entry: Option<ManifestEntry>,
}

/// Apply time correction to a ManifestEntry's timestamps
fn apply_time_correction(mut entry: ManifestEntry, offset_seconds: i64) -> ManifestEntry {
    let duration = Duration::seconds(offset_seconds);
    entry.start_time = entry.start_time + duration;
    entry.last_message_time = entry.last_message_time.map(|t| t + duration);
    entry
}

pub async fn get_qmdl_manifest(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<ManifestStats>, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    let time_correction = state.time_correction.read().await;
    let offset_seconds = time_correction.offset_seconds();

    let mut entries = qmdl_store.manifest.entries.clone();
    let current_entry = qmdl_store.current_entry.map(|index| entries.remove(index));

    // Apply time correction to all entries
    let entries = entries
        .into_iter()
        .map(|entry| apply_time_correction(entry, offset_seconds))
        .collect();
    let current_entry = current_entry.map(|entry| apply_time_correction(entry, offset_seconds));

    Ok(Json(ManifestStats {
        entries,
        current_entry,
    }))
}

pub async fn get_log() -> Result<String, (StatusCode, String)> {
    tokio::fs::read_to_string("/data/rayhunter/rayhunter.log")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, TimeZone};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[test]
    fn test_apply_time_correction_positive_offset() {
        let start_time = Local.with_ymd_and_hms(2025, 11, 30, 10, 0, 0).unwrap();
        let entry = ManifestEntry {
            name: "test".to_string(),
            start_time,
            last_message_time: Some(start_time),
            qmdl_size_bytes: 0,
            rayhunter_version: None,
            system_os: None,
            arch: None,
        };

        // Apply 1 hour offset
        let corrected = apply_time_correction(entry.clone(), 3600);

        assert_eq!(
            corrected.start_time,
            start_time + Duration::seconds(3600)
        );
        assert_eq!(
            corrected.last_message_time,
            Some(start_time + Duration::seconds(3600))
        );
    }

    #[test]
    fn test_apply_time_correction_negative_offset() {
        let start_time = Local.with_ymd_and_hms(2025, 11, 30, 10, 0, 0).unwrap();
        let entry = ManifestEntry {
            name: "test".to_string(),
            start_time,
            last_message_time: None,
            qmdl_size_bytes: 0,
            rayhunter_version: None,
            system_os: None,
            arch: None,
        };

        // Apply -1 hour offset
        let corrected = apply_time_correction(entry.clone(), -3600);

        assert_eq!(
            corrected.start_time,
            start_time + Duration::seconds(-3600)
        );
        assert_eq!(corrected.last_message_time, None);
    }

    #[test]
    fn test_apply_time_correction_zero_offset() {
        let start_time = Local.with_ymd_and_hms(2025, 11, 30, 10, 0, 0).unwrap();
        let entry = ManifestEntry {
            name: "test".to_string(),
            start_time,
            last_message_time: Some(start_time),
            qmdl_size_bytes: 0,
            rayhunter_version: None,
            system_os: None,
            arch: None,
        };

        // Apply zero offset
        let corrected = apply_time_correction(entry.clone(), 0);

        assert_eq!(corrected.start_time, start_time);
        assert_eq!(corrected.last_message_time, Some(start_time));
    }

    #[tokio::test]
    async fn test_get_qmdl_manifest_applies_time_correction() {
        use crate::qmdl_store::RecordingStore;
        use crate::time_correction::TimeCorrection;
        use tempfile::TempDir;
        use tokio_util::sync::CancellationToken;

        // Create a temporary QMDL store with test entries
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().to_path_buf();
        let mut store = RecordingStore::create(&store_path).await.unwrap();

        // Create a test entry
        let (_qmdl_file, _analysis_file) = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        let original_start_time = store.manifest.entries[entry_index].start_time;
        store.close_current_entry().await.unwrap();

        let store_lock = Arc::new(RwLock::new(store));

        // Create a time correction with a 1-hour offset
        let mut time_correction = TimeCorrection::new();
        let one_hour_future = (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp_millis();
        time_correction.set_from_browser(one_hour_future);
        let offset = time_correction.offset_seconds();

        // Create test server state
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let (analysis_tx, _analysis_rx) = tokio::sync::mpsc::channel(1);

        let analysis_status = {
            let store = store_lock.try_read().unwrap();
            crate::analysis::AnalysisStatus::new(&store)
        };

        let state = Arc::new(ServerState {
            config_path: "/tmp/test_config.toml".to_string(),
            config: crate::config::Config::default(),
            qmdl_store_lock: store_lock,
            diag_device_ctrl_sender: tx,
            analysis_status_lock: Arc::new(RwLock::new(analysis_status)),
            analysis_sender: analysis_tx,
            daemon_restart_token: CancellationToken::new(),
            ui_update_sender: None,
            time_correction: Arc::new(RwLock::new(time_correction)),
        });

        // Call get_qmdl_manifest
        let result = get_qmdl_manifest(State(state)).await;
        assert!(result.is_ok());

        let manifest = result.unwrap().0;

        // Verify that time correction was applied to entries
        assert!(!manifest.entries.is_empty());
        let corrected_entry = &manifest.entries[0];

        // The corrected time should be approximately 1 hour ahead of the original time
        let expected_time = original_start_time + Duration::seconds(offset);
        assert_eq!(corrected_entry.start_time, expected_time);
    }
}
