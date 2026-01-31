#[cfg(target_os = "windows")]
use std::io::stdin;

use std::io::ErrorKind;
use std::path::Path;
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice, RustADBError};
use anyhow::{Context, Result, anyhow, bail};
use nusb::Interface;
use nusb::transfer::{Control, ControlType, Recipient, RequestBuffer};
use sha2::{Digest, Sha256};
use tokio::time::sleep;

use crate::RAYHUNTER_DAEMON_INIT;
use crate::connection::{DeviceConnection, install_config};
use crate::output::{print, println};
use crate::util::open_usb_device;

pub const ORBIC_NOT_FOUND: &str = r#"No Orbic device found.
Make sure your device is plugged in and turned on.

If you're sure you've plugged in an Orbic device via USB, there may be a bug in
our installer. Please file a bug with the output of `lsusb` attached."#;

const ORBIC_BUSY: &str = r#"The Orbic is plugged in but is being used by another program.

Please close any program that might be using your USB devices.
If you have adb installed you may need to kill the adb daemon"#;

#[cfg(any(target_os = "macos", target_os = "windows"))]
const ORBIC_BUSY_MAC: &str = r#"Permission denied.

On macOS or windows this might be caused by another program using the Orbic.
Please close any program that might be using your Orbic.
If you have adb installed you may need to kill the adb daemon"#;

#[cfg(target_os = "windows")]
const WINDOWS_WARNING: &str = r#""WINDOWS IS NOT FULLY SUPPORTED

THIS MAY BRICK YOUR DEVICE

PLEASE INSTALL FROM MACOS OR LINUX INSTEAD IF POSSIBLE"#;

const VENDOR_ID: u16 = 0x05c6;
const PRODUCT_ID: u16 = 0xf601;

const INTERFACE: u8 = 1;

/// ADB-based connection wrapper for DeviceConnection trait
pub struct AdbConnection<'a> {
    device: &'a mut ADBUSBDevice,
}

impl DeviceConnection for AdbConnection<'_> {
    async fn run_command(&mut self, command: &str) -> Result<String> {
        adb_command(self.device, &["sh", "-c", command])
    }

    async fn write_file(&mut self, path: &str, content: &[u8]) -> Result<()> {
        install_file(self.device, path, content).await
    }
}

#[cfg(target_os = "windows")]
const RNDIS_INTERFACE: u8 = 0;

#[cfg(not(target_os = "windows"))]
const RNDIS_INTERFACE: u8 = 1;

#[cfg(target_os = "windows")]
async fn confirm() -> Result<bool> {
    println!("{}", WINDOWS_WARNING);
    print!("Do you wish to proceed? Enter 'yes' to install> ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim() == "yes")
}

pub async fn install(reset_config: bool) -> Result<()> {
    println!(
        "WARNING: The orbic USB installer is not recommended for most usecases. Consider using ./installer orbic instead, unless you want ADB access for other purposes."
    );

    #[cfg(target_os = "windows")]
    {
        let confirmation = confirm().await?;
        if confirmation != true {
            println!("Install aborted. Your device has not been modified.");
            return Ok(());
        }
    }

    let mut adb_device = force_debug_mode().await?;
    print!("Installing rootshell... ");
    setup_rootshell(&mut adb_device).await?;
    println!("done");
    print!("Installing rayhunter... ");
    let mut adb_device = setup_rayhunter(adb_device, reset_config).await?;
    println!("done");
    print!("Testing rayhunter... ");
    test_rayhunter(&mut adb_device).await?;
    println!("done");
    Ok(())
}

pub async fn shell() -> Result<()> {
    println!(
        "WARNING: The orbic USB installer is not recommended for most usecases. Consider using ./installer util orbic-shell instead, unless you want ADB access for other purposes."
    );

    println!("opening shell");
    let mut adb_device = get_adb().await?;
    adb_device.shell(&mut std::io::stdin(), Box::new(std::io::stdout()))?;
    Ok(())
}

async fn force_debug_mode() -> Result<ADBUSBDevice> {
    println!("Forcing a switch into the debug mode to enable ADB");
    enable_command_mode()?;
    print!("ADB enabled, waiting for reboot... ");
    let mut adb_device = get_adb().await?;
    adb_setup_serial(&mut adb_device).await?;
    println!("it's alive!");
    print!("Waiting for atfwd_daemon to startup... ");
    adb_command(&mut adb_device, &["pgrep", "atfwd_daemon"])?;
    println!("done");
    Ok(adb_device)
}

async fn setup_rootshell(adb_device: &mut ADBUSBDevice) -> Result<()> {
    let rootshell_bin = include_bytes!(env!("FILE_ROOTSHELL"));

    install_file(adb_device, "/bin/rootshell", rootshell_bin).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    adb_at_syscmd(adb_device, "chown root /bin/rootshell").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    adb_at_syscmd(adb_device, "chmod 4755 /bin/rootshell").await?;
    let output = adb_command(adb_device, &["/bin/rootshell", "-c", "id"])?;
    if !output.contains("uid=0") {
        bail!("rootshell is not giving us root.");
    }
    Ok(())
}

async fn setup_rayhunter(mut adb_device: ADBUSBDevice, reset_config: bool) -> Result<ADBUSBDevice> {
    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));

    adb_at_syscmd(&mut adb_device, "mkdir -p /data/rayhunter").await?;
    install_file(
        &mut adb_device,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
    )
    .await?;

    {
        let mut conn = AdbConnection {
            device: &mut adb_device,
        };
        install_config(
            &mut conn,
            "/data/rayhunter/config.toml",
            "orbic",
            reset_config,
        )
        .await?;
    }

    install_file(
        &mut adb_device,
        "/etc/init.d/rayhunter_daemon",
        RAYHUNTER_DAEMON_INIT.as_bytes(),
    )
    .await?;
    install_file(
        &mut adb_device,
        "/etc/init.d/misc-daemon",
        include_bytes!("../../dist/scripts/misc-daemon"),
    )
    .await?;
    adb_at_syscmd(&mut adb_device, "chmod 755 /etc/init.d/rayhunter_daemon").await?;
    adb_at_syscmd(&mut adb_device, "chmod 755 /etc/init.d/misc-daemon").await?;
    println!("done");
    print!("Waiting for reboot... ");
    adb_at_syscmd(&mut adb_device, "shutdown -r -t 1 now").await?;
    // first wait for shutdown (it can take ~10s)
    tokio::time::timeout(Duration::from_secs(30), async {
        while let Ok(dev) = adb_echo_test(adb_device).await {
            adb_device = dev;
            sleep(Duration::from_secs(1)).await;
        }
    })
    .await
    .context("Orbic took too long to shutdown")?;
    // now wait for boot to finish
    get_adb().await
}

/// Test rayhunter on the device over adb without forwarding.
pub async fn test_rayhunter(adb_device: &mut ADBUSBDevice) -> Result<()> {
    const MAX_FAILURES: u32 = 10;
    let mut failures = 0;
    while failures < MAX_FAILURES {
        if let Ok(output) = adb_command(
            adb_device,
            &["wget", "-O", "-", "http://localhost:8080/index.html"],
        ) && output.contains("html")
        {
            return Ok(());
        }
        failures += 1;
        sleep(Duration::from_secs(3)).await;
    }
    bail!("timeout reached! failed to reach rayhunter, something went wrong :(")
}

async fn install_file(adb_device: &mut ADBUSBDevice, dest: &str, payload: &[u8]) -> Result<()> {
    const MAX_FAILURES: u32 = 5;
    let mut failures = 0;
    loop {
        match install_file_impl(adb_device, dest, payload).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                if failures > MAX_FAILURES {
                    return Err(e);
                } else {
                    sleep(Duration::from_secs(1)).await;
                    failures += 1;
                }
            }
        }
    }
}

async fn install_file_impl(
    adb_device: &mut ADBUSBDevice,
    dest: &str,
    mut payload: &[u8],
) -> Result<()> {
    let file_name = Path::new(dest)
        .file_name()
        .ok_or_else(|| anyhow!("{dest} does not have a file name"))?
        .to_str()
        .ok_or_else(|| anyhow!("{dest}'s file name is not UTF8"))?
        .to_owned();
    let push_tmp_path = format!("/tmp/{file_name}");
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let file_hash_bytes = hasher.finalize();
    let file_hash = format!("{file_hash_bytes:x}");
    adb_device.push(&mut payload, &push_tmp_path)?;
    adb_at_syscmd(adb_device, &format!("mv {push_tmp_path} {dest}")).await?;
    let file_info = adb_device
        .stat(dest)
        .context("Failed to stat transfered file")?;
    if file_info.file_size == 0 {
        bail!("File transfer unsuccessful\nFile is empty");
    }
    let output = adb_command(adb_device, &["sha256sum", dest])?;
    if !output.contains(&file_hash) {
        bail!("File transfer unsuccessful\nBad hash expected {file_hash} got {output}");
    }
    Ok(())
}

fn adb_command(adb_device: &mut ADBUSBDevice, command: &[&str]) -> Result<String> {
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(command, &mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// Creates an ADB interface instance.
///
/// This function waits for the ADB device then checks that an ADB shell command runs.
async fn get_adb() -> Result<ADBUSBDevice> {
    const MAX_FAILURES: u32 = 10;
    let mut failures = 0;
    loop {
        match ADBUSBDevice::new(VENDOR_ID, PRODUCT_ID) {
            Ok(dev) => match adb_echo_test(dev).await {
                Ok(dev) => return Ok(dev),
                Err(e) => {
                    if failures > MAX_FAILURES {
                        return Err(e);
                    } else {
                        sleep(Duration::from_secs(1)).await;
                        failures += 1;
                    }
                }
            },
            Err(RustADBError::IOError(e)) if e.kind() == ErrorKind::ResourceBusy => {
                bail!(ORBIC_BUSY);
            }
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            Err(RustADBError::IOError(e)) if e.kind() == ErrorKind::PermissionDenied => {
                bail!(ORBIC_BUSY_MAC);
            }
            Err(RustADBError::DeviceNotFound(_)) => {
                tokio::time::timeout(
                    Duration::from_secs(30),
                    wait_for_usb_device(VENDOR_ID, PRODUCT_ID),
                )
                .await
                .context("Timeout waiting for Orbic to reconnect")??;
            }
            Err(e) => {
                if failures > MAX_FAILURES {
                    return Err(e.into());
                } else {
                    sleep(Duration::from_secs(1)).await;
                    failures += 1;
                }
            }
        }
    }
}

async fn adb_echo_test(mut adb_device: ADBUSBDevice) -> Result<ADBUSBDevice> {
    let mut buf = Vec::<u8>::new();
    // Random string to echo
    let test_echo = "qwertyzxcvbnm";
    let thread = std::thread::spawn(move || {
        // This call to run a shell command is run on a separate thread because it can block
        // indefinitely until the command runs, which is undesirable.
        adb_device.shell_command(&["echo", test_echo], &mut buf)?;
        Ok::<(ADBUSBDevice, Vec<u8>), RustADBError>((adb_device, buf))
    });
    sleep(Duration::from_secs(1)).await;
    if thread.is_finished()
        && let Ok(Ok((dev, buf))) = thread.join()
        && let Ok(s) = std::str::from_utf8(&buf)
        && s.contains(test_echo)
    {
        return Ok(dev);
    }
    //  I'd like to kill the background thread here if that was possible.
    bail!("Could not communicate with the Orbic. Try disconnecting and reconnecting.");
}

#[cfg(not(target_os = "macos"))]
async fn wait_for_usb_device(vendor_id: u16, product_id: u16) -> Result<()> {
    use nusb::hotplug::HotplugEvent;
    use tokio_stream::StreamExt;
    loop {
        let mut watcher = nusb::watch_devices()?;
        while let Some(event) = watcher.next().await {
            if let HotplugEvent::Connected(dev) = event
                && dev.vendor_id() == vendor_id
                && dev.product_id() == product_id
            {
                return Ok(());
            }
        }
    }
}

#[cfg(target_os = "macos")]
/// `nusb::watch_devices` doesn't appear to work on macOS to poll instead.
async fn wait_for_usb_device(vendor_id: u16, product_id: u16) -> Result<()> {
    loop {
        for device_info in nusb::list_devices()? {
            if device_info.vendor_id() == vendor_id && device_info.product_id() == product_id {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn adb_setup_serial(adb_device: &mut ADBUSBDevice) -> Result<()> {
    Ok(adb_device.get_transport_mut().claim_interface(INTERFACE)?)
}

async fn adb_at_syscmd(adb_device: &mut ADBUSBDevice, command: &str) -> Result<()> {
    adb_serial_cmd(adb_device, &format!("AT+SYSCMD={command}")).await
}

async fn adb_serial_cmd(adb_device: &mut ADBUSBDevice, command: &str) -> Result<()> {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    let timeout = Duration::from_secs(2);
    let mut response = [0; 256];

    // Set up the serial port appropriately
    adb_device
        .get_transport_mut()
        .send_usb_class_control_msg(INTERFACE, 0x22, 3, 1, &[], timeout)
        .context("Failed to send control request")?;

    // Send the command
    adb_device
        .get_transport_mut()
        .usb_bulk_write(INTERFACE, 0x2, data.as_bytes(), timeout)
        .context("Failed to write command")?;

    // Consume the echoed command
    adb_device
        .get_transport_mut()
        .usb_bulk_read(INTERFACE, 0x82, &mut response, timeout)
        .context("Failed to read submitted command")?;

    // Read the actual response
    adb_device
        .get_transport_mut()
        .usb_bulk_read(INTERFACE, 0x82, &mut response, timeout)
        .context("Failed to read response")?;

    // For some reason, on macOS the response buffer gets filled with garbage data that's
    // rarely valid UTF-8. Luckily we only care about the first couple bytes, so just drop
    // the garbage with `from_utf8_lossy` and look for our expected success string.
    let responsestr = String::from_utf8_lossy(&response);
    if !responsestr.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", responsestr);
    }

    Ok(())
}

/// Sends an AT command to the usb device over the serial port
///
/// First establish a USB handle and context by calling `open_orbic()`
pub async fn send_serial_cmd(interface: &Interface, command: &str) -> Result<()> {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    let timeout = Duration::from_secs(2);

    let enable_serial_port = Control {
        control_type: ControlType::Class,
        recipient: Recipient::Interface,
        request: 0x22,
        value: 3,
        index: 1,
    };

    // Set up the serial port appropriately
    interface
        .control_out_blocking(enable_serial_port, &[], timeout)
        .context("Failed to send control request")?;

    // Send the command
    tokio::time::timeout(timeout, interface.bulk_out(0x2, data.as_bytes().to_vec()))
        .await
        .context("Timed out writing command")?
        .into_result()
        .context("Failed to write command")?;

    // Consume the echoed command
    tokio::time::timeout(timeout, interface.bulk_in(0x82, RequestBuffer::new(256)))
        .await
        .context("Timed out reading submitted command")?
        .into_result()
        .context("Failed to read submitted command")?;

    // Read the actual response
    let response = tokio::time::timeout(timeout, interface.bulk_in(0x82, RequestBuffer::new(256)))
        .await
        .context("Timed out reading response")?
        .into_result()
        .context("Failed to read response")?;

    // For some reason, on macOS the response buffer gets filled with garbage data that's
    // rarely valid UTF-8. Luckily we only care about the first couple bytes, so just drop
    // the garbage with `from_utf8_lossy` and look for our expected success string.
    let responsestr = String::from_utf8_lossy(&response);
    if !responsestr.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", responsestr);
    }

    Ok(())
}

/// Send a command to switch the device into generic mode, exposing serial
///
/// If the device reboots while the command is still executing you may get a pipe error here, not sure what to do about this race condition.
pub fn enable_command_mode() -> Result<()> {
    if open_orbic()?.is_some() {
        println!("Device already in command mode. Doing nothing...");
        return Ok(());
    }

    let timeout = Duration::from_secs(1);

    if let Some(device) = open_usb_device(VENDOR_ID, 0xf626)? {
        let enable_command_mode = Control {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0xa0,
            value: 0,
            index: 0,
        };
        let interface = device
            .detach_and_claim_interface(RNDIS_INTERFACE)
            .context("detach_and_claim_interface(1) failed")?;
        if let Err(e) = interface.control_out_blocking(enable_command_mode, &[], timeout) {
            // If the device reboots while the command is still executing we
            // may get a pipe error here
            if e == nusb::transfer::TransferError::Stall {
                return Ok(());
            }
            bail!("Failed to send device switch control request: {0}", e)
        }
        return Ok(());
    }

    bail!(ORBIC_NOT_FOUND);
}

/// Get an Interface for the orbic device
pub fn open_orbic() -> Result<Option<Interface>> {
    // Device after initial mode switch
    if let Some(device) = open_usb_device(VENDOR_ID, PRODUCT_ID)? {
        let interface = device
            .detach_and_claim_interface(INTERFACE) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    // Device with rndis enabled as well
    if let Some(device) = open_usb_device(VENDOR_ID, 0xf622)? {
        let interface = device
            .detach_and_claim_interface(INTERFACE) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    Ok(None)
}
