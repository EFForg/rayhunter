/// Installer for the TMobile TMOHS1 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89527AA1_V045_M11_TMO_USR_MP
///   WT_PRODUCTION_VERSION=TMOHS1_00.05.20
///   WT_HARDWARE_VERSION=89527_1_11
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

use crate::TmobileArgs as Args;
use crate::util::{echo, http_ok_every, telnet_send_command, telnet_send_file};
use crate::wingtech::start_telnet;

pub async fn install(
    Args {
        admin_ip,
        admin_password,
    }: Args,
) -> Result<()> {
    run_install(admin_ip, admin_password).await
}

async fn run_install(admin_ip: String, admin_password: String) -> Result<()> {
    echo!("Starting telnet ... ");
    start_telnet(&admin_ip, &admin_password).await?;
    sleep(Duration::from_millis(200)).await;
    println!("ok");

    echo!("Connecting via telnet to {admin_ip} ... ");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();
    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0", true).await?;
    println!("ok");

    telnet_send_command(addr, "mount -o remount,rw /", "exit code 0", true).await?;

    telnet_send_file(
        addr,
        "/data/rayhunter/config.toml",
        crate::CONFIG_TOML
            .replace("#device = \"orbic\"", "device = \"tmobile\"")
            .as_bytes(),
        true,
    )
    .await?;

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

    println!("Rebooting device and waiting 30 seconds for it to start up.");
    telnet_send_command(addr, "reboot", "exit code 0", true).await?;
    sleep(Duration::from_secs(30)).await;

    echo!("Testing rayhunter ... ");
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
