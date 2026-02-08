/// Installer for ZTE MF920V hotspots using ADB (via USB_MODE_SWITCH with AD token).
///
/// Uses the legitimate USB_MODE_SWITCH goform with an AD token to enable ADB,
/// then installs rayhunter over ADB. Works on both unbranded and Drei-branded MF920V.
///
/// Note: MF920U login and AD token computation work, but USB_MODE_SWITCH returns
/// failure on all tested firmware — ADB cannot be enabled. No known exploit.
///
/// Exploit chain:
/// 1. Login via HTTP (base64 password encoding)
/// 2. Fetch wa_inner_version, cr_version, RD tokens
/// 3. Compute AD = md5(md5(wa_inner_version + cr_version) + RD)
/// 4. Enable ADB: POST USB_MODE_SWITCH with usb_mode=6 and AD token
/// 5. Connect via ADB and install rayhunter
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice};
use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::ZteArgs as Args;
use crate::connection::{DeviceConnection, install_config};
use crate::output::{print, println};

#[derive(Deserialize)]
struct TokenResponse {
    wa_inner_version: String,
    cr_version: String,
    #[serde(rename = "RD")]
    rd: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    result: String,
}

/// ADB-based connection wrapper implementing DeviceConnection trait
struct AdbConnection<'a> {
    device: &'a mut ADBUSBDevice,
}

impl DeviceConnection for AdbConnection<'_> {
    async fn run_command(&mut self, command: &str) -> Result<String> {
        let mut buf = Vec::new();
        self.device
            .shell_command(&["sh", "-c", command], &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    async fn write_file(&mut self, path: &str, content: &[u8]) -> Result<()> {
        let mut payload: &[u8] = content;
        self.device
            .push(&mut payload, &path)
            .context("ADB push failed")?;
        Ok(())
    }
}

/// Wait for an ADB device to appear on USB and return it.
async fn wait_for_adb() -> Result<ADBUSBDevice> {
    const MAX_ATTEMPTS: u32 = 30;

    // Wait for USB re-enumeration after mode switch
    sleep(Duration::from_secs(3)).await;

    for attempt in 1..=MAX_ATTEMPTS {
        match ADBUSBDevice::autodetect() {
            Ok(mut device) => {
                let mut buf = Vec::new();
                if device
                    .shell_command(&["echo", "adb_ready"], &mut buf)
                    .is_ok()
                {
                    let output = String::from_utf8_lossy(&buf);
                    if output.contains("adb_ready") {
                        return Ok(device);
                    }
                }
            }
            Err(_) if attempt < MAX_ATTEMPTS => {}
            Err(e) => bail!("Failed to connect to ADB device: {e}"),
        }

        sleep(Duration::from_secs(1)).await;
    }

    bail!("Timeout waiting for ADB device after {MAX_ATTEMPTS} seconds")
}

fn build_client(admin_ip: &str) -> Result<Client> {
    let referer = format!("http://{admin_ip}/index.html");
    Client::builder()
        .cookie_store(true)
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::REFERER,
                referer.parse().context("invalid referer header")?,
            );
            headers
        })
        .build()
        .context("Failed to build HTTP client")
}

/// Perform the full login + AD token + enable ADB sequence.
async fn login_and_enable_adb(admin_ip: &str, admin_password: &str) -> Result<()> {
    let client = build_client(admin_ip)?;

    print!("Logging in to {admin_ip} ... ");
    let encoded_password = base64_light::base64_encode_bytes(admin_password.as_bytes());
    let login_result: LoginResponse = client
        .post(format!("http://{admin_ip}/goform/goform_set_cmd_process"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "isTest=false&goformId=LOGIN&password={encoded_password}"
        ))
        .send()
        .await
        .context("Failed to send login request")?
        .error_for_status()
        .context("Error from login request")?
        .json()
        .await
        .context("Error parsing login response")?;

    if login_result.result != "0" {
        bail!(
            "Bad login response {}. Maybe the password is wrong?",
            login_result.result
        );
    }
    println!("ok");

    // Fetch tokens
    print!("Fetching tokens ... ");
    let token_body = client
        .get(format!(
            "http://{admin_ip}/goform/goform_get_cmd_process?cmd=wa_inner_version,cr_version,RD&multi_data=1"
        ))
        .send()
        .await
        .context("Failed to fetch tokens")?
        .error_for_status()
        .context("Error fetching tokens")?
        .text()
        .await
        .context("Failed to read token response body")?;

    let tokens: TokenResponse = serde_json::from_str(&token_body)
        .with_context(|| format!("Error parsing token response: {token_body}"))?;

    if tokens.rd.is_empty() {
        bail!("RD token is empty. Raw response: {token_body}");
    }
    println!("ok");

    // Compute AD token and enable ADB via USB_MODE_SWITCH
    print!("Computing AD token and enabling ADB ... ");
    let inner = format!("{}{}", tokens.wa_inner_version, tokens.cr_version);
    let inner_md5 = format!("{:x}", md5::compute(inner.as_bytes()));
    let outer = format!("{}{}", inner_md5, tokens.rd);
    let ad = format!("{:x}", md5::compute(outer.as_bytes()));

    let switch_resp = client
        .post(format!("http://{admin_ip}/goform/goform_set_cmd_process"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "isTest=false&goformId=USB_MODE_SWITCH&usb_mode=6&AD={ad}"
        ))
        .send()
        .await
        .context("Failed to send USB_MODE_SWITCH command")?;

    let status = switch_resp.status();
    let body = switch_resp
        .text()
        .await
        .context("Failed to read USB_MODE_SWITCH response")?;

    if !status.is_success() {
        bail!("USB_MODE_SWITCH failed with status {status}: {body}");
    }
    if !body.contains("success") {
        bail!("USB_MODE_SWITCH did not succeed. Response: {body}");
    }
    println!("ok");

    Ok(())
}

/// Install rayhunter on the device over ADB.
async fn install_rayhunter(
    conn: &mut AdbConnection<'_>,
    admin_ip: &str,
    reset_config: bool,
) -> Result<()> {
    print!("Remounting root filesystem read-write ... ");
    conn.run_command("mount -o remount,rw /").await?;
    println!("ok");

    print!("Setting up directories ... ");
    conn.run_command("mkdir -p /cache/rayhunter").await?;
    conn.run_command("mkdir -p /data").await?;
    conn.run_command("rm -f /data/rayhunter").await?;
    conn.run_command("ln -sf /cache/rayhunter /data/rayhunter")
        .await?;
    println!("ok");

    print!("Sending rayhunter daemon binary ... ");
    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    conn.write_file("/data/rayhunter/rayhunter-daemon", rayhunter_daemon_bin)
        .await?;
    conn.run_command("chmod 755 /data/rayhunter/rayhunter-daemon")
        .await?;
    println!("ok");

    install_config(conn, "zte", reset_config).await?;

    print!("Sending init script ... ");
    conn.write_file(
        "/etc/init.d/rayhunter_daemon",
        crate::RAYHUNTER_DAEMON_INIT.as_bytes(),
    )
    .await?;
    conn.run_command("chmod 755 /etc/init.d/rayhunter_daemon")
        .await?;
    println!("ok");

    print!("Registering autostart ... ");
    conn.run_command("ln -sf ../init.d/rayhunter_daemon /etc/rc5.d/S99rayhunter_daemon")
        .await?;
    println!("ok");

    println!(
        "Done. Rebooting device. After it's started up again, check out the web interface at http://{admin_ip}:8080"
    );
    conn.run_command("reboot").await?;

    Ok(())
}

/// Main installation function
pub async fn install(
    Args {
        admin_ip,
        admin_password,
        reset_config,
    }: Args,
) -> Result<()> {
    println!("Installing rayhunter on ZTE MF920V via ADB");

    login_and_enable_adb(&admin_ip, &admin_password).await?;

    print!("Waiting for ADB device ... ");
    let mut adb_device = wait_for_adb().await?;
    println!("ok");

    let mut conn = AdbConnection {
        device: &mut adb_device,
    };
    install_rayhunter(&mut conn, &admin_ip, reset_config).await?;

    Ok(())
}

/// Enable ADB on the ZTE device without installing rayhunter.
pub async fn start_adb(admin_ip: &str, admin_password: &str) -> Result<()> {
    login_and_enable_adb(admin_ip, admin_password).await?;

    print!("Waiting for ADB device ... ");
    let _adb_device = wait_for_adb().await?;
    println!("ok");

    println!("ADB is now available. You can connect with: adb shell");
    Ok(())
}

/// Open an interactive ADB shell on the ZTE device.
pub async fn shell(admin_ip: &str, admin_password: &str) -> Result<()> {
    login_and_enable_adb(admin_ip, admin_password).await?;

    print!("Waiting for ADB device ... ");
    let mut adb_device = wait_for_adb().await?;
    println!("ok");

    #[cfg(unix)]
    let _raw = crate::util::RawTerminal::new(std::os::fd::AsRawFd::as_raw_fd(&std::io::stdin()))?;

    adb_device.shell(&mut std::io::stdin(), Box::new(std::io::stdout()))?;
    Ok(())
}
