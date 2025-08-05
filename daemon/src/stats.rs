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
