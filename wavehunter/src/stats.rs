use crate::server::ServerState;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use log::error;
use serde::Serialize;
use tokio::process::Command;

#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub disk_stats: DiskStats,
    pub memory_stats: MemoryStats,
}

impl SystemStats {
    pub async fn new(qmdl_path: &str) -> Result<Self, String> {
        Ok(Self {
            disk_stats: DiskStats::new(qmdl_path).await?,
            memory_stats: MemoryStats::new().await?,
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
    // the QMDL file
    pub async fn new(qmdl_path: &str) -> Result<Self, String> {
        let mut df_cmd = Command::new("df");
        df_cmd.arg("-h");
        df_cmd.arg(qmdl_path);
        let stdout = get_cmd_output(df_cmd).await?;
        let mut parts = stdout.split_whitespace().skip(7).to_owned();
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
    let output = cmd.output().await
        .map_err(|e| format!("error running command {}: {}", &cmd_str, e))?;
    if !output.status.success() {
        return Err(format!("command {} failed with exit code {}", &cmd_str, output.status.code().unwrap()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

impl MemoryStats {
    // runs "free -k" and parses the output to retrieve memory stats
    pub async fn new() -> Result<Self, String> {
        let mut free_cmd = Command::new("free");
        free_cmd.arg("-k");
        let stdout = get_cmd_output(free_cmd).await?;
        let mut numbers = stdout.split_whitespace()
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
    if kb < 1000{
        return format!("{}K", kb);
    }
    format!("{:.1}M", kb as f64 / 1024.0)
}

pub async fn get_system_stats(State(state): State<Arc<ServerState>>) -> Result<Json<SystemStats>, (StatusCode, String)> {
    match SystemStats::new(&state.qmdl_path).await {
        Ok(stats) => Ok(Json(stats)),
        Err(err) => {
            error!("error getting system stats: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error getting system stats".to_string()
            ));
        },
    }
}

#[derive(Debug, Serialize)]
pub struct DiagStats {
    bytes_written: usize,
}

pub async fn get_diag_stats(State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    Json(DiagStats {
        bytes_written: *state.qmdl_bytes_written.read().await,
    })
}
