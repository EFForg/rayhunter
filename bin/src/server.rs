use axum::body::Body;
use axum::extract::Path;
use axum::extract::State;
use axum::http::header::{self, CONTENT_LENGTH, CONTENT_TYPE};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use include_dir::{include_dir, Dir};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
use tokio::sync::{oneshot, RwLock};
use tokio_util::io::ReaderStream;

use crate::analysis::{AnalysisCtrlMessage, AnalysisStatus};
use crate::qmdl_store::RecordingStore;
use crate::{display, DiagDeviceCtrlMessage};

pub struct ServerState {
    pub qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    pub diag_device_ctrl_sender: Sender<DiagDeviceCtrlMessage>,
    pub ui_update_sender: Sender<display::DisplayState>,
    pub analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    pub analysis_sender: Sender<AnalysisCtrlMessage>,
    pub debug_mode: bool,
    pub daemon_restart_tx: Arc<RwLock<Option<oneshot::Sender<()>>>>,
}

pub async fn get_qmdl(
    State(state): State<Arc<ServerState>>,
    Path(qmdl_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_idx = qmdl_name.trim_end_matches(".qmdl");
    let qmdl_store = state.qmdl_store_lock.read().await;
    let (entry_index, entry) = qmdl_store.entry_for_name(qmdl_idx).ok_or((
        StatusCode::NOT_FOUND,
        format!("couldn't find qmdl file with name {}", qmdl_idx),
    ))?;
    let qmdl_file = qmdl_store.open_entry_qmdl(entry_index).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error opening QMDL file: {}", e),
        )
    })?;
    let limited_qmdl_file = qmdl_file.take(entry.qmdl_size_bytes as u64);
    let qmdl_stream = ReaderStream::new(limited_qmdl_file);

    let headers = [
        (CONTENT_TYPE, "application/octet-stream"),
        (CONTENT_LENGTH, &entry.qmdl_size_bytes.to_string()),
    ];
    let body = Body::from_stream(qmdl_stream);
    Ok((headers, body).into_response())
}

// Bundles the server's static files (html/css/js) into the binary for easy distribution
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web/build");

pub async fn serve_static(
    State(_): State<Arc<ServerState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(Body::from(file.contents()))
            .unwrap(),
    }
}

pub async fn restart_daemon(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut restart_tx = state.daemon_restart_tx.write().await;

    if let Some(sender) = restart_tx.take() {
        sender.send(()).map_err(|()| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "couldn't send restart signal".to_string(),
            )
        })?;

        Ok((StatusCode::ACCEPTED, "restart signal sent".to_string()))
    } else {
        Ok((
            StatusCode::ACCEPTED,
            "restart already triggered".to_string(),
        ))
    }
}
