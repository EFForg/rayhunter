use std::{
    cmp::min,
    collections::HashMap,
    time::{Duration, Instant},
};

use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc::{self, error::TryRecvError};
use tokio_util::task::TaskTracker;

pub const DEFAULT_NOTIFICATION_TIMEOUT: u64 = 10; //seconds

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Server returned error status: {0}")]
    HttpError(reqwest::StatusCode),
}

/// Enum of valid notification types
#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub enum NotificationType {
    Warning,
    LowBattery,
}

pub struct Notification {
    notification_type: NotificationType,
    message: String,
    debounce: Option<Duration>,
}

impl Notification {
    pub fn new(
        notification_type: NotificationType,
        message: String,
        debounce: Option<Duration>,
    ) -> Self {
        Notification {
            notification_type,
            message,
            debounce,
        }
    }
}

struct NotificationStatus {
    message: String,
    needs_sending: bool,
    last_sent: Option<Instant>,
    last_attempt: Option<Instant>,
    failed_since_last_success: u32,
}

pub struct NotificationService {
    url: Option<String>,
    timeout: u64,
    tx: mpsc::Sender<Notification>,
    rx: mpsc::Receiver<Notification>,
}

impl NotificationService {
    pub fn new(url: Option<String>) -> Self {
        let (tx, rx) = mpsc::channel(10);
        Self {
            url,
            timeout: DEFAULT_NOTIFICATION_TIMEOUT,
            tx,
            rx,
        }
    }

    pub fn new_handler(&self) -> mpsc::Sender<Notification> {
        self.tx.clone()
    }
}

/// Sends a notification message to the specified URL.
pub async fn send_notification(
    http_client: &reqwest::Client,
    url: &str,
    message: String,
    timeout: u64,
) -> Result<(), NotificationError> {
    let response = http_client
        .post(url)
        .body(message)
        .timeout(Duration::from_secs(timeout))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(NotificationError::HttpError(response.status()))
    }
}

pub fn run_notification_worker(
    task_tracker: &TaskTracker,
    mut notification_service: NotificationService,
    enabled_notifications: Vec<NotificationType>,
) {
    task_tracker.spawn(async move {
        if let Some(url) = notification_service.url
            && !url.is_empty()
        {
            let mut notification_statuses = HashMap::new();
            let http_client = reqwest::Client::new();

            loop {
                // Get any notifications since the last time we checked
                loop {
                    match notification_service.rx.try_recv() {
                        Ok(notification) => {
                            if !enabled_notifications.contains(&notification.notification_type) {
                                continue;
                            }

                            let status = notification_statuses
                                .entry(notification.notification_type)
                                .or_insert_with(|| NotificationStatus {
                                    message: "".to_string(),
                                    needs_sending: true,
                                    last_sent: None,
                                    last_attempt: None,
                                    failed_since_last_success: 0,
                                });
                            // Ignore if we're in the debounce period
                            if let Some(debounce) = notification.debounce
                                && let Some(last_sent) = status.last_sent
                                && last_sent.elapsed() < debounce
                            {
                                continue;
                            }
                            status.message = notification.message;
                            status.needs_sending = true;
                        }
                        Err(TryRecvError::Empty) => {
                            break;
                        }
                        Err(TryRecvError::Disconnected) => {
                            return;
                        }
                    }
                }

                // Attempt to send pending notifications
                for notification in notification_statuses.values_mut() {
                    if !notification.needs_sending {
                        continue;
                    }

                    // Backoff retries, up to a maximum of 256 seconds.
                    if let Some(last_attempt) = notification.last_attempt {
                        let min_wait_time = Duration::from_secs(
                            2u64.pow(min(notification.failed_since_last_success, 8)),
                        );
                        if last_attempt.elapsed() < min_wait_time {
                            continue;
                        }
                    }

                    match send_notification(
                        &http_client,
                        &url,
                        notification.message.clone(),
                        notification_service.timeout,
                    )
                    .await
                    {
                        Ok(()) => {
                            notification.last_sent = Some(Instant::now());
                            notification.failed_since_last_success = 0;
                            notification.needs_sending = false;
                        }
                        Err(e) => {
                            error!("Failed to send notification: {e}");
                            notification.failed_since_last_success += 1;
                            notification.last_attempt = Some(Instant::now());
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
        // If there's no url to send to we'll just discard the notifications
        else {
            loop {
                if notification_service.rx.recv().await.is_none() {
                    break;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Bytes, extract::State, routing::post};
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;

    #[derive(Clone)]
    struct TestServerState {
        received_messages: Arc<Mutex<Vec<String>>>,
    }

    async fn capture_notification(
        State(state): State<TestServerState>,
        body: Bytes,
    ) -> &'static str {
        let message = String::from_utf8_lossy(&body).to_string();
        state.received_messages.lock().await.push(message);
        "OK"
    }

    async fn setup_test_server() -> (Arc<Mutex<Vec<String>>>, String) {
        crate::crypto_provider::install_default();

        let received_messages = Arc::new(Mutex::new(Vec::new()));
        let test_state = TestServerState {
            received_messages: received_messages.clone(),
        };

        let app = Router::new()
            .route("/", post(capture_notification))
            .with_state(test_state);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        (received_messages, url)
    }

    async fn setup_timeout_server(timeout: u64) -> String {
        #[cfg(feature = "rustcrypto-tls")]
        {
            let _ = rustls_rustcrypto::provider().install_default();
        }

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        tokio::spawn(async move {
            // Accept the connection but don't respond in the timeout
            let (_socket, _addr) = listener.accept().await.unwrap();
            tokio::time::sleep(Duration::from_secs(timeout * 2)).await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        url
    }

    async fn cleanup_worker(sender: mpsc::Sender<Notification>, tracker: TaskTracker) {
        drop(sender);
        tracker.close();
        tracker.wait().await;
    }

    #[tokio::test]
    async fn test_send_notification_times_out() {
        let timeout: u64 = 2;
        let url = setup_timeout_server(timeout).await;

        let http_client = reqwest::Client::new();
        let result = send_notification(
            &http_client,
            &url,
            "test warning message".to_string(),
            timeout,
        )
        .await;

        match result {
            Err(NotificationError::RequestFailed(reqwest_error)) => {
                println!("error = {:?}", reqwest_error);
                assert!(reqwest_error.is_timeout());
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_notification_worker_sends_message() {
        let (received_messages, url) = setup_test_server().await;

        let task_tracker = TaskTracker::new();
        let notification_service = NotificationService::new(Some(url));
        let notification_sender = notification_service.new_handler();

        run_notification_worker(
            &task_tracker,
            notification_service,
            vec![NotificationType::Warning],
        );

        notification_sender
            .send(Notification::new(
                NotificationType::Warning,
                "test warning message".to_string(),
                None,
            ))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(3)).await;

        let messages = received_messages.lock().await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test warning message");
        drop(messages);

        cleanup_worker(notification_sender, task_tracker).await;
    }

    #[tokio::test]
    async fn test_notification_worker_filters_disabled_types() {
        let (received_messages, url) = setup_test_server().await;

        let task_tracker = TaskTracker::new();
        let notification_service = NotificationService::new(Some(url));
        let notification_sender = notification_service.new_handler();

        run_notification_worker(
            &task_tracker,
            notification_service,
            vec![NotificationType::Warning],
        );

        notification_sender
            .send(Notification::new(
                NotificationType::Warning,
                "test warning".to_string(),
                None,
            ))
            .await
            .unwrap();

        notification_sender
            .send(Notification::new(
                NotificationType::LowBattery,
                "test low battery".to_string(),
                None,
            ))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(3)).await;

        let messages = received_messages.lock().await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test warning");
        drop(messages);

        cleanup_worker(notification_sender, task_tracker).await;
    }

    #[tokio::test]
    async fn test_notification_worker_sends_enabled_types() {
        let (received_messages, url) = setup_test_server().await;

        let task_tracker = TaskTracker::new();
        let notification_service = NotificationService::new(Some(url));
        let notification_sender = notification_service.new_handler();

        run_notification_worker(
            &task_tracker,
            notification_service,
            vec![NotificationType::Warning, NotificationType::LowBattery],
        );

        notification_sender
            .send(Notification::new(
                NotificationType::Warning,
                "test warning".to_string(),
                None,
            ))
            .await
            .unwrap();

        notification_sender
            .send(Notification::new(
                NotificationType::LowBattery,
                "test low battery".to_string(),
                None,
            ))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(3)).await;

        let messages = received_messages.lock().await;
        assert_eq!(messages.len(), 2);
        // these are interchangeable, ordering not guaranteed
        assert!(messages.contains(&"test warning".to_string()));
        assert!(messages.contains(&"test low battery".to_string()));
        drop(messages);

        cleanup_worker(notification_sender, task_tracker).await;
    }

    #[tokio::test]
    async fn test_notification_worker_with_no_url() {
        let task_tracker = TaskTracker::new();
        let notification_service = NotificationService::new(None);
        let notification_sender = notification_service.new_handler();

        run_notification_worker(
            &task_tracker,
            notification_service,
            vec![NotificationType::Warning],
        );

        notification_sender
            .send(Notification::new(
                NotificationType::Warning,
                "test warning".to_string(),
                None,
            ))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(500)).await;

        cleanup_worker(notification_sender, task_tracker).await;
    }
}
