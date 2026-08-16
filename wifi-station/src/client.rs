use std::path::Path;
use std::time::Duration;

use anyhow::{Result, anyhow, bail};
use log::info;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use wl_nl80211::Nl80211InterfaceType as IfType;

use crate::supplicant::{ActiveSupplicant, Supplicant, SupplicantParams};
use crate::{
    DEFAULT_CRASH_LOG_DIR, DEFAULT_DHCP_LEASE_PATH, DEFAULT_UDHCPC_HOOK_PATH, DEFAULT_WPA_BIN,
    DEFAULT_WPA_CONF_PATH, ERR_CREATE_STA, HOSTAPD_CONF, STA_IFACE, UDHCPC_HOOK_SCRIPT, WifiConfig,
};

pub(crate) const TX_STALL_THRESHOLD: u32 = 3;

pub(crate) struct WifiClient {
    pub(crate) iface: String,
    pub(crate) wpa_bin: String,
    pub(crate) hostapd_conf: String,
    pub(crate) supplicant: ActiveSupplicant,
    pub(crate) dhcp_child: Option<Child>,
    pub(crate) rt_table: u32,
    pub(crate) dns_servers: Vec<String>,
    pub(crate) saved_resolv: Option<String>,
    pub(crate) last_tx_packets: Option<u64>,
    pub(crate) last_rx_packets: Option<u64>,
    pub(crate) tx_stall_count: u32,
    pub(crate) udhcpc_hook_path: String,
    pub(crate) dhcp_lease_path: String,
    pub(crate) wpa_conf_path: String,
    pub(crate) udhcpc_bin: String,
    pub(crate) crash_log_dir: String,
}

impl WifiClient {
    pub(crate) fn new(dns_servers: Vec<String>, config: &WifiConfig) -> Self {
        WifiClient {
            iface: STA_IFACE.to_string(),
            wpa_bin: config
                .wpa_supplicant_bin
                .clone()
                .unwrap_or_else(|| DEFAULT_WPA_BIN.to_string()),
            hostapd_conf: config
                .hostapd_conf
                .clone()
                .unwrap_or_else(|| HOSTAPD_CONF.to_string()),
            supplicant: ActiveSupplicant::default(),
            dhcp_child: None,
            rt_table: 100,
            dns_servers,
            saved_resolv: None,
            last_tx_packets: None,
            last_rx_packets: None,
            tx_stall_count: 0,
            udhcpc_hook_path: config
                .udhcpc_hook_path
                .clone()
                .unwrap_or_else(|| DEFAULT_UDHCPC_HOOK_PATH.to_string()),
            dhcp_lease_path: config
                .dhcp_lease_path
                .clone()
                .unwrap_or_else(|| DEFAULT_DHCP_LEASE_PATH.to_string()),
            wpa_conf_path: config
                .wpa_conf_path
                .clone()
                .unwrap_or_else(|| DEFAULT_WPA_CONF_PATH.to_string()),
            udhcpc_bin: config
                .udhcpc_bin
                .clone()
                .unwrap_or_else(|| "udhcpc".to_string()),
            crash_log_dir: config
                .crash_log_dir
                .clone()
                .unwrap_or_else(|| DEFAULT_CRASH_LOG_DIR.to_string()),
        }
    }

    pub(crate) async fn start(&mut self) -> Result<()> {
        self.wait_for_interface().await?;
        self.set_managed_mode().await?;
        self.start_wpa_supplicant().await?;
        self.start_dhcp().await?;
        self.setup_routing().await?;
        self.allow_inbound().await;
        Ok(())
    }

    pub(crate) async fn stop(&mut self) {
        self.supplicant.stop().await;
        if let Some(mut child) = self.dhcp_child.take() {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        self.remove_inbound().await;
        self.cleanup_routing().await;
        self.interface_down().await;

        crate::routing::restore_cellular_default().await;

        if let Some(resolv) = self.saved_resolv.take() {
            let _ = Command::new("umount")
                .arg("/etc/resolv.conf")
                .status()
                .await;
            let _ = tokio::fs::write("/etc/resolv.conf", resolv).await;
        }
    }

    async fn create_sta_interface(&self) -> Result<()> {
        if crate::netlink::create_interface(crate::AP_IFACE, &self.iface, IfType::Station)
            .await
            .is_ok()
        {
            return Ok(());
        }
        info!("direct managed creation failed, trying P2P_CLIENT workaround");
        // Some vendor drivers (notably the Orbic's QCA6174) refuse a second
        // managed interface but will hand out a P2P client, which can then be
        // retyped to managed.
        if let Err(e) =
            crate::netlink::create_interface(crate::AP_IFACE, &self.iface, IfType::P2pClient).await
        {
            bail!("{ERR_CREATE_STA} ({}): {e:#}", self.iface);
        }
        if let Err(e) = crate::netlink::set_interface_type(&self.iface, IfType::Station).await {
            bail!("{ERR_CREATE_STA} ({}: set type managed): {e:#}", self.iface);
        }
        Ok(())
    }

    async fn wait_for_interface(&self) -> Result<()> {
        if !Path::new(&format!("/sys/class/net/{}", self.iface)).exists() {
            info!("{} not found, attempting to create it", self.iface);
            self.create_sta_interface().await?;
        }
        for _ in 0..30 {
            if Path::new(&format!("/sys/class/net/{}", self.iface)).exists() {
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
        }
        bail!("{} not found after 30s", self.iface);
    }

    async fn set_managed_mode(&self) -> Result<()> {
        crate::netlink::set_interface_type(&self.iface, IfType::Station)
            .await
            .map_err(|e| anyhow!("set type managed failed: {e:#}"))?;
        crate::link::set_up(&self.iface, true)
            .await
            .map_err(|e| anyhow!("link set up failed: {e:#}"))?;
        Ok(())
    }

    pub(crate) async fn start_wpa_supplicant(&mut self) -> Result<()> {
        let params = SupplicantParams {
            iface: &self.iface,
            wpa_conf_path: &self.wpa_conf_path,
            wpa_bin: &self.wpa_bin,
        };
        if self.supplicant.start(params).await.is_ok() {
            return Ok(());
        }

        // A scan that fails with -EIO means the radio is wedged; monitor.rs
        // keys off this message to escalate to a module reload.
        if let Err(e) = crate::netlink::scan(&self.iface).await {
            let msg = e.to_string();
            if msg.contains("-EIO") {
                bail!("{msg}");
            }
        }
        bail!("supplicant did not associate within 30s");
    }

    /// Ask the supplicant to reassociate (recovery ladder step 1).
    pub(crate) async fn reassociate(&mut self) {
        self.supplicant.reassociate(&self.iface).await;
    }

    /// Tear down just the supplicant, leaving routing and DHCP in place.
    pub(crate) async fn stop_supplicant(&mut self) {
        self.supplicant.stop().await;
    }

    /// Whether the supplicant died and needs restarting.
    pub(crate) async fn supplicant_exited(&mut self) -> bool {
        self.supplicant.has_exited().await
    }

    pub(crate) async fn start_dhcp(&mut self) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;
        use std::process::Stdio;

        let _ = tokio::fs::remove_file(&self.dhcp_lease_path).await;

        let script = UDHCPC_HOOK_SCRIPT.replacen("{}", &self.dhcp_lease_path, 1);
        tokio::fs::write(&self.udhcpc_hook_path, &script).await?;
        tokio::fs::set_permissions(
            &self.udhcpc_hook_path,
            std::fs::Permissions::from_mode(0o755),
        )
        .await?;

        let child = Command::new(&self.udhcpc_bin)
            .args([
                "-i",
                &self.iface,
                "-s",
                &self.udhcpc_hook_path,
                "-t",
                "10",
                "-A",
                "3",
                "-f",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.dhcp_child = Some(child);

        for _ in 0..30 {
            sleep(Duration::from_secs(1)).await;
            if tokio::fs::metadata(&self.dhcp_lease_path).await.is_ok() {
                return Ok(());
            }
        }
        bail!("DHCP did not assign an address within 30s");
    }

    pub(crate) async fn interface_down(&self) {
        let _ = crate::link::set_up(&self.iface, false).await;
    }

    pub(crate) fn interface_exists(&self) -> bool {
        Path::new(&format!("/sys/class/net/{}", self.iface)).exists()
    }

    pub(crate) async fn read_tx_packets(&self) -> Option<u64> {
        let path = format!("/sys/class/net/{}/statistics/tx_packets", self.iface);
        tokio::fs::read_to_string(&path)
            .await
            .ok()?
            .trim()
            .parse()
            .ok()
    }

    pub(crate) async fn read_rx_packets(&self) -> Option<u64> {
        let path = format!("/sys/class/net/{}/statistics/rx_packets", self.iface);
        tokio::fs::read_to_string(&path)
            .await
            .ok()?
            .trim()
            .parse()
            .ok()
    }

    pub(crate) async fn check_tx_advancing(&self) -> bool {
        let first = self.read_tx_packets().await;
        sleep(Duration::from_secs(5)).await;
        let second = self.read_tx_packets().await;
        match (first, second) {
            (Some(a), Some(b)) => b > a,
            _ => false,
        }
    }
}
