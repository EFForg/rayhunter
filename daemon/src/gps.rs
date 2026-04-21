use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use log::{error, info, warn};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::config::GpsMode;
use crate::server::ServerState;

fn deserialize_latitude<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    let v = f64::deserialize(deserializer)?;
    if !(-90.0..=90.0).contains(&v) {
        return Err(de::Error::custom(format!(
            "latitude {v} out of range [-90, 90]"
        )));
    }
    Ok(v)
}

fn deserialize_longitude<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    let v = f64::deserialize(deserializer)?;
    if !(-180.0..=180.0).contains(&v) {
        return Err(de::Error::custom(format!(
            "longitude {v} out of range [-180, 180]"
        )));
    }
    Ok(v)
}

fn deserialize_unix_ts<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    use serde_json::Value;
    match Value::deserialize(deserializer)? {
        Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_f64().map(|f| f as i64))
            .ok_or_else(|| de::Error::custom("timestamp out of range")),
        Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map(|f| f as i64)
            .map_err(|_| de::Error::custom("timestamp must be a numeric value")),
        _ => Err(de::Error::custom(
            "timestamp must be a number or numeric string",
        )),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpsData {
    #[serde(deserialize_with = "deserialize_latitude")]
    pub latitude: f64,
    #[serde(deserialize_with = "deserialize_longitude")]
    pub longitude: f64,
    #[serde(deserialize_with = "deserialize_unix_ts")]
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GpsRecord {
    pub unix_ts: i64,
    pub lat: f64,
    pub lon: f64,
}

/// Reads all GPS records from a sidecar NDJSON file, logging and skipping malformed lines.
pub async fn load_gps_records(file: tokio::fs::File) -> Vec<GpsRecord> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut records = Vec::new();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) => match serde_json::from_str::<GpsRecord>(&line) {
                Ok(record) => records.push(record),
                Err(e) => warn!("skipping malformed GPS sidecar line: {e}"),
            },
            Ok(None) => break,
            Err(e) => {
                error!("error reading GPS sidecar file: {e}");
                break;
            }
        }
    }
    records
}

pub async fn post_gps(
    State(state): State<Arc<ServerState>>,
    Json(gps_data): Json<GpsData>,
) -> Result<StatusCode, (StatusCode, String)> {
    if state.config.gps_mode != GpsMode::Api {
        return Err((
            StatusCode::FORBIDDEN,
            "GPS API endpoint is disabled. Set gps_mode to API endpoint in configuration."
                .to_string(),
        ));
    }
    let mut gps = state.gps_state.write().await;
    *gps = Some(gps_data.clone());
    drop(gps);

    let qmdl_store = state.qmdl_store_lock.read().await;
    if let Some((entry_idx, _)) = qmdl_store.get_current_entry() {
        match qmdl_store.open_entry_gps_for_append(entry_idx).await {
            Ok(Some(mut file)) => {
                let record = GpsRecord {
                    unix_ts: gps_data.timestamp,
                    lat: gps_data.latitude,
                    lon: gps_data.longitude,
                };
                let json = serde_json::to_string(&record).map_err(|e| {
                    error!("failed to serialize GPS record: {e}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("failed to serialize GPS record: {e}"),
                    )
                })?;
                file.write_all(format!("{json}\n").as_bytes())
                    .await
                    .map_err(|e| {
                        error!("failed to write GPS record to sidecar: {e}");
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("failed to write GPS record to sidecar: {e}"),
                        )
                    })?;
            }
            Ok(None) => error!("GPS sidecar directory not found, cannot write GPS record"),
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to open GPS sidecar: {e}"),
                ));
            }
        }
    } else {
        info!("GPS data received but no recording is active — position updated in memory only, not persisted to sidecar");
    }

    Ok(StatusCode::OK)
}

pub async fn get_gps(State(state): State<Arc<ServerState>>) -> Result<Json<GpsData>, StatusCode> {
    let gps = state.gps_state.read().await;
    match gps.as_ref() {
        Some(data) => Ok(Json(data.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
