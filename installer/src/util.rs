use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use nusb::Device;
use reqwest::Client;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

macro_rules! echo {
    ($($arg:tt)*) => {
        print!($($arg)*);
        let _ = std::io::stdout().flush();
    };
}
pub(crate) use echo;

pub async fn telnet_send_command(
    addr: SocketAddr,
    command: &str,
    expected_output: &str,
    wait_for_prompt: bool,
) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.into_split();

    if wait_for_prompt {
        // Wait for initial '#' prompt from telnetd
        loop {
            let mut next_byte = 0;
            reader
                .read_exact(std::slice::from_mut(&mut next_byte))
                .await?;
            if next_byte == b'#' {
                break;
            }
        }
    }

    writer.write_all(command.as_bytes()).await?;
    // by quoting the 'exit' here, we ensure that we do not read our own command line back as
    // "output" before we even hit enter, but the actual result of executing the echo.
    writer
        .write_all(b"; echo command done, 'exit' code $?\r\n")
        .await?;
    let mut read_buf = Vec::new();
    let _ = timeout(Duration::from_secs(10), async {
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

            // when we see this string we know the command is done and can terminate.
            // even if we sent command; exit, certain "telnet-like" shells (like nc contraptions)
            // may not terminate the connection appropriately on their own.
            let response = String::from_utf8_lossy(&read_buf);
            if response.contains("command done, exit code ") {
                break;
            }
        }
    })
    .await;
    let string = String::from_utf8_lossy(&read_buf);
    if !string.contains(expected_output) {
        bail!("{expected_output:?} not found in: {string}");
    }
    Ok(())
}

pub async fn telnet_send_file(
    addr: SocketAddr,
    filename: &str,
    payload: &[u8],
    wait_for_prompt: bool,
) -> Result<()> {
    echo!("Sending file {filename} ... ");
    {
        let filename = filename.to_owned();
        let handle = tokio::spawn(async move {
            telnet_send_command(
                addr,
                &format!("nc -l -p 8081 >{filename}.tmp"),
                "",
                wait_for_prompt,
            )
            .await
        });
        sleep(Duration::from_millis(100)).await;
        let mut addr = addr;
        addr.set_port(8081);

        {
            let mut stream = TcpStream::connect(addr).await?;
            stream.write_all(payload).await?;
            // ensure that stream is dropped before we wait for nc to terminate!
        }

        handle.await??;
    }
    let checksum = md5::compute(payload);
    telnet_send_command(
        addr,
        &format!("md5sum {filename}.tmp"),
        &format!("{checksum:x}  {filename}.tmp"),
        wait_for_prompt,
    )
    .await?;
    telnet_send_command(
        addr,
        &format!("mv {filename}.tmp {filename}"),
        "exit code 0",
        wait_for_prompt,
    )
    .await?;
    println!("ok");
    Ok(())
}

pub async fn send_file(admin_ip: &str, local_path: &str, remote_path: &str) -> Result<()> {
    let file_content = std::fs::read(local_path)
        .with_context(|| format!("Failed to read local file: {local_path}"))?;

    println!("Connecting to {admin_ip}");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23"))
        .with_context(|| format!("Invalid IP address: {admin_ip}"))?;

    telnet_send_file(addr, remote_path, &file_content, true)
        .await
        .with_context(|| format!("Failed to send file {local_path} to {remote_path}"))?;

    println!("Successfully sent {local_path} to {remote_path}");
    Ok(())
}

pub async fn http_ok_every(
    rayhunter_url: String,
    interval: Duration,
    max_failures: u32,
) -> Result<()> {
    let client = Client::new();
    let mut failures = 0;
    loop {
        match client.get(&rayhunter_url).send().await {
            Ok(test) => match test.status().is_success() {
                true => break,
                false => bail!(
                    "request for url ({rayhunter_url}) failed with status code: {:?}",
                    test.status()
                ),
            },
            Err(e) => match failures > max_failures {
                true => return Err(e.into()),
                false => failures += 1,
            },
        }
        sleep(interval).await;
    }
    Ok(())
}

/// General function to open a USB device
pub fn open_usb_device(vid: u16, pid: u16) -> Result<Option<Device>> {
    let devices = match nusb::list_devices() {
        Ok(d) => d,
        Err(_) => return Ok(None),
    };
    for device in devices {
        if device.vendor_id() == vid && device.product_id() == pid {
            match device.open() {
                Ok(d) => return Ok(Some(d)),
                Err(e) => bail!("device found but failed to open: {}", e),
            }
        }
    }
    Ok(None)
}
