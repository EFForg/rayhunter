use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use clap::Args;
use reqwest::{Client, header};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tokio::time::{sleep, timeout};

use crate::mr1100_transport::Connection;
use crate::output::println;

const ROOT: &str = "/data/rayhunter";
const STAGE: &str = "/data/.rayhunter-mr1100-install";
const INIT: &str = "/etc/init.d/rayhunter_mr1100";
const MARKER: &str = "mr1100-v1";
const CONFIG: &str = include_str!("../../dist/config.mr1100.toml");
const SERVICE: &str = include_str!("../../dist/scripts/rayhunter_mr1100");
const LINKS: [&str; 4] = [
    "/etc/rc5.d/S88rayhunter_mr1100_firewall",
    "/etc/rc5.d/S99rayhunter_mr1100",
    "/etc/rc0.d/K01rayhunter_mr1100",
    "/etc/rc6.d/K01rayhunter_mr1100",
];

#[derive(Args, Debug)]
pub(crate) struct Options {
    /// MR1100 USB gateway address. Disconnect other hotspots with the same address.
    #[arg(long, default_value = "192.168.1.1")]
    admin_ip: Ipv4Addr,
    /// This computer's IPv4 address on the MR1100 USB network (not the gateway).
    #[arg(long)]
    local_ip: Ipv4Addr,
    /// Linux/Android: bind every connection to this USB interface. May require root.
    #[arg(long)]
    interface: Option<String>,
    /// Acknowledge that default captures are RAM-backed and lost at reboot.
    #[arg(long)]
    volatile_captures: bool,
    /// Select the native LTE All preset. Requires an authenticated native web session.
    #[arg(long)]
    lte_only: bool,
    /// Path to sierrakeygen.py. Required for install: persistent root Telnet is closed.
    #[arg(long, required_unless_present = "uninstall")]
    keygen: Option<PathBuf>,
    /// Remove this installer's files and startup links; retain RAM captures.
    #[arg(long, conflicts_with = "lte_only")]
    uninstall: bool,
}

pub async fn install(options: Options) -> Result<()> {
    ensure!(
        options.uninstall || options.volatile_captures,
        "Specify --volatile-captures to acknowledge RAM-only recordings; no SD card is required"
    );
    ensure!(
        options.local_ip != options.admin_ip && !options.local_ip.is_unspecified(),
        "Specify the host's USB IPv4 address"
    );
    let binary: &[u8] = if options.uninstall {
        &[]
    } else {
        crate::get_file!("FILE_RAYHUNTER_DAEMON")
    };
    if !options.uninstall {
        validate_binary(binary)?;
        validate_config(CONFIG)?;
    }
    #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "fuchsia")))]
    ensure!(
        options.interface.is_none(),
        "--interface is supported on Linux/Android; otherwise disconnect other hotspots with the same subnet"
    );
    let conn = Connection {
        admin: options.admin_ip,
        local: options.local_ip,
        interface: options.interface,
    };
    verify_device(&conn).await?;
    let web = Web::new(&conn)?;
    let mut access_changed = false;
    let had_root = has_root(&conn).await;
    if !options.uninstall || !had_root {
        options.keygen.as_deref().context(
            "Supply --keygen: installation must close persistent root Telnet for safe boot",
        )?;
    }
    let original_telnet = if had_root {
        let value = conn
            .run("/usr/bin/bsinfo -sn")
            .await?
            .trim()
            .parse::<u8>()?;
        ensure!(value <= 1, "Unexpected persistent Telnet setting");
        value
    } else {
        0
    };
    let original_rd = if options.keygen.is_some() {
        Some(parse_rd_enable(&conn.at("AT!CUSTOM?").await?)?)
    } else {
        None
    };
    let result = async {
        if let Some(rd) = original_rd {
            if !had_root {
                ensure_root(&conn, options.keygen.as_deref(), &mut access_changed).await?;
            }
            // Do not depend on a boot-time firewall to contain persistent root Telnet.
            // The current shell remains usable until the final reboot.
            access_changed = true;
            restore_access_flags(&conn, options.keygen.as_deref().unwrap(), 0, rd).await?;
            // bsinfo reports the current boot's value, not a pending AT change.
            // Verify that TCP 23 is closed after the final restart instead.
        }
        if options.uninstall {
            uninstall(&conn, &web).await
        } else {
            install_rooted(&conn, &web, binary, options.lte_only).await
        }
    }
    .await;
    if access_changed {
        let telnet = if result.is_err() { original_telnet } else { 0 };
        println!(
            "Restoring debug settings and restarting (persistent Telnet={telnet}). RAM captures will be lost."
        );
        let cleanup = async {
            restore_access_flags(
                &conn,
                options.keygen.as_deref().unwrap(),
                telnet,
                original_rd.unwrap(),
            )
            .await?;
            let _ = conn.at("AT!RESET").await;
            sleep(Duration::from_secs(10)).await;
            timeout(Duration::from_secs(180), async {
                loop {
                    if verify_device(&conn).await.is_ok() {
                        break;
                    }
                    sleep(Duration::from_secs(2)).await;
                }
            })
            .await
            .context("Router did not return after closing temporary root access")?;
            if telnet == 0 {
                ensure!(
                    conn.connect(23).await.is_err(),
                    "Root Telnet is still listening after access cleanup"
                );
            } else {
                ensure!(
                    has_root(&conn).await,
                    "Could not verify restoration of original root access"
                );
            }
            Ok::<_, anyhow::Error>(())
        }
        .await;
        if let Err(error) = cleanup {
            bail!(
                "Temporary root cleanup failed: {error:#}. Keep the router isolated and disable TELEN/RDENABLE before use. Operation result: {result:?}"
            );
        }
    }
    result
}

async fn install_rooted(conn: &Connection, web: &Web, binary: &[u8], lte_only: bool) -> Result<()> {
    conn.run("test \"$(id -u)\" = 0\ntest \"$(uname -m)\" = armv7l\ntest \"$(uname -r)\" = 3.18.31\ntest -c /dev/diag\ngrep -q '^ubi0:usrfs /data ubifs rw' /proc/mounts\ngrep -q '^tmpfs /media/ram tmpfs ' /proc/mounts").await?;
    let existing = conn
        .run(&format!(
            "if [ -e {ROOT} ] || [ -L {ROOT} ]; then echo PRESENT; fi"
        ))
        .await?;
    if existing.trim() == "PRESENT" {
        let marker = conn.run(&format!("cat {ROOT}/.mr1100-install")).await?;
        ensure!(
            marker.trim() == MARKER,
            "Existing /data/rayhunter is not owned by the MR1100 installer"
        );
        validate_installation(conn).await.context("Installation is incomplete or modified. Use --uninstall to recover; configuration is not overwritten")?;
        println!("Complete MR1100 installation verified; files and configuration left unchanged.");
        return Ok(());
    }
    conn.run(&format!(
        "test ! -e {STAGE}\ntest ! -L {STAGE}\ntest ! -e {INIT}\ntest ! -L {INIT}\ntest ! -e {INIT}.tmp\ntest ! -L {INIT}.tmp"
    ))
    .await?;
    for path in LINKS {
        conn.run(&format!("test ! -e {path}\ntest ! -L {path}"))
            .await?;
    }
    conn.run("if iptables -S RH_MR1100 >/dev/null 2>&1; then exit 1; fi\nif iptables -S RH_MR_INSTALL >/dev/null 2>&1; then exit 1; fi\nif ip6tables -S RH_MR1100 >/dev/null 2>&1; then exit 1; fi").await?;
    let free = conn
        .run("df -Pk /data | tail -n 1 | awk '{print $4}'")
        .await?
        .trim()
        .parse::<u64>()?;
    ensure!(
        free * 1024 > binary.len() as u64 + 4 * 1024 * 1024,
        "Not enough /data space for the binary plus a 4 MiB reserve"
    );
    let status = conn.run("cat /sys/kernel/debug/diag/status").await?;
    require_idle_diag(&status)?;
    conn.run("test -d /sys/class/net/rndis0\ntest \"$(cat /proc/sys/net/bridge/bridge-nf-call-iptables)\" = 1").await?;

    // Only an explicit option changes the native radio profile. Save it for uninstall.
    let original_band = if lte_only {
        let index = web.current_band().await?;
        ensure!(matches!(index, 0..=2), "Unexpected native band preset");
        Some(index)
    } else {
        None
    };
    println!(
        "Installing to /data/rayhunter. Captures and logs remain in RAM; TCP 8080 and root Telnet will be restricted to USB."
    );
    conn.run(&format!(
        "umask 077\nmkdir {STAGE}\nprintf '{MARKER}\\n' > {STAGE}/.mr1100-install"
    ))
    .await?;
    let result = install_files(conn, web, binary, original_band).await;
    if let Err(error) = &result {
        println!("Installation failed; removing files created by this attempt.");
        if let Err(cleanup) = stop_owned(conn).await {
            bail!(
                "{:#}; rollback stopped to avoid deleting a live or unverified installation: {cleanup:#}",
                error
            );
        }
        if let Err(cleanup) = remove_files(conn).await {
            println!(
                "WARNING: cleanup incomplete; ownership metadata retained for --uninstall: {cleanup:#}"
            );
        }
        if let Some(index) = original_band
            && web.set_band(index).await.is_err()
        {
            println!("WARNING: restore the original native band preset manually (index {index}).");
        }
    }
    result?;
    println!(
        "Installed and started. Open http://{}:8080 through USB. RAM recordings stop when the configured reserve is reached and disappear at reboot.",
        conn.admin
    );
    println!(
        "The early firewall hook precedes native Telnet startup. Persistent root Telnet is disabled before the final reboot, so firewall failure cannot expose it. No native startup script was replaced."
    );
    Ok(())
}

async fn install_files(
    conn: &Connection,
    web: &Web,
    binary: &[u8],
    original_band: Option<u64>,
) -> Result<()> {
    // The temporary receiver is reachable only through the selected USB host.
    conn.run(&format!("echo owned > {STAGE}/.transfer-firewall"))
        .await?;
    conn.run("iptables -N RH_MR_INSTALL").await?;
    let transfer = async {
        conn.run(&format!("iptables -A RH_MR_INSTALL -s {} -m physdev --physdev-in rndis0 -j ACCEPT\niptables -A RH_MR_INSTALL -j REJECT\niptables -I INPUT 1 -p tcp --dport 8081 -j RH_MR_INSTALL", conn.local)).await?;
        conn.write(&format!("{STAGE}/rayhunter-daemon"), binary).await?;
        conn.write(&format!("{STAGE}/config.toml"), CONFIG.as_bytes()).await?;
        conn.write(&format!("{STAGE}/service"), SERVICE.as_bytes()).await?;
        if let Some(index) = original_band {
            conn.run(&format!("echo {index} > {STAGE}/original-band")).await?;
        }
        Ok::<_, anyhow::Error>(())
    }.await;
    let cleanup = conn.run("iptables -D INPUT -p tcp --dport 8081 -j RH_MR_INSTALL 2>/dev/null || true\niptables -F RH_MR_INSTALL\niptables -X RH_MR_INSTALL").await;
    transfer?;
    cleanup?;
    conn.run(&format!(
        "rm {STAGE}/.transfer-firewall\nprintf '{:x}\\n' > {STAGE}/.binary-sha256",
        Sha256::digest(binary)
    ))
    .await?;
    conn.run(&format!("chmod 700 {STAGE}/rayhunter-daemon {STAGE}/service\nsh -n {STAGE}/service\nmv {STAGE} {ROOT}\necho owned > {ROOT}/.init-transfer\ncp {ROOT}/service {INIT}.tmp\nchmod 755 {INIT}.tmp")).await?;
    validate_service(conn, &format!("{INIT}.tmp")).await?;
    conn.run(&format!("mv {INIT}.tmp {INIT}\nrm {ROOT}/.init-transfer"))
        .await?;
    conn.run("ln -s /media/ram/rayhunter-mr1100/rayhunter.log /data/rayhunter/rayhunter.log")
        .await?;
    if original_band.is_some() {
        web.set_band(2).await?;
        sleep(Duration::from_secs(15)).await;
    }
    conn.run(&format!(
        "echo owned > {ROOT}/.service-firewall\n{INIT} start"
    ))
    .await?;
    let captured = timeout(Duration::from_secs(60), async {
        loop {
            sleep(Duration::from_secs(5)).await;
            if let Ok(response) = web
                .client
                .get(format!("{}/api/qmdl-manifest", web.daemon_url()))
                .send()
                .await
                && let Ok(value) = response.json::<Value>().await
                && value["current_entry"]["qmdl_size_bytes"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
            {
                break;
            }
        }
    })
    .await
    .is_ok();
    ensure!(
        captured,
        "No diagnostic records arrived within 60 seconds. Check actual LTE reception; use the native LTE All preset or --lte-only"
    );
    conn.run(&format!("{INIT} status")).await?;
    for path in LINKS {
        conn.run(&format!("ln -s ../init.d/rayhunter_mr1100 {path}"))
            .await?;
    }
    conn.run(&format!("sync\nprintf '{MARKER}\\n' > {ROOT}/.mr1100-complete.tmp\nsync\nmv {ROOT}/.mr1100-complete.tmp {ROOT}/.mr1100-complete\nsync")).await?;
    println!(
        "Recording is growing. Startup links installed; inspect captured RRC/NAS before making detection claims."
    );
    Ok(())
}

async fn verify_device(conn: &Connection) -> Result<()> {
    validate_identity(&conn.at("ATI").await?)
}

fn validate_identity(info: &str) -> Result<()> {
    ensure!(
        info.lines().any(|line| line.trim() == "Model: MR1100"),
        "Target is not an MR1100; no changes made"
    );
    ensure!(
        info.lines()
            .any(|line| line.trim().starts_with("Revision: NTG9X50C_12.06.39.00 ")),
        "Only tested MR1100 firmware NTG9X50C_12.06.39.00 is supported; no changes made"
    );
    Ok(())
}

async fn unlock(conn: &Connection, keygen: &std::path::Path) -> Result<()> {
    let challenge = conn.at("AT!OPENLOCK?").await?;
    let challenge = challenge
        .lines()
        .map(str::trim)
        .find(|line| is_hex_key(line))
        .context("No valid OPENLOCK challenge")?;
    let output = timeout(
        Duration::from_secs(20),
        tokio::process::Command::new("python3")
            .arg(keygen)
            .args(["-d", "MDM9x40", "-l", challenge])
            .kill_on_drop(true)
            .output(),
    )
    .await
    .context("OPENLOCK helper timed out")??;
    ensure!(
        output.status.success(),
        "OPENLOCK helper failed; check Python and pyserial installation"
    );
    let response = parse_keygen_output(&String::from_utf8(output.stdout)?)?;
    conn.at_ok(&format!("AT!OPENLOCK=\"{response}\"")).await
}

async fn ensure_root(
    conn: &Connection,
    keygen: Option<&std::path::Path>,
    changed: &mut bool,
) -> Result<()> {
    let keygen = keygen.context("Supply --keygen for temporary root access")?;
    unlock(conn, keygen).await?;
    println!(
        "Enabling temporary root access. Keep the router isolated during bootstrap; it will restart."
    );
    *changed = true;
    conn.at_ok("AT!TELEN=1").await?;
    conn.at_ok("AT!CUSTOM=\"RDENABLE\",1").await?;
    let _ = conn.at("AT!RESET").await;
    sleep(Duration::from_secs(10)).await;
    timeout(Duration::from_secs(180), async {
        loop {
            if verify_device(conn).await.is_ok() && has_root(conn).await {
                break;
            }
            sleep(Duration::from_secs(2)).await;
        }
    })
    .await
    .context("Root did not become available after restart; check USB and local IP")?;
    Ok(())
}

fn parse_rd_enable(output: &str) -> Result<u8> {
    ensure!(
        output.lines().any(|l| l.trim() == "OK") && output.contains("!CUSTOM:"),
        "Cannot read original debug settings"
    );
    for line in output.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.first() == Some(&"RDENABLE") {
            let value = words.get(1).context("Missing RDENABLE value")?;
            let value = u8::from_str_radix(value.trim_start_matches("0x"), 16)?;
            ensure!(value <= 1, "Unexpected RDENABLE value");
            return Ok(value);
        }
    }
    Ok(0)
}

async fn restore_access_flags(
    conn: &Connection,
    keygen: &std::path::Path,
    telnet: u8,
    rd: u8,
) -> Result<()> {
    unlock(conn, keygen).await?;
    conn.at_ok(&format!("AT!CUSTOM=\"RDENABLE\",{rd}")).await?;
    conn.at_ok(&format!("AT!TELEN={telnet}")).await?;
    Ok(())
}

async fn has_root(conn: &Connection) -> bool {
    timeout(Duration::from_secs(8), conn.run("id -u"))
        .await
        .is_ok_and(|result| result.is_ok_and(|out| out.trim() == "0"))
}

fn is_hex_key(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|c| c.is_ascii_hexdigit())
}
fn parse_keygen_output(output: &str) -> Result<String> {
    let key = output
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("AT!OPENLOCK=\"")
                .and_then(|s| s.strip_suffix('"'))
        })
        .context("OPENLOCK helper returned no response")?;
    ensure!(is_hex_key(key), "Invalid OPENLOCK helper response");
    Ok(key.to_owned())
}

fn validate_binary(binary: &[u8]) -> Result<()> {
    ensure!(
        binary.len() > 52
            && &binary[..4] == b"\x7fELF"
            && binary[4] == 1
            && binary[5] == 1
            && binary[18..20] == [40, 0],
        "Installer payload is not a 32-bit ARM ELF"
    );
    ensure!(
        binary.windows(6).any(|bytes| bytes == b"mr1100"),
        "Daemon does not contain the MR1100 profile; rebuild it from this branch"
    );
    Ok(())
}
fn validate_config(text: &str) -> Result<()> {
    let value: toml::Value = toml::from_str(text)?;
    ensure!(
        value.get("device").and_then(toml::Value::as_str) == Some("mr1100")
            && value.get("port").and_then(toml::Value::as_integer) == Some(8080),
        "Unsafe MR1100 configuration"
    );
    ensure!(
        value.get("qmdl_store_path").and_then(toml::Value::as_str)
            == Some("/media/ram/rayhunter-mr1100/qmdl"),
        "Installer requires RAM-backed captures"
    );
    ensure!(
        value.get("wifi_enabled").and_then(toml::Value::as_bool) == Some(false)
            && value
                .get("auto_check_updates")
                .and_then(toml::Value::as_bool)
                == Some(false),
        "Unsupported integration enabled"
    );
    Ok(())
}
fn require_idle_diag(status: &str) -> Result<()> {
    for field in [
        "Logging Mode: 0",
        "MD session mode: 0",
        "MD session mask: 0",
    ] {
        ensure!(
            status.lines().any(|line| line.trim() == field),
            "DIAG is not in the tested idle state; do not stop native clients blindly"
        );
    }
    Ok(())
}

struct Web {
    client: Client,
    base: String,
}
impl Web {
    fn new(conn: &Connection) -> Result<Self> {
        let builder = Client::builder()
            .no_proxy()
            .local_address(std::net::IpAddr::V4(conn.local))
            .timeout(Duration::from_secs(12))
            .redirect(reqwest::redirect::Policy::none());
        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        let builder = if let Some(interface) = &conn.interface {
            builder.interface(interface)
        } else {
            builder
        };
        Ok(Self {
            client: builder.build()?,
            base: format!("http://{}", conn.admin),
        })
    }
    fn daemon_url(&self) -> String {
        format!("{}:8080", self.base)
    }
    async fn model(&self) -> Result<(Value, String)> {
        let origin = reqwest::Url::parse(&self.base)?;
        let mut url = origin.clone();
        let mut jar = std::collections::BTreeMap::new();
        let mut cookies = String::new();
        let mut ready = false;
        // The native server establishes its session through /sess_cd_tmp redirects.
        // Keep cookies on this origin only; do not follow arbitrary remote redirects.
        for _ in 0..8 {
            let response = self
                .client
                .get(url.clone())
                .header(header::COOKIE, &cookies)
                .send()
                .await?;
            for value in response.headers().get_all(header::SET_COOKIE) {
                if let Some((name, value)) = value
                    .to_str()?
                    .split(';')
                    .next()
                    .and_then(|p| p.split_once('='))
                {
                    jar.insert(name.trim().to_owned(), value.to_owned());
                }
            }
            cookies = jar
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("; ");
            if !response.status().is_redirection() {
                response.error_for_status()?;
                ready = true;
                break;
            }
            url = url.join(
                response
                    .headers()
                    .get(header::LOCATION)
                    .context("Missing session redirect")?
                    .to_str()?,
            )?;
            ensure!(
                url.origin() == origin.origin(),
                "Refusing cross-origin native session redirect"
            );
        }
        ensure!(ready, "Native session redirect limit reached");
        let model = self
            .client
            .get(format!("{}/api/model.json?internalapi=1", self.base))
            .header(header::COOKIE, &cookies)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;
        ensure!(
            model["general"]["model"].as_str() == Some("MR1100"),
            "Unexpected native web target"
        );
        Ok((model, cookies))
    }
    async fn current_band(&self) -> Result<u64> {
        let (model, _) = self.model().await?;
        model["wwan"]["bandRegion"]
            .as_array()
            .context("Native band presets unavailable")?
            .iter()
            .find(|entry| entry["current"].as_bool() == Some(true))
            .and_then(|entry| entry["index"].as_u64())
            .context("Native band preset unavailable")
    }
    async fn set_band(&self, index: u64) -> Result<()> {
        let (model, cookies) = self.model().await?;
        let expected = match index {
            0 => "Auto",
            1 => "WCDMA All",
            2 => "LTE All",
            _ => bail!("Unsupported band preset"),
        };
        ensure!(model["wwan"]["bandRegion"].as_array().context("No presets")?.iter().any(|v| v["index"].as_u64() == Some(index) && v["name"].as_str() == Some(expected)), "Native preset does not match tested firmware");
        let token = model["session"]["secToken"].as_str().context("Native admin session unavailable. Select LTE All in the web interface and omit --lte-only")?;
        let result = self
            .client
            .post(format!("{}/Forms/config", self.base))
            .header(header::COOKIE, cookies)
            .form(&[
                ("token", token),
                ("wwan.bandRegion.setIndex", &index.to_string()),
                ("ok_redirect", "/success.json"),
                ("err_redirect", "/error.json"),
            ])
            .send()
            .await?;
        ensure!(
            result.status().is_success() || result.status().is_redirection(),
            "Native preset request failed"
        );
        sleep(Duration::from_secs(2)).await;
        ensure!(
            self.current_band().await? == index,
            "Native preset was not changed; verify admin access"
        );
        Ok(())
    }
}

async fn validate_service(conn: &Connection, path: &str) -> Result<()> {
    conn.run(&format!("test -f {path}\ntest ! -L {path}"))
        .await?;
    let output = conn.run(&format!("sha256sum {path}")).await?;
    let expected = format!("{:x}", Sha256::digest(SERVICE.as_bytes()));
    ensure!(
        output.split_whitespace().next() == Some(expected.as_str()),
        "Service script differs from this installer; refusing to execute or delete it"
    );
    Ok(())
}

async fn validate_installation(conn: &Connection) -> Result<()> {
    conn.run(&format!(
        "test ! -L {ROOT}\ntest ! -L {ROOT}/.mr1100-complete"
    ))
    .await?;
    ensure!(
        conn.run(&format!("cat {ROOT}/.mr1100-complete"))
            .await?
            .trim()
            == MARKER,
        "Missing completion marker"
    );
    validate_service(conn, INIT).await?;
    validate_service(conn, &format!("{ROOT}/service")).await?;
    validate_config(&conn.run(&format!("cat {ROOT}/config.toml")).await?)?;
    let digest = conn.run(&format!("cat {ROOT}/.binary-sha256")).await?;
    let actual = conn
        .run(&format!("sha256sum {ROOT}/rayhunter-daemon"))
        .await?;
    ensure!(
        actual.split_whitespace().next() == Some(digest.trim()),
        "Installed daemon checksum changed"
    );
    for link in LINKS {
        ensure!(
            conn.run(&format!("readlink {link}")).await?.trim() == "../init.d/rayhunter_mr1100",
            "Startup link is missing or changed"
        );
    }
    conn.run(&format!("{ROOT}/service status")).await?;
    for port in [23, 8080] {
        conn.run(&format!(
            "iptables -C INPUT -p tcp --dport {port} -j RH_MR1100"
        ))
        .await?;
        conn.run(&format!("if [ -e /proc/net/if_inet6 ]; then ip6tables -C INPUT -p tcp --dport {port} -j RH_MR1100; fi")).await?;
    }
    Ok(())
}

async fn stop_owned(conn: &Connection) -> Result<()> {
    let present = conn
        .run(&format!(
            "if [ -e {ROOT}/service ] || [ -L {ROOT}/service ]; then echo PRESENT; fi"
        ))
        .await?;
    if present.trim() == "PRESENT" {
        validate_service(conn, &format!("{ROOT}/service")).await?;
        conn.run(&format!("{ROOT}/service stop")).await?;
    }
    // A missing PID file must not let rollback unlink a still-running executable.
    conn.run("for f in /proc/[0-9]*/exe; do\ncase \"$(readlink \"$f\" 2>/dev/null)\" in\n/data/rayhunter/rayhunter-daemon*) exit 1;;\nesac\ndone").await?;
    Ok(())
}

async fn uninstall(conn: &Connection, web: &Web) -> Result<()> {
    let mut found = false;
    for base in [ROOT, STAGE] {
        let present = conn
            .run(&format!(
                "if [ -e {base} ] || [ -L {base} ]; then echo PRESENT; fi"
            ))
            .await?;
        if present.trim() == "PRESENT" {
            conn.run(&format!(
                "test ! -L {base}\ntest ! -L {base}/.mr1100-install"
            ))
            .await?;
            ensure!(
                conn.run(&format!("cat {base}/.mr1100-install"))
                    .await?
                    .trim()
                    == MARKER,
                "Not an MR1100 installer-owned directory"
            );
            found = true;
        }
    }
    ensure!(
        found,
        "No owned MR1100 installation or staging directory found"
    );
    stop_owned(conn).await?;
    for base in [ROOT, STAGE] {
        let band = conn
            .run(&format!(
                "if [ -f {base}/original-band ]; then cat {base}/original-band; fi"
            ))
            .await?;
        if !band.trim().is_empty() {
            web.set_band(band.trim().parse()?).await?;
        }
    }
    remove_files(conn).await?;
    println!(
        "Application and startup links removed. Capture files were not deleted, but an access-cleanup restart will erase RAM. Without --keygen, existing root access keeps its prior exposure."
    );
    Ok(())
}

async fn remove_files(conn: &Connection) -> Result<()> {
    let mut errors = Vec::new();
    // Do independent cleanup even after a failure, but retain directory ownership
    // and the verified service copy until all external references are removed.
    for path in LINKS {
        let result = conn.run(&format!("if [ -e {path} ] || [ -L {path} ]; then\ntest \"$(readlink {path})\" = ../init.d/rayhunter_mr1100\nrm {path}\nfi")).await;
        if let Err(error) = result {
            errors.push(error.to_string());
        }
    }
    let present = conn
        .run(&format!(
            "if [ -e {INIT} ] || [ -L {INIT} ]; then echo PRESENT; fi"
        ))
        .await?;
    if present.trim() == "PRESENT" {
        match validate_service(conn, INIT).await {
            Ok(()) => {
                if let Err(e) = conn.run(&format!("rm {INIT}")).await {
                    errors.push(e.to_string());
                }
            }
            Err(e) => errors.push(e.to_string()),
        }
    }
    let partial_init = conn
        .run(&format!(
            "if [ -f {ROOT}/.init-transfer ]; then echo OWNED; fi"
        ))
        .await?;
    if partial_init.trim() == "OWNED"
        && let Err(error) = conn.run(&format!("rm -f {INIT}.tmp")).await
    {
        errors.push(error.to_string());
    }
    // All callers have an owned directory; fresh installs also preflight chain absence.
    for (tool, chain, ports) in [
        ("iptables", "RH_MR_INSTALL", &[8081][..]),
        ("iptables", "RH_MR1100", &[23, 8080][..]),
        ("ip6tables", "RH_MR1100", &[23, 8080][..]),
    ] {
        let journal = if chain == "RH_MR_INSTALL" {
            ".transfer-firewall"
        } else {
            ".service-firewall"
        };
        let owned = conn
            .run(&format!(
                "if [ -f {ROOT}/{journal} ] || [ -f {STAGE}/{journal} ]; then echo OWNED; fi"
            ))
            .await?;
        if owned.trim() != "OWNED" {
            continue;
        }
        for port in ports {
            let result = conn.run(&format!("if command -v {tool} >/dev/null; then\nwhile {tool} -C INPUT -p tcp --dport {port} -j {chain} 2>/dev/null; do\n{tool} -D INPUT -p tcp --dport {port} -j {chain}\ndone\nfi")).await;
            if let Err(e) = result {
                errors.push(e.to_string());
            }
        }
        let result = conn.run(&format!("if command -v {tool} >/dev/null && {tool} -S {chain} >/dev/null 2>&1; then\n{tool} -F {chain}\n{tool} -X {chain}\nfi")).await;
        if let Err(e) = result {
            errors.push(e.to_string());
        }
    }
    ensure!(
        errors.is_empty(),
        "Cleanup incomplete; ownership metadata retained: {}",
        errors.join("; ")
    );
    for base in [ROOT, STAGE] {
        let result = conn.run(&format!("if [ \"$(cat {base}/.mr1100-install 2>/dev/null)\" = {MARKER} ]; then\nrm -f {base}/rayhunter-daemon {base}/rayhunter-daemon.tmp\nrm -f {base}/config.toml {base}/config.toml.tmp\nrm -f {base}/service {base}/service.tmp {base}/original-band\nrm -f {base}/rayhunter.log {base}/.binary-sha256\nrm -f {base}/.transfer-firewall {base}/.service-firewall {base}/.init-transfer\nrm -f {base}/.mr1100-complete {base}/.mr1100-complete.tmp\nfi")).await;
        if let Err(e) = result {
            errors.push(e.to_string());
            continue;
        }
        let result = conn.run(&format!("if [ -f {base}/.mr1100-install ]; then\nif [ \"$(find {base} -mindepth 1 -maxdepth 1 ! -name .mr1100-install | wc -l)\" -eq 0 ]; then\nrm {base}/.mr1100-install\nif ! rmdir {base}; then echo {MARKER} > {base}/.mr1100-install; exit 1; fi\nelse\necho USER_FILES_RETAINED\nfi\nfi")).await;
        match result {
            Ok(output) if output.contains("USER_FILES_RETAINED") => {
                println!("Retained {base} and ownership marker because it contains user files.")
            }
            Ok(_) => {}
            Err(e) => errors.push(e.to_string()),
        }
    }
    ensure!(
        errors.is_empty(),
        "Cleanup incomplete; retry --uninstall: {}",
        errors.join("; ")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    #[test]
    fn startup_failure_checks() {
        let result = std::process::Command::new("sh")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/rayhunter_mr1100_test.sh"
            ))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    #[test]
    fn firmware_and_device_gate() {
        assert!(validate_identity("Model: MR1100\nRevision: NTG9X50C_12.06.39.00 r4374\n").is_ok());
        assert!(validate_identity("Model: RC400L\nRevision: NTG9X50C_12.06.39.00 r4374").is_err());
        assert!(validate_identity("Model: MR1100\nRevision: NTG9X50C_12.06.11.00 r1").is_err());
    }
    #[test]
    fn helper_output_is_not_shell_code() {
        assert_eq!(
            parse_keygen_output("AT!OPENLOCK=\"1033773720F6EE66\"\n").unwrap(),
            "1033773720F6EE66"
        );
        assert!(parse_keygen_output("AT!OPENLOCK=\"x\"; reboot").is_err());
        assert!(parse_keygen_output("AT!OPENCND=\"1033773720F6EE66\"").is_err());
    }
    #[test]
    fn original_debug_state_is_checked_before_bootstrap() {
        assert_eq!(parse_rd_enable("!CUSTOM:\nRDENABLE 0x01\nOK\n").unwrap(), 1);
        assert_eq!(
            parse_rd_enable("!CUSTOM:\nNATENABLED 0x01\nOK\n").unwrap(),
            0
        );
        assert!(parse_rd_enable("ERROR\n").is_err());
        assert!(parse_rd_enable("!CUSTOM:\nRDENABLE 0xff\nOK\n").is_err());
    }

    #[test]
    fn safe_config_and_idle_session() {
        assert!(validate_config("").is_err());
        validate_config(CONFIG).unwrap();
        assert!(validate_config(&CONFIG.replace("port = 8080", "port = 80")).is_err());
        require_idle_diag("Logging Mode: 0\nMD session mode: 0\nMD session mask: 0\n").unwrap();
        assert!(
            require_idle_diag("Logging Mode: 1\nMD session mode: 1\nMD session mask: 31").is_err()
        );
        assert!(validate_binary(b"not a firmware binary").is_err());
    }
}
