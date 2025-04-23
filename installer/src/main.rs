use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Error};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install rayhunter on the TP-Link M7350.
    InstallTplink(InstallTpLink),
}

#[derive(Parser, Debug)]
struct InstallTpLink {
    /// Do not enforce use of SD card. All data will be stored in /mnt/card regardless, which means
    /// that if an SD card is later added, your existing installation is shadowed!
    #[arg(long)]
    skip_sdcard: bool,

    /// Username for TP-Link admin interface, if custom.
    #[arg(long, default_value = "admin")]
    username: String,

    /// Password for TP-Link admin interface, if custom.
    #[arg(long, default_value = "admin")]
    password: String,

    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args { command } = Args::parse();

    match command {
        Command::InstallTplink(tplink) => main_tplink(tplink).await.context("Failed to install rayhunter on the TP-Link M7350. Make sure your computer is connected to the hotspot using USB tethering or WiFi. Currently only Hardware Revision v3 is supported.")?,
    }

    Ok(())
}

async fn main_tplink(args: InstallTpLink) -> Result<(), Error> {
    let InstallTpLink {
        skip_sdcard,
        username,
        password,
        admin_ip,
    } = args;

    let qcmap_auth_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_auth");
    let qcmap_web_cgi_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_web_cgi");

    #[derive(Deserialize)]
    struct NonceResponse {
        nonce: String,
    }

    let client = reqwest::Client::new();

    let NonceResponse { nonce } = client
        .post(&qcmap_auth_endpoint)
        .body(r#"{"module":"authenticator","action":0}"#)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    println!("Successfully detected TP-Link M7350 v3, nonce: {nonce}");

    let digest_md5 = md5::compute(format!("{username}:{password}:{nonce}").as_bytes());

    #[derive(Deserialize)]
    struct TokenResponse {
        token: String,
    }

    let TokenResponse { token } = client
        .post(&qcmap_auth_endpoint)
        .json(&json!({
            "module": "authenticator",
            "action": 1,
            "digest": format!("{:x}", digest_md5)
        }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    println!("Got token: {token}");

    for language in &["$(busybox telnetd -l /bin/sh)", "en"] {
        println!("Setting language of device to {language}");
        client
            .post(&qcmap_web_cgi_endpoint)
            .header("Cookie", format!("tpweb_token={token}"))
            .json(&json!({
                "token": token,
                "module": "webServer",
                "action": 1,
                "language": language
            }))
            .send()
            .await?
            .error_for_status()?;
    }

    println!("Connecting via telnet to {admin_ip}");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();

    if !skip_sdcard {
        println!("Mounting sdcard");
        telnet_send_command(addr, "mount /dev/mmcblk0p1 /mnt/card", "exit code 0").await.context("Rayhunter needs a FAT-formatted SD card to function for more than a few minutes. Insert one and rerun this installer, or pass --skip-sdcard")?;
    }

    // there is too little space on the internal flash to store anything, but the initrd script
    // expects things to be at this location
    telnet_send_command(addr, "rm -rf /data/rayhunter", "exit code 0").await?;
    telnet_send_command(addr, "mkdir -p /data", "exit code 0").await?;
    telnet_send_command(addr, "ln -sf /mnt/card /data/rayhunter", "exit code 0").await?;

    telnet_send_file(
        addr,
        "/mnt/card/config.toml",
        include_bytes!("../../dist/config.toml.example"),
    )
    .await?;

    #[cfg(feature = "vendor")]
    let rayhunter_daemon_bin =
        include_bytes!("../../rayhunter-daemon-tplink/rayhunter-daemon");

    #[cfg(not(feature = "vendor"))]
    let rayhunter_daemon_bin =
        &tokio::fs::read("target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon").await?;

    telnet_send_file(addr, "/mnt/card/rayhunter-daemon", rayhunter_daemon_bin).await?;
    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        include_bytes!("../../dist/scripts/rayhunter_daemon"),
    )
    .await?;

    telnet_send_command(
        addr,
        "chmod ugo+x /mnt/card/rayhunter-daemon",
        "exit code 0",
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
    )
    .await?;
    telnet_send_command(addr, "update-rc.d rayhunter_daemon defaults", "exit code 0").await?;

    println!(
        "Done. Rebooting device. After it's started up again, check out the web interface at http://{admin_ip}:8080"
    );

    telnet_send_command(addr, "reboot", "exit code 0").await?;

    Ok(())
}

async fn telnet_send_file(addr: SocketAddr, filename: &str, payload: &[u8]) -> Result<(), Error> {
    println!("Sending file {filename}");

    // remove the old file just in case we are close to disk capacity.
    telnet_send_command(addr, &format!("rm {filename}"), "").await?;

    {
        let filename = filename.to_owned();
        let handle = tokio::spawn(async move {
            telnet_send_command(addr, &format!("nc -l 0.0.0.0:8081 > {filename}.tmp"), "").await
        });

        sleep(Duration::from_millis(100)).await;

        let mut addr = addr;
        addr.set_port(8081);
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(payload).await?;

        handle.await??;
    }

    let checksum = md5::compute(payload);

    telnet_send_command(
        addr,
        &format!("md5sum {filename}.tmp"),
        &format!("{checksum:x}  {filename}.tmp"),
    )
    .await?;

    telnet_send_command(
        addr,
        &format!("mv {filename}.tmp {filename}"),
        "exit code 0",
    )
    .await?;

    Ok(())
}

async fn telnet_send_command(
    addr: SocketAddr,
    command: &str,
    expected_output: &str,
) -> Result<(), Error> {
    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.into_split();

    loop {
        let mut buf = [0];
        reader.read(&mut buf).await?;
        if buf[0] == b'#' {
            break;
        }
    }

    writer.write_all(command.as_bytes()).await?;
    writer.write_all(b"; echo exit code $?\r\n").await?;

    let mut read_buf = Vec::new();

    let _ = timeout(Duration::from_secs(5), async {
        let mut buf = [0; 4096];
        loop {
            let Ok(bytes_read) = reader.read(&mut buf).await else {
                break;
            };
            let bytes = &buf[..bytes_read];
            if bytes.is_empty() {
                continue;
            }

            read_buf.extend(bytes);

            if read_buf.ends_with(b"/ # ") {
                break;
            }
        }
    })
    .await;

    let string = String::from_utf8_lossy(&read_buf);

    if !string.contains(expected_output) {
        anyhow::bail!("{expected_output:?} not found in: {string}");
    }

    Ok(())
}
