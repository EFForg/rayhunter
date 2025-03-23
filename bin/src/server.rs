use axum::body::Body;
use axum::http::header::{CONTENT_TYPE, self};
use axum::extract::State;
use axum::http::{StatusCode, HeaderValue};
use axum::response::{Response, IntoResponse};
use axum::extract::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::io::ReaderStream;
use include_dir::{include_dir, Dir};

use crate::{framebuffer, DiagDeviceCtrlMessage};
use crate::analysis::{AnalysisCtrlMessage, AnalysisStatus};
use crate::qmdl_store::RecordingStore;

pub struct ServerState {
    pub qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    pub diag_device_ctrl_sender: Sender<DiagDeviceCtrlMessage>,
    pub ui_update_sender: Sender<framebuffer::DisplayState>,
    pub analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    pub analysis_sender: Sender<AnalysisCtrlMessage>,
    pub debug_mode: bool,
    pub colorblind_mode: bool,
}

pub async fn get_qmdl(State(state): State<Arc<ServerState>>, Path(qmdl_name): Path<String>) -> Result<Response, (StatusCode, String)> {
    let qmdl_idx = qmdl_name.trim_end_matches(".qmdl");
    let qmdl_store = state.qmdl_store_lock.read().await;
    let (entry_index, entry) = qmdl_store.entry_for_name(qmdl_idx)
        .ok_or((StatusCode::NOT_FOUND, format!("couldn't find qmdl file with name {}", qmdl_idx)))?;
    let qmdl_file = qmdl_store.open_entry_qmdl(entry_index).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("error opening QMDL file: {}", e)))?;
    let limited_qmdl_file = qmdl_file.take(entry.qmdl_size_bytes as u64);
    let qmdl_stream = ReaderStream::new(limited_qmdl_file);

    let headers = [(CONTENT_TYPE, "application/octet-stream")];
    let body = Body::from_stream(qmdl_stream);
    Ok((headers, body).into_response())
}

// Bundles the server's static files (html/css/js) into the binary for easy distribution
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn serve_static(State(state): State<Arc<ServerState>>, Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    // if we're in debug mode, return the files from the build directory so we
    // don't have to rebuild every time the JS/HTML change
    if state.debug_mode {
        let mut build_path = std::path::PathBuf::new();
        build_path.push("bin");
        build_path.push("static");
        for part in path.split("/") {
            build_path.push(part);
        }
        return match File::open(build_path).await {
            Ok(mut file) => {
                let mut body = String::new();
                file.read_to_string(&mut body).await.expect("failed to read file");
                Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                    )
                    .body(Body::from(body))
                    .unwrap()
            },
            Err(_) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        };
    }

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
