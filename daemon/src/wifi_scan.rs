use chrono::{DateTime, FixedOffset, Local};
use log::{debug, error, info, warn};
use rayhunter::analysis::analyzer::EventType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::{mpsc, oneshot};
use tokio::{select, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use wifi_station::{STA_IFACE, WifiNetwork, scan_wifi_networks};

use crate::display;
use crate::notifications::{Notification, NotificationType};
use crate::wifi_store::{WifiStore, WifiStoreError};

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
pub struct WifiScan {
    pub start_ts: DateTime<FixedOffset>,
    pub end_ts: DateTime<FixedOffset>,
    pub networks: Vec<WifiNetwork>,
}

impl WifiScan {
    fn new() -> Self {
        let mut scan = Self::default();
        scan.start_ts = Local::now().fixed_offset();
        scan
    }

    fn finish(&mut self, networks: Vec<WifiNetwork>) {
        self.end_ts = Local::now().fixed_offset();
        self.networks = networks;
    }
}

pub async fn run_wifi_scanner(
    task_tracker: &TaskTracker,
    wifi_scan_lock: Arc<RwLock<()>>,
    shutdown_token: CancellationToken,
    mut wifi_rx: mpsc::Receiver<WifiScanCtrlMessage>,
    wifi_store_lock: Arc<RwLock<WifiStore>>,
    min_space_to_start_mb: u64,
    min_space_to_continue_mb: u64,
    wifi_ouis: Option<Vec<String>>,
    notification_channel: mpsc::Sender<Notification>,
    ui_update_sender: mpsc::Sender<display::DisplayState>,
) {
    // Don't bother if we don't have OUIs specified
    if wifi_ouis.is_some() {
        let wifi_ouis = wifi_ouis.unwrap();
        info!("starting wifi scanner");
        task_tracker.spawn(async move {
            let mut started = false;
            let mut max_type_seen = EventType::Informational;
            loop {
                select! {
                    message = wifi_rx.recv() => {
                        match message {
                            Some(WifiScanCtrlMessage::StartRecording { response_tx }) => {
                                started = true;
                                // Lock wifi store
                                let wifi_store = wifi_store_lock.write().await;

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
                        if wifi_scan_lock.try_write().is_err() {
                            warn!("WiFi scan already in progress");
                            continue;
                        }
                        debug!("Calling scan_wifi_networks()");
                        let mut scan = WifiScan::new();
                        match scan_wifi_networks(STA_IFACE).await {
                            Ok(networks) => {
                                let wifi_store = wifi_store_lock.write().await;
                                debug!("Found {} networks", networks.len());
                                scan.finish(networks);
                                if let Err(error) = wifi_store.write_scan_file(&scan).await {
                                    error!("Error writing to Wifi file: {error}");
                                }
                                let max_type = match wifi_store.write_analysis_file(&scan, &wifi_ouis).await {
                                    Ok(t) => t,
                                    Err(e) => {
                                        warn!("failed to analyze wifi scan: {e}");
                                        EventType::Informational
                                    }
                                };

                                // Code duplicated from diag.rs
                                if max_type > EventType::Informational {
                                    info!("a heuristic triggered on this run!");
                                    notification_channel
                                        .send(Notification::new(
                                            NotificationType::Warning,
                                            format!("Rayhunter has detected a {:?} severity event", max_type),
                                            Some(Duration::from_secs(60 * 5)),
                                        ))
                                        .await
                                        .expect("Failed to send to notification channel");
                                }

                                if max_type > max_type_seen {
                                    max_type_seen = max_type;
                                    if max_type_seen > EventType::Informational {
                                        ui_update_sender
                                            .send(display::DisplayState::WarningDetected {
                                                event_type: max_type_seen,
                                            })
                                            .await
                                            .expect("couldn't send ui update message: {}");
                                    }
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
