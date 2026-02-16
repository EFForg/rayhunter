use anyhow::Error;
use async_zip::Compression;
use async_zip::ZipEntryBuilder;
use async_zip::tokio::write::ZipFileWriter;
use axum::Json;
use axum::body::Body;
use axum::extract::Path;
use axum::extract::State;
use axum::http::header::{self, CONTENT_LENGTH, CONTENT_TYPE};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Local};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs::write;
use tokio::io::{AsyncReadExt, copy, duplex};
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use tokio_util::compat::FuturesAsyncWriteCompatExt;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

use crate::analysis::{AnalysisCtrlMessage, AnalysisStatus};
use crate::config::Config;
use crate::diag::DiagDeviceCtrlMessage;
use crate::display::DisplayState;
use crate::pcap::generate_pcap_data;
use crate::qmdl_store::RecordingStore;

pub struct ServerState {
    pub config_path: String,
    pub config: Config,
    pub qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    pub diag_device_ctrl_sender: Sender<DiagDeviceCtrlMessage>,
    pub analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    pub analysis_sender: Sender<AnalysisCtrlMessage>,
    pub daemon_restart_token: CancellationToken,
    pub ui_update_sender: Option<Sender<DisplayState>>,
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/qmdl/{name}",
    tag = "Recordings",
    responses(
        (status = StatusCode::OK, description = "QMDL download successful", content_type = "application/octet-stream"),
        (status = StatusCode::NOT_FOUND, description = "Could not find file {name}"),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "QMDL file is empty, or error opening file")
    ),
    params(
        ("name" = String, Path, description = "QMDL filename to convert and download")
    ),
    summary = "Download a QMDL file",
    description = "Stream the QMDL file {name} to the client."
))]
pub async fn get_qmdl(
    State(state): State<Arc<ServerState>>,
    Path(qmdl_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_idx = qmdl_name.trim_end_matches(".qmdl");
    let qmdl_store = state.qmdl_store_lock.read().await;
    let (entry_index, entry) = qmdl_store.entry_for_name(qmdl_idx).ok_or((
        StatusCode::NOT_FOUND,
        format!("couldn't find qmdl file with name {qmdl_idx}"),
    ))?;
    let qmdl_file = qmdl_store
        .open_entry_qmdl(entry_index)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error opening QMDL file: {err}"),
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

pub async fn serve_static(
    State(_): State<Arc<ServerState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    match path {
        "rayhunter_orca_only.png" => (
            [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
            include_bytes!("../web/build/rayhunter_orca_only.png"),
        )
            .into_response(),
        "rayhunter_text.png" => (
            [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
            include_bytes!("../web/build/rayhunter_text.png"),
        )
            .into_response(),
        "favicon.png" => (
            [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
            include_bytes!("../web/build/favicon.png"),
        )
            .into_response(),
        "index.html" => (
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("text/html")),
                (header::CONTENT_ENCODING, HeaderValue::from_static("gzip")),
            ],
            include_bytes!("../web/build/index.html.gz"),
        )
            .into_response(),
        path => {
            warn!("404 on path: {path}");
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/config",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Success", body = Config)
    ),
    summary = "Get config",
    description = "Show the running configuration for Rayhunter."
))]
pub async fn get_config(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<Config>, (StatusCode, String)> {
    Ok(Json(state.config.clone()))
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    post,
    path = "/api/config",
    tag = "Configuration",
    request_body(
        content = Option<[Config]>,
        description = "Any or all configuration elements from the valid config schema to be altered may be passed. Invalid keys will be discarded. Invalid values or value types will return an error."
    ),
    responses(
        (status = StatusCode::ACCEPTED, description = "Success"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to parse or write config file"),
        (status = 422, description = "Failed to deserialize JSON body")
    ),
    summary = "Set config",
    description = "Write a new configuration for Rayhunter and trigger a restart."
))]
pub async fn set_config(
    State(state): State<Arc<ServerState>>,
    Json(config): Json<Config>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let config_str = toml::to_string_pretty(&config).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to serialize config as TOML: {err}"),
        )
    })?;

    write(&state.config_path, config_str).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to write config file: {err}"),
        )
    })?;

    // Trigger daemon restart after writing config
    state.daemon_restart_token.cancel();
    Ok((
        StatusCode::ACCEPTED,
        "wrote config and triggered restart".to_string(),
    ))
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    post,
    path = "/api/test-notification",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Success"),
        (status = StatusCode::BAD_REQUEST, description = "No notification URL set"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Failed to send HTTP request. Ensure your device can reach the internet.")
    ),
    summary = "Test ntfy notification",
    description = "Send a test notification to the ntfy_url in the running configuration for Rayhunter."
))]
pub async fn test_notification(
    State(state): State<Arc<ServerState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let url = state.config.ntfy_url.as_ref().ok_or((
        StatusCode::BAD_REQUEST,
        "No notification URL configured".to_string(),
    ))?;

    if url.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Notification URL is empty".to_string(),
        ));
    }

    let http_client = reqwest::Client::new();
    let message = "Test notification from Rayhunter".to_string();

    crate::notifications::send_notification(&http_client, url, message)
        .await
        .map(|()| {
            (
                StatusCode::OK,
                "Test notification sent successfully".to_string(),
            )
        })
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send test notification: {e}"),
            )
        })
}

/// Response for GET /api/time
#[derive(Serialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct TimeResponse {
    /// The raw system time (without clock offset)
    #[cfg_attr(feature = "apidocs", schema(value_type = String))]
    pub system_time: DateTime<Local>,
    /// The adjusted time (system time + offset)
    #[cfg_attr(feature = "apidocs", schema(value_type = String))]
    pub adjusted_time: DateTime<Local>,
    /// The current offset in seconds
    pub offset_seconds: i64,
}

/// Request for POST /api/time-offset
#[derive(Deserialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct SetTimeOffsetRequest {
    /// The offset to set, in seconds
    pub offset_seconds: i64,
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/time",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Success", body = TimeResponse)
    ),
    summary = "Get time",
    description = "Get the current time and offset (in seconds) of the device."
))]
pub async fn get_time() -> Json<TimeResponse> {
    let system_time = Local::now();
    let adjusted_time = rayhunter::clock::get_adjusted_now();
    let offset_seconds = adjusted_time
        .signed_duration_since(system_time)
        .num_seconds();
    Json(TimeResponse {
        system_time,
        adjusted_time,
        offset_seconds,
    })
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/time-offset",
    tag = "Configuration",
    request_body(
        content = SetTimeOffsetRequest
    ),
    responses(
        (status = StatusCode::OK, description = "Success", body = TimeResponse)
    ),
    summary = "Set time offset",
    description = "Set the difference (in seconds) between the system time and the adjusted time for Rayhunter."
))]
pub async fn set_time_offset(Json(req): Json<SetTimeOffsetRequest>) -> StatusCode {
    rayhunter::clock::set_offset(chrono::TimeDelta::seconds(req.offset_seconds));
    StatusCode::OK
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/zip/{name}",
    tag = "Recordings",
    responses(
        (status = StatusCode::OK, description = "ZIP download successful. It is possible that if the PCAP fails to convert, the same status will be returned, but the file will contain only the QMDL file.", content_type = "application/zip"),
        (status = StatusCode::NOT_FOUND, description = "Could not find file {name}"),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "QMDL file is empty, or error opening file")
    ),
    params(
        ("name" = String, Path, description = "QMDL filename to convert and download")
    ),
    summary = "Download a ZIP file",
    description = "Stream a ZIP file to the client which contains the QMDL file {name} and a PCAP generated from the same file."
))]
pub async fn get_zip(
    State(state): State<Arc<ServerState>>,
    Path(entry_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_idx = entry_name.trim_end_matches(".zip").to_owned();
    let (entry_index, qmdl_size_bytes) = {
        let qmdl_store = state.qmdl_store_lock.read().await;
        let (entry_index, entry) = qmdl_store.entry_for_name(&qmdl_idx).ok_or((
            StatusCode::NOT_FOUND,
            format!("couldn't find entry with name {qmdl_idx}"),
        ))?;

        if entry.qmdl_size_bytes == 0 {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                "QMDL file is empty, try again in a bit!".to_string(),
            ));
        }

        (entry_index, entry.qmdl_size_bytes)
    };

    let qmdl_store_lock = state.qmdl_store_lock.clone();

    let (reader, writer) = duplex(8192);

    tokio::spawn(async move {
        let result: Result<(), Error> = async {
            let mut zip = ZipFileWriter::with_tokio(writer);

            // Add QMDL file
            {
                let entry =
                    ZipEntryBuilder::new(format!("{qmdl_idx}.qmdl").into(), Compression::Stored);
                // FuturesAsyncWriteCompatExt::compat_write because async-zip's entrystream does
                // not impl tokio's AsyncWrite, but only future's AsyncWrite. This can be removed
                // once https://github.com/Majored/rs-async-zip/pull/160 is released.
                let mut entry_writer = zip.write_entry_stream(entry).await?.compat_write();

                let mut qmdl_file = {
                    let qmdl_store = qmdl_store_lock.read().await;
                    qmdl_store
                        .open_entry_qmdl(entry_index)
                        .await?
                        .take(qmdl_size_bytes as u64)
                };

                copy(&mut qmdl_file, &mut entry_writer).await?;
                entry_writer.into_inner().close().await?;
            }

            // Add PCAP file
            {
                let entry =
                    ZipEntryBuilder::new(format!("{qmdl_idx}.pcapng").into(), Compression::Stored);
                let mut entry_writer = zip.write_entry_stream(entry).await?.compat_write();

                let qmdl_file_for_pcap = {
                    let qmdl_store = qmdl_store_lock.read().await;
                    qmdl_store
                        .open_entry_qmdl(entry_index)
                        .await?
                        .take(qmdl_size_bytes as u64)
                };

                if let Err(e) =
                    generate_pcap_data(&mut entry_writer, qmdl_file_for_pcap, qmdl_size_bytes).await
                {
                    // if we fail to generate the PCAP file, we should still continue and give the
                    // user the QMDL.
                    error!("Failed to generate PCAP: {e:?}");
                }

                entry_writer.into_inner().close().await?;
            }

            zip.close().await?;
            Ok(())
        }
        .await;

        if let Err(e) = result {
            error!("Error generating ZIP file: {e:?}");
        }
    });

    let headers = [(CONTENT_TYPE, "application/zip")];
    let body = Body::from_stream(ReaderStream::new(reader));
    Ok((headers, body).into_response())
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    post,
    path = "/api/debug/display-state",
    tag = "Configuration",
    request_body(
        content = DisplayState
    ),
    responses(
        (status = StatusCode::OK, description = "Display state updated successfully"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Error sending update to the display"),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "Display system not available")
    ),
    summary = "Set display state",
    description = "Change the display state (color bar or otherwise) of the device for debugging purposes."
))]
pub async fn debug_set_display_state(
    State(state): State<Arc<ServerState>>,
    Json(display_state): Json<DisplayState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if let Some(ui_sender) = &state.ui_update_sender {
        ui_sender.send(display_state).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to send display state update".to_string(),
            )
        })?;
        Ok((
            StatusCode::OK,
            "display state updated successfully".to_string(),
        ))
    } else {
        Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "display system not available".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_zip::base::read::mem::ZipFileReader;
    use axum::extract::{Path, State};
    use tempfile::TempDir;

    async fn create_test_qmdl_store() -> (TempDir, Arc<RwLock<crate::qmdl_store::RecordingStore>>) {
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().to_path_buf();
        let store = crate::qmdl_store::RecordingStore::create(&store_path)
            .await
            .unwrap();
        (temp_dir, Arc::new(RwLock::new(store)))
    }

    async fn create_test_entry_with_data(
        store_lock: &Arc<RwLock<crate::qmdl_store::RecordingStore>>,
        test_data: &[u8],
    ) -> String {
        let entry_name = {
            let mut store = store_lock.write().await;
            let (mut qmdl_file, _analysis_file) = store.new_entry().await.unwrap();

            if !test_data.is_empty() {
                use tokio::io::AsyncWriteExt;
                qmdl_file.write_all(test_data).await.unwrap();
                qmdl_file.flush().await.unwrap();
            }

            let current_entry = store.current_entry.unwrap();
            let entry = &store.manifest.entries[current_entry];
            let entry_name = entry.name.clone();

            store
                .update_entry_qmdl_size(current_entry, test_data.len())
                .await
                .unwrap();
            entry_name
        };

        let mut store = store_lock.write().await;
        store.close_current_entry().await.unwrap();
        entry_name
    }

    fn create_test_server_state(
        store_lock: Arc<RwLock<crate::qmdl_store::RecordingStore>>,
    ) -> Arc<ServerState> {
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let (analysis_tx, _analysis_rx) = tokio::sync::mpsc::channel(1);

        let analysis_status = {
            let store = store_lock.try_read().unwrap();
            crate::analysis::AnalysisStatus::new(&store)
        };

        Arc::new(ServerState {
            config_path: "/tmp/test_config.toml".to_string(),
            config: Config::default(),
            qmdl_store_lock: store_lock,
            diag_device_ctrl_sender: tx,
            analysis_status_lock: Arc::new(RwLock::new(analysis_status)),
            analysis_sender: analysis_tx,
            daemon_restart_token: CancellationToken::new(),
            ui_update_sender: None,
        })
    }

    #[tokio::test]
    async fn test_get_zip_success() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        let test_qmdl_data = vec![0x7E, 0x00, 0x00, 0x00, 0x10, 0x00, 0x7E];
        let entry_name = create_test_entry_with_data(&store_lock, &test_qmdl_data).await;
        let state = create_test_server_state(store_lock);

        let result = get_zip(State(state), Path(entry_name.clone())).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        let headers = response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "application/zip");

        let body = response.into_body();
        let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

        let zip_reader = ZipFileReader::new(body_bytes.to_vec()).await.unwrap();

        let filenames = zip_reader
            .file()
            .entries()
            .iter()
            .map(|entry| entry.filename().as_str().unwrap().to_owned())
            .collect::<Vec<String>>();

        assert_eq!(
            filenames,
            vec![format!("{entry_name}.qmdl"), format!("{entry_name}.pcapng"),]
        );
    }
}
