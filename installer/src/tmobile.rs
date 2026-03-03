/// Installer for the TMobile TMOHS1 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89527AA1_V045_M11_TMO_USR_MP
///   WT_PRODUCTION_VERSION=TMOHS1_00.05.20
///   WT_HARDWARE_VERSION=89527_1_11
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

use crate::TmobileArgs as Args;
use crate::connection::{TelnetConnection, install_config, install_wifi_creds};
use crate::output::{print, println};
use crate::util::{http_ok_every, telnet_send_command, telnet_send_file};
use crate::wingtech::start_telnet;

pub async fn install(
    Args {
        admin_ip,
        admin_password,
        wifi_ssid,
        wifi_password,
    }: Args,
) -> Result<()> {
    run_install(
        admin_ip,
        admin_password,
        wifi_ssid.as_deref(),
        wifi_password.as_deref(),
    )
    .await
}

async fn run_install(
    admin_ip: String,
    admin_password: String,
    wifi_ssid: Option<&str>,
    wifi_password: Option<&str>,
) -> Result<()> {
    print!("Starting telnet ... ");
    start_telnet(&admin_ip, &admin_password).await?;
    sleep(Duration::from_millis(200)).await;
    println!("ok");

    print!("Connecting via telnet to {admin_ip} ... ");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();
    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0", true).await?;
    println!("ok");

    telnet_send_command(addr, "mount -o remount,rw /", "exit code 0", true).await?;

    let mut conn = TelnetConnection::new(addr, true);
    let wifi_enabled = wifi_ssid.is_some() && wifi_password.is_some();
    install_config(&mut conn, "tmobile", false, wifi_enabled).await?;
    install_wifi_creds(&mut conn, wifi_ssid, wifi_password).await?;

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    telnet_send_file(
        addr,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
        true,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /data/rayhunter/rayhunter-daemon",
        "exit code 0",
        true,
    )
    .await?;
    #[cfg(feature = "wifi-client")]
    {
        telnet_send_command(addr, "mkdir -p /data/rayhunter/bin", "exit code 0", true).await?;
        let wpa_supplicant_bin = include_bytes!(env!("FILE_WPA_SUPPLICANT"));
        let wpa_cli_bin = include_bytes!(env!("FILE_WPA_CLI"));
        telnet_send_file(
            addr,
            "/data/rayhunter/bin/wpa_supplicant",
            wpa_supplicant_bin,
            true,
        )
        .await?;
        telnet_send_file(addr, "/data/rayhunter/bin/wpa_cli", wpa_cli_bin, true).await?;
        telnet_send_file(
            addr,
            "/data/rayhunter/udhcpc-hook.sh",
            include_bytes!("../../dist/scripts/udhcpc-hook.sh"),
            true,
        )
        .await?;
        telnet_send_command(
            addr,
            "chmod +x /data/rayhunter/bin/wpa_supplicant /data/rayhunter/bin/wpa_cli /data/rayhunter/udhcpc-hook.sh",
            "exit code 0",
            true,
        )
        .await?;
    }
    telnet_send_file(
        addr,
        "/etc/init.d/misc-daemon",
        include_bytes!("../../dist/scripts/misc-daemon"),
        true,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/misc-daemon",
        "exit code 0",
        true,
    )
    .await?;
    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        crate::RAYHUNTER_DAEMON_INIT.as_bytes(),
        true,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
        true,
    )
    .await?;
    telnet_send_file(
        addr,
        "/etc/init.d/S01iptables",
        include_bytes!("../../dist/scripts/S01iptables"),
        true,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/S01iptables",
        "exit code 0",
        true,
    )
    .await?;

    println!("Rebooting device and waiting 30 seconds for it to start up.");
    telnet_send_command(addr, "reboot", "exit code 0", true).await?;
    sleep(Duration::from_secs(30)).await;

    print!("Testing rayhunter ... ");
    let max_failures = 10;
    http_ok_every(
        format!("http://{admin_ip}:8080/index.html"),
        Duration::from_secs(3),
        max_failures,
    )
    .await?;
    println!("ok");
    println!("rayhunter is running at http://{admin_ip}:8080");

    Ok(())
}
