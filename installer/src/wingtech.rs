/// Installer for the Wingtech CT2MHS01 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
///   WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
///   WT_HARDWARE_VERSION=89323_1_20
use std::io::Write;
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
use crate::util::{echo, http_ok_every, telnet_send_command, telnet_send_file};

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

pub async fn install(
    Args {
        admin_ip,
        admin_password,
    }: Args,
) -> Result<()> {
    wingtech_run_install(admin_ip, admin_password).await
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

async fn wingtech_run_install(admin_ip: String, admin_password: String) -> Result<()> {
    echo!("Starting telnet ... ");
    start_telnet(&admin_ip, &admin_password).await?;
    println!("ok");

    echo!("Connecting via telnet to {admin_ip} ... ");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();
    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0", true).await?;
    println!("ok");

    telnet_send_file(
        addr,
        "/data/rayhunter/config.toml",
        crate::CONFIG_TOML
            .replace("#device = \"orbic\"", "device = \"wingtech\"")
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

    println!("Rebooting device and waiting 30 seconds for it to start up.");
    telnet_send_command(addr, "shutdown -r -t 1 now", "exit code 0", true).await?;
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

#[test]
fn test_encrypt_password() {
    let p = b"80536913";
    let s = encrypt_password(p).ok();
    let expected = Some("5brvd8xl732cSoFTAy67ig==".to_string());
    assert_eq!(s, expected);
}
