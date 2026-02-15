use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use nusb::Device;
use reqwest::Client;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

use crate::output::{print, println};

#[cfg(unix)]
use std::os::fd::AsRawFd;

pub async fn telnet_send_command_with_output(
    addr: SocketAddr,
    command: &str,
    wait_for_prompt: bool,
) -> Result<String> {
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
    let string = String::from_utf8_lossy(&read_buf).to_string();
    Ok(string)
}

pub async fn telnet_send_command(
    addr: SocketAddr,
    command: &str,
    expected_output: &str,
    wait_for_prompt: bool,
) -> Result<()> {
    let output = telnet_send_command_with_output(addr, command, wait_for_prompt).await?;
    if !output.contains(expected_output) {
        bail!("{expected_output:?} not found in: {output}");
    }
    Ok(())
}

pub async fn wait_for_telnet(addr: SocketAddr) -> Result<()> {
    let timeout = Duration::from_secs(60);
    let start_time = std::time::Instant::now();

    while telnet_send_command(addr, "true", "exit code 0", false)
        .await
        .is_err()
    {
        if start_time.elapsed() >= timeout {
            bail!(
                "Timeout waiting for shell to become available after {:?}",
                timeout
            );
        }
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

pub async fn telnet_send_file(
    addr: SocketAddr,
    filename: &str,
    payload: &[u8],
    wait_for_prompt: bool,
) -> Result<()> {
    print!("Sending file {filename} ... ");
    let nc_output = {
        let filename = filename.to_owned();
        let handle = tokio::spawn(async move {
            telnet_send_command_with_output(
                addr,
                &format!("nc -l -p 8081 >{filename}.tmp"),
                wait_for_prompt,
            )
            .await
        });

        let mut addr = addr;
        addr.set_port(8081);

        let mut stream;
        let mut attempts = 0;

        loop {
            // wait for nc to become available, with exponential backoff.
            //
            // if the installer fails with connection refused, this
            // likely is not high enough.
            sleep(Duration::from_millis(100 * (1 << attempts))).await;

            stream = TcpStream::connect(addr).await;
            attempts += 1;
            if stream.is_ok() || attempts > 3 {
                break;
            }

            print!("attempt {attempts}... ");
        }

        {
            let mut stream = stream?;
            stream.write_all(payload).await?;

            // if the orbic is sluggish, we need for nc to write the data to disk before
            // terminating the connection. if we terminate the connection while there is unflushed
            // data, that data will just not be written from nc's buffer into OS disk buffer. the
            // symptom is mismatched md5 hashes.
            //
            // this is NOT fixed by calling fsync or similar, we're talking about dropped
            // application buffers here.
            sleep(Duration::from_millis(1000)).await;

            // ensure that stream is dropped before we wait for nc to terminate.
            drop(stream);
        }

        handle.await??
    };

    let checksum = md5::compute(payload);
    telnet_send_command(
        addr,
        &format!("md5sum {filename}.tmp"),
        &format!("{checksum:x}  {filename}.tmp"),
        wait_for_prompt,
    )
    .await
    .with_context(|| {
        format!(
            "File transfer failed. nc command output: '{}'. Expected checksum: {:x}",
            nc_output.trim(),
            checksum
        )
    })?;

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
#[cfg(not(target_os = "android"))]
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

/// Open an interactive shell to a device
///
/// Connects to a shell service on the device and forwards stdin/stdout bidirectionally.
pub async fn interactive_shell(admin_ip: &str, shell_port: u16, raw_mode: bool) -> Result<()> {
    let shell_addr = SocketAddr::from_str(&format!("{admin_ip}:{shell_port}"))?;
    let mut stream = TcpStream::connect(shell_addr)
        .await
        .context("Failed to connect to shell. Make sure the device is reachable.")?;

    let stdin = tokio::io::stdin();

    #[cfg(unix)]
    let raw_terminal_guard = if raw_mode {
        Some(RawTerminal::new(stdin.as_raw_fd())?)
    } else {
        None
    };

    // suppress "unused variable" lint
    #[cfg(not(unix))]
    let _used = raw_mode;

    let mut stdio = tokio::io::join(stdin, tokio::io::stdout());
    let _ = tokio::io::copy_bidirectional(&mut stream, &mut stdio).await;

    // hitting ctrl-d will not print a trailing newline on tplink at least, which messes up the
    // next prompt
    println!();

    // The current_thread runtime in tokio will block forever until stdin receives a read error. To
    // work around this cleanup issue we just exit directly from here.
    //
    // This is documented as a flaw in tokio::io::stdin()'s own docs, but the recommended
    // workaround to spawn your own OS thread doesn't work.
    //
    // For some reason this only happens when the terminal is being put in raw mode (removing
    // RawTerminal fixes it)
    //
    // We have to drop the RawTerminal guard before exiting, otherwise we will
    // mess up the terminal.
    #[cfg(unix)]
    drop(raw_terminal_guard);
    std::process::exit(0)
}

#[cfg(unix)]
struct RawTerminal {
    fd: std::os::fd::RawFd,
    original_termios: termios::Termios,
}

#[cfg(unix)]
impl RawTerminal {
    fn new(fd: std::os::fd::RawFd) -> Result<Self> {
        // put terminal in raw mode so that arrow keys, tab etc are correctly forwarded to the
        // device's shell
        let original_termios = termios::Termios::from_fd(fd)?;
        let mut new_termios = original_termios;

        // set flags on the struct
        termios::cfmakeraw(&mut new_termios);

        // apply changes
        termios::tcsetattr(fd, termios::TCSANOW, &new_termios)?;

        Ok(RawTerminal {
            fd,
            original_termios,
        })
    }
}

#[cfg(unix)]
impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = termios::tcsetattr(self.fd, termios::TCSANOW, &self.original_termios);
    }
}
