//! Display backend for the Verizon Jetpack MiFi 7730L.
//!
//! This device's screen is not usable as a plain framebuffer. It's a 320x240
//! packed-RGB666 panel behind a custom "NVTL LCD driver" where a bare
//! `write()` to `/dev/fb0` returns success but never updates the panel (the
//! real update path is an undocumented `ioctl(0x9999)` after an `mmap`), and
//! the framebuffer is continuously owned and redrawn by `devuiappd`, a stock
//! Qt5 compositor. Drawing pixels directly means fighting that compositor and
//! disabling it, which also freezes the boot splash.
//!
//! Instead this drives the device's own native alert system ("ANS"), the same
//! mechanism the stock firmware uses for battery/SIM/roaming banners, via the
//! `/opt/nvtl/bin/ans_cli` helper. Three custom notification IDs
//! (`rayhunterLow` / `rayhunterMedium` / `rayhunterWarning`), one per
//! non-informational `EventType`, are registered out of band by the installer;
//! this module just asserts/clears the one matching the current severity.

use crate::config;
use crate::display::DisplayState;
use rayhunter::analysis::analyzer::EventType;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

// `ans_cli` is a private CLI for devuiappd's ANS ("Alert Notification
// Service"). `trigger_notification` is interactive — it prompts on stdin for
// an id and add(1)/remove(0) — so it's driven as a child process.
//
// The IDs below are genuinely new, custom entries added to ANS's settings
// store by the installer, not stock IDs repurposed. Reusing a stock ID is not
// safe: each is either backed by a real daemon that overrides a forced state
// within ~1s (e.g. `lowCharger`), or wired to a real action flow — notably
// `systemDiagnostics`, which triggers Verizon's OMA-DM remote-config consent
// dialog ("...requesting to modify several settings on your device... Accept?")
// on real hardware.
//
// ANS has no API to set notification text at trigger time (it's static
// per-ID), so distinct on-screen wording per severity needs one ID per tier.
const ANS_CLI: &str = "/opt/nvtl/bin/ans_cli";
const NOTIF_LOW: &str = "rayhunterLow";
const NOTIF_MEDIUM: &str = "rayhunterMedium";
const NOTIF_HIGH: &str = "rayhunterWarning"; // named before tiering existed; kept to avoid re-registering
const POLL_INTERVAL: Duration = Duration::from_millis(1000);

// ANS auto-clears any active notification after ~8s (`ExpiryDuration` in
// `/opt/nvtl/etc/ans/config.xml`), regardless of `Lifetime=state`. Rather than
// edge-trigger and fight that timer, the loop re-asserts the desired ID every
// tick, well inside the window. Our IDs have `wake_display_enabled=0`, so
// re-asserting can't pop a modal.
const WATCHDOG_FAILURE_THRESHOLD: u32 = 15; // ~15s of sustained failure at POLL_INTERVAL
const SELF_HEAL_COOLDOWN: Duration = Duration::from_secs(60);
// A WarningDetected state is otherwise sticky forever: `desired_id` only
// changes when a *new* DisplayState arrives, so a single transient warning
// keeps the alert asserted every tick indefinitely — and, via the watchdog,
// drives restart_ansd() on a permanent loop. Seen on real hardware: with a
// dead RTC, an analyzer misfired on a backwards clock jump shortly after boot
// and the alert plus a ~75s ansd-restart cycle then ran for hours. Auto-revert
// to neutral if no fresh WarningDetected arrives within this window; a genuine
// ongoing detection re-emits well inside it. The web UI / stored report stays
// the source of truth for real detections.
const WARNING_MAX_AGE: Duration = Duration::from_secs(600);
// Give up restarting ansd after this many consecutive self-heals fail to
// restore the notification. An endless ansd-restart loop has device-wide blast
// radius (every notification, plus an audible cycle) and is worse than one
// stuck icon; at that point a human needs to look.
const MAX_CONSECUTIVE_SELF_HEALS: u32 = 5;

fn notification_id_for(event_type: EventType) -> Option<&'static str> {
    match event_type {
        EventType::Informational => None,
        EventType::Low => Some(NOTIF_LOW),
        EventType::Medium => Some(NOTIF_MEDIUM),
        EventType::High => Some(NOTIF_HIGH),
    }
}

async fn set_notification_active(id: &str, active: bool) {
    let input = format!("{id}\n{}\n", if active { 1 } else { 0 });
    let mut child = match tokio::process::Command::new(ANS_CLI)
        .arg("trigger_notification")
        // ans_cli dynamically links against /opt/nvtl/lib/*.so — the stock
        // launcher scripts export this, but our init.d-launched environment
        // doesn't, so without it the child fails at the dynamic linker before
        // it even reads stdin.
        .env("LD_LIBRARY_PATH", "/opt/qt5/lib:/opt/nvtl/lib")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            log::warn!("failed to spawn ans_cli: {e}");
            return;
        }
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(input.as_bytes()).await;
        let _ = stdin.flush().await;
        drop(stdin);
    }
    match child.wait_with_output().await {
        Ok(output) if output.status.success() => {
            log::info!(
                "ans_cli trigger_notification({id}, {active}) -> {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
        Ok(output) => {
            log::warn!(
                "ans_cli trigger_notification({id}, {active}) failed, status={:?}, stdout={}, stderr={}",
                output.status,
                String::from_utf8_lossy(&output.stdout).trim(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Err(e) => log::warn!("failed to wait on ans_cli: {e}"),
    }
}

// `last_asserted_id` tracks what was last turned on — distinct from
// `desired_id` (latest analysis state) — so a quiet tick with nothing desired
// skips spawning ans_cli entirely.
async fn reconcile_notification(
    desired_id: Option<&'static str>,
    last_asserted_id: &mut Option<&'static str>,
) {
    if *last_asserted_id != desired_id {
        if let Some(prev) = *last_asserted_id {
            set_notification_active(prev, false).await;
        }
        if let Some(cur) = desired_id {
            set_notification_active(cur, true).await;
        }
        *last_asserted_id = desired_id;
    } else if let Some(cur) = desired_id {
        // No transition, but keep refreshing the active one to defeat the
        // ~8-second expiry.
        set_notification_active(cur, true).await;
    }
}

async fn is_notification_active(id: &str) -> bool {
    let output = match tokio::process::Command::new(ANS_CLI)
        .arg("get_current_notification_list")
        .env("LD_LIBRARY_PATH", "/opt/qt5/lib:/opt/nvtl/lib")
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            log::warn!("failed to run ans_cli get_current_notification_list: {e}");
            return false;
        }
    };
    String::from_utf8_lossy(&output.stdout).contains(&format!("id:[{id}]"))
}

// Recovery for an observed ANS quirk: occasionally a notification stops
// appearing at all despite `trigger_notification` reporting success every
// time, and the only fix found on real hardware is restarting `ansd`. This has
// blast radius — restarting ansd briefly drops every notification on the
// device — so it only fires after a sustained failure window and is
// rate-limited by SELF_HEAL_COOLDOWN.
async fn restart_ansd() {
    log::warn!(
        "self-healing: desired notification hasn't confirmed active for {WATCHDOG_FAILURE_THRESHOLD} consecutive ticks — restarting ansd"
    );
    // ansd isn't managed by /etc/init.d, so a plain kill + relaunch is the
    // restart mechanism. Run as one shell script so `nohup ... &` detaches the
    // same way it does when run manually over adb.
    let script = "kill $(pidof ansd) 2>/dev/null; sleep 1; \
                   LD_LIBRARY_PATH=/opt/qt5/lib:/opt/nvtl/lib nohup /opt/nvtl/bin/ansd \
                   </dev/null >/tmp/ansd_selfheal.log 2>&1 & sleep 2; pidof ansd";
    match tokio::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .await
    {
        Ok(output) => log::info!(
            "self-heal ansd restart: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ),
        Err(e) => log::error!("self-heal: failed to run ansd restart script: {e}"),
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    _config: &config::Config,
    shutdown_token: CancellationToken,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    task_tracker.spawn(async move {
        // Only one tier's notification should ever be showing; on a severity
        // change the previous ID must be explicitly cleared (registering a new
        // ID doesn't replace another).
        let mut desired_id: Option<&'static str> = None;
        let mut last_asserted_id: Option<&'static str> = None;
        // When the current WarningDetected state was last (re)affirmed by an
        // incoming message — used to expire a stale warning (WARNING_MAX_AGE).
        let mut warning_since: Option<Instant> = None;
        // Watchdog: consecutive ticks where the desired notification failed to
        // confirm active, plus rate-limiting state for restart_ansd().
        let mut consecutive_confirm_failures: u32 = 0;
        let mut last_self_heal: Option<Instant> = None;
        // Consecutive self-heals that haven't restored the notification;
        // capped by MAX_CONSECUTIVE_SELF_HEALS. Reset on recovery / neutral.
        let mut consecutive_self_heals: u32 = 0;
        loop {
            if shutdown_token.is_cancelled() {
                log::info!("received UI shutdown");
                reconcile_notification(None, &mut last_asserted_id).await;
                break;
            }

            match ui_update_rx.try_recv() {
                Ok(state) => {
                    desired_id = match state {
                        DisplayState::WarningDetected { event_type } => {
                            notification_id_for(event_type)
                        }
                        DisplayState::Recording | DisplayState::Paused => None,
                    };
                    warning_since = desired_id.map(|_| Instant::now());
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(e) => log::error!("error receiving display update message: {e}"),
            }

            // Expire a warning that hasn't been re-affirmed within
            // WARNING_MAX_AGE so a one-shot false positive can't hold the alert
            // (and the self-heal loop) on indefinitely.
            if let Some(since) = warning_since
                && since.elapsed() >= WARNING_MAX_AGE
            {
                log::warn!(
                    "warning state not re-affirmed for {}s — clearing on-screen alert",
                    WARNING_MAX_AGE.as_secs()
                );
                desired_id = None;
                warning_since = None;
            }

            reconcile_notification(desired_id, &mut last_asserted_id).await;

            match desired_id {
                Some(id) if is_notification_active(id).await => {
                    consecutive_confirm_failures = 0;
                    consecutive_self_heals = 0;
                }
                Some(_) => {
                    consecutive_confirm_failures += 1;
                    if consecutive_confirm_failures >= WATCHDOG_FAILURE_THRESHOLD {
                        let cooldown_elapsed = last_self_heal
                            .map(|t| t.elapsed() >= SELF_HEAL_COOLDOWN)
                            .unwrap_or(true);
                        if consecutive_self_heals >= MAX_CONSECUTIVE_SELF_HEALS {
                            log::error!(
                                "self-healing gave up: {MAX_CONSECUTIVE_SELF_HEALS} consecutive ansd restarts did not restore the notification — leaving ansd alone for a human to investigate"
                            );
                        } else if cooldown_elapsed {
                            restart_ansd().await;
                            last_self_heal = Some(Instant::now());
                            consecutive_self_heals += 1;
                            // Force reconcile_notification to treat the next
                            // tick as a fresh transition (re-issuing the
                            // trigger) rather than trusting pre-restart state.
                            last_asserted_id = None;
                        }
                        consecutive_confirm_failures = 0;
                    }
                }
                None => {
                    consecutive_confirm_failures = 0;
                    consecutive_self_heals = 0;
                }
            }

            tokio::time::sleep(POLL_INTERVAL).await;
        }
    });
}
