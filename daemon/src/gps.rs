use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::server::ServerState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct GpsRecord {
    pub unix_ts: u32,
    pub lat: f64,
    pub lon: f64,
}

/// Reads all GPS records from a sidecar file, skipping malformed lines.
pub async fn load_gps_records(file: tokio::fs::File) -> Vec<GpsRecord> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut records = Vec::new();
    while let Ok(Some(line)) = lines.next_line().await {
        if let Ok(record) = serde_json::from_str::<GpsRecord>(&line) {
            records.push(record);
        }
    }
    records
}

pub async fn post_gps(
    State(state): State<Arc<ServerState>>,
    Json(gps_data): Json<GpsData>,
) -> Result<StatusCode, (StatusCode, String)> {
    if state.config.gps_mode != 2 {
        return Err((
            StatusCode::FORBIDDEN,
            "GPS API endpoint is disabled. Set gps_mode to 2 in configuration.".to_string(),
        ));
    }
    let mut gps = state.gps_state.write().await;
    *gps = Some(gps_data.clone());
    drop(gps);

    // Append the GPS fix to the current recording's sidecar file.
    let qmdl_store = state.qmdl_store_lock.read().await;
    if let Some((entry_idx, _)) = qmdl_store.get_current_entry() {
        if let Ok(mut file) = qmdl_store.open_entry_gps_for_append(entry_idx).await {
            let unix_ts = chrono::Utc::now().timestamp() as u32;
            let record = GpsRecord {
                unix_ts,
                lat: gps_data.latitude,
                lon: gps_data.longitude,
            };
            if let Ok(json) = serde_json::to_string(&record) {
                let _ = file.write_all(format!("{json}\n").as_bytes()).await;
            }
        }
    }

    Ok(StatusCode::OK)
}

pub async fn get_gps(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<GpsData>, StatusCode> {
    let gps = state.gps_state.read().await;
    match gps.as_ref() {
        Some(data) => Ok(Json(data.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
