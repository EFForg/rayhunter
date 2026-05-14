use log::{debug, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::{select, task::JoinHandle, time};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use wifi_station::{scan_wifi_networks, STA_IFACE};

use crate::{analysis::AnalysisCtrlMessage, server::ServerState};

pub async fn run_wifi_scanner(
    task_tracker: &TaskTracker,
    state: Arc<ServerState>,
    shutdown_token: CancellationToken,
) -> JoinHandle<()> {
    info!("starting wifi scanner");

    task_tracker.spawn(async move {
        loop {
            select! {
                _ = shutdown_token.cancelled() => break,
                _ = time::sleep(Duration::from_secs(15)) => {
                    if state.wifi_scan_lock.try_lock().is_err() {
                        warn!("WiFi scan already in progress");
                        continue;
                    }
                    debug!("Calling scan_wifi_networks()");
                    match scan_wifi_networks(STA_IFACE).await {
                        Ok(networks) => {
                            debug!("Found {} networks", networks.len());
                            if let Err(e) = state.analysis_sender.send(
                                AnalysisCtrlMessage::WifiNetworksDetected(networks)
                            ).await {
                                warn!("couldn't send analysis message: {e}");
                            }
                        }
                        Err(e) => {
                            warn!("Error scanning wifi networks: {e}");
                        }
                    }
                }
            }
        }
    })
}
