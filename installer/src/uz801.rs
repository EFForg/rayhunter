use std::io::ErrorKind;
use std::path::Path;
/// Installer for the UZ801 and compatible MSM8916 USB modem sticks.
///
/// Handles both genuine UZ801 devices (USB PID 0x90b6, requires HTTP backdoor
/// for ADB) and HiMI_UFI variants (USB PID 0x9024, ADB enabled by default).
/// Unknown MSM8916 variants are detected via USB ADB interface autodetection.
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice, RustADBError};
use anyhow::{Result, anyhow};
use md5::compute as md5_compute;
use tokio::time::sleep;

use crate::Uz801Args as Args;
use crate::output::{print, println};

const QUALCOMM_VENDOR_ID: u16 = 0x05c6;
const KNOWN_PRODUCT_IDS: &[u16] = &[
    0x90b6, // UZ801
];

const STARTUP_SCRIPTS: &[&str] = &[
    "/system/bin/initmifiservice.sh",     // UZ801
    "/system/etc/init.qcom.post_boot.sh", // HiMI_UFI and other MSM8916 variants
];

// Services that compete for /dev/diag and must be removed from startup scripts.
const DIAG_COMPETITORS: &[&str] = &[
    "startRIDL", // Qualcomm LogKit II client, seen on HiMI_UFI firmware
];

pub async fn install(
    Args {
        admin_ip,
        skip_backdoor,
    }: Args,
) -> Result<()> {
    run_install(admin_ip, skip_backdoor).await
}

async fn run_install(admin_ip: String, skip_backdoor: bool) -> Result<()> {
    let backdoor_ok = if !skip_backdoor {
        print!("Activating USB debugging backdoor... ");
        match activate_usb_debug(&admin_ip).await {
            Ok(()) => {
                println!("ok");
                true
            }
            Err(e) => {
                println!("failed ({e}), will try ADB anyway");
                false
            }
        }
    } else {
        false
    };

    print!("Waiting for ADB connection... ");
    let mut adb_device = wait_for_adb(backdoor_ok).await?;
    println!("ok");

    print!("Installing rayhunter files... ");
    install_rayhunter_files(&mut adb_device).await?;
    println!("ok");

    kill_diag_competitors(&mut adb_device);

    print!("Modifying startup script... ");
    modify_startup_script(&mut adb_device).await?;
    println!("ok");

    print!("Rebooting the device... ");
    let _ = adb_device.reboot(adb_client::RebootType::System);
    println!("ok");

    println!("Installation complete!");
    println!("Please wait for the device to reboot (light will turn green)");
    println!("Then access rayhunter at: http://{admin_ip}:8080");

    Ok(())
}

pub async fn activate_usb_debug(admin_ip: &str) -> Result<()> {
    let url = format!("http://{admin_ip}/ajax");
    let referer = format!("http://{admin_ip}/usbdebug.html");
    let origin = format!("http://{admin_ip}");

    print!("Checking if device is online... ");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    match client.get(&origin).send().await {
        Ok(response) if response.status().is_success() => println!("ok"),
        Ok(response) => anyhow::bail!(
            "Device at {admin_ip} returned error status: {}",
            response.status()
        ),
        Err(e) => anyhow::bail!("Failed to reach device at {admin_ip}: {}", e),
    }

    let _handle = tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let _response = client
            .post(&url)
            .header("Accept", "application/json, text/javascript, */*; q=0.01")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Referer", &referer)
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Origin", &origin)
            .body(r#"{"funcNo":2001}"#)
            .send()
            .await;
    });

    Ok(())
}

/// Try to connect to an ADB device, preferring known UZ801 product IDs
/// before falling back to autodetection of any ADB-capable USB device.
fn try_connect_adb() -> std::result::Result<ADBUSBDevice, RustADBError> {
    for &pid in KNOWN_PRODUCT_IDS {
        match ADBUSBDevice::new(QUALCOMM_VENDOR_ID, pid) {
            Ok(device) => return Ok(device),
            Err(RustADBError::DeviceNotFound(_)) => continue,
            Err(e) => return Err(e),
        }
    }
    ADBUSBDevice::autodetect()
}

async fn wait_for_adb(backdoor_activated: bool) -> Result<ADBUSBDevice> {
    const MAX_ATTEMPTS: u32 = 30;
    let mut attempts = 0;

    if backdoor_activated {
        sleep(Duration::from_secs(10)).await;
    }

    loop {
        if attempts >= MAX_ATTEMPTS {
            anyhow::bail!(
                "Timeout waiting for ADB connection.\n\
                 Make sure you don't have an `adb` daemon running (try `adb kill-server`).\n\
                 If your device already has ADB enabled, try --skip-backdoor."
            );
        }

        match try_connect_adb() {
            Ok(mut device) => {
                if test_adb_connection(&mut device).await.is_ok() {
                    return Ok(device);
                }
            }
            Err(RustADBError::DeviceNotFound(_)) => {}
            Err(RustADBError::IOError(ref e)) if e.kind() == ErrorKind::ResourceBusy => {
                anyhow::bail!(
                    "ADB device found but is busy. If you have adb installed, run `adb kill-server` first."
                );
            }
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            Err(RustADBError::IOError(ref e)) if e.kind() == ErrorKind::PermissionDenied => {
                anyhow::bail!(
                    "ADB device found but access denied. If you have adb installed, run `adb kill-server` first."
                );
            }
            Err(e) => {
                anyhow::bail!("ADB connection error: {}", e);
            }
        }

        sleep(Duration::from_secs(1)).await;
        attempts += 1;
    }
}

async fn test_adb_connection(adb_device: &mut ADBUSBDevice) -> Result<()> {
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(&["echo", "test"], &mut buf)?;
    let output = String::from_utf8_lossy(&buf);
    if output.contains("test") {
        Ok(())
    } else {
        anyhow::bail!("ADB connection test failed")
    }
}

async fn install_rayhunter_files(adb_device: &mut ADBUSBDevice) -> Result<()> {
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(&["mkdir", "-p", "/data/rayhunter"], &mut buf)?;

    adb_device.shell_command(&["mount", "-o", "remount,rw", "/system"], &mut buf)?;

    install_busybox_symlinks(adb_device);

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    install_file(
        adb_device,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
    )?;

    let config_content = crate::CONFIG_TOML.replace("#device = \"orbic\"", "device = \"uz801\"");
    let mut config_data = config_content.as_bytes();
    adb_device.push(&mut config_data, &"/data/rayhunter/config.toml")?;

    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(
        &["chmod", "755", "/data/rayhunter/rayhunter-daemon"],
        &mut buf,
    )?;

    Ok(())
}

fn install_busybox_symlinks(adb_device: &mut ADBUSBDevice) {
    let mut buf = Vec::<u8>::new();
    if adb_device
        .shell_command(
            &["test", "-x", "/system/bin/cat", "&&", "echo", "found"],
            &mut buf,
        )
        .is_ok()
    {
        let output = String::from_utf8_lossy(&buf);
        if output.contains("found") {
            return;
        }
    }
    let mut buf = Vec::<u8>::new();
    let _ = adb_device.shell_command(&["busybox", "--install", "-s", "/system/bin"], &mut buf);
}

/// Transfer a file to the device's filesystem with adb push.
/// Validates the file sends successfully to /data/local/tmp
/// before overwriting the destination.
fn install_file(adb_device: &mut ADBUSBDevice, dest: &str, payload: &[u8]) -> Result<()> {
    const MAX_RETRIES: u32 = 3;

    let file_name = Path::new(dest)
        .file_name()
        .ok_or_else(|| anyhow!("{dest} does not have a file name"))?
        .to_str()
        .ok_or_else(|| anyhow!("{dest}'s file name is not UTF8"))?
        .to_owned();
    let push_tmp_path = format!("/data/local/tmp/{file_name}");
    let file_hash = md5_compute(payload);

    for attempt in 1..=MAX_RETRIES {
        let mut payload_copy = payload;
        if let Err(e) = adb_device.push(&mut payload_copy, &push_tmp_path) {
            if attempt == MAX_RETRIES {
                return Err(e.into());
            }
            continue;
        }

        let mut buf = Vec::<u8>::new();
        if adb_device
            .shell_command(&["busybox", "md5sum", &push_tmp_path], &mut buf)
            .is_ok()
        {
            let output = String::from_utf8_lossy(&buf);
            if output.contains(&format!("{file_hash:x}")) {
                let mut buf = Vec::<u8>::new();
                adb_device.shell_command(&["mv", &push_tmp_path, dest], &mut buf)?;
                println!("ok");
                return Ok(());
            }
        }

        if attempt < MAX_RETRIES {
            println!("MD5 verification failed on attempt {attempt}, retrying...");
            let mut buf = Vec::<u8>::new();
            adb_device
                .shell_command(&["rm", "-f", &push_tmp_path], &mut buf)
                .ok();
        }
    }

    anyhow::bail!("MD5 verification failed for {dest} after {MAX_RETRIES} attempts")
}

fn find_startup_script(adb_device: &mut ADBUSBDevice) -> Result<String> {
    for path in STARTUP_SCRIPTS {
        let mut buf = Vec::<u8>::new();
        if adb_device
            .shell_command(&["test", "-f", path, "&&", "echo", "found"], &mut buf)
            .is_ok()
        {
            let output = String::from_utf8_lossy(&buf);
            if output.contains("found") {
                return Ok(path.to_string());
            }
        }
    }
    anyhow::bail!(
        "Could not find a startup script to modify.\n\
         Checked: {}\n\
         You may need to start rayhunter manually or add it to your device's init script.",
        STARTUP_SCRIPTS.join(", ")
    )
}

fn kill_diag_competitors(adb_device: &mut ADBUSBDevice) {
    for name in DIAG_COMPETITORS {
        let mut buf = Vec::<u8>::new();
        let _ = adb_device.shell_command(&["pkill", "-f", name], &mut buf);
    }
}

async fn modify_startup_script(adb_device: &mut ADBUSBDevice) -> Result<()> {
    let script_path = find_startup_script(adb_device)?;

    let mut script_content = Vec::<u8>::new();
    adb_device.pull(&script_path, &mut script_content)?;

    let script_str = String::from_utf8_lossy(&script_content).into_owned();
    let mut lines: Vec<&str> = script_str.lines().collect();

    let original_len = lines.len();
    lines.retain(|line| {
        let trimmed = line.trim();
        !DIAG_COMPETITORS
            .iter()
            .any(|competitor| trimmed.contains(competitor))
    });
    if lines.len() < original_len {
        println!("removed competing DIAG service entries from {script_path}");
    }

    let has_rayhunter = lines
        .iter()
        .any(|l| l.contains("/data/rayhunter/rayhunter-daemon"));
    if !has_rayhunter {
        lines.push("/data/rayhunter/rayhunter-daemon /data/rayhunter/config.toml &");
    }

    let mut modified = lines.join("\n");
    if !modified.ends_with('\n') {
        modified.push('\n');
    }

    let mut modified_bytes = modified.as_bytes();
    adb_device.push(&mut modified_bytes, &script_path)?;

    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(&["chmod", "755", &script_path], &mut buf)?;

    Ok(())
}
