use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use futures::{StreamExt, TryStreamExt};
use log::{debug, error, info, warn};
use rayhunter::analysis::analyzer::AnalyzerConfig;
use rayhunter::diag::DataType;
use rayhunter::diag_device::DiagDevice;
use rayhunter::qmdl::QmdlWriter;
use tokio::fs::File;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::io::ReaderStream;
use tokio_util::task::TaskTracker;

use crate::analysis::{AnalysisCtrlMessage, AnalysisWriter};
use crate::display;
use crate::notifications::Notification;
use crate::qmdl_store::{RecordingStore, RecordingStoreError};
use crate::server::ServerState;

pub enum DiagDeviceCtrlMessage {
    StopRecording,
    StartRecording,
    Exit,
}

#[allow(clippy::too_many_arguments)]
pub fn run_diag_read_thread(
    task_tracker: &TaskTracker,
    mut dev: DiagDevice,
    mut qmdl_file_rx: Receiver<DiagDeviceCtrlMessage>,
    ui_update_sender: Sender<display::DisplayState>,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    analysis_sender: Sender<AnalysisCtrlMessage>,
    enable_dummy_analyzer: bool,
    analyzer_config: AnalyzerConfig,
    notification_channel: tokio::sync::mpsc::Sender<Notification>,
) {
    task_tracker.spawn(async move {
        let (initial_qmdl_file, initial_analysis_file) = qmdl_store_lock.write().await.new_entry().await.expect("failed creating QMDL file entry");
        let mut maybe_qmdl_writer: Option<QmdlWriter<File>> = Some(QmdlWriter::new(initial_qmdl_file));
        let mut diag_stream = pin!(dev.as_stream().into_stream());
        let mut maybe_analysis_writer = Some(AnalysisWriter::new(initial_analysis_file, enable_dummy_analyzer, &analyzer_config).await
            .expect("failed to create analysis writer"));

        loop {
            tokio::select! {
                msg = qmdl_file_rx.recv() => {
                    match msg {
                        Some(DiagDeviceCtrlMessage::StartRecording) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            let (qmdl_file, new_analysis_file) = match qmdl_store.new_entry().await {
                                Ok(x) => x,
                                Err(e) => {
                                    error!("couldn't create new qmdl entry: {e}");
                                    continue;
                                }
                            };

                            maybe_qmdl_writer = Some(QmdlWriter::new(qmdl_file));

                            if let Some(analysis_writer) = maybe_analysis_writer {
                                analysis_writer.close().await.expect("failed to close analysis writer");
                            }

                            maybe_analysis_writer = Some(AnalysisWriter::new(new_analysis_file, enable_dummy_analyzer, &analyzer_config).await
                                .expect("failed to write to analysis file"));

                            if let Err(e) = ui_update_sender.send(display::DisplayState::Recording).await {
                                warn!("couldn't send ui update message: {e}");
                            }
                        },
                        Some(DiagDeviceCtrlMessage::StopRecording) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            if let Some((_, entry)) = qmdl_store.get_current_entry() {
                                    if let Err(e) = analysis_sender
                                    .send(AnalysisCtrlMessage::RecordingFinished(
                                            entry.name.to_string(),
                                    ))
                                    .await {
                                        warn!("couldn't send analysis message: {e}");
                                    }
                            }
                            if let Err(e) = qmdl_store.close_current_entry().await {
                                error!("couldn't close current entry: {e}");
                            }

                            maybe_qmdl_writer = None;
                            if let Some(analysis_writer) = maybe_analysis_writer {
                                analysis_writer.close().await.expect("failed to close analysis writer");
                            }
                            maybe_analysis_writer = None;

                            if let Err(e) = ui_update_sender.send(display::DisplayState::Paused).await {
                                warn!("couldn't send ui update message: {e}");
                            }
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
                                let (analysis_file_len, heuristic_warning) = analysis_writer.analyze(container).await
                                    .expect("failed to analyze container");
                                if heuristic_warning {
                                    info!("a heuristic triggered on this run!");
                                    ui_update_sender.send(display::DisplayState::WarningDetected).await
                                        .expect("couldn't send ui update message: {}");
                                    notification_channel.send(
                                        Notification::new(
                                            "heuristic-warning".to_string(),
                                            "New warning triggered!".to_string(),
                                            Some(Duration::from_secs(60*5)))
                                        ).await.expect("Failed to send to notification channel");
                                }
                                let mut qmdl_store = qmdl_store_lock.write().await;
                                let index = qmdl_store.current_entry.expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
                                qmdl_store.update_entry_analysis_size(index, analysis_file_len).await
                                    .expect("failed to update analysis file size");
                            }
                        },
                        Err(err) => {
                            error!("error reading diag device: {err}");
                            return Err(err);
                        }
                    }
                }
            }
        }
    });
}

pub async fn start_recording(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.config.debug_mode {
        return Err((StatusCode::FORBIDDEN, "server is in debug mode".to_string()));
    }

    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::StartRecording)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send start recording message: {e}"),
            )
        })?;

    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn stop_recording(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.config.debug_mode {
        return Err((StatusCode::FORBIDDEN, "server is in debug mode".to_string()));
    }
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::StopRecording)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send stop recording message: {e}"),
            )
        })?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn delete_recording(
    State(state): State<Arc<ServerState>>,
    Path(qmdl_name): Path<String>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.config.debug_mode {
        return Err((StatusCode::FORBIDDEN, "server is in debug mode".to_string()));
    }
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    match qmdl_store.delete_entry(&qmdl_name).await {
        Err(RecordingStoreError::NoSuchEntryError) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("no recording with name {qmdl_name}"),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't delete recording: {e}"),
            ));
        }
        Ok(_) => {}
    }
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::StopRecording)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send stop recording message: {e}"),
            )
        })?;
    state
        .ui_update_sender
        .send(display::DisplayState::Paused)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send ui update message: {e}"),
            )
        })?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn delete_all_recordings(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.config.debug_mode {
        return Err((StatusCode::FORBIDDEN, "server is in debug mode".to_string()));
    }
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::StopRecording)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send stop recording message: {e}"),
            )
        })?;
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    qmdl_store.delete_all_entries().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("couldn't delete all recordings: {e}"),
        )
    })?;
    state
        .ui_update_sender
        .send(display::DisplayState::Paused)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send ui update message: {e}"),
            )
        })?;
    Ok((StatusCode::ACCEPTED, "ok".to_string()))
}

pub async fn get_analysis_report(
    State(state): State<Arc<ServerState>>,
    Path(qmdl_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    let (entry_index, _) = if qmdl_name == "live" {
        qmdl_store.get_current_entry().ok_or((
            StatusCode::SERVICE_UNAVAILABLE,
            "No QMDL data's being recorded to analyze, try starting a new recording!".to_string(),
        ))?
    } else {
        qmdl_store.entry_for_name(&qmdl_name).ok_or((
            StatusCode::NOT_FOUND,
            format!("Couldn't find QMDL entry with name \"{qmdl_name}\""),
        ))?
    };
    let analysis_file = qmdl_store
        .open_entry_analysis(entry_index)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")))?;
    let analysis_stream = ReaderStream::new(analysis_file);

    let headers = [(CONTENT_TYPE, "application/x-ndjson")];
    let body = Body::from_stream(analysis_stream);
    Ok((headers, body).into_response())
}
