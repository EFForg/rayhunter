use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use orca::diag_device::DiagDevice;
use orca::diag_reader::DiagReader;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{Receiver, self};
use orca::qmdl::QmdlWriter;
use log::{debug, info};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;

use crate::error::WavehunterError;
use crate::qmdl_store::QmdlStore;
use crate::server::ServerState;

pub enum DiagDeviceCtrlMessage {
    StopRecording,
    StartRecording(QmdlWriter<std::fs::File>),
    Exit,
}

pub fn run_diag_read_thread(task_tracker: &TaskTracker, mut dev: DiagDevice, mut qmdl_file_rx: Receiver<DiagDeviceCtrlMessage>, qmdl_store_lock: Arc<RwLock<QmdlStore>>) -> JoinHandle<Result<(), WavehunterError>> {
    let (tx, mut rx) = mpsc::channel::<(usize, usize)>(1);
    let qmdl_store_lock_clone = qmdl_store_lock.clone();
    task_tracker.spawn(async move {
        while let Some((entry_idx, new_size)) = rx.recv().await {
            let mut qmdl_store = qmdl_store_lock_clone.write().await;
            qmdl_store.update_entry_size(entry_idx, new_size).await
                .expect("failed to update qmdl file size");
        }
        info!("QMDL store size updater thread exiting...");
    });

    task_tracker.spawn_blocking(move || {
        loop {
            match qmdl_file_rx.try_recv() {
                Ok(DiagDeviceCtrlMessage::StartRecording(qmdl_writer)) => {
                    dev.qmdl_writer = Some(qmdl_writer);
                },
                Ok(DiagDeviceCtrlMessage::StopRecording) => dev.qmdl_writer = None,
                Ok(DiagDeviceCtrlMessage::Exit) | Err(TryRecvError::Disconnected) => {
                    info!("Diag reader thread exiting...");
                    return Ok(())
                },
                // empty just means there's no message for us, so continue as normal
                Err(TryRecvError::Empty) => {},
            }

            // remember the QmdlStore current entry index so we can update its size later
            let qmdl_store_index = qmdl_store_lock.blocking_read().current_entry;

            // TODO: once we're actually doing analysis, we'll wanna use the messages
            // returned here. Until then, the DiagDevice has already written those messages
            // to the QMDL file, so we can just ignore them.
            debug!("reading response from diag device...");
            let _messages = dev.read_response().map_err(WavehunterError::DiagReadError)?;
            debug!("got diag response ({} messages)", _messages.len());

            // keep track of how many bytes were written to the QMDL file so we can read
            // a valid block of data from it in the HTTP server
            if let Some(qmdl_writer) = dev.qmdl_writer.as_ref() {
                debug!("total QMDL bytes written: {}, sending update...", qmdl_writer.total_written);
                let index = qmdl_store_index.expect("DiagDevice had qmdl_writer, but QmdlStore didn't have current entry???");
                tx.blocking_send((index, qmdl_writer.total_written)).unwrap();
                debug!("done!");
            } else {
                debug!("no qmdl_writer set, continuing...");
            }
        }
    })
}

pub async fn start_recording(State(state): State<Arc<ServerState>>) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.readonly_mode {
        return Err((StatusCode::FORBIDDEN, format!("server is in readonly mode")));
    }
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    let qmdl_file = qmdl_store.new_entry().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't create new qmdl entry: {}", e)))?;
    let qmdl_writer = QmdlWriter::new(qmdl_file.into_std().await);
    state.diag_device_ctrl_sender.send(DiagDeviceCtrlMessage::StartRecording(qmdl_writer)).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send stop recording message: {}", e)))?;
    Ok((StatusCode::ACCEPTED, format!("ok")))
}

pub async fn stop_recording(State(state): State<Arc<ServerState>>) -> Result<(StatusCode, String), (StatusCode, String)> {
    if state.readonly_mode {
        return Err((StatusCode::FORBIDDEN, format!("server is in readonly mode")));
    }
    let mut qmdl_store = state.qmdl_store_lock.write().await;
    qmdl_store.close_current_entry().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't close current qmdl entry: {}", e)))?;
    state.diag_device_ctrl_sender.send(DiagDeviceCtrlMessage::StopRecording).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("couldn't send stop recording message: {}", e)))?;
    Ok((StatusCode::ACCEPTED, format!("ok")))
}
