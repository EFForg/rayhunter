use std::{
    cmp::min,
    collections::HashMap,
    time::{Duration, Instant},
};

use log::error;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, error::TryRecvError};
use tokio_util::task::TaskTracker;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
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
    tx: mpsc::Sender<Notification>,
    rx: mpsc::Receiver<Notification>,
}

impl NotificationService {
    pub fn new(url: Option<String>) -> Self {
        let (tx, rx) = mpsc::channel(10);
        Self { url, tx, rx }
    }

    pub fn new_handler(&self) -> mpsc::Sender<Notification> {
        self.tx.clone()
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

                    match http_client
                        .post(&url)
                        .body(notification.message.clone())
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                notification.last_sent = Some(Instant::now());
                                notification.failed_since_last_success = 0;
                                notification.needs_sending = false;
                            } else {
                                notification.failed_since_last_success += 1;
                                notification.last_attempt = Some(Instant::now());
                            }
                        }
                        Err(e) => {
                            error!("Failed to send notification to ntfy: {e}");
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
