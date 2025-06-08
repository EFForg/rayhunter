/// Installer for the Wingtech CT2MHS01 hotspot.
///
/// Tested on (from `/etc/wt_version`):
///   WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
///   WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
///   WT_HARDWARE_VERSION=89323_1_20
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use anyhow::{Result, bail};
use base64_light::base64_encode_bytes;
use block_padding::{Padding, Pkcs7};
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::InstallWingtech as Args;
use crate::orbic::echo;
use crate::tplink::telnet_send_command;

pub async fn install(
    Args {
        admin_ip,
        admin_password,
    }: Args,
) -> Result<()> {
    wingtech_run_install(admin_ip, admin_password).await
}

const KEY: &[u8] = b"abcdefghijklmn12";

/// Returns password encrypted in AES128 ECB mode with the key b"abcdefghijklmn12",
/// with Pkcs7 padding, encoded in base64.
fn encrypt_password(password: &[u8]) -> Result<String> {
    let c = Aes128::new_from_slice(KEY)?;
    let mut b = GenericArray::from([0u8; 16]);
    b[..password.len()].copy_from_slice(password);
    Pkcs7::pad(&mut b, password.len());
    c.encrypt_block(&mut b);
    Ok(base64_encode_bytes(&b))
}

pub async fn start_telnet(admin_ip: &str, admin_password: &str) -> Result<bool> {
    let qcmap_auth_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_auth");
    let qcmap_web_cgi_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_web_cgi");

    let encrypted_pw = encrypt_password(admin_password.as_bytes()).ok().unwrap();

    let client = Client::new();
    let login = client
        .post(&qcmap_auth_endpoint)
        .body(format!(
            "type=login&pwd={encrypted_pw}&timeout=60000&user=admin"
        ))
        .send()
        .await?
        .text()
        .await?;
    let token = match login.find("token") {
        Some(n) => &login[n + 8..n + 8 + 16],
        None => bail!("login did not return a token in response: {}", login),
    };

    let cmd = "busybox telnetd -l /bin/sh";
    let telnet = client.post(&qcmap_web_cgi_endpoint)
        .body(format!("page=setFWMacFilter&cmd=add&mode=0&mac=50:5A:CA:B5:05:AC||{cmd}&key=50:5A:CA:B5:05:AC&token={token}"))
        .send()
        .await?;
    if telnet.status() != 200 {
        bail!(
            "starting telnet failed with status code: {:?}",
            telnet.status()
        );
    }

    Ok(true)
}

async fn wingtech_run_install(admin_ip: String, admin_password: String) -> Result<()> {
    echo!("Starting telnet ... ");
    start_telnet(&admin_ip, &admin_password).await?;
    println!("ok");

    echo!("Connecting via telnet to {admin_ip} ... ");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();
    println!("ok");

    telnet_send_command(addr, "mkdir -p /data/rayhunter", "exit code 0").await?;

    telnet_send_file(
        addr,
        "/data/rayhunter/config.toml",
        crate::CONFIG_TOML.as_bytes(),
    )
    .await?;

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON_WINGTECH"));
    telnet_send_file(
        addr,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /data/rayhunter/rayhunter-daemon",
        "exit code 0",
    )
    .await?;
    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        crate::RAYHUNTER_DAEMON_INIT.as_bytes(),
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
    )
    .await?;
    telnet_send_command(addr, "update-rc.d rayhunter_daemon defaults", "exit code 0").await?;

    println!("Rebooting device and waiting 30 seconds for it to start up.");
    telnet_send_command(addr, "reboot", "exit code 0").await?;
    sleep(Duration::from_secs(30)).await;

    echo!("Testing rayhunter... ");
    const MAX_FAILURES: u32 = 10;
    let mut failures = 0;
    let rayhunter_url = format!("http://{admin_ip}:8080/index.html");
    let client = Client::new();
    loop {
        match client.get(&rayhunter_url).send().await {
            Ok(test) => {
                if test.status() == 200 {
                    println!("rayhunter is running at http://{admin_ip}:8080");
                    return Ok(());
                } else {
                    bail!(
                        "request for url ({rayhunter_url}) failed with status code: {:?}",
                        test.status()
                    );
                }
            }
            Err(e) => {
                if failures > MAX_FAILURES {
                    return Err(e.into());
                } else {
                    failures += 1;
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }
    }
}

async fn telnet_send_file(addr: SocketAddr, filename: &str, payload: &[u8]) -> Result<()> {
    println!("Sending file {filename}");

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

    Ok(())
}

#[test]
fn test_encrypt_password() {
    let p = b"80536913";
    let s = encrypt_password(p).ok();
    let expected = Some("5brvd8xl732cSoFTAy67ig==".to_string());
    assert_eq!(s, expected);
}
