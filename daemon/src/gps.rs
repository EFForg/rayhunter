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
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
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
    records.sort_by_key(|r| r.unix_ts);
    records
}

/// Submit GPS coordinates
#[cfg_attr(feature = "apidocs", utoipa::path(
    post,
    path = "/api/gps",
    tag = "Configuration",
    request_body = GpsData,
    responses(
        (status = StatusCode::OK, description = "GPS data accepted"),
        (status = StatusCode::FORBIDDEN, description = "GPS API endpoint is disabled"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to write GPS record")
    ),
    summary = "Submit GPS coordinates",
    description = "Submit GPS coordinates from an external source (e.g. a phone app). Requires gps_mode to be set to 'Api' in configuration. latitude is in decimal degrees from -90 to 90, longitude is in decimal degrees from -180 to 180, timestamp is a Unix timestamp in seconds."
))]
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
                let mut json = serde_json::to_vec(&record).map_err(|e| {
                    error!("failed to serialize GPS record: {e}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("failed to serialize GPS record: {e}"),
                    )
                })?;
                json.push(b'\n');
                file.write_all(&json).await.map_err(|e| {
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
        info!(
            "GPS data received but no recording is active — position updated in memory only, not persisted to sidecar"
        );
    }

    Ok(StatusCode::OK)
}

/// Get the current GPS coordinates
#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/gps",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Current GPS data", body = GpsData),
        (status = StatusCode::NOT_FOUND, description = "No GPS data available")
    ),
    summary = "Get current GPS coordinates",
    description = "Returns the most recently submitted GPS coordinates. Returns 404 if no coordinates have been submitted yet this session."
))]
pub async fn get_gps(State(state): State<Arc<ServerState>>) -> Result<Json<GpsData>, StatusCode> {
    let gps = state.gps_state.read().await;
    match gps.as_ref() {
        Some(data) => Ok(Json(data.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
