use std::io::Write;
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{Result, bail};
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
) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.into_split();
    loop {
        let mut next_byte = 0;
        reader
            .read_exact(std::slice::from_mut(&mut next_byte))
            .await?;
        if next_byte == b'#' {
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
        bail!("{expected_output:?} not found in: {string}");
    }
    Ok(())
}

pub async fn telnet_send_file(addr: SocketAddr, filename: &str, payload: &[u8]) -> Result<()> {
    echo!("Sending file {filename} ... ");
    {
        let filename = filename.to_owned();
        let handle = tokio::spawn(async move {
            telnet_send_command(addr, &format!("nc -l -p 8081 >{filename}.tmp"), "").await
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
    println!("ok");
    Ok(())
}
