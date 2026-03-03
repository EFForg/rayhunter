/// Installer for the Wingtech CT2MHS01 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
///   WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
///   WT_HARDWARE_VERSION=89323_1_20
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use anyhow::{Context, Result, bail};
use base64_light::base64_encode_bytes;
use block_padding::{Padding, Pkcs7};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::WingtechArgs as Args;
use crate::connection::{TelnetConnection, install_config, install_wifi_creds};
use crate::output::{print, println};
use crate::util::{http_ok_every, telnet_send_command, telnet_send_file};

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

pub async fn install(
    Args {
        admin_ip,
        admin_password,
        wifi_ssid,
        wifi_password,
    }: Args,
) -> Result<()> {
    wingtech_run_install(
        admin_ip,
        admin_password,
        wifi_ssid.as_deref(),
        wifi_password.as_deref(),
    )
    .await
}

const KEY: &[u8] = b"abcdefghijklmn12";

/// Returns password encrypted in AES128 ECB mode with the key b"abcdefghijklmn12",
/// with Pkcs7 padding, encoded in base64.
fn encrypt_password(password: &[u8]) -> Result<String> {
    let c = Aes128::new_from_slice(KEY)?;
    let mut b = GenericArray::from([0u8; 16]);
    b[..password.len()].copy_from_slice(password);
    Pkcs7::pad(&mut b, password.len());
    c.encrypt_block(&mut b);
    Ok(base64_encode_bytes(&b))
}

pub async fn start_telnet(admin_ip: &str, admin_password: &str) -> Result<()> {
    run_command(admin_ip, admin_password, "busybox telnetd -l /bin/sh").await
}

pub async fn start_adb(admin_ip: &str, admin_password: &str) -> Result<()> {
    run_command(admin_ip, admin_password, "/sbin/usb/compositions/9025").await
}

pub async fn run_command(admin_ip: &str, admin_password: &str, cmd: &str) -> Result<()> {
    let qcmap_auth_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_auth");
    let qcmap_web_cgi_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_web_cgi");

    let encrypted_pw = encrypt_password(admin_password.as_bytes()).ok().unwrap();

    let client = Client::new();
    let LoginResponse { token } = client
        .post(&qcmap_auth_endpoint)
        .body(format!(
            "type=login&pwd={encrypted_pw}&timeout=60000&user=admin"
        ))
        .send()
        .await?
        .json()
        .await
        .context("login did not return a token in response")?;

    let command = client.post(&qcmap_web_cgi_endpoint)
        .body(format!("page=setFWMacFilter&cmd=del&mode=0&mac=50:5A:CA:B5:05||{cmd}&key=50:5A:CA:B5:05:AC&token={token}"))
        .send()
        .await?;
    if command.status() != 200 {
        bail!(
            "running command failed with status code: {:?}",
            command.status()
        );
    }

    Ok(())
}

async fn wingtech_run_install(
    admin_ip: String,
    admin_password: String,
    wifi_ssid: Option<&str>,
    wifi_password: Option<&str>,
) -> Result<()> {
    print!("Starting telnet ... ");
    start_telnet(&admin_ip, &admin_password).await?;
    println!("ok");

    print!("Connecting via telnet to {admin_ip} ... ");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();
    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0", true).await?;
    println!("ok");

    let mut conn = TelnetConnection::new(addr, true);
    let wifi_enabled = wifi_ssid.is_some() && wifi_password.is_some();
    install_config(&mut conn, "wingtech", false, wifi_enabled).await?;
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
    telnet_send_command(
        addr,
        "update-rc.d rayhunter_daemon defaults",
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
    telnet_send_command(addr, "shutdown -r -t 1 now", "exit code 0", true).await?;
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

#[test]
fn test_encrypt_password() {
    let p = b"80536913";
    let s = encrypt_password(p).ok();
    let expected = Some("5brvd8xl732cSoFTAy67ig==".to_string());
    assert_eq!(s, expected);
}
