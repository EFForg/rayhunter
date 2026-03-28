use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::server::ServerState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
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
    *gps = Some(gps_data);
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
