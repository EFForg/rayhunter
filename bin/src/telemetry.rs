//! # Telemetry Module
//!
//! This module provides functionality for collecting and sending telemetry data from Rayhunter
//! to a configured endpoint. It can be used to monitor the health of deployed Rayhunter instances,
//! track warning events, and collect anonymized system information.
//!
//! ## Features
//!
//! - Anonymous device identification
//! - Configurable data collection (warnings, system stats)
//! - Periodic data transmission to a configured endpoint
//! - API for manual telemetry triggers
//! - Web UI configuration interface
//!
//! ## Privacy
//!
//! Telemetry is disabled by default and requires explicit configuration to enable.
//! The device ID is generated as a hash of system properties to avoid direct identification.

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

/// Configuration for the telemetry system
#[derive(Serialize, Deserialize)]
pub struct TelemetrySettings {
    /// Whether telemetry is enabled
    pub enabled: bool,
    /// URL endpoint where telemetry data is sent
    pub endpoint: String,
    /// API key for authentication with the telemetry service
    pub api_key: String,
    /// Interval in seconds between telemetry data transmissions
    pub send_interval_secs: u64,
    /// Whether to include warning events in telemetry
    pub include_warnings: bool,
    /// Whether to include system statistics in telemetry
    pub include_stats: bool,
    /// Anonymous device identifier (not user-configurable)
    #[serde(skip_deserializing)]
    pub device_id: String,
}

/// Data structure for sending telemetry to the server
#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryData {
    /// Timestamp when the telemetry data was collected
    timestamp: DateTime<Local>,
    /// Anonymous device identifier
    device_id: String,
    /// Rayhunter version
    version: String,
    /// List of warning events (if configured)
    warnings: Option<Vec<TelemetryWarning>>,
    /// System statistics (if configured)
    system_stats: Option<SystemStats>,
    /// Recording summary information
    recordings: Option<Vec<RecordingSummary>>,
}

/// Representation of a warning event for telemetry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryWarning {
    /// When the warning occurred
    pub timestamp: DateTime<FixedOffset>,
    /// Type of warning (e.g., "QualitativeWarning")
    pub warning_type: String,
    /// Warning message text
    pub message: String,
    /// Severity level of the warning (e.g., "Low", "Medium", "High")
    pub severity: String,
}

/// Summary of a recording session for telemetry
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingSummary {
    /// Recording identifier
    name: String,
    /// When the recording started
    start_time: DateTime<Local>,
    /// How long the recording lasted in seconds
    duration_secs: u64,
    /// Number of warnings detected during recording
    warning_count: u32,
    /// Size of the recording in bytes
    size_bytes: usize,
}

/// Messages that can be sent to the telemetry system
pub enum TelemetryMessage {
    /// Add a warning to the telemetry data
    AddWarning(TelemetryWarning),
    /// Trigger an immediate telemetry data transmission
    SendNow,
    /// Signal the telemetry thread to exit
    Exit,
}

/// Central manager for the telemetry subsystem
///
/// `TelemetryManager` coordinates the collection and transmission of telemetry data.
/// It manages a background thread that periodically sends data to a configured endpoint
/// and provides interfaces for other components to interact with the telemetry system.
///
/// # Functionality
///
/// - Initializes the telemetry system with configuration settings
/// - Generates and maintains a stable device identifier
/// - Provides communication channels for sending telemetry messages
/// - Manages the telemetry background thread
/// - Collects and transmits telemetry data according to configuration
///
/// # Example
///
/// ```
/// let config = Config::default();
/// match TelemetryManager::new(config) {
///     Ok(manager) => {
///         // Start the telemetry thread
///         manager.run_telemetry_thread(&task_tracker, qmdl_store_lock, receiver);
///         
///         // Get a sender to communicate with telemetry
///         let sender = manager.get_sender();
///         
///         // Send a warning to telemetry
///         let warning = TelemetryWarning {
///             timestamp: chrono::Local::now().into(),
///             warning_type: "ExampleWarning".to_string(),
///             message: "This is a test warning".to_string(),
///             severity: "Low".to_string(),
///         };
///         sender.send(TelemetryMessage::AddWarning(warning)).await.unwrap();
///     },
///     Err(e) => {
///         log::error!("Failed to initialize telemetry: {}", e);
///     }
/// }
/// ```
pub struct TelemetryManager {
    config: Config,
    sender: mpsc::Sender<TelemetryMessage>,
    device_id: String,
}

impl TelemetryManager {
    /// Creates a new telemetry manager with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The Rayhunter configuration containing telemetry settings
    ///
    /// # Returns
    ///
    /// A new `TelemetryManager` instance or an error if initialization fails
    pub fn new(config: Config) -> Result<Self, RayhunterError> {
        // Generate a stable device ID based on machine-specific information
        // For privacy reasons, we use a hash of hardware identifiers
        let device_id = match generate_device_id() {
            Ok(id) => id,
            Err(err) => return Err(RayhunterError::TelemetryInitError(
                format!("Failed to generate device ID: {}", err)
            )),
        };
        
        // Create a channel for telemetry messages
        let (sender, _) = mpsc::channel(100);
        
        Ok(TelemetryManager {
            config,
            sender,
            device_id,
        })
    }
    
    /// Gets a sender channel to communicate with the telemetry thread
    ///
    /// This allows other components to send messages to the telemetry system,
    /// such as adding warnings or triggering immediate data transmission.
    pub fn get_sender(&self) -> mpsc::Sender<TelemetryMessage> {
        self.sender.clone()
    }
    
    /// Gets the anonymous device identifier
    ///
    /// This is a stable identifier generated from system properties
    /// that uniquely identifies the device without exposing personally
    /// identifiable information.
    pub fn get_device_id(&self) -> &String {
        &self.device_id
    }
    
    /// Starts the telemetry background thread
    ///
    /// This method initializes the telemetry system and begins collecting
    /// and sending data according to the configured settings. If telemetry
    /// is disabled or missing required configuration, the thread will not start.
    ///
    /// # Arguments
    ///
    /// * `task_tracker` - Tracker for the spawned task
    /// * `qmdl_store_lock` - Access to the recording store
    /// * `receiver` - Channel for receiving telemetry messages
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

/// Sends telemetry data to the configured endpoint
///
/// Collects and transmits telemetry information according to the configuration settings.
/// This includes warnings, system statistics, and recording information as configured.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `config` - Application configuration
/// * `device_id` - Anonymous device identifier
/// * `qmdl_store_lock` - Access to recording data
/// * `warnings` - List of warning events to include
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

/// Creates summaries of recordings for telemetry
///
/// Transforms detailed recording entries into simplified summaries
/// suitable for inclusion in telemetry data.
///
/// # Arguments
///
/// * `entries` - List of recording entries from the manifest
///
/// # Returns
///
/// A vector of recording summaries
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

/// Generates a stable, anonymous device identifier
///
/// The device ID is created by hashing system-specific information to
/// create a unique identifier that doesn't contain personally identifiable
/// information.
///
/// # Returns
///
/// A string containing the device identifier, or an error if generation fails
fn generate_device_id() -> Result<String, String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Try to use more stable identifiers
    if let Ok(hostname) = std::process::Command::new("hostname").output() {
        hostname.stdout.hash(&mut hasher);
    } else {
        return Err("Failed to get hostname information".to_string());
    }
    
    // Hash the MAC address if available (this is platform-specific and might need adjustment)
    if let Ok(ifconfig) = std::process::Command::new("ifconfig").output() {
        ifconfig.stdout.hash(&mut hasher);
    } else {
        // Not fatal, just a warning
        warn!("Could not get network interface information for device ID");
    }
    
    Ok(format!("{:016x}", hasher.finish()))
}

/// Fetches current telemetry settings
///
/// Retrieves the active telemetry configuration from the server state.
///
/// # Arguments
///
/// * `state` - Server state containing configuration
///
/// # Returns
///
/// Current telemetry configuration settings or an error
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

/// Updates telemetry settings
///
/// Applies new telemetry configuration settings and persists them to disk.
/// If telemetry is enabled, also triggers an immediate data transmission
/// to verify the new settings.
///
/// # Arguments
///
/// * `state` - Server state including configuration
/// * `new_settings` - Updated telemetry settings
///
/// # Returns
///
/// The updated telemetry settings or an error
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

/// Writes the configuration to disk
///
/// Serializes the configuration to TOML format and saves it to the specified path.
///
/// # Arguments
///
/// * `config` - Configuration to write
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// Success or an error message
fn write_config_to_disk(config: &crate::config::Config, path: &str) -> Result<(), String> {
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(path, config_str)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}