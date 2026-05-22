use chrono::{DateTime, Local};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::select;
use tokio::sync::{RwLock, mpsc::Sender};
use tokio::time;
use tokio::time::{Duration, MissedTickBehavior};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::notifications::{Notification, NotificationType};

const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);
const GITHUB_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/EFForg/rayhunter/releases/latest";

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct UpdateStatus {
    pub current_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release_url: Option<String>,
    pub update_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "apidocs", schema(value_type = Option<String>, format = "date-time"))]
    pub last_checked: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self {
            current_version: get_current_version(),
            // To-be-populated by update check worker
            latest_version: None,
            latest_release_url: None,
            update_available: false,
            last_checked: None,
            last_error: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseResponse {
    tag_name: String,
    html_url: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct VersionParts {
    major: u64,
    minor: u64,
    patch: u64,
}

fn get_current_version() -> String {
    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    env!("CARGO_PKG_VERSION").to_owned()
}

fn parse_release_tagname(version: &str) -> Option<(VersionParts, String)> {
    // Trim whitespace and leading `v`, if any
    let trimmed_version = version.trim().trim_start_matches('v');
    let mut parts = trimmed_version.split('.');

    // Ignore any pre-release/build metadata by splitting on '-'
    // TODO: is this okay?
    let major = parts.next()?.split('-').next()?.parse().ok()?;
    let minor = parts.next()?.split('-').next()?.parse().ok()?;
    let patch = parts.next()?.split('-').next()?.parse().ok()?;
    let version = format!("{}.{}.{}", major, minor, patch);
    Some((
        VersionParts {
            major,
            minor,
            patch,
        },
        version.to_string(),
    ))
}

fn format_update_message(current_version: &str, latest_version: &str, release_url: &str) -> String {
    format!(
        "Rayhunter {current_version} is installed, but {latest_version} is available. Open {release_url} to download the update."
    )
}

async fn refresh_update_status(
    status_lock: &Arc<RwLock<UpdateStatus>>,
    http_client: &reqwest::Client,
) -> Result<Option<(String, String)>, String> {
    let response = http_client
        .get(GITHUB_LATEST_RELEASE_URL)
        .timeout(Duration::from_secs(5))
        .header(reqwest::header::USER_AGENT, "rayhunter-update-checker")
        .send()
        .await
        .map_err(|err| format!("failed to query GitHub releases: {err}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "GitHub release check returned {}",
            response.status()
        ));
    }

    let response_text = response
        .text()
        .await
        .map_err(|err| format!("failed to read GitHub release response: {err}"))?;
    let release: GitHubReleaseResponse = serde_json::from_str(&response_text)
        .map_err(|err| format!("failed to parse GitHub release response: {err}"))?;

    let current_version = get_current_version();
    let (current_version_parts, current_version) = parse_release_tagname(&current_version)
        .ok_or_else(|| format!("failed to parse current version {current_version}"))?;
    let (latest_version_parts, latest_version) = parse_release_tagname(&release.tag_name)
        .ok_or_else(|| {
            format!(
                "failed to parse latest release version {}",
                release.tag_name
            )
        })?;

    let update_available = latest_version_parts > current_version_parts;
    {
        let mut status = status_lock.write().await;
        status.current_version = current_version;
        status.latest_version = Some(latest_version.to_owned());
        status.latest_release_url = Some(release.html_url.to_owned());
        status.update_available = update_available;
        status.last_checked = Some(Local::now());
        status.last_error = None;
    }

    if update_available {
        Ok(Some((latest_version, release.html_url)))
    } else {
        Ok(None)
    }
}

pub fn run_update_check_worker(
    task_tracker: &TaskTracker,
    shutdown_token: CancellationToken,
    update_status_lock: Arc<RwLock<UpdateStatus>>,
    notification_sender: Sender<Notification>,
    enabled_notifications: Vec<NotificationType>,
) {
    task_tracker.spawn(async move {
        let http_client = match reqwest::Client::builder().build() {
            Ok(client) => client,
            Err(err) => {
                error!("failed to create update check client: {err}");
                return;
            }
        };
        let mut interval = time::interval(UPDATE_CHECK_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        // Keep track of last notified version
        let mut last_notified_version: Option<String> = None;

        loop {
            if shutdown_token.is_cancelled() {
                break;
            }

            match refresh_update_status(&update_status_lock, &http_client).await {
                Ok(Some((latest_version, latest_release_url))) => {
                    if last_notified_version.as_deref() != Some(latest_version.as_str()) {
                        let current_version =
                            update_status_lock.read().await.current_version.clone();
                        let message = format_update_message(
                            &current_version,
                            &latest_version,
                            &latest_release_url,
                        );
                        if enabled_notifications.contains(&NotificationType::Update) {
                            if let Err(err) = notification_sender
                                .send(Notification::new(NotificationType::Update, message, None))
                                .await
                            {
                                error!("failed to enqueue update notification: {err}");
                            } else {
                                info!("notified about Rayhunter update {latest_version}");
                            }
                        }
                        last_notified_version = Some(latest_version);
                    }
                }
                Ok(None) => {
                    last_notified_version = None;
                }
                Err(err) => {
                    warn!("update check failed: {err}");
                    let mut status = update_status_lock.write().await;
                    status.last_error = Some(err);
                    status.last_checked = Some(Local::now());
                }
            }

            select! {
                _ = shutdown_token.cancelled() => break,
                _ = interval.tick() => {}
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::parse_release_tagname;

    #[test]
    fn parses_simple_versions() {
        let (parts, version) = parse_release_tagname("0.11.1").unwrap();
        assert_eq!(parts.major, 0);
        assert_eq!(parts.minor, 11);
        assert_eq!(parts.patch, 1);
        assert_eq!(version, "0.11.1");
    }

    #[test]
    fn parses_versions_with_v_prefix_and_prerelease() {
        let (parts, version) = parse_release_tagname("v0.11.1-beta.1").unwrap();
        assert_eq!(parts.major, 0);
        assert_eq!(parts.minor, 11);
        assert_eq!(parts.patch, 1);
        assert_eq!(version, "0.11.1");
    }

    #[test]
    fn returns_none_for_invalid_versions() {
        assert!(parse_release_tagname("invalid").is_none());
        assert!(parse_release_tagname("v1.2").is_none());
        assert!(parse_release_tagname("v1.2.x").is_none());
    }

    #[test]
    fn compares_versions_numerically() {
        let (newer_version_parts, newer_version) = parse_release_tagname("v0.11.2").unwrap();
        let (older_version_parts, older_version) = parse_release_tagname("v0.11.1").unwrap();
        assert!(newer_version_parts > older_version_parts);
        assert_eq!(newer_version, "0.11.2");
        assert_eq!(older_version, "0.11.1");
    }

    #[test]
    fn compares_major_minor_patch_correctly() {
        let (v1_parts, v1) = parse_release_tagname("v1.0.0").unwrap();
        let (v2_parts, v2) = parse_release_tagname("v1.0.1").unwrap();
        let (v3_parts, v3) = parse_release_tagname("v1.1.0").unwrap();
        let (v4_parts, v4) = parse_release_tagname("v2.0.0").unwrap();

        assert!(v2_parts > v1_parts);
        assert!(v3_parts > v2_parts);
        assert!(v4_parts > v3_parts);

        assert_eq!(v1, "1.0.0");
        assert_eq!(v2, "1.0.1");
        assert_eq!(v3, "1.1.0");
        assert_eq!(v4, "2.0.0");
    }
}
