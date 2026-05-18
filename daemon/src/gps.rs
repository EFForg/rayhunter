use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use log::{error, warn};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::config::GpsMode;
use crate::diag::DiagDeviceCtrlMessage;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct GpsData {
    #[serde(deserialize_with = "deserialize_latitude")]
    pub latitude: f64,
    #[serde(deserialize_with = "deserialize_longitude")]
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GpsRecord {
    /// Packet timestamp (modem clock) for correlation with captured packets.
    /// None if no packets have been received yet.
    pub latest_packet_timestamp: Option<i64>,
    /// Drift-corrected system time when this GPS fix was received
    pub system_time: i64,
    pub lat: f64,
    pub lon: f64,
}

/// Reads all GPS records from a storage NDJSON file, logging and skipping malformed lines.
pub async fn load_gps_records(file: tokio::fs::File) -> Vec<GpsRecord> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut records = Vec::new();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) => match serde_json::from_str::<GpsRecord>(&line) {
                Ok(record) => records.push(record),
                Err(e) => warn!("skipping malformed GPS storage line: {e}"),
            },
            Ok(None) => break,
            Err(e) => {
                error!("error reading GPS storage file: {e}");
                break;
            }
        }
    }
    records.sort_by_key(|r| r.latest_packet_timestamp.unwrap_or(i64::MIN));
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
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to send GPS update")
    ),
    summary = "Submit GPS coordinates",
    description = "Submit GPS coordinates from an external source (e.g. a phone app). Requires gps_mode to be set to 'Api' in configuration. latitude is in decimal degrees from -90 to 90, longitude is in decimal degrees from -180 to 180. The timestamp is derived from the most recent packet's modem timestamp."
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

    // Update in-memory state for GET /api/gps
    let mut gps = state.gps_state.write().await;
    *gps = Some(gps_data.clone());
    drop(gps);

    // Send to DiagTask to write to storage with packet timestamp
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::GpsUpdate {
            lat: gps_data.latitude,
            lon: gps_data.longitude,
        })
        .await
        .map_err(|e| {
            error!("failed to send GPS update: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to send GPS update: {e}"),
            )
        })?;

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
