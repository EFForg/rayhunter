//! Serial communication with the orbic device
//!
//! This binary has two main functions, putting the orbic device in update mode which enables ADB
//! and running AT commands on the serial modem interface which can be used to upload a shell and chown it to root
//!
//! # Errors
//!
//! No device found - make sure your device is plugged in and turned on. If it is, it's possible you have a device with a different
//! usb id, file a bug with the output of `lsusb` attached.
use std::str;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use nusb::transfer::{Control, ControlType, Recipient, RequestBuffer};
use nusb::{Device, Interface};

const ORBIC_NOT_FOUND: &str = r#"No Orbic device found.
Make sure your device is plugged in and turned on.

If it's possible you have a device with a different usb id:
please file a bug with the output of `lsusb` attached."#;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        println!("usage: {0} [<command> | --root]", args[0]);
        std::process::exit(1);
    }

    if args[1] == "--root" {
        enable_command_mode()
    } else {
        match open_orbic()? {
            Some(interface) => send_command(interface, &args[1]).await,
            None => bail!(ORBIC_NOT_FOUND),
        }
    }
}

/// Sends an AT command to the usb device over the serial port
///
/// First establish a USB handle and context by calling `open_orbic(<T>)
async fn send_command(interface: Interface, command: &str) -> Result<()> {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    let timeout = Duration::from_secs(1);

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
        println!("Received unexpected response: {0}", responsestr);
        std::process::exit(1);
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

    if let Some(interface) = open_device(0x05c6, 0xf626)? {
        let enable_command_mode = Control {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0xa0,
            value: 0,
            index: 0,
        };
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
    if let Some(device) = open_device(0x05c6, 0xf601)? {
        let interface = device
            .detach_and_claim_interface(1) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    // Device with rndis enabled as well
    if let Some(device) = open_device(0x05c6, 0xf622)? {
        let interface = device
            .detach_and_claim_interface(1) // will reattach drivers on release
            .context("detach_and_claim_interface(1) failed")?;
        return Ok(Some(interface));
    }

    Ok(None)
}

/// General function to open a USB device
fn open_device(vid: u16, pid: u16) -> Result<Option<Device>> {
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
