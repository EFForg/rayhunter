use std::sync::Arc;
use std::time::Duration;
use log::{info, error, warn, debug};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use tokio_util::task::TaskTracker;
use chrono::{DateTime, FixedOffset, Local};
use axum::http::StatusCode;
use axum::Json;
use axum::extract::State;

use crate::server::ServerState;
use crate::config::Config;
use crate::qmdl_store::{RecordingStore, ManifestEntry};
use crate::stats::SystemStats;
use crate::error::RayhunterError;

#[derive(Serialize, Deserialize)]
pub struct TelemetrySettings {
    pub enabled: bool,
    pub endpoint: String,
    pub api_key: String,
    pub send_interval_secs: u64,
    pub include_warnings: bool,
    pub include_stats: bool,
    #[serde(skip_deserializing)]
    pub device_id: String,
}

// Define the data structure for telemetry
#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryData {
    timestamp: DateTime<Local>,
    device_id: String,
    version: String,
    warnings: Option<Vec<TelemetryWarning>>,
    system_stats: Option<SystemStats>,
    recordings: Option<Vec<RecordingSummary>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryWarning {
    pub timestamp: DateTime<FixedOffset>,
    pub warning_type: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingSummary {
    name: String,
    start_time: DateTime<Local>,
    duration_secs: u64,
    warning_count: u32,
    size_bytes: usize,
}

pub enum TelemetryMessage {
    AddWarning(TelemetryWarning),
    SendNow,
    Exit,
}

pub struct TelemetryManager {
    config: Config,
    sender: mpsc::Sender<TelemetryMessage>,
    device_id: String,
}

impl TelemetryManager {
    pub fn new(config: Config) -> Self {
        // Generate a stable device ID based on machine-specific information
        // For privacy reasons, we use a hash of hardware identifiers
        let device_id = generate_device_id();
        
        // Create a channel for telemetry messages
        let (sender, _) = mpsc::channel(100);
        
        TelemetryManager {
            config,
            sender,
            device_id,
        }
    }
    
    pub fn get_sender(&self) -> mpsc::Sender<TelemetryMessage> {
        self.sender.clone()
    }
    
    pub fn get_device_id(&self) -> &String {
        &self.device_id
    }
    
    pub fn run_telemetry_thread(
        &self,
        task_tracker: &TaskTracker,
        qmdl_store_lock: Arc<RwLock<RecordingStore>>,
        mut receiver: mpsc::Receiver<TelemetryMessage>,
    ) {
        if !self.config.telemetry_enabled {
            info!("Telemetry is disabled, not starting telemetry thread");
            return;
        }
        
        if self.config.telemetry_api_key.is_empty() {
            warn!("Telemetry is enabled but no API key is provided, not starting telemetry thread");
            return;
        }
        
        info!("Starting telemetry thread with endpoint: {}", self.config.telemetry_endpoint);
        
        let config = self.config.clone();
        let device_id = self.device_id.clone();
        let interval = Duration::from_secs(config.telemetry_send_interval_secs);
        
        task_tracker.spawn(async move {
            let mut warnings = Vec::new();
            let client = reqwest::Client::new();
            
            loop {
                tokio::select! {
                    msg = receiver.recv() => {
                        match msg {
                            Some(TelemetryMessage::AddWarning(warning)) => {
                                if config.telemetry_include_warnings {
                                    warnings.push(warning);
                                }
                            }
                            Some(TelemetryMessage::SendNow) => {
                                send_telemetry_data(&client, &config, &device_id, &qmdl_store_lock, &warnings).await;
                                warnings.clear();
                            }
                            Some(TelemetryMessage::Exit) | None => {
                                info!("Telemetry thread exiting...");
                                // Try to send final data before exiting
                                send_telemetry_data(&client, &config, &device_id, &qmdl_store_lock, &warnings).await;
                                break;
                            }
                        }
                    }
                    _ = sleep(interval) => {
                        debug!("Sending scheduled telemetry data...");
                        send_telemetry_data(&client, &config, &device_id, &qmdl_store_lock, &warnings).await;
                        warnings.clear();
                    }
                }
            }
            
            Ok::<(), RayhunterError>(())
        });
    }
}

async fn send_telemetry_data(
    client: &reqwest::Client,
    config: &Config,
    device_id: &str,
    qmdl_store_lock: &Arc<RwLock<RecordingStore>>,
    warnings: &[TelemetryWarning],
) {
    let mut telemetry_data = TelemetryData {
        timestamp: Local::now(),
        device_id: device_id.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        warnings: None,
        system_stats: None,
        recordings: None,
    };
    
    // Include warnings if configured
    if config.telemetry_include_warnings && !warnings.is_empty() {
        telemetry_data.warnings = Some(warnings.to_vec());
    }
    
    // Include system stats if configured
    if config.telemetry_include_stats {
        match SystemStats::new(&config.qmdl_store_path).await {
            Ok(stats) => telemetry_data.system_stats = Some(stats),
            Err(e) => error!("Failed to get system stats for telemetry: {}", e),
        }
    }
    
    // Summarize recordings
    let qmdl_store = qmdl_store_lock.read().await;
    let recordings = summarize_recordings(&qmdl_store.manifest.entries);
    if !recordings.is_empty() {
        telemetry_data.recordings = Some(recordings);
    }
    
    // Send the data
    match client.post(&config.telemetry_endpoint)
        .header("X-API-Key", &config.telemetry_api_key)
        .json(&telemetry_data)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    debug!("Successfully sent telemetry data");
                } else {
                    error!("Failed to send telemetry data: HTTP {}", response.status());
                }
            }
            Err(e) => {
                error!("Failed to send telemetry data: {}", e);
            }
        }
}

fn summarize_recordings(entries: &[ManifestEntry]) -> Vec<RecordingSummary> {
    entries.iter().map(|entry| {
        let duration = entry.last_message_time
            .map(|end| (end - entry.start_time).num_seconds() as u64)
            .unwrap_or(0);
            
        RecordingSummary {
            name: entry.name.clone(),
            start_time: entry.start_time,
            duration_secs: duration,
            warning_count: 0, // TODO: parse the analysis file to count warnings
            size_bytes: entry.qmdl_size_bytes,
        }
    }).collect()
}

fn generate_device_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Try to use more stable identifiers
    if let Ok(hostname) = std::process::Command::new("hostname").output() {
        hostname.stdout.hash(&mut hasher);
    }
    
    // Hash the MAC address if available (this is platform-specific and might need adjustment)
    if let Ok(ifconfig) = std::process::Command::new("ifconfig").output() {
        ifconfig.stdout.hash(&mut hasher);
    }
    
    format!("{:016x}", hasher.finish())
}

pub async fn get_telemetry_status(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<TelemetrySettings>, (StatusCode, String)> {
    // Return the current telemetry configuration
    let settings = TelemetrySettings {
        enabled: state.config.telemetry_enabled,
        endpoint: state.config.telemetry_endpoint.clone(),
        api_key: state.config.telemetry_api_key.clone(),
        send_interval_secs: state.config.telemetry_send_interval_secs,
        include_warnings: state.config.telemetry_include_warnings,
        include_stats: state.config.telemetry_include_stats,
        device_id: state.telemetry_device_id.clone(),
    };
    
    Ok(Json(settings))
}

pub async fn update_telemetry_settings(
    State(state): State<Arc<ServerState>>,
    Json(new_settings): Json<TelemetrySettings>,
) -> Result<Json<TelemetrySettings>, (StatusCode, String)> {
    // Create a response with the merged settings
    let updated_settings = TelemetrySettings {
        enabled: new_settings.enabled,
        endpoint: new_settings.endpoint.clone(),
        api_key: new_settings.api_key.clone(),
        send_interval_secs: new_settings.send_interval_secs,
        include_warnings: new_settings.include_warnings,
        include_stats: new_settings.include_stats,
        device_id: state.telemetry_device_id.clone(),
    };

    let mut config = state.config.clone();
    config.telemetry_enabled = new_settings.enabled;
    config.telemetry_endpoint = new_settings.endpoint;
    config.telemetry_api_key = new_settings.api_key;
    config.telemetry_send_interval_secs = new_settings.send_interval_secs;
    config.telemetry_include_warnings = new_settings.include_warnings;
    config.telemetry_include_stats = new_settings.include_stats;

    // Persist to disk
    write_config_to_disk(&config, &state.config_path)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if state.telemetry_enabled {
        let _ = state.telemetry_sender.try_send(TelemetryMessage::SendNow);
    }

    Ok(Json(updated_settings))
}

fn write_config_to_disk(config: &crate::config::Config, path: &str) -> Result<(), String> {
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(path, config_str)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}