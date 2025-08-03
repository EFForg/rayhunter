/// Installer for the Uz801 hotspot.
///
/// Installation process:
/// 1. Use curl to activate USB debugging backdoor
/// 2. Wait for device reboot and ADB availability
/// 3. Use ADB to install rayhunter files
/// 4. Modify startup script to launch rayhunter on boot
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice, RustADBError};
use anyhow::Result;
use std::io::ErrorKind;
use tokio::time::sleep;

use crate::Uz801Args as Args;
use crate::util::echo;

pub async fn install(
    Args {
        admin_ip,
    }: Args,
) -> Result<()> {
    run_install(admin_ip).await
}

async fn run_install(admin_ip: String) -> Result<()> {
    echo!("Activating USB debugging backdoor... ");
    activate_usb_debug(&admin_ip).await?;
    println!("ok");

    echo!("Waiting for device reboot and ADB connection... ");
    let mut adb_device = wait_for_adb().await?;
    println!("ok");

    echo!("Installing rayhunter files... ");
    install_rayhunter_files(&mut adb_device).await?;
    println!("ok");

    echo!("Modifying startup script... ");
    modify_startup_script(&mut adb_device).await?;
    println!("ok");

    echo!("Starting rayhunter daemon... ");
    start_rayhunter(&mut adb_device).await?;
    println!("ok");

    echo!("Testing rayhunter... ");
    test_rayhunter(&admin_ip).await?;
    println!("ok");
    println!("rayhunter is running at http://{admin_ip}:8080");

    Ok(())
}

pub async fn activate_usb_debug(admin_ip: &str) -> Result<()> {
    let url = format!("http://{}/usbdebug.html", admin_ip);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to activate USB debug: HTTP {}", response.status());
    }

    Ok(())
}

async fn wait_for_adb() -> Result<ADBUSBDevice> {
    const MAX_ATTEMPTS: u32 = 30; // 30 seconds
    let mut attempts = 0;

    // Wait a bit for the reboot to start
    sleep(Duration::from_secs(5)).await;

    loop {
        if attempts >= MAX_ATTEMPTS {
            anyhow::bail!("Timeout waiting for ADB connection after USB debug activation");
        }

        match ADBUSBDevice::new(0x05c6, 0x9025) {
            // Common Qualcomm ADB VID/PID
            Ok(mut device) => {
                // Test ADB connection
                if test_adb_connection(&mut device).await.is_ok() {
                    return Ok(device);
                }
            }
            Err(RustADBError::DeviceNotFound(_)) => {
                // Device not ready yet, continue waiting
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
    // Create rayhunter directory
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(&["mkdir", "-p", "/data/rayhunter"], &mut buf)?;

    // Install rayhunter daemon binary
    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    let mut daemon_data = rayhunter_daemon_bin.as_slice();
    adb_device.push(&mut daemon_data, &"/data/rayhunter/rayhunter-daemon")?;

    // Install config file
    let config_content = crate::CONFIG_TOML.replace("#device = \"orbic\"", "device = \"uz801\"");
    let mut config_data = config_content.as_bytes();
    adb_device.push(&mut config_data, &"/data/rayhunter/config.toml")?;

    // Make daemon executable
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(
        &["chmod", "755", "/data/rayhunter/rayhunter-daemon"],
        &mut buf,
    )?;

    Ok(())
}

async fn modify_startup_script(adb_device: &mut ADBUSBDevice) -> Result<()> {
    // Pull the existing startup script
    let mut script_content = Vec::<u8>::new();
    adb_device.pull(&"/system/bin/initmifiservice.sh", &mut script_content)?;

    // Convert to string and add our line
    let mut script_str = String::from_utf8_lossy(&script_content).into_owned();

    // Add rayhunter startup line if not already present
    let rayhunter_line = "/data/rayhunter/rayhunter-daemon /data/rayhunter/config.toml &\n";
    if !script_str.contains("/data/rayhunter/rayhunter-daemon") {
        script_str.push_str(rayhunter_line);
    }

    // Push the modified script back
    let mut modified_script = script_str.as_bytes();
    adb_device.push(&mut modified_script, &"/system/bin/initmifiservice.sh")?;

    // Make sure it's executable
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(
        &["chmod", "755", "/system/bin/initmifiservice.sh"],
        &mut buf,
    )?;

    Ok(())
}

async fn start_rayhunter(adb_device: &mut ADBUSBDevice) -> Result<()> {
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(
        &[
            "/data/rayhunter/rayhunter-daemon",
            "/data/rayhunter/config.toml",
            "&",
        ],
        &mut buf,
    )?;

    // Give it a moment to start
    sleep(Duration::from_secs(3)).await;

    Ok(())
}

async fn test_rayhunter(admin_ip: &str) -> Result<()> {
    const MAX_FAILURES: u32 = 10;
    let mut failures = 0;

    let client = reqwest::Client::new();

    while failures < MAX_FAILURES {
        let url = format!("http://{}:8080/index.html", admin_ip);

        if let Ok(response) = client.get(&url).send().await {
            if response.status().is_success() {
                if let Ok(body) = response.text().await {
                    if body.contains("html") {
                        return Ok(());
                    }
                }
            }
        }

        failures += 1;
        sleep(Duration::from_secs(3)).await;
    }

    anyhow::bail!("timeout reached! failed to reach rayhunter, something went wrong :(")
}
