use std::fmt::Display;
use std::{sync::Arc, time::Duration};

use chrono::TimeDelta;
use log::{info, warn};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::{Body, Client, Response};
use tokio::fs::File;
use tokio::join;
use tokio::{select, sync::RwLock, time};
use tokio_util::io::ReaderStream;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::config::WebdavConfig;
use crate::qmdl_store::RecordingStore;

pub struct WebdavUploadWorkerConfig {
    poll_interval: Duration,
    min_age: TimeDelta,
    remote_path: String,
    host: String,
    username: Option<String>,
    password: Option<String>,
    timeout: Duration,
    delete_on_upload: bool,
}

impl From<WebdavConfig> for WebdavUploadWorkerConfig {
    fn from(value: WebdavConfig) -> Self {
        WebdavUploadWorkerConfig {
            poll_interval: Duration::from_secs(value.poll_interval_secs),
            min_age: TimeDelta::seconds(value.min_age_secs),
            remote_path: value.remote_path,
            host: value.host,
            username: value.username,
            password: value.password,
            timeout: Duration::from_secs(value.upload_timeout_secs),
            delete_on_upload: value.delete_on_upload,
        }
    }
}

enum FileKind {
    Analysis,
    Qmdl,
}

impl FileKind {
    fn as_extension(&self) -> &'static str {
        match self {
            FileKind::Analysis => ".ndjson",
            FileKind::Qmdl => ".qmdl",
        }
    }
}

impl Display for FileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileKind::Analysis => write!(f, "analysis"),
            FileKind::Qmdl => write!(f, "QMDL"),
        }
    }
}

#[derive(Debug, Clone)]
struct WebDavClient {
    client: Client,
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl WebDavClient {
    fn new(
        host: String,
        username: Option<String>,
        password: Option<String>,
        timeout: Duration,
        root_dir: String,
    ) -> Result<Self, reqwest::Error> {
        let host = host.trim_end_matches('/');
        let root = root_dir.trim_matches('/');
        let base_url = if root.is_empty() {
            format!("{}/", host)
        } else {
            format!("{}/{}/", host, root)
        };

        Ok(Self {
            client: reqwest::Client::builder().timeout(timeout).build()?,
            base_url,
            username,
            password,
        })
    }

    async fn try_upload_file(&self, file: File, name: &str) -> anyhow::Result<Response> {
        let file_size = file.metadata().await?.len();

        let stream = ReaderStream::new(file);
        let body = Body::wrap_stream(stream);

        let target = format!("{}{}", self.base_url, name);

        let client = self
            .client
            .put(&target)
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, file_size);

        let client = match (&self.username, &self.password) {
            (Some(username), Some(password)) => client.basic_auth(username, Some(password)),
            (Some(username), None) => client.basic_auth(username, None::<&str>),
            (None, None) => client,
            (None, Some(_)) => {
                warn!(
                    "Got WebDAV auth setting with no username but with a password, skipping authentication"
                );
                client
            }
        };

        let resp = client.body(body).send().await?.error_for_status();
        Ok(resp?)
    }
}

async fn try_upload_entry(
    client: WebDavClient,
    store: Arc<RwLock<RecordingStore>>,
    entry_name: String,
    file_kind: FileKind,
    shutdown_token: CancellationToken,
) -> Option<()> {
    let read_lock = store.read().await;
    let entry_idx = read_lock.entry_for_name(&entry_name)?.0;
    let file = match file_kind {
        FileKind::Analysis => read_lock.open_entry_analysis(entry_idx).await,
        FileKind::Qmdl => read_lock.open_entry_qmdl(entry_idx).await,
    };
    drop(read_lock);

    let Ok(file) = file.map_err(|err| {
        warn!(
            "Unable to open entry: {} {} file: {:?}",
            entry_name, file_kind, err
        )
    }) else {
        return None;
    };

    let file_name = format!("{}{}", entry_name, file_kind.as_extension());

    let res = select! {
        _ = shutdown_token.cancelled() => {
            warn!(
                "Cancelling upload for entry {} {} file: received shutdown signal",
                entry_name, file_kind
            );
            return None;
        },
        res = client.try_upload_file(file, &file_name) => res,
    };

    match res {
        Ok(_) => {
            info!("Uploaded {} file for entry {}", file_kind, entry_name);
            Some(())
        }
        Err(err) => {
            warn!(
                "Failed to upload {} file for entry {}: {:?}",
                file_kind, entry_name, err
            );
            None
        }
    }
}

pub fn run_webdav_upload_worker(
    task_tracker: &TaskTracker,
    shutdown_token: CancellationToken,
    qmdl_store_lock: Arc<RwLock<RecordingStore>>,
    config: WebdavUploadWorkerConfig,
) {
    task_tracker.spawn(async move {
        let mut interval = time::interval(config.poll_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        let webdav_client = match WebDavClient::new(
            config.host,
            config.username,
            config.password,
            config.timeout,
            config.remote_path,
        ) {
            Ok(client) => client,
            Err(err) => {
                warn!("Unable to create WebDAV client: {:?}", err);
                return;
            }
        };

        loop {
            select! {
                _ = shutdown_token.cancelled() => break,
                _ = interval.tick() => {
                    let entries = qmdl_store_lock
                        .read()
                        .await
                        .get_unuploaded_entries_with_age(config.min_age);

                    for entry_name in entries {
                        if shutdown_token.is_cancelled() {
                            break;
                        }

                        let (Some(()), Some(())) = join!(
                            try_upload_entry(
                                webdav_client.clone(),
                                qmdl_store_lock.clone(),
                                entry_name.clone(),
                                FileKind::Qmdl,
                                shutdown_token.clone(),
                            ),
                            try_upload_entry(
                                webdav_client.clone(),
                                qmdl_store_lock.clone(),
                                entry_name.clone(),
                                FileKind::Analysis,
                                shutdown_token.clone()
                            ),
                        ) else {
                            continue;
                        };

                        if config.delete_on_upload {
                            match qmdl_store_lock.write().await.delete_entry(&entry_name).await {
                                Ok(_) => info!("Successfully deleted entry: {} after upload to WebDAV", entry_name),
                                Err(err) => warn!("Unable to delete entry: {} after upload to WebDAV: {}", entry_name, err),
                            }
                        } else {
                            match qmdl_store_lock.write().await.mark_entry_as_uploaded(&entry_name, rayhunter::clock::get_adjusted_now()).await {
                                Ok(_) => info!("Successfully marked entry: {} as uploaded", entry_name),
                                Err(err) => warn!("Unable to mark entry: {} as uploaded: {}", entry_name, err),
                            }
                        }

                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Bytes,
        extract::{Path as AxumPath, State},
        http::{HeaderMap, StatusCode},
        routing::put,
    };
    use tempfile::Builder;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;

    #[derive(Clone, Debug)]
    struct RecordedPut {
        path: String,
        auth: Option<String>,
        body: Vec<u8>,
    }

    async fn capture_put(
        State(state): State<Arc<Mutex<Vec<RecordedPut>>>>,
        AxumPath(path): AxumPath<String>,
        headers: HeaderMap,
        body: Bytes,
    ) -> StatusCode {
        let auth = headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        state.lock().await.push(RecordedPut {
            path,
            auth,
            body: body.to_vec(),
        });
        StatusCode::CREATED
    }

    async fn setup_webdav_server() -> (Arc<Mutex<Vec<RecordedPut>>>, String) {
        crate::crypto_provider::install_default();

        let state = Arc::new(Mutex::new(Vec::new()));
        let app = Router::new()
            .route("/{*path}", put(capture_put))
            .with_state(state.clone());

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        (state, url)
    }

    async fn cleanup_worker(shutdown: CancellationToken, tracker: TaskTracker) {
        shutdown.cancel();
        tracker.close();
        tracker.wait().await;
    }

    async fn make_store_with_closed_entry(
        dir: &std::path::Path,
    ) -> (Arc<RwLock<RecordingStore>>, String) {
        let mut store = RecordingStore::create(dir).await.unwrap();
        let (mut qmdl_file, mut analysis_file) = store.new_entry().await.unwrap();
        qmdl_file.write_all(b"fake qmdl payload").await.unwrap();
        qmdl_file.flush().await.unwrap();
        analysis_file
            .write_all(b"fake ndjson payload")
            .await
            .unwrap();
        analysis_file.flush().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        let name = store.manifest.entries[entry_index].name.clone();
        store.update_entry_qmdl_size(entry_index, 17).await.unwrap();
        store.close_current_entry().await.unwrap();
        (Arc::new(RwLock::new(store)), name)
    }

    #[tokio::test]
    async fn test_webdav_upload_worker_uploads_entry() {
        let (captured, url) = setup_webdav_server().await;

        let dir = Builder::new().prefix("webdav_test").tempdir().unwrap();
        let (store, entry_name) = make_store_with_closed_entry(dir.path()).await;

        let shutdown = CancellationToken::new();
        let tracker = TaskTracker::new();
        let config = WebdavUploadWorkerConfig {
            poll_interval: Duration::from_millis(50),
            min_age: TimeDelta::seconds(-1),
            remote_path: "dav".to_string(),
            host: url,
            username: Some("user".to_string()),
            password: Some("password".to_string()),
            timeout: Duration::from_secs(1),
            delete_on_upload: false,
        };

        run_webdav_upload_worker(&tracker, shutdown.clone(), store.clone(), config);

        tokio::time::sleep(Duration::from_millis(500)).await;
        cleanup_worker(shutdown, tracker).await;

        let recorded = captured.lock().await;
        assert_eq!(recorded.len(), 2);
        let paths: Vec<&str> = recorded.iter().map(|r| r.path.as_str()).collect();
        let qmdl_path = format!("dav/{}.qmdl", entry_name);
        let ndjson_path = format!("dav/{}.ndjson", entry_name);
        assert!(paths.contains(&qmdl_path.as_str()));
        assert!(paths.contains(&ndjson_path.as_str()));
        for put in recorded.iter() {
            assert_eq!(put.auth.as_deref(), Some("Basic dXNlcjpwYXNzd29yZA=="));
        }
        let qmdl_body = recorded
            .iter()
            .find(|r| r.path == qmdl_path)
            .unwrap()
            .body
            .clone();
        let ndjson_body = recorded
            .iter()
            .find(|r| r.path == ndjson_path)
            .unwrap()
            .body
            .clone();
        drop(recorded);
        assert_eq!(qmdl_body, b"fake qmdl payload");
        assert_eq!(ndjson_body, b"fake ndjson payload");

        let store_read = store.read().await;
        let (_, entry) = store_read.entry_for_name(&entry_name).unwrap();
        assert!(entry.upload_time.is_some());
    }

    #[tokio::test]
    async fn test_webdav_upload_worker_deletes_when_configured() {
        let (captured, url) = setup_webdav_server().await;

        let dir = Builder::new().prefix("webdav_test").tempdir().unwrap();
        let (store, entry_name) = make_store_with_closed_entry(dir.path()).await;

        let shutdown = CancellationToken::new();
        let tracker = TaskTracker::new();
        let config = WebdavUploadWorkerConfig {
            poll_interval: Duration::from_millis(50),
            min_age: TimeDelta::seconds(-1),
            remote_path: "dav".to_string(),
            host: url,
            username: None,
            password: None,
            timeout: Duration::from_secs(1),
            delete_on_upload: true,
        };

        run_webdav_upload_worker(&tracker, shutdown.clone(), store.clone(), config);

        tokio::time::sleep(Duration::from_millis(500)).await;
        cleanup_worker(shutdown, tracker).await;

        assert_eq!(captured.lock().await.len(), 2);

        let store_read = store.read().await;
        assert!(store_read.entry_for_name(&entry_name).is_none());
    }

    #[tokio::test]
    async fn test_webdav_upload_worker_respects_min_age() {
        let (captured, url) = setup_webdav_server().await;

        let dir = Builder::new().prefix("webdav_test").tempdir().unwrap();
        let (store, entry_name) = make_store_with_closed_entry(dir.path()).await;

        let shutdown = CancellationToken::new();
        let tracker = TaskTracker::new();
        let config = WebdavUploadWorkerConfig {
            poll_interval: Duration::from_millis(50),
            min_age: TimeDelta::seconds(3600),
            remote_path: "dav".to_string(),
            host: url,
            username: None,
            password: None,
            timeout: Duration::from_secs(1),
            delete_on_upload: false,
        };

        run_webdav_upload_worker(&tracker, shutdown.clone(), store.clone(), config);

        tokio::time::sleep(Duration::from_millis(500)).await;
        cleanup_worker(shutdown, tracker).await;

        assert!(captured.lock().await.is_empty());

        let store_read = store.read().await;
        let (_, entry) = store_read.entry_for_name(&entry_name).unwrap();
        assert!(entry.upload_time.is_none());
    }
}
