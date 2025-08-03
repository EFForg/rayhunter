use std::sync::Arc;

use crate::qmdl_store::ManifestEntry;
use crate::server::ServerState;

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
}

impl SystemStats {
    pub async fn new(qmdl_path: &str, device: &Device) -> Result<Self, String> {
        Ok(Self {
            disk_stats: DiskStats::new(qmdl_path, device).await?,
            memory_stats: MemoryStats::new(device).await?,
            runtime_metadata: RuntimeMetadata::new(),
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
    // the QMDL file. The Uz801 device doesn't support the -h flag, so we skip it for that device.
    pub async fn new(qmdl_path: &str, device: &Device) -> Result<Self, String> {
        let mut df_cmd = Command::new("df");
        // Only add -h flag for devices other than Uz801, as Uz801's df doesn't support it
        if !matches!(device, Device::Uz801) {
            df_cmd.arg("-h");
        }
        df_cmd.arg(qmdl_path);
        let stdout = get_cmd_output(df_cmd).await?;
        
        if matches!(device, Device::Uz801) {
            // Handle Uz801 format:
            // Filesystem               Size     Used     Free   Blksize
            // /data/rayhunter/       774.9M    68.0M   706.9M   4096
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() < 2 {
                return Err("error parsing df output: insufficient lines".to_string());
            }
            let data_line = lines[1];
            let mut parts = data_line.split_whitespace();
            Ok(Self {
                partition: parts.next().ok_or("error parsing df output: missing filesystem")?.to_string(),
                total_size: parts.next().ok_or("error parsing df output: missing size")?.to_string(),
                used_size: parts.next().ok_or("error parsing df output: missing used")?.to_string(),
                available_size: parts.next().ok_or("error parsing df output: missing free")?.to_string(),
                used_percent: "N/A".to_string(), // Uz801 df doesn't provide percentage
                mounted_on: qmdl_path.to_string(), // Use the path we queried
            })
        } else {
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
    // or reads /proc/meminfo for Uz801 which doesn't have the free command
    pub async fn new(device: &Device) -> Result<Self, String> {
        if matches!(device, Device::Uz801) {
            // Read /proc/meminfo for Uz801
            let meminfo_content = tokio::fs::read_to_string("/proc/meminfo")
                .await
                .map_err(|e| format!("error reading /proc/meminfo: {}", e))?;
            
            let mut mem_total_kb = None;
            let mut mem_free_kb = None;
            
            for line in meminfo_content.lines() {
                if let Some(value_str) = line.strip_prefix("MemTotal:") {
                    if let Some(kb_str) = value_str.trim().strip_suffix(" kB") {
                        mem_total_kb = kb_str.trim().parse::<usize>().ok();
                    }
                } else if let Some(value_str) = line.strip_prefix("MemFree:") {
                    if let Some(kb_str) = value_str.trim().strip_suffix(" kB") {
                        mem_free_kb = kb_str.trim().parse::<usize>().ok();
                    }
                }
            }
            
            let total = mem_total_kb.ok_or("error parsing MemTotal from /proc/meminfo")?;
            let free = mem_free_kb.ok_or("error parsing MemFree from /proc/meminfo")?;
            let used = total - free;
            
            Ok(Self {
                total: humanize_kb(total),
                used: humanize_kb(used),
                free: humanize_kb(free),
            })
        } else {
            // Use free command for other devices
            let mut free_cmd = Command::new("free");
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
