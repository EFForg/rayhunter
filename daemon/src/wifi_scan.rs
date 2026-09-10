use chrono::{DateTime, Local, TimeDelta};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::{select, task::JoinHandle, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use wifi_station::{STA_IFACE, WifiNetwork, scan_wifi_networks};

use crate::qmdl_store::RecordingStoreError;
use crate::server::ServerState;

pub enum WifiScanCtrlMessage {
    StopRecording,
    StartRecording {
        response_tx: Option<oneshot::Sender<Result<(), RecordingStoreError>>>,
    },
    DeleteEntry {
        name: String,
        response_tx: oneshot::Sender<Result<(), RecordingStoreError>>,
    },
    DeleteAllEntries {
        response_tx: oneshot::Sender<Result<(), RecordingStoreError>>,
    },
    Exit,
}

// Structure of the wifi capture files (similar to QMDL files)
// JSON file containing the start and end timestamps of the scan,
// followed by zero or more WiFi network structs, containing the
// BSSID, SSID, signal strength and security for the network
#[derive(Deserialize, Serialize)]
struct WifiScanEntry {
    start_ts: DateTime<Local>,
    end_ts: DateTime<Local>,
    networks: Vec<WifiNetwork>,
}

pub async fn run_wifi_scanner(
    task_tracker: &TaskTracker,
    state: Arc<ServerState>,
    shutdown_token: CancellationToken,
    mut wifi_rx: mpsc::Receiver<WifiScanCtrlMessage>,
) {
    info!("starting wifi scanner");

    task_tracker.spawn(async move {
        let mut started = false;
        loop {
            select! {
                message = wifi_rx.recv() => {
                    match message {
                        Some(WifiScanCtrlMessage::StartRecording { response_tx }) => {
                            started = true;
                            // TODO:
                            // - lock wifi store
                            // - check disk space
                            // - create WifiWriter
                            // - create AnalysisWriter
                            // - set state to "recording" somehow, including writers in state
                            // - update UI state?
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
                    match scan_wifi_networks(STA_IFACE).await {
                        Ok(networks) => {
                            debug!("Found {} networks", networks.len());
                            // TODO Call AnalysisWriter.analyze_networks()
                            // TODO Handle return value from AnalysisWriter
                            // (see diag.rs:427)
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
