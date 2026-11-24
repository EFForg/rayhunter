use anyhow::Error;
use async_zip::Compression;
use async_zip::ZipEntryBuilder;
use async_zip::tokio::write::ZipFileWriter;
use axum::Json;
use axum::body::Body;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header::{self, CONTENT_TYPE};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Local};
use futures::TryStreamExt;
use log::{error, warn};
use rayhunter::analysis::analyzer::{AnalyzerMetadata, get_analyzers_metadata};
use rayhunter::qmdl::QmdlMessageReader;
use serde::{Deserialize, Serialize};
use std::pin::pin;
use std::sync::Arc;
use tokio::fs::write;
use tokio::io::DuplexStream;
use tokio::io::copy;
use tokio::io::duplex;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::compat::FuturesAsyncWriteCompatExt;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

use crate::analysis::{AnalysisCtrlMessage, AnalysisStatus};
use crate::config::{Config, GpsMode};
use crate::diag::DiagDeviceCtrlMessage;
use crate::display::DisplayState;
use crate::gps::{GpsData, GpsRecord};
use crate::notifications::DEFAULT_NOTIFICATION_TIMEOUT;
use crate::pcap::{generate_pcap_data, load_gps_records_for_entry};
use crate::qmdl_store::{FileKind, RecordingStore};
use crate::update::UpdateStatus;

pub struct ServerState {
    pub config_path: String,
    pub config: Config,
    pub qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    pub diag_device_ctrl_sender: Sender<DiagDeviceCtrlMessage>,
    pub analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    pub analysis_sender: Sender<AnalysisCtrlMessage>,
    pub daemon_restart_token: CancellationToken,
    pub ui_update_sender: Option<Sender<DisplayState>>,
    pub wifi_status: Arc<RwLock<wifi_station::WifiStatus>>,
    pub wifi_scan_lock: tokio::sync::Mutex<()>,
    pub gps_state: Arc<RwLock<Option<GpsData>>>,
    pub update_status_lock: Arc<RwLock<UpdateStatus>>,
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
    let (entry_index, _) = qmdl_store.entry_for_name(qmdl_idx).ok_or((
        StatusCode::NOT_FOUND,
        format!("couldn't find qmdl file with name {qmdl_idx}"),
    ))?;
    let qmdl_file = qmdl_store
        .open_file(entry_index, FileKind::Qmdl)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error opening QMDL file: {err}"),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "QMDL file not found".to_string()))?;
    let qmdl_reader = QmdlMessageReader::new(qmdl_file).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error reading QMDL file: {err}"),
        )
    })?;

    let headers = [(CONTENT_TYPE, "application/octet-stream")];
    let body = Body::from_stream(qmdl_reader.into_qmdl_stream());
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
    let mut config = state.config.clone();
    config.wifi_password = None;
    Ok(Json(config))
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/analyzers",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Success", body = Vec<AnalyzerMetadata>)
    ),
    summary = "List analyzers",
    description = "List the analyzers compiled into Rayhunter, including the config key that enables each of them."
))]
pub async fn get_analyzers() -> Json<Vec<AnalyzerMetadata>> {
    Json(get_analyzers_metadata())
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
    Json(mut config): Json<Config>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if config.gps_mode != GpsMode::Fixed {
        config.gps_fixed_latitude = None;
        config.gps_fixed_longitude = None;
    }
    let mut config_to_write = config.clone();
    config_to_write.wifi_ssid = None;
    config_to_write.wifi_password = None;
    config_to_write.wifi_security = None;

    let config_str = toml::to_string_pretty(&config_to_write).map_err(|err| {
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

    wifi_station::update_wpa_conf(&config.wifi_config()).await;

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

    let http_client = crate::http_client::client().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create HTTP client: {err}"),
        )
    })?;
    let message = "Test notification from Rayhunter".to_string();

    crate::notifications::send_notification(
        &http_client,
        url,
        message,
        DEFAULT_NOTIFICATION_TIMEOUT,
    )
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
    description = "Stream a ZIP file to the client which contains the QMDL file {name}, its NDJSON analysis report, its GPS NDJSON file (if present), and a PCAP generated from the QMDL."
))]
pub async fn get_zip(
    State(state): State<Arc<ServerState>>,
    Path(entry_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_idx = entry_name.trim_end_matches(".zip").to_owned();
    let entry_index = {
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

        entry_index
    };

    let qmdl_store_lock = state.qmdl_store_lock.clone();
    let gps_records = load_gps_records_for_entry(&state, entry_index).await;

    let (reader, writer) = duplex(8192);

    tokio::spawn(async move {
        let result: Result<(), Error> = async {
            let mut zip = ZipFileWriter::with_tokio(writer);
            write_recording_to_zip(
                &mut zip,
                &qmdl_store_lock,
                entry_index,
                &qmdl_idx,
                gps_records,
                "",
            )
            .await?;
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

/// Writes every file belonging to one recording into `zip`: the stored files
/// (QMDL, analysis report, GPS log) followed by a PCAP generated from the
/// QMDL. Each file name is prefixed with `path_prefix`, which is empty for the
/// single-recording ZIP and `"{name}/"` for the multi-recording ZIP.
async fn write_recording_to_zip(
    zip: &mut ZipFileWriter<DuplexStream>,
    qmdl_store_lock: &RwLock<RecordingStore>,
    entry_index: usize,
    entry_name: &str,
    gps_records: Vec<GpsRecord>,
    path_prefix: &str,
) -> Result<(), Error> {
    // Add stored files
    for &file_kind in FileKind::ALL {
        let file_opt = {
            let qmdl_store = qmdl_store_lock.read().await;
            qmdl_store.open_file(entry_index, file_kind).await?
        };

        let Some(mut file) = file_opt else {
            continue;
        };

        /*
         * `qmdl_compressed` is always false here because even if the
         * QMDL was already compressed, we decompress it before zipping.
         * This is for two reasons
         * 1. If this is the current entry, it's still being written and
         *    lacks a GZIP footer. If we zipped up this partial .gz
         *    file, some software might consider it damaged and refuse to
         *    extract it.
         * 2. Zipping an already-GZIP'd file is redundant and
         *    inconvenient for the user.
         */
        let zip_entry = ZipEntryBuilder::new(
            format!("{path_prefix}{}", file_kind.get_filename(entry_name, false)).into(),
            Compression::Stored,
        );
        // FuturesAsyncWriteCompatExt::compat_write because async-zip's entrystream does
        // not impl tokio's AsyncWrite, but only future's AsyncWrite. This can be removed
        // once https://github.com/Majored/rs-async-zip/pull/160 is released.
        let mut entry_writer = zip.write_entry_stream(zip_entry).await?.compat_write();

        if file_kind == FileKind::Qmdl {
            let reader = QmdlMessageReader::new(&mut file).await?;
            let stream = reader.into_qmdl_stream();
            let mut reader = pin!(stream.into_async_read().compat());
            copy(&mut reader, &mut entry_writer).await?;
        } else {
            copy(&mut file, &mut entry_writer).await?;
        }
        entry_writer.into_inner().close().await?;
    }

    // Add PCAP file
    {
        let entry = ZipEntryBuilder::new(
            format!("{path_prefix}{entry_name}.pcapng").into(),
            Compression::Stored,
        );
        let mut entry_writer = zip.write_entry_stream(entry).await?.compat_write();

        let qmdl_file_for_pcap = {
            let qmdl_store = qmdl_store_lock.read().await;
            qmdl_store
                .open_file(entry_index, FileKind::Qmdl)
                .await?
                .ok_or_else(|| anyhow::anyhow!("QMDL file not found"))?
        };
        let qmdl_reader = QmdlMessageReader::new(qmdl_file_for_pcap).await?;

        if let Err(e) = generate_pcap_data(&mut entry_writer, qmdl_reader, gps_records).await {
            // if we fail to generate the PCAP file, we should still continue and give the
            // user the QMDL.
            error!("Failed to generate PCAP: {e:?}");
        }

        entry_writer.into_inner().close().await?;
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct ZipRecordingsQuery {
    names: Option<String>,
}

/// Splits a comma-separated list of recording names, dropping blanks and
/// duplicates while keeping the order they were requested in.
fn parse_recording_names(names: Option<&str>) -> Vec<String> {
    let mut parsed: Vec<String> = Vec::new();
    for name in names.unwrap_or_default().split(',').map(str::trim) {
        if !name.is_empty() && !parsed.iter().any(|existing| existing == name) {
            parsed.push(name.to_string());
        }
    }
    parsed
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/zip-recordings",
    tag = "Recordings",
    responses(
        (status = StatusCode::OK, description = "ZIP download successful. Recordings whose QMDL file is still empty are left out. It is possible that if a PCAP fails to convert, the same status will be returned, but that recording's PCAP will be empty or incomplete.", content_type = "application/zip"),
        (status = StatusCode::BAD_REQUEST, description = "No recording names were given"),
        (status = StatusCode::NOT_FOUND, description = "Could not find one of the named recordings"),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "Every named recording has an empty QMDL file")
    ),
    params(
        ("names" = String, Query, description = "Comma-separated list of recording names to include. Duplicates are ignored.")
    ),
    summary = "Download several recordings as one ZIP file",
    description = "Stream a single ZIP file to the client with one folder per requested recording, in the order given. Each folder contains the same files as the ZIP download for that recording. Recordings are added one at a time."
))]
pub async fn get_zip_recordings(
    State(state): State<Arc<ServerState>>,
    Query(query): Query<ZipRecordingsQuery>,
) -> Result<Response, (StatusCode, String)> {
    let requested_names = parse_recording_names(query.names.as_deref());
    if requested_names.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "no recording names given".to_string(),
        ));
    }

    let names_to_zip = {
        let qmdl_store = state.qmdl_store_lock.read().await;
        let mut names_to_zip = Vec::with_capacity(requested_names.len());
        for name in requested_names {
            let (_, entry) = qmdl_store.entry_for_name(&name).ok_or((
                StatusCode::NOT_FOUND,
                format!("couldn't find entry with name {name}"),
            ))?;
            if entry.qmdl_size_bytes == 0 {
                warn!("skipping recording {name} in multi-recording ZIP, its QMDL file is empty");
                continue;
            }
            names_to_zip.push(name);
        }
        names_to_zip
    };

    if names_to_zip.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "QMDL files are empty, try again in a bit!".to_string(),
        ));
    }

    let (reader, writer) = duplex(8192);

    tokio::spawn(async move {
        let result: Result<(), Error> = async {
            let mut zip = ZipFileWriter::with_tokio(writer);

            // Recordings are written one after another, never concurrently, so
            // that a large selection doesn't multiply the PCAP conversion load.
            for name in &names_to_zip {
                // Look the entry up again by name rather than reusing an index
                // from validation, since deleting another recording while this
                // ZIP is streaming shifts the indexes of later entries.
                let entry_index = {
                    let qmdl_store = state.qmdl_store_lock.read().await;
                    qmdl_store
                        .entry_for_name(name)
                        .map(|(entry_index, _)| entry_index)
                };
                let Some(entry_index) = entry_index else {
                    warn!("recording {name} was deleted before it could be zipped, skipping");
                    continue;
                };

                let gps_records = load_gps_records_for_entry(&state, entry_index).await;
                write_recording_to_zip(
                    &mut zip,
                    &state.qmdl_store_lock,
                    entry_index,
                    name,
                    gps_records,
                    &format!("{name}/"),
                )
                .await?;
            }

            zip.close().await?;
            Ok(())
        }
        .await;

        if let Err(e) = result {
            error!("Error generating multi-recording ZIP file: {e:?}");
        }
    });

    let headers = [
        (CONTENT_TYPE, "application/zip"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"rayhunter-recordings.zip\"",
        ),
    ];
    let body = Body::from_stream(ReaderStream::new(reader));
    Ok((headers, body).into_response())
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/wifi-status",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Success", body = wifi_station::WifiStatus)
    ),
    summary = "Get wifi status",
    description = "Show the status of the wifi client."
))]
pub async fn get_wifi_status(
    State(state): State<Arc<ServerState>>,
) -> Json<wifi_station::WifiStatus> {
    let status = state.wifi_status.read().await;
    Json(status.clone())
}

#[cfg_attr(feature = "apidocs", utoipa::path(
    post,
    path = "/api/wifi-scan",
    tag = "Configuration",
    responses(
        (status = StatusCode::OK, description = "Scan success", body = inline(Vec<wifi_station::WifiNetwork>), content_type = "application/json"),
        (status = StatusCode::TOO_MANY_REQUESTS, description = "Scan already in progress"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Scan failed"),
    ),
    summary = "Wifi SSID scan",
    description = "Poll for a list of available wifi networks. Returns an array of WifiNetwork objects."
))]
pub async fn scan_wifi(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<Vec<wifi_station::WifiNetwork>>, (StatusCode, String)> {
    let _guard = state.wifi_scan_lock.try_lock().map_err(|_| {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "WiFi scan already in progress".to_string(),
        )
    })?;
    let networks = wifi_station::scan_wifi_networks(wifi_station::STA_IFACE)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("WiFi scan failed: {e}"),
            )
        })?;
    Ok(Json(networks))
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
    use std::io::Cursor;

    use super::*;
    use crate::config::GpsMode;
    use async_zip::base::read::mem::ZipFileReader;
    use axum::extract::{Path, State};
    use futures::AsyncReadExt;
    use rayhunter::{
        diag::{DataType, HdlcEncapsulatedMessage, Message, MessagesContainer},
        qmdl::{QmdlMessageReader, QmdlWriter},
    };
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
        test_data: &MessagesContainer,
    ) -> String {
        let entry_name = {
            let mut store = store_lock.write().await;
            let (mut qmdl_gz_file, _analysis_file) =
                store.new_entry(GpsMode::Disabled).await.unwrap();

            let mut writer = QmdlWriter::new(&mut qmdl_gz_file);
            writer.write_container(test_data).await.unwrap();
            writer.close().await.unwrap();

            let qmdl_file_size = qmdl_gz_file.metadata().await.unwrap().len() as usize;

            let current_entry = store.current_entry.unwrap();
            let entry = &store.manifest.entries[current_entry];
            let entry_name = entry.name.clone();

            store
                .update_current_entry_qmdl_size(qmdl_file_size)
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
            wifi_status: Arc::new(RwLock::new(wifi_station::WifiStatus::default())),
            wifi_scan_lock: tokio::sync::Mutex::new(()),
            gps_state: Arc::new(RwLock::new(None)),
            update_status_lock: Arc::new(RwLock::new(UpdateStatus::default())),
        })
    }

    // valid HDLC encapsulated diag message generated from
    // rayhunter::diag::test::get_test_message
    fn create_test_container() -> MessagesContainer {
        MessagesContainer {
            data_type: DataType::UserSpace,
            num_messages: 1,
            messages: vec![HdlcEncapsulatedMessage {
                len: 39,
                data: vec![
                    16, 0, 32, 0, 32, 0, 192, 176, 26, 165, 245, 135, 118, 35, 2, 1, 20, 14, 48, 0,
                    160, 0, 2, 8, 0, 0, 217, 15, 5, 0, 0, 0, 0, 1, 0, 10, 13, 196, 126,
                ],
            }],
        }
    }

    #[tokio::test]
    async fn test_get_zip_success() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        let test_qmdl_data = create_test_container();
        let entry_name = create_test_entry_with_data(&store_lock, &test_qmdl_data).await;
        let state = create_test_server_state(store_lock);

        let response = get_zip(State(state), Path(entry_name.clone()))
            .await
            .unwrap();

        let headers = response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "application/zip");

        let body = response.into_body();
        let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

        let zip_reader = ZipFileReader::new(body_bytes.to_vec()).await.unwrap();
        let zip_reader_file = zip_reader.file();
        let filenames: Vec<String> = zip_reader_file
            .entries()
            .iter()
            .map(|entry| entry.filename().as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            filenames,
            vec![
                format!("{entry_name}.qmdl"),
                format!("{entry_name}.ndjson"),
                format!("{entry_name}-gps.ndjson"),
                format!("{entry_name}.pcapng"),
            ]
        );

        let mut qmdl_body = Vec::with_capacity(128);
        zip_reader
            .reader_without_entry(0)
            .await
            .unwrap()
            .read_to_end(&mut qmdl_body)
            .await
            .unwrap();
        let mut qmdl_reader = QmdlMessageReader::new(Cursor::new(qmdl_body))
            .await
            .unwrap();
        let expected_message = Message::from_hdlc(&test_qmdl_data.messages[0].data).unwrap();
        assert_eq!(
            qmdl_reader.get_next_message().await.unwrap(),
            Some(Ok(expected_message)),
        );
    }

    // Entry names come from the creation timestamp in seconds, so entries made
    // back to back in a test would collide. This creates an entry and then
    // renames it (files and manifest) to a fixed name. Passing `None` for the
    // data leaves the QMDL file empty.
    async fn create_named_test_entry(
        store_lock: &Arc<RwLock<crate::qmdl_store::RecordingStore>>,
        name: &str,
        test_data: Option<&MessagesContainer>,
    ) {
        let mut store = store_lock.write().await;
        let (mut qmdl_file, analysis_file) = store.new_entry(GpsMode::Disabled).await.unwrap();
        if let Some(test_data) = test_data {
            let mut writer = QmdlWriter::new(&mut qmdl_file);
            writer.write_container(test_data).await.unwrap();
            writer.close().await.unwrap();
            let qmdl_file_size = qmdl_file.metadata().await.unwrap().len() as usize;
            store
                .update_current_entry_qmdl_size(qmdl_file_size)
                .await
                .unwrap();
        }
        drop(qmdl_file);
        drop(analysis_file);

        let entry_index = store.current_entry.unwrap();
        store.close_current_entry().await.unwrap();

        let store_path = store.path.clone();
        let entry = &mut store.manifest.entries[entry_index];
        for &file_kind in FileKind::ALL {
            tokio::fs::rename(
                file_kind.get_filepath(&entry.name, &store_path, entry.compressed),
                file_kind.get_filepath(name, &store_path, entry.compressed),
            )
            .await
            .unwrap();
        }
        entry.name = name.to_string();
    }

    fn zip_recordings_query(names: Option<&str>) -> Query<ZipRecordingsQuery> {
        Query(ZipRecordingsQuery {
            names: names.map(str::to_string),
        })
    }

    async fn zip_entry_filenames(response: Response) -> (ZipFileReader, Vec<String>) {
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let zip_reader = ZipFileReader::new(body_bytes.to_vec()).await.unwrap();
        let filenames = zip_reader
            .file()
            .entries()
            .iter()
            .map(|entry| entry.filename().as_str().unwrap().to_string())
            .collect();
        (zip_reader, filenames)
    }

    #[test]
    fn test_parse_recording_names() {
        assert!(parse_recording_names(None).is_empty());
        assert!(parse_recording_names(Some("")).is_empty());
        assert!(parse_recording_names(Some(" , ,")).is_empty());
        assert_eq!(
            parse_recording_names(Some("2, 1,2,,3,1")),
            vec!["2", "1", "3"]
        );
    }

    #[tokio::test]
    async fn test_get_zip_recordings_requires_names() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        create_named_test_entry(&store_lock, "100", Some(&create_test_container())).await;
        let state = create_test_server_state(store_lock);

        for names in [None, Some(""), Some(" , ")] {
            let Err((status, _)) =
                get_zip_recordings(State(state.clone()), zip_recordings_query(names)).await
            else {
                panic!("expected an error for names {names:?}");
            };
            assert_eq!(status, StatusCode::BAD_REQUEST);
        }
    }

    #[tokio::test]
    async fn test_get_zip_recordings_unknown_name() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        create_named_test_entry(&store_lock, "100", Some(&create_test_container())).await;
        let state = create_test_server_state(store_lock);

        let Err((status, message)) =
            get_zip_recordings(State(state), zip_recordings_query(Some("100,999"))).await
        else {
            panic!("expected an error for an unknown recording");
        };
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(message.contains("999"), "message was {message:?}");
    }

    #[tokio::test]
    async fn test_get_zip_recordings_success() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        let test_qmdl_data = create_test_container();
        create_named_test_entry(&store_lock, "100", Some(&test_qmdl_data)).await;
        create_named_test_entry(&store_lock, "200", Some(&test_qmdl_data)).await;
        let state = create_test_server_state(store_lock);

        // Requested order is kept, and the repeated name is only zipped once.
        let response = get_zip_recordings(State(state), zip_recordings_query(Some("200,100,200")))
            .await
            .unwrap();

        let headers = response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "application/zip");
        assert_eq!(
            headers.get("content-disposition").unwrap(),
            "attachment; filename=\"rayhunter-recordings.zip\""
        );

        let (zip_reader, filenames) = zip_entry_filenames(response).await;
        assert_eq!(
            filenames,
            vec![
                "200/200.qmdl",
                "200/200.ndjson",
                "200/200-gps.ndjson",
                "200/200.pcapng",
                "100/100.qmdl",
                "100/100.ndjson",
                "100/100-gps.ndjson",
                "100/100.pcapng",
            ]
        );

        // The QMDL inside each folder is decompressed, same as the single
        // recording ZIP.
        for qmdl_index in [0, 4] {
            let mut qmdl_body = Vec::with_capacity(128);
            zip_reader
                .reader_without_entry(qmdl_index)
                .await
                .unwrap()
                .read_to_end(&mut qmdl_body)
                .await
                .unwrap();
            let mut qmdl_reader = QmdlMessageReader::new(Cursor::new(qmdl_body))
                .await
                .unwrap();
            let expected_message = Message::from_hdlc(&test_qmdl_data.messages[0].data).unwrap();
            assert_eq!(
                qmdl_reader.get_next_message().await.unwrap(),
                Some(Ok(expected_message)),
            );
        }
    }

    #[tokio::test]
    async fn test_get_zip_recordings_skips_empty_recordings() {
        let (_temp_dir, store_lock) = create_test_qmdl_store().await;
        create_named_test_entry(&store_lock, "100", Some(&create_test_container())).await;
        create_named_test_entry(&store_lock, "200", None).await;
        let state = create_test_server_state(store_lock);

        let response =
            get_zip_recordings(State(state.clone()), zip_recordings_query(Some("100,200")))
                .await
                .unwrap();
        let (_zip_reader, filenames) = zip_entry_filenames(response).await;
        assert_eq!(
            filenames,
            vec![
                "100/100.qmdl",
                "100/100.ndjson",
                "100/100-gps.ndjson",
                "100/100.pcapng",
            ]
        );

        let Err((status, _)) =
            get_zip_recordings(State(state), zip_recordings_query(Some("200"))).await
        else {
            panic!("expected an error when every recording is empty");
        };
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }
}
