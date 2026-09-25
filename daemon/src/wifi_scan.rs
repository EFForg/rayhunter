use chrono::{DateTime, FixedOffset, Local, TimeDelta};
use log::{debug, error, info, warn};
use rayhunter::DeviceMetadata;
use rayhunter::analysis::analyzer::{AnalyzerConfig, EventType};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::sync::{RwLock, mpsc, oneshot};
use tokio::{select, task::JoinHandle, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use wifi_station::{STA_IFACE, WifiNetwork, scan_wifi_networks};

use crate::diag::{DiskSpaceCheck, check_disk_space};
use crate::qmdl_store::RecordingStoreError;
use crate::server::ServerState;
use crate::wifi_store::{WifiAnalysisWriter, WifiStore, WifiStoreError, WifiWriter};

pub enum WifiScanCtrlMessage {
    StopRecording,
    StartRecording {
        response_tx: Option<oneshot::Sender<Result<(), WifiStoreError>>>,
    },
    DeleteEntry {
        name: String,
        response_tx: oneshot::Sender<Result<(), WifiStoreError>>,
    },
    DeleteAllEntries {
        response_tx: oneshot::Sender<Result<(), WifiStoreError>>,
    },
    Exit,
}

// Structure of the wifi capture files (similar to QMDL files)
// JSON file containing the start and end timestamps of the scan,
// followed by zero or more WiFi network structs, containing the
// BSSID, SSID, signal strength and security for the network
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct WifiScanEntry {
    pub start_ts: DateTime<FixedOffset>,
    pub end_ts: DateTime<FixedOffset>,
    pub networks: Vec<WifiNetwork>,
}

impl WifiScanEntry {
    fn new() -> Self {
        let mut entry = Self::default();
        entry.start_ts = Local::now().fixed_offset();
        entry
    }

    fn finish(&mut self, networks: Vec<WifiNetwork>) {
        self.end_ts = Local::now().fixed_offset();
        self.networks = networks;
    }
}

pub async fn run_wifi_scanner(
    task_tracker: &TaskTracker,
    state: Arc<ServerState>,
    shutdown_token: CancellationToken,
    mut wifi_rx: mpsc::Receiver<WifiScanCtrlMessage>,
    wifi_store_lock: Arc<RwLock<WifiStore>>,
    min_space_to_start_mb: u64,
    min_space_to_continue_mb: u64,
    wifi_ouis: Option<Vec<String>>,
) {
    // Don't bother if we don't have OUIs specified
    if wifi_ouis.is_some() {
        let wifi_ouis = wifi_ouis.unwrap();
        info!("starting wifi scanner");
        task_tracker.spawn(async move {
            let mut started = false;
            let mut wifi_writer: Option<WifiWriter<File>> = None;
            let mut analysis_writer: Option<WifiAnalysisWriter> = None;
            loop {
                select! {
                    message = wifi_rx.recv() => {
                        match message {
                            Some(WifiScanCtrlMessage::StartRecording { response_tx }) => {
                                started = true;
                                // Lock wifi store
                                let mut wifi_store = wifi_store_lock.write().await;

                                // Check disk space
                                match wifi_store.check_disk_space(min_space_to_start_mb, min_space_to_continue_mb).await {
                                    Ok(_) => {}
                                    Err(error) => {
                                        if let Some(tx) = response_tx {
                                            tx.send(Err(error)).ok();
                                        }
                                        break;
                                    }
                                }
                                // Create WifiWriter
                                let (wifi_file, analysis_file) = match wifi_store.new_entry().await {
                                    Ok((wifi, analysis)) => (wifi, analysis),
                                    Err(error) => {
                                        error!("Error creating wifi file: {error}");
                                        return;
                                    }
                                };
                                wifi_writer = Some(WifiWriter::new(wifi_file));

                                analysis_writer = match WifiAnalysisWriter::new(
                                    analysis_file,
                                    Some(wifi_ouis.clone()),
                                ).await.map_err(WifiStoreError::IOError) {
                                    Ok(writer) => Some(writer),
                                    Err(error) => {
                                        if let Some(tx) = response_tx {
                                            tx.send(Err(error)).ok();
                                        }
                                        break;
                                    }
                                };
                            }
                            Some(WifiScanCtrlMessage::StopRecording) => {
                                started = false;
                                // TODO:
                                // - Stop writing to wifi ndjson file
                                // - Close writers
                                // - Close entry
                            }
                            Some(WifiScanCtrlMessage::DeleteEntry { name, response_tx }) => {}
                            Some(WifiScanCtrlMessage::DeleteAllEntries { response_tx }) => {}
                            Some(WifiScanCtrlMessage::Exit) | None => {
                                return;
                            }
                        }
                    }
                    _ = shutdown_token.cancelled() => {
                        return;
                    }
                    _ = time::sleep(Duration::from_secs(15)), if started => {
                        if state.wifi_scan_lock.try_lock().is_err() {
                            warn!("WiFi scan already in progress");
                            continue;
                        }
                        debug!("Calling scan_wifi_networks()");
                        let mut entry = WifiScanEntry::new();
                        match scan_wifi_networks(STA_IFACE).await {
                            Ok(networks) => {
                                debug!("Found {} networks", networks.len());
                                entry.finish(networks);
                                let timestamp = Local::now();
                                if let Some(ref mut wifi_writer) = wifi_writer {
                                    if let Err(error) = wifi_writer.write(entry.clone()).await {
                                        error!("Error writing to Wifi file: {error}");
                                    }
                                }
                                // Call AnalysisWriter.analyze_networks()
                                if let Some(ref mut analysis_writer) = analysis_writer {
                                    let max_type = match analysis_writer.analyze_networks(entry).await {
                                        Ok(t) => t,
                                        Err(e) => {
                                            warn!("failed to analyze container: {e}");
                                            EventType::Informational
                                        }
                                    };
                                }
                            }
                            Err(e) => {
                                warn!("Error scanning wifi networks: {e}");
                            }
                        }
                    }
                }
            }
        });
    }
}
