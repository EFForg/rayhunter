use axum::body::Body;
use axum::http::header::{CONTENT_TYPE, self};
use axum::extract::State;
use axum::http::{StatusCode, HeaderValue};
use axum::response::{Response, IntoResponse};
use axum::extract::Path;
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs::File as AsyncFile;
use tokio_util::io::ReaderStream;
use include_dir::{include_dir, Dir};

pub struct ServerState {
    pub qmdl_bytes_written: Arc<RwLock<usize>>,
    pub qmdl_path: String,
}

pub async fn get_qmdl(State(state): State<Arc<ServerState>>) -> Result<Response, (StatusCode, String)> {
    let qmdl_file = AsyncFile::open(&state.qmdl_path).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("error opening QMDL file: {}", e)))?;
    let qmdl_bytes_written = *state.qmdl_bytes_written.read().await;
    let limited_qmdl_file = qmdl_file.take(qmdl_bytes_written as u64);
    let qmdl_stream = ReaderStream::new(limited_qmdl_file);

    let headers = [(CONTENT_TYPE, "application/octet-stream")];
    let body = Body::from_stream(qmdl_stream);
    Ok((headers, body).into_response())
}

// Bundles the server's static files (html/css/js) into the binary for easy distribution
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
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
