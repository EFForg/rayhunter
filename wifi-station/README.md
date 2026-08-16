# wifi-station

Async WiFi station (STA) lifecycle manager for embedded Linux.

> **Vendored into Rayhunter.** This is a copy of
> [BeigeBox/wifi-station](https://github.com/BeigeBox/wifi-station) at tag
> `v0.10.1` (commit `b011de5a4330a8ceac9a6011dc1952082fb9ff65`), vendored as a
> workspace member so Rayhunter can replace the `iw`/`wpa_cli`/`wpa_supplicant`
> subprocess calls with in-process netlink. Changes made here should be offered
> back upstream.

[![Crates.io](https://img.shields.io/crates/v/wifi-station.svg)](https://crates.io/crates/wifi-station)
[![Docs.rs](https://docs.rs/wifi-station/badge.svg)](https://docs.rs/wifi-station)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

`wifi-station` runs and supervises a `wlan1` STA interface alongside an
existing `wlan0` AP on dongles and MiFi-style devices, so a host can roam
onto upstream WiFi while continuing to serve its own clients.

It targets the messy realities of small embedded systems: quirky vendor
drivers, busybox userspace, AP+STA coexistence locks, kernel module crashes,
and stalled data paths. Recovery is built in.

> **Platform:** Linux only. Requires root (or `CAP_NET_ADMIN` plus
> `CAP_SYS_MODULE` for the recovery path). Will not build/run meaningfully
> on macOS, Windows, or BSD.

## Why this crate

Most existing Rust WiFi helpers assume a NetworkManager-equipped desktop or a
fully-featured `wpa_supplicant` on a generic rootfs. `wifi-station` is built
for the other end of the spectrum — Android-derived MiFi devices and similar
small targets, where:

- the vendor ships hostapd already running on `wlan0` and the `wlan.ko`
  driver is fragile or radio-locked to AP mode,
- userspace is busybox (so `pkill -f` silently fails on regex patterns),
- routing must coexist with a cellular `rmnet*` default route,
- `/etc/resolv.conf` may be on a read-only partition,
- `wpa_supplicant` and `udhcpc` exit unexpectedly under load, and
- recovering from a crashed driver means `rmmod` / `insmod` plus
  reconstructing the AP bridge.

## Install

```toml
[dependencies]
wifi-station = "0.10"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync"] }
tokio-util = { version = "0.7", features = ["rt"] }
```

`tokio-util`'s `rt` feature is required for `TaskTracker`, which the public
API uses to spawn the supervisor task.

## Quick start

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use wifi_station::{WifiConfig, WifiStatus, run_wifi_client};

#[tokio::main]
async fn main() {
    let tasks = TaskTracker::new();
    let shutdown = CancellationToken::new();
    let status = Arc::new(RwLock::new(WifiStatus::default()));

    let config = WifiConfig {
        wifi_enabled: true,
        ..Default::default()
    };

    run_wifi_client(&tasks, &config, shutdown.clone(), status.clone());

    // Your application reads `status.read().await` whenever it needs to know
    // whether wifi is connected, the SSID, the IP, and so on. Here, just
    // wait for Ctrl-C before tearing down.
    let _ = tokio::signal::ctrl_c().await;

    shutdown.cancel();
    tasks.close();
    tasks.wait().await;
}
```

`run_wifi_client` is fire-and-forget: it spawns a long-lived task on the
provided `TaskTracker` and returns `()` immediately. The task supervises the
link until the cancellation token fires, writing live state — including any
fatal error message — into the shared `WifiStatus` (read it via
`status.read().await`; check `WifiState::Failed` and `status.error` to
detect terminal failures).

> **Heads up:** the supervisor silently does nothing if `config.wifi_enabled`
> is `false` *or* if the file at `wpa_conf_path` doesn't yet exist. Call
> `update_wpa_conf` first (next section) so the supplicant config is on
> disk, otherwise `run_wifi_client` will return without logging anything
> and `WifiStatus::state` will stay `Disabled`.

Only one supervisor instance should run on a host at a time: it grabs a
global kernel wakelock, rewrites `/etc/resolv.conf`, and mutates routing
tables and `iptables` rules — concurrent instances will fight.

## Provisioning a network

`run_wifi_client` reads the `wpa_supplicant` config at `wpa_conf_path`
(default `/etc/wpa_supplicant/wpa_sta.conf`); it does **not** consume
`config.wifi_ssid` / `wifi_password` directly. To provision or rotate
credentials, call `update_wpa_conf` with a populated `WifiConfig`:

```rust
use wifi_station::{SecurityType, WifiConfig, update_wpa_conf};

let config = WifiConfig {
    wifi_enabled: true,
    wifi_ssid: Some("MyNetwork".into()),
    wifi_password: Some("hunter2".into()),
    security_type: Some(SecurityType::WpaPsk), // or SecurityType::Sae for WPA3
    ..Default::default()
};
update_wpa_conf(&config).await;
```

`update_wpa_conf` is safe with arbitrary user input — SSIDs and passwords are
escaped (`"` and `\` quoted, embedded newlines stripped) before being written
into the supplicant config, and the file is `chmod 600`. If `wifi_ssid` is
empty or absent the file is removed; if a password is missing, the call is a
no-op.

The crate also exposes `format_wpa_conf` (the pure formatter) and
`read_network_from_wpa_conf` / `read_ssid_from_wpa_conf` for round-tripping
existing configs.

## Scanning

```rust
use wifi_station::{STA_IFACE, scan_wifi_networks};

let networks = scan_wifi_networks(STA_IFACE).await?;
for net in networks {
    println!("{} ({} dBm) - {}", net.ssid, net.signal_dbm, net.security);
}
```

`scan_wifi_networks` brings the interface up if needed, runs `iw dev <iface> scan`,
parses results, deduplicates by SSID (keeping the strongest signal), and
sorts strongest-first. Hidden SSIDs are filtered out. Security is reported as
`Open`, `WPA`, `WPA2`, `WPA3`, or `WPA3 (transition)`.

## What the supervisor does

Once `run_wifi_client` is running, it:

- **Creates the STA interface.** Tries `iw dev wlan0 interface add wlan1 type
  managed` first, falls back to a P2P-client workaround for drivers that
  reject direct managed-mode creation, then promotes the result back to
  managed.
- **Runs `wpa_supplicant`** against the configured conf path and waits up to
  30s for association.
- **Runs `udhcpc`** with a generated hook script that writes lease info
  (gateway, DNS) to `dhcp_lease_path`.
- **Installs policy routing.** Adds a per-source route table for STA traffic,
  demotes any non-STA default route to metric 1000 so WiFi takes priority
  while the link is up, and restores the original default route on shutdown.
- **Replaces `/etc/resolv.conf`** with DHCP-provided DNS (or configured
  fallbacks). If the file is read-only, falls back to a bind mount from
  `/tmp/resolv.conf`.
- **Allows inbound traffic** on the STA interface via `iptables` rules.
- **Holds a kernel wakelock** (`/sys/power/wake_lock`) while the link is
  active, releasing it on shutdown.

## Recovery ladder

`wifi-station` watches `/sys/class/net/wlan1/statistics/{tx,rx}_packets` and
the existence of the STA interface on a 30-second poll cadence (so the
"3 consecutive polls" stall threshold below is roughly 90 seconds of dead
data path before recovery kicks in), and walks a graduated recovery ladder
when something goes wrong:

| Trigger | Recovery |
|---|---|
| `wpa_supplicant` exits | restart it |
| `udhcpc` exits | restart DHCP, re-establish routing |
| TX *and* RX counters frozen for 3 consecutive polls | `wpa_cli reassociate` → restart `wpa_supplicant` → `ip link` cycle |
| Stall ladder above exhausted without recovery | `rmmod wlan` + `insmod wlan.ko` + recreate STA + restart hostapd |
| `wlan1` disappears (driver crash) | full module reload immediately |
| `iw scan` fails with `-EIO` (radio locked to AP) | module reload, then create STA *before* hostapd restarts |

After 5 consecutive module-reload attempts the supervisor reports
`WifiState::Failed` and exits. Each module-reload-class event also writes a
diagnostic snapshot — `dmesg`, `iw link`, `iw station dump`, `/proc/net/dev`,
`wpa_cli status`, ARP table, all routing tables, `iptables -L`,
`/proc/modules`, `ip addr`, and `ps` — to
`<crash_log_dir>/wifi-diag-<timestamp>.log`. The 10 most recent logs are
retained.

## Configuration reference

`WifiConfig` fields and their defaults:

| Field | Default | Purpose |
|---|---|---|
| `wifi_enabled` | `false` | Master switch — when `false`, `run_wifi_client` is a no-op |
| `wifi_ssid`, `wifi_password`, `security_type` | `None` | Consumed by `update_wpa_conf` to write the supplicant config |
| `dns_servers` | `9.9.9.9`, `149.112.112.112` | Fallback when DHCP doesn't provide DNS |
| `wpa_supplicant_bin` | `wpa_supplicant` | Override binary path |
| `udhcpc_bin` | `udhcpc` | Override DHCP client binary |
| `iw_bin` | `iw` | Override `iw` binary |
| `wpa_conf_path` | `/etc/wpa_supplicant/wpa_sta.conf` | Supplicant config path; read by the supervisor and written by `update_wpa_conf` |
| `udhcpc_hook_path` | `/tmp/wifi-station-udhcpc-hook.sh` | Where the DHCP hook script is generated |
| `dhcp_lease_path` | `/tmp/wifi-station-dhcp-lease` | Where the DHCP hook stores `gateway=` and `dns=` |
| `crash_log_dir` | `/tmp/wifi-station-crash-logs` | Diagnostic snapshot directory |
| `wakelock_name` | `wifi-station` | Kernel wakelock name |
| `hostapd_conf` | `/data/misc/wifi/hostapd.conf` | Used during AP restoration after a module reload |
| `ctrl_interface` | `/var/run/wpa_supplicant` | Written into the supplicant config |

The interface names themselves are hard-coded: `wlan0` for AP, `wlan1` for
STA.

## Runtime requirements

The host rootfs must provide these binaries (on `PATH`, or override the
relevant `WifiConfig` field):

- always: `iw`, `wpa_supplicant`, `udhcpc`, `ip`, `killall`
- for module-reload recovery: `rmmod`, `insmod`, `hostapd`, `brctl`,
  `ifconfig`, `wpa_cli`
- for diagnostics: `dmesg`, `ps`, `iptables`

The crate looks for `wlan.ko` at, in order:

1. `/system/lib/modules/wlan.ko` (Android convention)
2. `/lib/modules/$(uname -r)/extra/wlan.ko`
3. `/usr/lib/modules/$(uname -r)/extra/wlan.ko`

A bridge interface (`bridge0` or `br0`) is auto-detected at runtime for AP
restoration after a module reload.

## Cargo features

- `utoipa` — derives `utoipa::ToSchema` on `WifiStatus`, `WifiState`,
  `WifiNetwork`, and `SecurityType` so they can be exposed in OpenAPI specs.

## Logging

`wifi-station` uses the [`log`](https://crates.io/crates/log) crate. State
transitions, recovery attempts, and failures are logged at `info` / `warn` /
`error`; pair with any `log`-compatible backend (`env_logger`, `tracing-log`,
…) to surface them.

## Status

Pre-1.0. The public API is small and unlikely to shift much, but minor
version bumps may still introduce breaking changes.

The minimum supported Rust version (MSRV) is **1.88**. MSRV bumps are
treated as a minor version change.

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
