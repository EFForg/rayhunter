use std::pin::pin;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rayhunter::analysis::analyzer::Harness;
use rayhunter::diag::{DataType, MessagesContainer};
use rayhunter::diag_device::DiagDevice;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Receiver;
use rayhunter::qmdl::QmdlWriter;
use log::{debug, error, info};
use tokio::fs::File;
use tokio::io::{BufWriter, AsyncWriteExt};
use tokio_util::io::ReaderStream;
use tokio_util::task::TaskTracker;
use futures::{StreamExt, TryStreamExt};

use crate::framebuffer;
use crate::qmdl_store::RecordingStore;
use crate::server::ServerState;

pub enum DiagDeviceCtrlMessage {
    StopRecording,
    StartRecording((QmdlWriter<File>, File)),
    Exit,
}

struct AnalysisWriter {
    writer: BufWriter<File>,
    harness: Harness,
    bytes_written: usize,
    has_warning: bool,
}

// We write our analysis results to a file immediately to minimize the amount of
// state Rayhunter has to keep track of in memory. The analysis file's format is
// Newline Delimited JSON
// (https://docs.mulesoft.com/dataweave/latest/dataweave-formats-ndjson), which
// lets us simply append new rows to the end without parsing the entire JSON
// object beforehand.
impl AnalysisWriter {
    pub async fn new(file: File) -> Result<Self, std::io::Error> {
        let mut result = Self {
            writer: BufWriter::new(file),
            harness: Harness::new_with_all_analyzers(),
            bytes_written: 0,
            has_warning: false,
        };
        let metadata = result.harness.get_metadata();
        result.write(&metadata).await?;
        Ok(result)
    }

    // Runs the analysis harness on the given container, serializing the results
    // to the analysis file and returning the file's new length.
    pub async fn analyze(&mut self, container: MessagesContainer) -> Result<usize, std::io::Error> {
        let row = self.harness.analyze_qmdl_messages(container);
        if !row.is_empty() {
            self.write(&row).await?;
            self.has_warning = ! &row.analysis.is_empty()
        }
        Ok(self.bytes_written)
    }

    async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut value_str = serde_json::to_string(value).unwrap();
        value_str.push('\n');
        self.bytes_written += value_str.len();
        self.writer.write_all(value_str.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    // Flushes any pending I/O to disk before dropping the writer
    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.writer.flush().await?;
        Ok(())
    }
}

pub fn run_diag_read_thread(
    task_tracker: &TaskTracker,
    mut dev: DiagDevice,
    mut qmdl_file_rx: Receiver<DiagDeviceCtrlMessage>,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>
) {
    task_tracker.spawn(async move {
        let (initial_qmdl_file, initial_analysis_file) = qmdl_store_lock.write().await.new_entry().await.expect("failed creating QMDL file entry");
        let mut maybe_qmdl_writer: Option<QmdlWriter<File>> = Some(QmdlWriter::new(initial_qmdl_file));
        let mut diag_stream = pin!(dev.as_stream().into_stream());
        let mut maybe_analysis_writer = Some(AnalysisWriter::new(initial_analysis_file).await
            .expect("failed to create analysis writer"));
        loop {
            tokio::select! {
                msg = qmdl_file_rx.recv() => {
                    match msg {
                        Some(DiagDeviceCtrlMessage::StartRecording((new_writer, new_analysis_file))) => {
                            maybe_qmdl_writer = Some(new_writer);
                            if let Some(analysis_writer) = maybe_analysis_writer {
                                analysis_writer.close().await.expect("failed to close analysis writer");
                            }
                            maybe_analysis_writer = Some(AnalysisWriter::new(new_analysis_file).await
                                .expect("failed to write to analysis file"));
                        },
                        Some(DiagDeviceCtrlMessage::StopRecording) => {
                            maybe_qmdl_writer = None;
                            if let Some(analysis_writer) = maybe_analysis_writer {
                                analysis_writer.close().await.expect("failed to close analysis writer");
                            }
                            maybe_analysis_writer = None;
                        },
                        // None means all the Senders have been dropped, so it's
                        // time to go
                        Some(DiagDeviceCtrlMessage::Exit) | None => {
                            info!("Diag reader thread exiting...");
                            if let Some(analysis_writer) = maybe_analysis_writer {
                                analysis_writer.close().await.expect("failed to close analysis writer");
                            }
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
                            if let Some(qmdl_writer) = maybe_qmdl_writer.as_mut() {
                                qmdl_writer.write_container(&container).await.expect("failed to write to QMDL writer");
                                debug!("total QMDL bytes written: {}, updating manifest...", qmdl_writer.total_written);
                                let mut qmdl_store = qmdl_store_lock.write().await;
                                let index = qmdl_store.current_entry.expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
                                qmdl_store.update_entry_qmdl_size(index, qmdl_writer.total_written).await
                                    .expect("failed to update qmdl file size");
                                debug!("done!");
                            } else {
                                debug!("no qmdl_writer set, continuing...");
                            }

                            if let Some(analysis_writer) = maybe_analysis_writer.as_mut() {
                                let analysis_file_len = analysis_writer.analyze(container).await
                                    .expect("failed to analyze container");
                                let mut qmdl_store = qmdl_store_lock.write().await;
                                let index = qmdl_store.current_entry.expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
                                qmdl_store.update_entry_analysis_size(index, analysis_file_len as usize).await
                                    .expect("failed to update analysis file size");
                                qmdl_store.update_entry_has_warning(index, analysis_writer.has_warning).await
                                    .expect("failed to update analysis file has warning");
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
    let (qmdl_file, analysis_file) = qmdl_store.new_entry().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't create new qmdl entry: {}", e)))?;
    let qmdl_writer = QmdlWriter::new(qmdl_file);
    state.diag_device_ctrl_sender.send(DiagDeviceCtrlMessage::StartRecording((qmdl_writer, analysis_file))).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send stop recording message: {}", e)))?;
    state.ui_update_sender.send(framebuffer::Color565::Green).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send ui update message: {}", e)))?;
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
    state.ui_update_sender.send(framebuffer::Color565::White).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send ui update message: {}", e)))?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn get_analysis_report(State(state): State<Arc<ServerState>>) -> Result<Response, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    let Some(entry) = qmdl_store.get_current_entry() else {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "No QMDL data's being recorded to analyze, try starting a new recording!".to_string()
        ));
    };
    let analysis_file = qmdl_store.open_entry_analysis(entry).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;
    let analysis_stream = ReaderStream::new(analysis_file);

    let headers = [(CONTENT_TYPE, "application/x-ndjson")];
    let body = Body::from_stream(analysis_stream);
    Ok((headers, body).into_response())
}
