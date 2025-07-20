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
use include_dir::{Dir, include_dir};
use log::error;
use std::sync::Arc;
use tokio::fs::write;
use tokio::io::{AsyncReadExt, copy, duplex};
use tokio::sync::mpsc::Sender;
use tokio::sync::{RwLock, oneshot};
use tokio_util::compat::FuturesAsyncWriteCompatExt;
use tokio_util::io::ReaderStream;

use crate::analysis::{AnalysisCtrlMessage, AnalysisStatus};
use crate::config::Config;
use crate::gps::GpsCoordinate;
use crate::pcap::generate_pcap_data;
use crate::qmdl_store::RecordingStore;
use crate::{DiagDeviceCtrlMessage, display};

pub struct ServerState {
    pub config_path: String,
    pub config: Config,
    pub qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    pub diag_device_ctrl_sender: Sender<DiagDeviceCtrlMessage>,
    pub ui_update_sender: Sender<display::DisplayState>,
    pub analysis_status_lock: Arc<RwLock<AnalysisStatus>>,
    pub analysis_sender: Sender<AnalysisCtrlMessage>,
    pub gps_sender: Sender<GpsCoordinate>,
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

pub async fn get_config(
    State(state): State<Arc<ServerState>>,
) -> Result<Json<Config>, (StatusCode, String)> {
    Ok(Json(state.config.clone()))
}

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
    let mut restart_tx = state.daemon_restart_tx.write().await;
    if let Some(sender) = restart_tx.take() {
        sender.send(()).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "couldn't send restart signal".to_string(),
            )
        })?;
        Ok((
            StatusCode::ACCEPTED,
            "wrote config and triggered restart".to_string(),
        ))
    } else {
        Ok((
            StatusCode::ACCEPTED,
            "wrote config but restart already triggered".to_string(),
        ))
    }
}

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

            // Add GPS file if it exists
            {
                let gps_file_path = {
                    let qmdl_store = qmdl_store_lock.read().await;
                    let entry = &qmdl_store.manifest.entries[entry_index];
                    entry.get_gps_filepath(&qmdl_store.path)
                };

                if tokio::fs::metadata(&gps_file_path).await.is_ok() {
                    let entry =
                        ZipEntryBuilder::new(format!("{qmdl_idx}.gps").into(), Compression::Stored);
                    let mut entry_writer = zip.write_entry_stream(entry).await?.compat_write();

                    let mut gps_file = tokio::fs::File::open(&gps_file_path).await?;
                    copy(&mut gps_file, &mut entry_writer).await?;
                    entry_writer.into_inner().close().await?;
                }
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
        let (ui_tx, _ui_rx) = tokio::sync::mpsc::channel(1);
        let (analysis_tx, _analysis_rx) = tokio::sync::mpsc::channel(1);
        let (gps_tx, _gps_rx) = tokio::sync::mpsc::channel(1);

        let analysis_status = {
            let store = store_lock.try_read().unwrap();
            crate::analysis::AnalysisStatus::new(&store)
        };

        Arc::new(ServerState {
            config_path: "/tmp/test_config.toml".to_string(),
            config: Config::default(),
            qmdl_store_lock: store_lock,
            diag_device_ctrl_sender: tx,
            ui_update_sender: ui_tx,
            analysis_status_lock: Arc::new(RwLock::new(analysis_status)),
            analysis_sender: analysis_tx,
            gps_sender: gps_tx,
            daemon_restart_tx: Arc::new(RwLock::new(None)),
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

        // GPS file may or may not exist depending on test conditions
        assert!(filenames.contains(&format!("{entry_name}.qmdl")));
        assert!(filenames.contains(&format!("{entry_name}.pcapng")));
        // GPS file is optional - it may or may not be present
        assert!(filenames.len() >= 2 && filenames.len() <= 3);
    }
}
