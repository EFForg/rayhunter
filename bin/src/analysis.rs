use std::sync::Arc;
use std::{future, pin};

use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use futures::TryStreamExt;
use log::{debug, error, info};
use rayhunter::analysis::analyzer::Harness;
use rayhunter::diag::{DataType, MessagesContainer};
use rayhunter::qmdl::QmdlReader;
use serde::Serialize;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio_util::task::TaskTracker;

use crate::qmdl_store::RecordingStore;
use crate::server::ServerState;
use crate::dummy_analyzer::TestAnalyzer;

pub struct AnalysisWriter {
    writer: BufWriter<File>,
    harness: Harness,
    bytes_written: usize,
}

// We write our analysis results to a file immediately to minimize the amount of
// state Rayhunter has to keep track of in memory. The analysis file's format is
// Newline Delimited JSON
// (https://docs.mulesoft.com/dataweave/latest/dataweave-formats-ndjson), which
// lets us simply append new rows to the end without parsing the entire JSON
// object beforehand.
impl AnalysisWriter {
    pub async fn new(file: File, enable_dummy_analyzer: bool) -> Result<Self, std::io::Error> {
        let mut harness = Harness::new_with_all_analyzers();
        if enable_dummy_analyzer {
            harness.add_analyzer(Box::new(TestAnalyzer { count: 0 }));
        }

        let mut result = Self {
            writer: BufWriter::new(file),
            bytes_written: 0,
            harness,
        };
        let metadata = result.harness.get_metadata();
        result.write(&metadata).await?;
        Ok(result)
    }

    // Runs the analysis harness on the given container, serializing the results
    // to the analysis file and returning the file's new length.
    pub async fn analyze(&mut self, container: MessagesContainer) -> Result<(usize, bool), std::io::Error> {
        let row = self.harness.analyze_qmdl_messages(container);
        if !row.is_empty() {
            self.write(&row).await?;
        }
        Ok((self.bytes_written, row.contains_warnings()))
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

#[derive(Debug, Serialize, Clone, Default)]
pub struct AnalysisStatus {
    queued: Vec<String>,
    running: Option<String>,
}

pub enum AnalysisCtrlMessage {
    NewFilesQueued,
    Exit,
}

async fn queued_len(analysis_status_lock: Arc<RwLock<AnalysisStatus>>) -> usize {
    analysis_status_lock.read().await.queued.len()
}

async fn dequeue_to_running(analysis_status_lock: Arc<RwLock<AnalysisStatus>>) -> String {
    let mut analysis_status = analysis_status_lock.write().await;
    let name = analysis_status.queued.remove(0);
    assert!(analysis_status.running.is_none());
    analysis_status.running = Some(name.clone());
    name
}

async fn clear_running(analysis_status_lock: Arc<RwLock<AnalysisStatus>>) {
    let mut analysis_status = analysis_status_lock.write().await;
    analysis_status.running = None;
}

async fn perform_analysis(
    name: &str,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    enable_dummy_analyzer: bool,
) -> Result<(), String> {
    info!("Opening QMDL and analysis file for {}...", name);
    let (analysis_file, qmdl_file, entry_index) = {
        let mut qmdl_store = qmdl_store_lock.write().await;
        let (entry_index, _) = qmdl_store
            .entry_for_name(name)
            .ok_or(format!("failed to find QMDL store entry for {}", name))?;
        let analysis_file = qmdl_store
            .clear_and_open_entry_analysis(entry_index)
            .await
            .map_err(|e| format!("{:?}", e))?;
        let qmdl_file = qmdl_store
            .open_entry_qmdl(entry_index)
            .await
            .map_err(|e| format!("{:?}", e))?;

        (analysis_file, qmdl_file, entry_index)
    };

    let mut analysis_writer = AnalysisWriter::new(analysis_file, enable_dummy_analyzer)
        .await
        .map_err(|e| format!("{:?}", e))?;
    let file_size = qmdl_file
        .metadata()
        .await
        .expect("failed to get QMDL file metadata")
        .len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin::pin!(qmdl_reader
        .as_stream()
        .try_filter(|container| future::ready(container.data_type == DataType::UserSpace)));

    info!("Starting analysis for {}...", name);
    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        let (size_bytes, _) = analysis_writer
            .analyze(container)
            .await
            .map_err(|e| format!("{:?}", e))?;
        debug!("{} analysis: {} bytes written", name, size_bytes);
        let mut qmdl_store = qmdl_store_lock.write().await;
        qmdl_store
            .update_entry_analysis_size(entry_index, size_bytes)
            .await
            .map_err(|e| format!("{:?}", e))?;
    }

    analysis_writer
        .close()
        .await
        .map_err(|e| format!("{:?}", e))?;
    info!("Analysis for {} complete!", name);

    Ok(())
}

pub fn run_analysis_thread(
    task_tracker: &TaskTracker,
    mut analysis_rx: Receiver<AnalysisCtrlMessage>,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    enable_dummy_analyzer: bool,
) {
    task_tracker.spawn(async move {
        loop {
            match analysis_rx.recv().await {
                Some(AnalysisCtrlMessage::NewFilesQueued) => {
                    let count = queued_len(analysis_status_lock.clone()).await;
                    for _ in 0..count {
                        let name = dequeue_to_running(analysis_status_lock.clone()).await;
                        if let Err(err) = perform_analysis(&name, qmdl_store_lock.clone(), enable_dummy_analyzer).await {
                            error!("failed to analyze {}: {}", name, err);
                        }
                        clear_running(analysis_status_lock.clone()).await;
                    }
                }
                Some(AnalysisCtrlMessage::Exit) | None => return,
            }
        }
    });
}

pub async fn get_analysis_status(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<AnalysisStatus>, (StatusCode, String)> {
    Ok(Json(state.analysis_status_lock.read().await.clone()))
}

fn queue_qmdl(name: &str, analysis_status: &mut RwLockWriteGuard<AnalysisStatus>) -> bool {
    if analysis_status.queued.iter().any(|n| n == name)
        || analysis_status.running.iter().any(|n| n == name)
    {
        return false;
    }
    analysis_status.queued.push(name.to_string());
    true
}

pub async fn start_analysis(
    State(state): State<Arc<ServerState>>,
    Path(qmdl_name): Path<String>,
) -> Result<(StatusCode, Json<AnalysisStatus>), (StatusCode, String)> {
    let mut analysis_status = state.analysis_status_lock.write().await;
    let store = state.qmdl_store_lock.read().await;
    let queued = if qmdl_name.is_empty() {
        let mut entry_names: Vec<&str> = store
            .manifest
            .entries
            .iter()
            .map(|e| e.name.as_str())
            .collect();
        if let Some(current_entry) = store.current_entry {
            entry_names.remove(current_entry);
        }
        entry_names
            .iter()
            .any(|name| queue_qmdl(name, &mut analysis_status))
    } else {
        queue_qmdl(&qmdl_name, &mut analysis_status)
    };
    if queued {
        state
            .analysis_sender
            .send(AnalysisCtrlMessage::NewFilesQueued)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to queue new analysis files: {:?}", e),
                )
            })?;
    }
    Ok((StatusCode::ACCEPTED, Json(analysis_status.clone())))
}
