//! GPS data handling module
//! 
//! This module provides functionality to receive GPS coordinates via REST API
//! and save them to per-scan GPS files for tracking location data.

use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Json, Response},
    body::Body,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::{create_dir_all, OpenOptions, File};
use tokio::io::AsyncWriteExt;
use log::{info, error};

use crate::server::ServerState;

/// GPS coordinate data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinate {
    pub timestamp: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

/// Response structure for GPS API calls
#[derive(Serialize)]
pub struct GpsResponse {
    pub status: String,
    pub message: String,
    pub data: GpsCoordinate,
}

/// Error response structure
#[derive(Serialize)]
pub struct GpsError {
    pub status: String,
    pub error: String,
}

/// Per-scan GPS file writer
pub struct GpsWriter {
    file: File,
    #[allow(dead_code)]
    scan_id: String,
}

impl GpsWriter {
    /// Create a new GPS writer for a specific scan
    pub async fn new(scan_id: String, base_path: &str) -> Result<Self, std::io::Error> {
        let gps_path = std::path::Path::new(base_path).join(format!("{}.gps", scan_id));
        let file = File::create(gps_path).await?;
        Ok(GpsWriter { file, scan_id })
    }

    /// Write GPS coordinate to the scan's GPS file
    pub async fn write_coordinate(&mut self, coord: &GpsCoordinate) -> Result<(), std::io::Error> {
        let gps_line = format!("{}, {}, {}\n", 
                              coord.timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                              coord.latitude, 
                              coord.longitude);
        self.file.write_all(gps_line.as_bytes()).await?;
        self.file.flush().await?;
        Ok(())
    }

    /// Close the GPS file
    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.file.flush().await?;
        Ok(())
    }
}

// Legacy GPS data storage has been removed - all GPS data now uses per-scan files with UNIX timestamps

/// Parse latitude and longitude from path parameter
fn parse_coordinates(lat_lon: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = lat_lon.split(',').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Expected: lat,lon".to_string());
    }

    let latitude = parts[0].parse::<f64>()
        .map_err(|_| "Invalid latitude format".to_string())?;
    let longitude = parts[1].parse::<f64>()
        .map_err(|_| "Invalid longitude format".to_string())?;

    // Basic validation for GPS coordinates
    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90 degrees".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180 degrees".to_string());
    }

    Ok((latitude, longitude))
}

/// API handler for receiving GPS coordinates
/// Accepts both GET and POST requests to /api/v1/gps/{lat,lon}
/// Compatible with GPS2REST-Android app which sends GET requests
pub async fn receive_gps_coordinate(
    State(state): State<Arc<ServerState>>,
    Path(lat_lon): Path<String>,
) -> Result<Json<GpsResponse>, (StatusCode, Json<GpsError>)> {
    // Parse coordinates
    let (latitude, longitude) = parse_coordinates(&lat_lon)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(GpsError {
                status: "error".to_string(),
                error: e,
            })
        ))?;

    // Create GPS coordinate with current timestamp
    let coordinate = GpsCoordinate {
        timestamp: Utc::now(),
        latitude,
        longitude,
    };

    // Send GPS coordinate to the diag thread to be written to the current scan's GPS file
    if let Err(e) = state.gps_sender.send(coordinate.clone()).await {
        error!("Failed to send GPS coordinate to diag thread: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GpsError {
                status: "error".to_string(),
                error: "Failed to process GPS data".to_string(),
            })
        ));
    }

    // Log the GPS request in the requested format to main log
    info!("{}, {}, {}", 
          coordinate.timestamp.timestamp(),
          coordinate.latitude,
          coordinate.longitude);

    Ok(Json(GpsResponse {
        status: "success".to_string(),
        message: "GPS coordinate saved successfully".to_string(),
        data: coordinate,
    }))
}

/// Handler for downloading GPS data for a recording session
/// Returns the raw GPS file in the format: timestamp, latitude, longitude
pub async fn get_gps_for_recording(
    Path(recording_name): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Response<Body>, (StatusCode, Json<GpsError>)> {
    // Get the recording entry from the manifest
    let store = state.qmdl_store_lock.read().await;
    let manifest = &store.manifest;

    let recording_entry = manifest.entries.iter()
        .find(|entry| entry.name == recording_name)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(GpsError {
                    status: "error".to_string(),
                    error: format!("Recording '{}' not found", recording_name),
                })
            )
        })?;

    // Check if GPS file exists for this recording
    let gps_file_path = recording_entry.get_gps_filepath(&store.path);
    if !gps_file_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(GpsError {
                status: "error".to_string(),
                error: format!("No GPS data available for recording '{}'", recording_name),
            })
        ));
    }

    // Read the GPS file content
    let content = tokio::fs::read_to_string(&gps_file_path)
        .await
        .map_err(|e| {
            error!("Failed to read GPS file: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GpsError {
                    status: "error".to_string(),
                    error: "Failed to read GPS file".to_string(),
                })
            )
        })?;

    let filename = format!("{}.gps", recording_name);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))
        .body(Body::from(content))
        .unwrap())
}

/// Handler for checking if GPS data exists for a recording session (HEAD request)
pub async fn head_gps_for_recording(
    Path(recording_name): Path<String>,
    State(state): State<Arc<ServerState>>,
) -> Result<Response<Body>, (StatusCode, Json<GpsError>)> {
    // Get the recording entry from the manifest
    let store = state.qmdl_store_lock.read().await;
    let manifest = &store.manifest;

    let recording_entry = manifest.entries.iter()
        .find(|entry| entry.name == recording_name)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(GpsError {
                    status: "error".to_string(),
                    error: format!("Recording '{}' not found", recording_name),
                })
            )
        })?;

    // Check if GPS file exists for this recording
    let gps_file_path = recording_entry.get_gps_filepath(&store.path);
    if !gps_file_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(GpsError {
                status: "error".to_string(),
                error: format!("No GPS data available for recording '{}'", recording_name),
            })
        ));
    }

    let filename = format!("{}.gps", recording_name);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))
        .body(Body::empty())
        .unwrap())
}
