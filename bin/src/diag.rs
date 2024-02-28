use std::pin::pin;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use rayhunter::diag::DataType;
use rayhunter::diag_device::DiagDevice;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Receiver;
use rayhunter::qmdl::QmdlWriter;
use log::{debug, error, info};
use tokio::fs::File;
use tokio_util::task::TaskTracker;
use futures::{StreamExt, TryStreamExt};

use crate::qmdl_store::QmdlStore;
use crate::server::ServerState;

pub enum DiagDeviceCtrlMessage {
    StopRecording,
    StartRecording(QmdlWriter<File>),
    Exit,
}

pub fn run_diag_read_thread(task_tracker: &TaskTracker, mut dev: DiagDevice, mut qmdl_file_rx: Receiver<DiagDeviceCtrlMessage>, qmdl_store_lock: Arc<RwLock<QmdlStore>>) {
    task_tracker.spawn(async move {
        let initial_file = qmdl_store_lock.write().await.new_entry().await.expect("failed creating QMDL file entry");
        let mut qmdl_writer: Option<QmdlWriter<File>> = Some(QmdlWriter::new(initial_file));
        let mut diag_stream = pin!(dev.as_stream().into_stream());
        loop {
            tokio::select! {
                msg = qmdl_file_rx.recv() => {
                    match msg {
                        Some(DiagDeviceCtrlMessage::StartRecording(new_writer)) => {
                            qmdl_writer = Some(new_writer);
                        },
                        Some(DiagDeviceCtrlMessage::StopRecording) => qmdl_writer = None,
                        // None means all the Senders have been dropped, so it's
                        // time to go
                        Some(DiagDeviceCtrlMessage::Exit) | None => {
                            info!("Diag reader thread exiting...");
                            return Ok(())
                        },
                    }
                }
                maybe_container = diag_stream.next() => {
                    match maybe_container.unwrap() {
                        Ok(container) => {
                            if container.data_type != DataType::UserSpace {
                                debug!("skipping non-userspace diag messages...");
                                continue;
                            }
                            // keep track of how many bytes were written to the QMDL file so we can read
                            // a valid block of data from it in the HTTP server
                            if let Some(writer) = qmdl_writer.as_mut() {
                                writer.write_container(&container).await.expect("failed to write to QMDL writer");
                                debug!("total QMDL bytes written: {}, updating manifest...", writer.total_written);
                                let mut qmdl_store = qmdl_store_lock.write().await;
                                let index = qmdl_store.current_entry.expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
                                qmdl_store.update_entry(index, writer.total_written).await
                                    .expect("failed to update qmdl file size");
                                debug!("done!");
                            } else {
                                debug!("no qmdl_writer set, continuing...");
                            }
                        },
                        Err(err) => {
                            error!("error reading diag device: {}", err);
                            return Err(err);
                        }
                    }
                }
            }
        }
    });
}

pub async fn start_recording(State(state): State<Arc<ServerState>>) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.readonly_mode {
        return Err((StatusCode::FORBIDDEN, "server is in readonly mode".to_string()));
    }
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    let qmdl_file = qmdl_store.new_entry().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't create new qmdl entry: {}", e)))?;
    let qmdl_writer = QmdlWriter::new(qmdl_file);
    state.diag_device_ctrl_sender.send(DiagDeviceCtrlMessage::StartRecording(qmdl_writer)).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send stop recording message: {}", e)))?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn stop_recording(State(state): State<Arc<ServerState>>) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.readonly_mode {
        return Err((StatusCode::FORBIDDEN, "server is in readonly mode".to_string()));
    }
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    qmdl_store.close_current_entry().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't close current qmdl entry: {}", e)))?;
    state.diag_device_ctrl_sender.send(DiagDeviceCtrlMessage::StopRecording).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send stop recording message: {}", e)))?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}
