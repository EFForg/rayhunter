use std::io::{ErrorKind, Write};
use std::path::Path;
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice, RustADBError};
use anyhow::{Context, Result, anyhow, bail};
use nusb::hotplug::HotplugEvent;
use nusb::transfer::{Control, ControlType, Recipient, RequestBuffer};
use nusb::{Device, Interface};
use sha2::{Digest, Sha256};
use tokio::time::sleep;
use tokio_stream::StreamExt;

use crate::{CONFIG_TOML, RAYHUNTER_DAEMON_INIT};

const ORBIC_NOT_FOUND: &str = r#"No Orbic device found.
Make sure your device is plugged in and turned on.

If you're sure you've plugged in an Orbic device via USB, there may be a bug in
our installer. Please file a bug with the output of `lsusb` attached."#;

const ORBIC_BUSY: &str = r#"The Orbic is plugged in but is being used by another program.

Please close any program that might be using your USB devices.
If you have adb installed you may need to kill the adb daemon"#;

const VENDOR_ID: u16 = 0x05c6;
const PRODUCT_ID: u16 = 0xf601;

macro_rules! echo {
    ($($arg:tt)*) => {
        print!($($arg)*);
        let _ = std::io::stdout().flush();
    };
}

pub async fn install() -> Result<()> {
    let mut adb_device = force_debug_mode().await?;
    let serial_interface = open_orbic()?.ok_or_else(|| anyhow!(ORBIC_NOT_FOUND))?;
    echo!("Installing rootshell... ");
    setup_rootshell(&serial_interface, &mut adb_device).await?;
    println!("done");
    echo!("Installing rayhunter... ");
    let mut adb_device = setup_rayhunter(&serial_interface, adb_device).await?;
    println!("done");
    echo!("Testing rayhunter... ");
    test_rayhunter(&mut adb_device).await?;
    println!("done");
    Ok(())
}

async fn force_debug_mode() -> Result<ADBUSBDevice> {
    println!("Forcing a switch into the debug mode to enable ADB");
    enable_command_mode()?;
    echo!("ADB enabled, waiting for reboot... ");
    let mut adb_device = wait_for_adb_shell().await?;
    println!("it's alive!");
    echo!("Waiting for atfwd_daemon to startup... ");
    adb_command(&mut adb_device, &["pgrep", "atfwd_daemon"])?;
    println!("done");
    Ok(adb_device)
}

async fn setup_rootshell(
    serial_interface: &Interface,
    adb_device: &mut ADBUSBDevice,
) -> Result<()> {
    #[cfg(feature = "vendor")]
    let rootshell_bin = include_bytes!("../../rootshell/rootshell");

    #[cfg(not(feature = "vendor"))]
    let rootshell_bin = &tokio::fs::read("target/armv7-unknown-linux-musleabihf/release/rootshell")
        .await
        .context("Error reading rootshell from local file system")?;

    install_file(
        serial_interface,
        adb_device,
        "/bin/rootshell",
        rootshell_bin,
    )
    .await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    at_syscmd(serial_interface, "chown root /bin/rootshell").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    at_syscmd(serial_interface, "chmod 4755 /bin/rootshell").await?;
    let output = adb_command(adb_device, &["/bin/rootshell", "-c", "id"])?;
    if !output.contains("uid=0") {
        bail!("rootshell is not giving us root.");
    }
    Ok(())
}

async fn setup_rayhunter(
    serial_interface: &Interface,
    mut adb_device: ADBUSBDevice,
) -> Result<ADBUSBDevice> {
    #[cfg(feature = "vendor")]
    let rayhunter_daemon_bin = include_bytes!("../../rayhunter-daemon-orbic/rayhunter-daemon");

    #[cfg(not(feature = "vendor"))]
    let rayhunter_daemon_bin =
        &tokio::fs::read("target/armv7-unknown-linux-musleabihf/release/rayhunter-daemon")
            .await
            .context("Error reading rayhunter-daemon from local file system")?;

    at_syscmd(serial_interface, "mkdir -p /data/rayhunter").await?;
    install_file(
        serial_interface,
        &mut adb_device,
        "/data/rayhunter/rayhunter-daemon",
        rayhunter_daemon_bin,
    )
    .await?;
    install_file(
        serial_interface,
        &mut adb_device,
        "/data/rayhunter/config.toml",
        CONFIG_TOML,
    )
    .await?;
    install_file(
        serial_interface,
        &mut adb_device,
        "/etc/init.d/rayhunter_daemon",
        RAYHUNTER_DAEMON_INIT,
    )
    .await?;
    install_file(
        serial_interface,
        &mut adb_device,
        "/etc/init.d/misc-daemon",
        include_bytes!("../../dist/scripts/misc-daemon"),
    )
    .await?;
    at_syscmd(serial_interface, "chmod 755 /etc/init.d/rayhunter_daemon").await?;
    at_syscmd(serial_interface, "chmod 755 /etc/init.d/misc-daemon").await?;
    println!("done");
    echo!("Waiting for reboot... ");
    at_syscmd(serial_interface, "shutdown -r -t 1 now").await?;
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
    let adb_device = wait_for_adb_shell().await?;
    Ok(adb_device)
}

async fn test_rayhunter(adb_device: &mut ADBUSBDevice) -> Result<()> {
    const MAX_FAILURES: u32 = 10;
    let mut failures = 0;
    while failures < MAX_FAILURES {
        if let Ok(output) = adb_command(
            adb_device,
            &["wget", "-O", "-", "http://localhost:8080/index.html"],
        ) {
            if output.contains("html") {
                return Ok(());
            }
        }
        failures += 1;
        sleep(Duration::from_secs(3)).await;
    }
    bail!("timeout reached! failed to reach rayhunter, something went wrong :(")
}

async fn install_file(
    serial_interface: &Interface,
    adb_device: &mut ADBUSBDevice,
    dest: &str,
    payload: &[u8],
) -> Result<()> {
    const MAX_FAILURES: u32 = 5;
    let mut failures = 0;
    loop {
        match install_file_impl(serial_interface, adb_device, dest, payload).await {
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
    serial_interface: &Interface,
    adb_device: &mut ADBUSBDevice,
    dest: &str,
    payload: &[u8],
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
    #[allow(clippy::useless_asref)]
    adb_device.push(&mut payload.as_ref(), &push_tmp_path)?;
    at_syscmd(serial_interface, &format!("mv {push_tmp_path} {dest}")).await?;
    let file_info = adb_device
        .stat(dest)
        .context("Failed to stat transfered file")?;
    if file_info.file_size == 0 {
        bail!("File transfer unseccessful\nFile is empty");
    }
    let ouput = adb_command(adb_device, &["sha256sum", dest])?;
    if !ouput.contains(&file_hash) {
        bail!("File transfer unseccessful\nBad hash expected {file_hash} got {ouput}");
    }
    Ok(())
}

fn adb_command(adb_device: &mut ADBUSBDevice, command: &[&str]) -> Result<String> {
    let mut buf = Vec::<u8>::new();
    adb_device.shell_command(command, &mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

async fn wait_for_adb_shell() -> Result<ADBUSBDevice> {
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
            Err(RustADBError::DeviceNotFound(_)) => {
                wait_for_usb_device(VENDOR_ID, PRODUCT_ID).await?;
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
    if thread.is_finished() {
        if let Ok(Ok((dev, buf))) = thread.join() {
            if let Ok(s) = std::str::from_utf8(&buf) {
                if s.contains(test_echo) {
                    return Ok(dev);
                }
            }
        }
    }
    //  I'd like to kill the background thread here if that was possible.
    bail!("Could not communicate with the Orbic. Try disconnecting and reconnecting.");
}

async fn wait_for_usb_device(vendor_id: u16, product_id: u16) -> Result<()> {
    loop {
        let mut watcher = nusb::watch_devices()?;
        while let Some(event) = watcher.next().await {
            if let HotplugEvent::Connected(dev) = event {
                if dev.vendor_id() == vendor_id && dev.product_id() == product_id {
                    return Ok(());
                }
            }
        }
    }
}

/// Sends an AT command to the usb device over the serial port
///
/// First establish a USB handle and context by calling `open_orbic(<T>)
async fn at_syscmd(interface: &Interface, command: &str) -> Result<()> {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(&format!("AT+SYSCMD={command}"));
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
fn enable_command_mode() -> Result<()> {
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
            .detach_and_claim_interface(1)
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
fn open_orbic() -> Result<Option<Interface>> {
    // Device after initial mode switch
    if let Some(device) = open_usb_device(VENDOR_ID, PRODUCT_ID)? {
        let interface = device
            .detach_and_claim_interface(1) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    // Device with rndis enabled as well
    if let Some(device) = open_usb_device(VENDOR_ID, 0xf622)? {
        let interface = device
            .detach_and_claim_interface(1) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    Ok(None)
}

/// General function to open a USB device
fn open_usb_device(vid: u16, pid: u16) -> Result<Option<Device>> {
    let devices = match nusb::list_devices() {
        Ok(d) => d,
        Err(_) => return Ok(None),
    };

    for device in devices {
        if device.vendor_id() == vid && device.product_id() == pid {
            match device.open() {
                Ok(d) => return Ok(Some(d)),
                Err(e) => bail!("device found but failed to open: {}", e),
            }
        }
    }

    Ok(None)
}
