//! In-process nl80211 access, replacing the `iw` command line tool.
//!
//! Every function here opens its own short-lived netlink connection. These
//! calls happen at most a few times per minute (interface setup, scans,
//! recovery), so the cost is irrelevant next to the process spawn it replaces,
//! and it keeps callers free of connection lifetime concerns.

use anyhow::{Context, Result, bail};
use futures::TryStreamExt;
use netlink_packet_core::Parseable;
use wl_nl80211::{
    Nl80211Attr, Nl80211AttrsBuilder, Nl80211BssInfo, Nl80211Element, Nl80211ElementRsn,
    Nl80211Elements, Nl80211InterfaceType, Nl80211NewInterface, Nl80211Scan,
};

use crate::scan::WifiNetwork;

/// Wraps a connection plus its driver task, so the driver is aborted on drop.
struct Nl80211Conn {
    handle: wl_nl80211::Nl80211Handle,
    _task: tokio::task::JoinHandle<()>,
}

impl Nl80211Conn {
    async fn open() -> Result<Self> {
        let (connection, handle, _) =
            wl_nl80211::new_connection().context("failed to open nl80211 netlink socket")?;
        let task = tokio::spawn(connection);
        Ok(Nl80211Conn {
            handle,
            _task: task,
        })
    }
}

impl Drop for Nl80211Conn {
    fn drop(&mut self) {
        self._task.abort();
    }
}

/// Identifies a wireless interface: its kernel index and owning wiphy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IfaceInfo {
    pub(crate) if_index: u32,
    pub(crate) wiphy: u32,
}

/// Look up an interface by name via a `GetInterface` dump.
///
/// `NL80211_CMD_NEW_INTERFACE` needs the *wiphy* id, whereas
/// `iw dev <parent> interface add` took a parent interface name, so callers
/// creating an interface must resolve the parent first.
pub(crate) async fn get_interface(name: &str) -> Result<Option<IfaceInfo>> {
    let conn = Nl80211Conn::open().await?;
    let mut dump = conn.handle.interface().get(Vec::new()).execute().await;

    while let Some(msg) = dump
        .try_next()
        .await
        .context("nl80211 GetInterface dump failed")?
    {
        let mut if_index = None;
        let mut wiphy = None;
        let mut if_name = None;
        for attr in &msg.payload.attributes {
            match attr {
                Nl80211Attr::IfIndex(i) => if_index = Some(i),
                Nl80211Attr::Wiphy(w) => wiphy = Some(w),
                Nl80211Attr::IfName(n) => if_name = Some(n.clone()),
                _ => {}
            }
        }
        if if_name.as_deref() == Some(name)
            && let (Some(if_index), Some(wiphy)) = (if_index, wiphy)
        {
            return Ok(Some(IfaceInfo {
                if_index: *if_index,
                wiphy: *wiphy,
            }));
        }
    }
    Ok(None)
}

/// Resolve an interface name to its index, erroring if it does not exist.
pub(crate) async fn if_index(name: &str) -> Result<u32> {
    Ok(get_interface(name)
        .await?
        .with_context(|| format!("wireless interface {name} not found"))?
        .if_index)
}

/// Create a virtual interface on the same wiphy as `parent`.
///
/// Equivalent to `iw dev <parent> interface add <name> type <type>`.
pub(crate) async fn create_interface(
    parent: &str,
    name: &str,
    if_type: Nl80211InterfaceType,
) -> Result<()> {
    let parent_info = get_interface(parent)
        .await?
        .with_context(|| format!("parent interface {parent} not found"))?;

    let conn = Nl80211Conn::open().await?;
    let attrs = Nl80211NewInterface::new(parent_info.wiphy, if_type, name.to_string()).build();
    let mut resp = conn.handle.interface().add(attrs).execute().await;
    resp.try_next()
        .await
        .with_context(|| format!("failed to create interface {name} on {parent}"))?;
    Ok(())
}

/// Change an existing interface's type.
///
/// Equivalent to `iw dev <name> set type <type>`. The kernel requires the
/// interface to be down for most transitions; callers are responsible for
/// that ordering, exactly as they were with `iw`.
pub(crate) async fn set_interface_type(name: &str, if_type: Nl80211InterfaceType) -> Result<()> {
    let idx = if_index(name).await?;
    let conn = Nl80211Conn::open().await?;
    let attrs = Nl80211AttrsBuilder::<wl_nl80211::Nl80211Interface>::new()
        .if_index(idx)
        .interface_type(if_type)
        .build();
    let mut resp = conn.handle.interface().set(attrs).execute().await;
    resp.try_next()
        .await
        .with_context(|| format!("failed to set {name} to type {if_type:?}"))?;
    Ok(())
}

/// Trigger a scan and return the current BSS table.
///
/// Equivalent to `iw dev <iface> scan`. A scan trigger that fails because one
/// is already in progress (`EBUSY`) is not fatal: the dump below still returns
/// the previous results, which is what the old text-parsing path effectively
/// did too.
pub(crate) async fn scan(iface: &str) -> Result<Vec<WifiNetwork>> {
    let idx = if_index(iface).await?;

    {
        let conn = Nl80211Conn::open().await?;
        let attrs = Nl80211Scan::new(idx).build();
        let mut resp = conn.handle.scan().trigger(attrs).execute().await;
        if let Err(e) = resp.try_next().await {
            let msg = e.to_string();
            // -EIO from a wedged radio is the signal monitor.rs uses to decide
            // a module reload is needed; surface it rather than swallowing it.
            if is_io_error(&msg) {
                bail!("scan failed with -EIO; radio may be busy (module reload needed)");
            }
            log::debug!("scan trigger on {iface} failed ({msg}), using cached results");
        } else {
            // The trigger is asynchronous; give the hardware time to sweep the
            // channels before dumping. `iw dev scan` blocked for us before.
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
        }
    }

    dump_scan(idx).await
}

/// Read the kernel's BSS table without triggering a fresh scan.
async fn dump_scan(if_index: u32) -> Result<Vec<WifiNetwork>> {
    let conn = Nl80211Conn::open().await?;
    let mut dump = conn.handle.scan().dump(if_index).execute().await;

    let mut networks: Vec<WifiNetwork> = Vec::new();
    while let Some(msg) = dump.try_next().await.context("nl80211 scan dump failed")? {
        for attr in &msg.payload.attributes {
            if let Nl80211Attr::Bss(bss) = attr
                && let Some(network) = bss_to_network(bss)
            {
                crate::scan::push_or_update(
                    &mut networks,
                    network.ssid,
                    network.signal_dbm,
                    &network.security,
                );
            }
        }
    }
    networks.sort_by_key(|n| std::cmp::Reverse(n.signal_dbm));
    Ok(networks)
}

/// Convert one BSS entry into a [`WifiNetwork`], or `None` for hidden SSIDs.
pub(crate) fn bss_to_network(bss: &[Nl80211BssInfo]) -> Option<WifiNetwork> {
    let mut ssid: Option<String> = None;
    let mut signal_dbm: i32 = -100;
    let mut has_rsn = false;
    let mut has_wpa = false;
    let mut has_sae = false;
    let mut has_psk = false;

    for info in bss {
        let raw_ies = match info {
            // SignalMbm is in mBm (dBm * 100).
            Nl80211BssInfo::SignalMbm(mbm) => {
                signal_dbm = *mbm / 100;
                continue;
            }
            // Probe response IEs are preferred over beacon IEs (a beacon may
            // carry an empty SSID for a hidden network), but either will do.
            Nl80211BssInfo::RawInformationElements(v)
            | Nl80211BssInfo::RawProbeResponseInformationElements(v)
            | Nl80211BssInfo::RawBeaconInformationElements(v) => v,
            _ => continue,
        };

        // Vendor hardware can emit malformed IEs; the crate deliberately hands
        // them over unparsed. A bad BSS must not abort the whole scan.
        let Ok(elements) = Nl80211Elements::parse(raw_ies.as_slice()) else {
            log::debug!("skipping BSS with unparsable information elements");
            continue;
        };

        for element in &elements.0 {
            match element {
                // Prefer the first non-empty SSID across IE sets: a beacon may
                // advertise an empty SSID for a hidden network while the probe
                // response carries the real one. An empty value is still
                // recorded so hidden networks are filtered out below.
                Nl80211Element::Ssid(s) if ssid.as_deref().is_none_or(str::is_empty) => {
                    ssid = Some(s.clone());
                }
                Nl80211Element::Rsn(rsn) => {
                    has_rsn = true;
                    let (psk, sae) = rsn_akms(rsn);
                    has_psk |= psk;
                    has_sae |= sae;
                }
                // WPA1 has no dedicated element; it rides in a Microsoft
                // vendor IE (OUI 00:50:f2, OUI type 1).
                Nl80211Element::Vendor(data) if data.starts_with(&[0x00, 0x50, 0xf2, 0x01]) => {
                    has_wpa = true;
                }
                _ => {}
            }
        }
    }

    let ssid = ssid?;
    if ssid.is_empty() {
        return None;
    }
    Some(WifiNetwork {
        security: crate::scan::resolve_security(has_rsn, has_wpa, has_sae, has_psk),
        ssid,
        signal_dbm,
    })
}

/// Returns `(has_psk, has_sae)` for an RSN element's AKM suites.
fn rsn_akms(rsn: &Nl80211ElementRsn) -> (bool, bool) {
    use wl_nl80211::Nl80211AkmSuite as Akm;
    let mut psk = false;
    let mut sae = false;
    for akm in &rsn.akm_suits {
        match akm {
            Akm::Psk | Akm::PskSha256 | Akm::PskSha384 | Akm::FtPsk | Akm::FtPskSha384 => {
                psk = true;
            }
            Akm::Sae | Akm::FtSae | Akm::SaeGroupDependentHash | Akm::FtSaeGroupDependentHash => {
                sae = true;
            }
            _ => {}
        }
    }
    (psk, sae)
}

/// Human-readable dump of an interface's nl80211 state, for crash diagnostics.
///
/// Replaces the `iw dev <iface> link` / `iw dev <iface> station dump` /
/// `wpa_cli -i <iface> status` snapshot. Never fails: any error is rendered
/// into the returned text so the crash log still gets written.
pub(crate) async fn diagnostics_dump(iface: &str) -> String {
    let mut out = String::new();

    let info = match get_interface(iface).await {
        Ok(Some(info)) => {
            out.push_str(&format!(
                "  if_index: {}\n  wiphy: {}\n",
                info.if_index, info.wiphy
            ));
            info
        }
        Ok(None) => {
            out.push_str(&format!("  (interface {iface} not present)\n"));
            return out;
        }
        Err(e) => {
            out.push_str(&format!("  (interface query failed: {e:#})\n"));
            return out;
        }
    };

    match interface_attrs(info.if_index).await {
        Ok(attrs) => {
            for line in attrs {
                out.push_str(&format!("  {line}\n"));
            }
        }
        Err(e) => out.push_str(&format!("  (attribute query failed: {e:#})\n")),
    }

    match dump_scan(info.if_index).await {
        Ok(networks) => {
            out.push_str(&format!("  visible BSSes: {}\n", networks.len()));
            for n in networks.iter().take(20) {
                out.push_str(&format!(
                    "    {} ({} dBm, {})\n",
                    n.ssid, n.signal_dbm, n.security
                ));
            }
        }
        Err(e) => out.push_str(&format!("  (scan dump failed: {e:#})\n")),
    }

    out
}

/// Selected nl80211 attributes for one interface, rendered as text.
async fn interface_attrs(if_index: u32) -> Result<Vec<String>> {
    let conn = Nl80211Conn::open().await?;
    let mut resp = conn
        .handle
        .interface()
        .get(vec![Nl80211Attr::IfIndex(if_index)])
        .execute()
        .await;

    let mut lines = Vec::new();
    while let Some(msg) = resp
        .try_next()
        .await
        .context("nl80211 GetInterface failed")?
    {
        for attr in &msg.payload.attributes {
            match attr {
                Nl80211Attr::IfName(n) => lines.push(format!("ifname: {n}")),
                Nl80211Attr::IfType(t) => lines.push(format!("iftype: {t:?}")),
                Nl80211Attr::Mac(m) => lines.push(format!(
                    "mac: {}",
                    m.iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<Vec<_>>()
                        .join(":")
                )),
                Nl80211Attr::Ssid(s) => lines.push(format!("ssid: {s}")),
                Nl80211Attr::WiphyFreq(f) => lines.push(format!("freq: {f} MHz")),
                Nl80211Attr::Generation(_) => {}
                _ => {}
            }
        }
    }
    Ok(lines)
}

/// Whether a netlink error string denotes `-EIO`.
fn is_io_error(msg: &str) -> bool {
    msg.contains("os error 5") || msg.contains("Input/output error") || msg.contains("(-5)")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an SSID information element (id 0).
    fn ssid_ie(ssid: &str) -> Vec<u8> {
        let mut v = vec![0x00, ssid.len() as u8];
        v.extend_from_slice(ssid.as_bytes());
        v
    }

    /// Build a minimal RSN element (id 48) advertising the given AKM suites.
    /// Layout: version, group cipher, pairwise count + suite, AKM count + suites.
    fn rsn_ie(akms: &[u8]) -> Vec<u8> {
        let mut body = vec![0x01, 0x00]; // version 1
        body.extend_from_slice(&[0x00, 0x0f, 0xac, 0x04]); // group: CCMP
        body.extend_from_slice(&[0x01, 0x00, 0x00, 0x0f, 0xac, 0x04]); // pairwise: CCMP
        body.extend_from_slice(&[akms.len() as u8, 0x00]);
        for akm in akms {
            body.extend_from_slice(&[0x00, 0x0f, 0xac, *akm]);
        }
        let mut v = vec![48, body.len() as u8];
        v.extend_from_slice(&body);
        v
    }

    /// WPA1 vendor element (id 221) with the Microsoft OUI + type 1.
    fn wpa1_ie() -> Vec<u8> {
        let body = vec![0x00, 0x50, 0xf2, 0x01, 0x01, 0x00];
        let mut v = vec![221, body.len() as u8];
        v.extend_from_slice(&body);
        v
    }

    fn bss(ies: Vec<u8>, signal_mbm: i32) -> Vec<Nl80211BssInfo> {
        vec![
            Nl80211BssInfo::SignalMbm(signal_mbm),
            Nl80211BssInfo::RawInformationElements(ies),
        ]
    }

    // The security strings below must match `parse_iw_scan`'s output exactly:
    // the web UI dropdown renders them verbatim.

    #[test]
    fn bss_wpa2_psk() {
        let mut ies = ssid_ie("WPA2Net");
        ies.extend(rsn_ie(&[0x02])); // PSK
        let n = bss_to_network(&bss(ies, -5500)).expect("network");
        assert_eq!(n.ssid, "WPA2Net");
        assert_eq!(n.signal_dbm, -55);
        assert_eq!(n.security, "WPA2");
    }

    #[test]
    fn bss_wpa3_sae() {
        let mut ies = ssid_ie("WPA3Net");
        ies.extend(rsn_ie(&[0x08])); // SAE
        let n = bss_to_network(&bss(ies, -4500)).expect("network");
        assert_eq!(n.security, "WPA3");
    }

    #[test]
    fn bss_wpa3_transition() {
        let mut ies = ssid_ie("TransitionNet");
        ies.extend(rsn_ie(&[0x02, 0x08])); // PSK + SAE
        let n = bss_to_network(&bss(ies, -5000)).expect("network");
        assert_eq!(n.security, "WPA3 (transition)");
    }

    #[test]
    fn bss_wpa1_vendor_ie() {
        let mut ies = ssid_ie("OldNet");
        ies.extend(wpa1_ie());
        let n = bss_to_network(&bss(ies, -7200)).expect("network");
        assert_eq!(n.security, "WPA");
    }

    #[test]
    fn bss_open_network() {
        let n = bss_to_network(&bss(ssid_ie("OpenCafe"), -6000)).expect("network");
        assert_eq!(n.security, "Open");
    }

    #[test]
    fn bss_hidden_ssid_filtered() {
        assert!(bss_to_network(&bss(ssid_ie(""), -4500)).is_none());
    }

    #[test]
    fn bss_without_ssid_element_filtered() {
        assert!(bss_to_network(&[Nl80211BssInfo::SignalMbm(-4500)]).is_none());
    }

    #[test]
    fn bss_malformed_ies_do_not_panic() {
        // A truncated element header must be skipped, not abort the scan.
        let malformed = vec![48, 0xff, 0x01];
        assert!(bss_to_network(&bss(malformed, -5000)).is_none());
    }

    #[test]
    fn is_io_error_matches_eio_forms() {
        assert!(is_io_error(
            "Netlink error: Input/output error (os error 5)"
        ));
        assert!(is_io_error("failed (-5)"));
        assert!(!is_io_error(
            "Netlink error: Device or resource busy (os error 16)"
        ));
    }
}
