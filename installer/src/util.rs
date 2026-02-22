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
    if command.contains('\n') {
        bail!("multi-line commands are not allowed");
    }

    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.into_split();

    if wait_for_prompt {
        // Wait for the shell prompt. This also consumes any telnet IAC negotiation
        // the server sends at connection start, and ensures the shell is ready
        // for input.
        while reader.read_u8().await? != b'#' {}
    }

    // This contraption is there so we clearly know where the command output starts and ends,
    // skipping telnet echoing the command back using START, and terminating the connection right
    // after the command exits.
    //
    // 'TELNET' is quoted so that when the command gets echoed back, it does not match against
    // RAYHUNTER_TELNET_COMMAND_DONE search string.
    writer.write_all(format!("echo RAYHUNTER_'TELNET'_COMMAND_START; {command}; echo RAYHUNTER_'TELNET'_COMMAND_DONE\r\n").as_bytes()).await?;

    let mut read_buf = Vec::new();
    let _ = timeout(Duration::from_secs(10), async {
        loop {
            let Ok(byte) = reader.read_u8().await else {
                break;
            };
            read_buf.push(byte);

            // when we see this string we know the command is done and can terminate.
            // even if we sent command; exit, certain "telnet-like" shells (like nc contraptions)
            // may not terminate the connection appropriately on their own.
            if byte == b'\n' {
                let response = String::from_utf8_lossy(&read_buf);
                if response.contains("RAYHUNTER_TELNET_COMMAND_DONE") {
                    break;
                }
            }
        }
    })
    .await;
    let string = String::from_utf8_lossy(&read_buf);
    let start = string.rfind("RAYHUNTER_TELNET_COMMAND_START");
    let end = string.rfind("RAYHUNTER_TELNET_COMMAND_DONE");
    let string = match (start, end) {
        (Some(start), Some(end)) => {
            // skip past the START marker and the trailing \r\n of the echoed command line
            let start = start + "RAYHUNTER_TELNET_COMMAND_START".len();
            string[start..end].trim_start_matches(['\r', '\n'])
        }
        _ => bail!("failed to parse command output from string: {string:?}"),
    };
    Ok(string.to_string())
}

pub async fn telnet_send_command(
    addr: SocketAddr,
    command: &str,
    expected_output: &str,
    wait_for_prompt: bool,
) -> Result<()> {
    let command = format!("{command}; echo command done, exit code $?");
    let output = telnet_send_command_with_output(addr, &command, wait_for_prompt).await?;
    if !output.contains(expected_output) {
        bail!("{expected_output:?} not found in: {output}");
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
