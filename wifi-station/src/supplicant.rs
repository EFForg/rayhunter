//! Pluggable supplicant backends.
//!
//! Two implementations satisfy [`Supplicant`]:
//!
//! - [`SubprocessSupplicant`] spawns `wpa_supplicant` and watches sysfs for
//!   association. This is the default, and the only option for devices whose
//!   firmware already ships a `wpa_supplicant` (T-Mobile, Wingtech, UZ801).
//! - `ShuliSupplicant` (feature `rust-supplicant`) drives the `shuli` crate
//!   in-process, so no external binary is needed at all.
//!
//! The `rust-supplicant` feature is deliberately *not* enabled by default:
//! `shuli` pulls in `aws-lc-rs`, whose C code needs a real cross-compiler.
//! Enabling it unconditionally would break the pure-Rust devel build.

use std::time::Duration;

use anyhow::{Result, bail};
use log::info;
use tokio::process::{Child, Command};
use tokio::time::sleep;

/// Credentials and interface for one connection attempt.
///
/// Not every field is used by every backend: the in-process supplicant needs
/// no `wpa_bin`, and the subprocess backend hands the config path straight to
/// `wpa_supplicant` instead of parsing it.
#[cfg_attr(feature = "rust-supplicant", allow(dead_code))]
pub(crate) struct SupplicantParams<'a> {
    pub(crate) iface: &'a str,
    /// Path to the `wpa_supplicant`-format credential store. This remains the
    /// source of truth for SSID/password regardless of backend, so existing
    /// installs and the daemon's API contract are unchanged.
    pub(crate) wpa_conf_path: &'a str,
    pub(crate) wpa_bin: &'a str,
}

/// Drives association for a STA interface.
#[allow(async_fn_in_trait)]
pub(crate) trait Supplicant {
    /// Associate with the configured network, returning once the link is up.
    async fn start(&mut self, params: SupplicantParams<'_>) -> Result<()>;

    /// Tear down, releasing any child process or netlink state.
    async fn stop(&mut self);

    /// Nudge the supplicant to reassociate without a full restart.
    ///
    /// Best-effort: recovery escalates to a supplicant restart and then a
    /// module reload when this does not help.
    async fn reassociate(&mut self, iface: &str);

    /// Whether the supplicant died and needs restarting.
    ///
    /// Returns `false` when no supplicant was started, so a stopped client is
    /// never mistaken for a crashed one.
    async fn has_exited(&mut self) -> bool;
}

/// Spawns and supervises an external `wpa_supplicant` process.
///
/// Retained even when `rust-supplicant` is enabled so the two backends stay
/// compile-tested together and switching back is a one-line change.
#[cfg_attr(feature = "rust-supplicant", allow(dead_code))]
#[derive(Default)]
pub(crate) struct SubprocessSupplicant {
    child: Option<Child>,
}

#[cfg_attr(feature = "rust-supplicant", allow(dead_code))]
impl SubprocessSupplicant {
    /// Poll sysfs until the interface reports `operstate == up`.
    async fn wait_for_association(iface: &str) -> Result<()> {
        let operstate_path = format!("/sys/class/net/{iface}/operstate");
        for i in 0..30 {
            if let Ok(state) = tokio::fs::read_to_string(&operstate_path).await
                && state.trim() == "up"
            {
                info!("wpa_supplicant associated after {}s", i + 1);
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
        }
        bail!("wpa_supplicant did not associate within 30s");
    }
}

impl Supplicant for SubprocessSupplicant {
    async fn start(&mut self, params: SupplicantParams<'_>) -> Result<()> {
        use std::process::Stdio;

        // Kill any stale wpa_supplicant from a previous daemon run.
        // Use killall instead of pkill -f: older busybox (e.g. Moxee v1.23.2)
        // silently fails with pkill -f regex patterns.
        let _ = Command::new("killall")
            .args(["wpa_supplicant"])
            .output()
            .await;
        sleep(Duration::from_millis(500)).await;

        let child = Command::new(params.wpa_bin)
            .args(["-i", params.iface, "-Dnl80211", "-c", params.wpa_conf_path])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.child = Some(child);

        if Self::wait_for_association(params.iface).await.is_ok() {
            return Ok(());
        }
        self.stop().await;
        bail!("wpa_supplicant did not associate within 30s");
    }

    async fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
    }

    async fn reassociate(&mut self, iface: &str) {
        // `wpa_cli` is no longer shipped by Rayhunter, but firmware images
        // that provide their own wpa_supplicant usually provide it too.
        let _ = Command::new("wpa_cli")
            .args(["-i", iface, "reassociate"])
            .output()
            .await;
    }

    async fn has_exited(&mut self) -> bool {
        match self.child {
            Some(ref mut child) => matches!(child.try_wait(), Ok(Some(_))),
            None => false,
        }
    }
}

/// The supplicant backend selected at compile time.
///
/// With `rust-supplicant` enabled this is the in-process `shuli` client;
/// otherwise it is the external `wpa_supplicant` process. Devices whose
/// firmware ships its own `wpa_supplicant` keep working either way, since a
/// build without the feature is unchanged.
#[cfg(feature = "rust-supplicant")]
pub(crate) type ActiveSupplicant = ShuliSupplicant;

/// The supplicant backend selected at compile time.
#[cfg(not(feature = "rust-supplicant"))]
pub(crate) type ActiveSupplicant = SubprocessSupplicant;

#[cfg(feature = "rust-supplicant")]
pub(crate) use shuli_backend::ShuliSupplicant;

#[cfg(feature = "rust-supplicant")]
mod shuli_backend {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio_util::sync::CancellationToken;

    /// Drives `shuli::WifiClient` on a background task.
    ///
    /// `shuli` exposes a step function (`run()`) that must keep being called
    /// after association to service group rekeys and disconnect events, so the
    /// client lives in its own task for the lifetime of the connection.
    #[derive(Default)]
    pub(crate) struct ShuliSupplicant {
        task: Option<tokio::task::JoinHandle<()>>,
        cancel: Option<CancellationToken>,
        connected: Option<Arc<AtomicBool>>,
    }

    impl Supplicant for ShuliSupplicant {
        async fn start(&mut self, params: SupplicantParams<'_>) -> Result<()> {
            let Some((ssid, security)) =
                crate::config::read_network_from_wpa_conf(params.wpa_conf_path)
            else {
                bail!("no network configured in {}", params.wpa_conf_path);
            };
            let password = crate::config::read_password_from_wpa_conf(params.wpa_conf_path);

            // shuli negotiates SAE vs PSK from the AP's RSN element, so the
            // stored SecurityType is only used for logging here.
            info!("connecting to {ssid} ({security:?}) via shuli");

            let mut config = shuli::WifiConfig::new(params.iface, &ssid);
            if let Some(password) = password.as_deref() {
                config.set_password(password);
            }

            let mut client = shuli::WifiClient::init(config)
                .await
                .map_err(|e| anyhow::anyhow!("shuli init failed: {e}"))?;

            let cancel = CancellationToken::new();
            let connected = Arc::new(AtomicBool::new(false));

            let task_cancel = cancel.clone();
            let task_connected = connected.clone();
            let task = tokio::spawn(async move {
                loop {
                    let step = tokio::select! {
                        biased;
                        _ = task_cancel.cancelled() => break,
                        step = client.run() => step,
                    };
                    match step {
                        Ok(
                            shuli::WifiState::ConnectedWithOffloadRekey
                            | shuli::WifiState::ConnectedWithoutOffloadRekey,
                        ) => task_connected.store(true, Ordering::Relaxed),
                        Ok(_) => task_connected.store(false, Ordering::Relaxed),
                        Err(e) => {
                            log::warn!("shuli: {e}");
                            task_connected.store(false, Ordering::Relaxed);
                        }
                    }
                }
                // Deauthenticate cleanly so the AP frees the association.
                client.shutdown().await;
            });

            self.task = Some(task);
            self.cancel = Some(cancel);
            self.connected = Some(connected.clone());

            for _ in 0..30 {
                if connected.load(Ordering::Relaxed) {
                    return Ok(());
                }
                sleep(Duration::from_secs(1)).await;
            }
            self.stop().await;
            bail!("shuli did not associate within 30s");
        }

        async fn stop(&mut self) {
            if let Some(cancel) = self.cancel.take() {
                cancel.cancel();
            }
            if let Some(task) = self.task.take() {
                // Wait for the loop to observe cancellation so shutdown() can
                // send the deauth, but do not hang if a run() step is wedged.
                if tokio::time::timeout(Duration::from_secs(5), task)
                    .await
                    .is_err()
                {
                    log::warn!("shuli task did not stop within 5s");
                }
            }
            self.connected = None;
        }

        async fn reassociate(&mut self, _iface: &str) {
            // shuli reconnects on its own from the run() loop; there is no
            // separate reassociate command to issue.
        }

        async fn has_exited(&mut self) -> bool {
            match self.task {
                Some(ref task) => task.is_finished(),
                None => false,
            }
        }
    }
}
