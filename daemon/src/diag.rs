use std::ops::DerefMut;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use futures::{StreamExt, TryStreamExt, future};
use log::{debug, error, info, warn};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{RwLock, oneshot};
use tokio_stream::wrappers::LinesStream;
use tokio_util::task::TaskTracker;

use rayhunter::analysis::analyzer::{AnalysisLineNormalizer, AnalyzerConfig, EventType};
use rayhunter::diag::{DataType, MessagesContainer};
use rayhunter::diag_device::DiagDevice;
use rayhunter::qmdl::QmdlWriter;

use crate::analysis::{AnalysisCtrlMessage, AnalysisWriter};
use crate::display;
use crate::notifications::{Notification, NotificationType};
use crate::qmdl_store::{RecordingStore, RecordingStoreError};
use crate::server::ServerState;

pub enum DiagDeviceCtrlMessage {
    StopRecording,
    StartRecording,
    DeleteEntry {
        name: String,
        response_tx: oneshot::Sender<Result<(), RecordingStoreError>>,
    },
    DeleteAllEntries {
        response_tx: oneshot::Sender<Result<(), RecordingStoreError>>,
    },
    Exit,
}

pub struct DiagTask {
    ui_update_sender: Sender<display::DisplayState>,
    analysis_sender: Sender<AnalysisCtrlMessage>,
    analyzer_config: AnalyzerConfig,
    notification_channel: tokio::sync::mpsc::Sender<Notification>,
    state: DiagState,
    max_type_seen: EventType,
}

enum DiagState {
    Recording {
        qmdl_writer: QmdlWriter<File>,
        analysis_writer: Box<AnalysisWriter>,
    },
    Stopped,
}

impl DiagTask {
    fn new(
        ui_update_sender: Sender<display::DisplayState>,
        analysis_sender: Sender<AnalysisCtrlMessage>,
        analyzer_config: AnalyzerConfig,
        notification_channel: tokio::sync::mpsc::Sender<Notification>,
    ) -> Self {
        Self {
            ui_update_sender,
            analysis_sender,
            analyzer_config,
            notification_channel,
            state: DiagState::Stopped,
            max_type_seen: EventType::Informational,
        }
    }

    /// Start recording
    async fn start(&mut self, qmdl_store: &mut RecordingStore) {
        self.max_type_seen = EventType::Informational;
        let (qmdl_file, analysis_file) = qmdl_store
            .new_entry()
            .await
            .expect("failed creating QMDL file entry");
        self.stop_current_recording().await;
        let qmdl_writer = QmdlWriter::new(qmdl_file);
        let analysis_writer = AnalysisWriter::new(analysis_file, &self.analyzer_config)
            .await
            .map(Box::new)
            .expect("failed to write to analysis file");
        self.state = DiagState::Recording {
            qmdl_writer,
            analysis_writer,
        };
        if let Err(e) = self
            .ui_update_sender
            .send(display::DisplayState::Recording)
            .await
        {
            warn!("couldn't send ui update message: {e}");
        }
    }

    /// Stop recording
    async fn stop(&mut self, qmdl_store: &mut RecordingStore) {
        self.stop_current_recording().await;
        if let Some((_, entry)) = qmdl_store.get_current_entry()
            && let Err(e) = self
                .analysis_sender
                .send(AnalysisCtrlMessage::RecordingFinished(
                    entry.name.to_string(),
                ))
                .await
        {
            warn!("couldn't send analysis message: {e}");
        }
        if let Err(e) = qmdl_store.close_current_entry().await {
            error!("couldn't close current entry: {e}");
        }
        if let Err(e) = self
            .ui_update_sender
            .send(display::DisplayState::Paused)
            .await
        {
            warn!("couldn't send ui update message: {e}");
        }
    }

    async fn delete_entry(
        &mut self,
        qmdl_store: &mut RecordingStore,
        name: &str,
    ) -> Result<(), RecordingStoreError> {
        if qmdl_store.is_current_entry(name) {
            self.stop(qmdl_store).await;
        }
        let res = qmdl_store.delete_entry(name).await;
        if let Err(e) = res.as_ref() {
            error!("Error deleting QMDL entry {e}");
        }
        res
    }

    async fn delete_all_entries(
        &mut self,
        qmdl_store: &mut RecordingStore,
    ) -> Result<(), RecordingStoreError> {
        self.stop(qmdl_store).await;
        let res = qmdl_store.delete_all_entries().await;
        if let Err(e) = res.as_ref() {
            error!("Error deleting QMDL entries {e}");
        }
        res
    }

    async fn stop_current_recording(&mut self) {
        let mut state = DiagState::Stopped;
        std::mem::swap(&mut self.state, &mut state);
        if let DiagState::Recording {
            analysis_writer, ..
        } = state
        {
            analysis_writer
                .close()
                .await
                .expect("failed to close analysis writer");
        }
    }

    async fn process_container(
        &mut self,
        qmdl_store: &mut RecordingStore,
        container: MessagesContainer,
    ) {
        if container.data_type != DataType::UserSpace {
            debug!("skipping non-userspace diag messages...");
            return;
        }
        // keep track of how many bytes were written to the QMDL file so we can read
        // a valid block of data from it in the HTTP server
        if let DiagState::Recording {
            qmdl_writer,
            analysis_writer,
        } = &mut self.state
        {
            qmdl_writer
                .write_container(&container)
                .await
                .expect("failed to write to QMDL writer");
            debug!(
                "total QMDL bytes written: {}, updating manifest...",
                qmdl_writer.total_written
            );
            let index = qmdl_store
                .current_entry
                .expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
            qmdl_store
                .update_entry_qmdl_size(index, qmdl_writer.total_written)
                .await
                .expect("failed to update qmdl file size");
            debug!("done!");
            let max_type = analysis_writer
                .analyze(container)
                .await
                .expect("failed to analyze container");

            if max_type > EventType::Informational {
                info!("a heuristic triggered on this run!");
                self.notification_channel
                    .send(Notification::new(
                        NotificationType::Warning,
                        format!("Rayhunter has detected a {:?} severity event", max_type),
                        Some(Duration::from_secs(60 * 5)),
                    ))
                    .await
                    .expect("Failed to send to notification channel");
            }

            if max_type > self.max_type_seen {
                self.max_type_seen = max_type;
                if self.max_type_seen > EventType::Informational {
                    self.ui_update_sender
                        .send(display::DisplayState::WarningDetected {
                            event_type: self.max_type_seen,
                        })
                        .await
                        .expect("couldn't send ui update message: {}");
                }
            }
        } else {
            debug!("no qmdl_writer set, continuing...");
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_diag_read_thread(
    task_tracker: &TaskTracker,
    mut dev: DiagDevice,
    mut qmdl_file_rx: Receiver<DiagDeviceCtrlMessage>,
    qmdl_file_tx: Sender<DiagDeviceCtrlMessage>,
    ui_update_sender: Sender<display::DisplayState>,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    analysis_sender: Sender<AnalysisCtrlMessage>,
    analyzer_config: AnalyzerConfig,
    notification_channel: tokio::sync::mpsc::Sender<Notification>,
) {
    task_tracker.spawn(async move {
        let mut diag_stream = pin!(dev.as_stream().into_stream());
        let mut diag_task = DiagTask::new(ui_update_sender, analysis_sender, analyzer_config, notification_channel);
        qmdl_file_tx
            .send(DiagDeviceCtrlMessage::StartRecording)
            .await
            .unwrap();
        loop {
            tokio::select! {
                msg = qmdl_file_rx.recv() => {
                    match msg {
                        Some(DiagDeviceCtrlMessage::StartRecording) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            diag_task.start(qmdl_store.deref_mut()).await;
                        },
                        Some(DiagDeviceCtrlMessage::StopRecording) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            diag_task.stop(qmdl_store.deref_mut()).await;
                        },
                        // None means all the Senders have been dropped, so it's
                        // time to go
                        Some(DiagDeviceCtrlMessage::Exit) | None => {
                            info!("Diag reader thread exiting...");
                            diag_task.stop_current_recording().await;
                            return Ok(())
                        },
                        Some(DiagDeviceCtrlMessage::DeleteEntry { name, response_tx }) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            let resp = diag_task.delete_entry(qmdl_store.deref_mut(), name.as_str()).await;
                            if response_tx.send(resp).is_err() {
                                error!("Failed to send delete entry respons, receiver dropped");
                            }
                        },
                        Some(DiagDeviceCtrlMessage::DeleteAllEntries { response_tx }) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            let resp = diag_task.delete_all_entries(qmdl_store.deref_mut()).await;
                            if response_tx.send(resp).is_err() {
                                error!("Failed to send delete all entries respons, receiver dropped");
                            }
                        },
                    }
                }
                maybe_container = diag_stream.next() => {
                    match maybe_container.unwrap() {
                        Ok(container) => {
                            let mut qmdl_store = qmdl_store_lock.write().await;
                            diag_task.process_container(qmdl_store.deref_mut(), container).await
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

/// Start recording API for web thread
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

/// Stop recording API for web thread
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
    let (response_tx, response_rx) = oneshot::channel();
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::DeleteEntry {
            name: qmdl_name.clone(),
            response_tx,
        })
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send delete entry message: {e}"),
            )
        })?;
    match response_rx.await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to receive delete response: {e}"),
        )
    })? {
        Ok(_) => Ok((StatusCode::ACCEPTED, "ok".to_string())),
        Err(RecordingStoreError::NoSuchEntryError) => Err((
            StatusCode::BAD_REQUEST,
            format!("no recording with name {qmdl_name}"),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("couldn't delete recording: {e}"),
        )),
    }
}

pub async fn delete_all_recordings(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.config.debug_mode {
        return Err((StatusCode::FORBIDDEN, "server is in debug mode".to_string()));
    }
    let (response_tx, response_rx) = oneshot::channel();
    state
        .diag_device_ctrl_sender
        .send(DiagDeviceCtrlMessage::DeleteAllEntries { response_tx })
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("couldn't send delete all entries message: {e}"),
            )
        })?;
    match response_rx.await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to receive delete all response: {e}"),
        )
    })? {
        Ok(_) => Ok((StatusCode::ACCEPTED, "ok".to_string())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("couldn't delete recordings: {e}"),
        )),
    }
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

    // Read and normalize the NDJSON file
    let reader = BufReader::new(analysis_file);
    let lines_stream = LinesStream::new(reader.lines());

    let mut normalizer = AnalysisLineNormalizer::new();
    let normalized_stream = lines_stream
        .try_filter(|line| future::ready(!line.is_empty()))
        .map_ok(move |line| normalizer.normalize_line(line));

    let headers = [(CONTENT_TYPE, "application/x-ndjson")];
    let body = Body::from_stream(normalized_stream);
    Ok((headers, body).into_response())
}
