use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};
use tokio::time::{sleep, timeout};

#[derive(Clone)]
pub(super) struct Connection {
    pub admin: Ipv4Addr,
    pub local: Ipv4Addr,
    pub interface: Option<String>,
}

impl Connection {
    pub async fn connect(&self, port: u16) -> Result<TcpStream> {
        #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "fuchsia")))]
        ensure!(
            self.interface.is_none(),
            "Explicit interface binding is not supported on this platform"
        );
        let socket = TcpSocket::new_v4()?;
        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        if let Some(interface) = &self.interface {
            socket
                .bind_device(Some(interface.as_bytes()))
                .context("Cannot bind USB interface; check its name and permissions")?;
        }
        socket.bind(SocketAddr::from((self.local, 0)))?;
        Ok(timeout(
            Duration::from_secs(5),
            socket.connect((self.admin, port).into()),
        )
        .await
        .context("MR1100 connection timed out")??)
    }

    pub async fn at(&self, command: &str) -> Result<String> {
        ensure!(!command.contains(['\r', '\n']), "Invalid AT command");
        let mut stream = timeout(Duration::from_secs(15), async {
            loop {
                if let Ok(stream) = self.connect(5510).await {
                    break stream;
                }
                sleep(Duration::from_millis(300)).await;
            }
        })
        .await
        .context("MR1100 AT service unavailable on the selected USB interface")?;
        timeout(Duration::from_secs(8), async {
            stream
                .write_all(format!("{command}\r\n").as_bytes())
                .await?;
            let mut output = Vec::new();
            loop {
                let byte = stream.read_u8().await?;
                output.push(byte);
                ensure!(output.len() < 16384, "AT response too large");
                let text = String::from_utf8_lossy(&output);
                // A previous TCP client can leave a delayed modem reply behind.
                // The tested AT bridge echoes commands; accept only our own reply.
                if let Some(response) = at_reply(&text, command) {
                    return Ok(response.to_owned());
                }
            }
        })
        .await
        .context("MR1100 AT command timed out")?
    }

    pub async fn at_ok(&self, command: &str) -> Result<()> {
        let output = self.at(command).await?;
        ensure!(
            output.lines().any(|line| line.trim() == "OK"),
            "MR1100 rejected AT command"
        );
        Ok(())
    }

    pub async fn run(&self, command: &str) -> Result<String> {
        ensure!(
            command.lines().all(|line| line.len() <= 200),
            "MR1100 shell line exceeds 200 bytes"
        );
        let mut stream = self.connect(23).await?;
        timeout(Duration::from_secs(75), async {
            let mut greeting = Vec::new();
            while !greeting.ends_with(b"# ") {
                greeting.push(telnet_byte(&mut stream).await?);
                ensure!(greeting.len() < 16384, "Root shell prompt not found");
            }
            stream.write_all(b"stty -echo\nPS1='' PS2=''\nprintf '\\nRH_READY\\n'\n").await?;
            let mut ready = Vec::new();
            while !ready.ends_with(b"RH_READY\r\n") && !ready.ends_with(b"RH_READY\n") {
                ready.push(telnet_byte(&mut stream).await?);
                ensure!(ready.len() < 16384, "Shell setup failed");
            }
            // Separate short lines avoid the stock shell's input-line truncation.
            let request = format!("echo RH_'BEGIN'\n(\nset -e\n{command}\n)\nr=$?\nprintf '\\nRH_EXIT:%s\\n' \"$r\"\necho RH_'END'\r\n");
            stream.write_all(request.as_bytes()).await?;
            let mut bytes = Vec::new();
            loop {
                bytes.push(telnet_byte(&mut stream).await?);
                ensure!(bytes.len() <= 2 * 1024 * 1024, "Shell response too large");
                if bytes.ends_with(b"\nRH_END\r\n") || bytes.ends_with(b"\nRH_END\n") {
                    return parse_shell_output(&String::from_utf8_lossy(&bytes));
                }
            }
        })
        .await
        .context("MR1100 shell command timed out")?
    }

    pub async fn write(&self, path: &str, bytes: &[u8]) -> Result<()> {
        ensure!(
            path.bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"/._-".contains(&c)),
            "Unsafe remote path"
        );
        let receiver = self.clone();
        let command =
            format!("umask 077\ntimeout -t 60 -s KILL nc -l -p 8081 </dev/null > {path}.tmp");
        let task = tokio::spawn(async move { receiver.run(&command).await });
        let send: Result<()> = async {
            let mut connected = None;
            for _ in 0..10 {
                sleep(Duration::from_millis(250)).await;
                if let Ok(stream) = self.connect(8081).await {
                    connected = Some(stream);
                    break;
                }
            }
            let mut stream = connected.context("File receiver did not start")?;
            timeout(Duration::from_secs(45), async {
                stream.write_all(bytes).await?;
                stream.shutdown().await
            })
            .await
            .context("File transfer timed out")??;
            Ok(())
        }
        .await;
        let received = task.await.context("File receiver task failed")?;
        send?;
        received?;
        let expected = format!("{:x}", Sha256::digest(bytes));
        let output = self.run(&format!("sha256sum {path}.tmp")).await?;
        ensure!(
            output.split_whitespace().next() == Some(expected.as_str()),
            "Transferred file checksum mismatch"
        );
        self.run(&format!("mv {path}.tmp {path}")).await?;
        Ok(())
    }
}

fn at_reply<'a>(text: &'a str, command: &str) -> Option<&'a str> {
    let start = text.rfind(&format!("{command}\r"))?;
    let response = &text[start + command.len()..];
    (response.contains("\r\nOK\r\n") || response.contains("\r\nERROR\r\n"))
        .then_some(&text[start..])
}

async fn telnet_byte(stream: &mut TcpStream) -> Result<u8> {
    loop {
        let byte = stream.read_u8().await?;
        if byte != 255 {
            return Ok(byte);
        }
        match stream.read_u8().await? {
            255 => return Ok(255),
            op @ (251..=254) => {
                let option = stream.read_u8().await?;
                if op == 251 {
                    stream.write_all(&[255, 254, option]).await?;
                }
                if op == 253 {
                    stream.write_all(&[255, 252, option]).await?;
                }
            }
            250 => {
                let mut previous = 0;
                loop {
                    let next = stream.read_u8().await?;
                    if previous == 255 && next == 240 {
                        break;
                    }
                    previous = next;
                }
            }
            _ => {}
        }
    }
}

fn parse_shell_output(text: &str) -> Result<String> {
    let lines: Vec<_> = text
        .lines()
        .map(|line| line.trim_end_matches('\r'))
        .collect();
    let start = lines
        .iter()
        .position(|line| *line == "RH_BEGIN")
        .context("Missing shell start marker")?;
    let end = lines
        .iter()
        .rposition(|line| *line == "RH_END")
        .context("Missing shell end marker")?;
    let status = lines[..end]
        .iter()
        .rposition(|line| line.starts_with("RH_EXIT:"))
        .context("Missing shell exit status")?;
    ensure!(start < status, "Invalid shell markers");
    if lines[status] != "RH_EXIT:0" {
        bail!("MR1100 shell command failed ({})", lines[status]);
    }
    Ok(lines[start + 1..status].join("\n").trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delayed_at_reply_cannot_complete_a_new_command() {
        assert!(at_reply("AT!CUSTOM?\r\r\nOK\r\nATI\r\r\nModel: MR1100\r\n", "ATI").is_none());
        assert_eq!(
            at_reply("old\r\nOK\r\nATI\r\r\nModel: MR1100\r\nOK\r\n", "ATI"),
            Some("ATI\r\r\nModel: MR1100\r\nOK\r\n")
        );
    }

    #[test]
    fn shell_requires_real_markers_and_success() {
        assert_eq!(
            parse_shell_output("echo RH_'BEGIN'\r\nRH_BEGIN\r\nvalue\r\nRH_EXIT:0\r\nRH_END\r\n")
                .unwrap(),
            "value"
        );
        assert!(parse_shell_output("RH_BEGIN\nRH_EXIT:1\nRH_END\n").is_err());
        assert!(parse_shell_output("echo RH_BEGIN; echo RH_EXIT:0; echo RH_END").is_err());
    }
}
