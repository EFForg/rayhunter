use std::ffi::CString;
use std::sync::Arc;

use crate::battery::get_battery_status;
use crate::error::RayhunterError;
use crate::server::ServerState;
use crate::{battery::BatteryState, qmdl_store::ManifestEntry};

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
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
            disk_stats: DiskStats::new(qmdl_path)?,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_bytes: Option<u64>,
}

impl DiskStats {
    #[allow(clippy::unnecessary_cast)] // c_ulong is u32 on ARM, u64 on macOS
    pub fn new(qmdl_path: &str) -> Result<Self, String> {
        let c_path =
            CString::new(qmdl_path).map_err(|e| format!("invalid path {qmdl_path}: {e}"))?;
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
            return Err(format!(
                "statvfs({qmdl_path}) failed: {}",
                std::io::Error::last_os_error()
            ));
        }

        let block_size = stat.f_frsize as u64;
        let total_kb = (stat.f_blocks as u64 * block_size / 1024) as usize;
        let free_kb = (stat.f_bfree as u64 * block_size / 1024) as usize;
        let available_kb = (stat.f_bavail as u64 * block_size / 1024) as usize;
        let used_kb = total_kb.saturating_sub(free_kb);
        let used_percent = if stat.f_blocks > 0 {
            format!("{}%", (stat.f_blocks - stat.f_bfree) * 100 / stat.f_blocks)
        } else {
            "0%".to_string()
        };

        Ok(Self {
            partition: qmdl_path.to_string(),
            total_size: humanize_kb(total_kb),
            used_size: humanize_kb(used_kb),
            available_size: humanize_kb(available_kb),
            used_percent,
            mounted_on: qmdl_path.to_string(),
            available_bytes: Some(stat.f_bavail as u64 * block_size),
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

pub async fn get_qmdl_manifest(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<ManifestStats>, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    let mut entries = qmdl_store.manifest.entries.clone();
    let current_entry = qmdl_store.current_entry.map(|index| entries.remove(index));
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
