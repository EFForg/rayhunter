//! Serial communication with the orbic device
//!
//! This binary has two main functions, putting the orbic device in update mode which enables ADB
//! and running AT commands on the serial modem interface which can be used to upload a shell and chown it to root
//!
//! # Panics
//!
//! No device found - make sure your device is plugged in and turned on. If it is, it's possible you have a device with a different
//! usb id, file a bug with the output of `lsusb` attached.
//!
//! # Examples
//! ```
//! match rusb::Context::new() {
//!	    Ok(mut context) => match open_orbic(&mut context) {
//!	    Some(mut handle) => {
//!		send_command(&mut handle, &args[1])
//!	    },
//!	    None => panic!("No Orbic device found"),
//!	},
//!	Err(e) => panic!("Failed to initialize libusb: {0}", e),
//! ````
use std::str;
use std::thread::sleep;
use std::time::Duration;

use rusb::{Context, DeviceHandle, UsbContext};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: {0} <command>", args[0]);
        return;
    }

    match Context::new() {
        Ok(mut context) => match open_orbic(&mut context) {
            Some(mut handle) => {
                if &args[1] != "--root" {
                    send_command(&mut handle, &args[1])
                }
            }
            None => panic!("No Orbic device found"),
        },
        Err(e) => panic!("Failed to initialize libusb: {0}", e),
    }
}
/// Sends an AT command to the usb device over the serial port
///
/// First establish a USB handle and context by calling `open_orbic(<T>)
fn send_command<T: UsbContext>(handle: &mut DeviceHandle<T>, command: &str) {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    let timeout = Duration::from_secs(1);
    let mut response = [0; 256];

    // Set up the serial port appropriately
    handle
        .write_control(0x21, 0x22, 3, 1, &[], timeout)
        .expect("Failed to send control request");

    // Send the command
    handle
        .write_bulk(0x2, data.as_bytes(), timeout)
        .expect("Failed to write command");

    // Consume the echoed command
    handle
        .read_bulk(0x82, &mut response, timeout)
        .expect("Failed to read submitted command");

    // Read the actual response
    handle
        .read_bulk(0x82, &mut response, timeout)
        .expect("Failed to read response");

    let responsestr = str::from_utf8(&response).expect("Failed to parse response");
    if !responsestr.starts_with("\r\nOK\r\n") {
        println!("Received unexpected response{0}", responsestr)
    }
}

/// Send a command to switch the device into generic mode, exposing serial
///
/// If the device reboots while the command is still executing you may get a pipe error here, not sure what to do about this race condition.
fn switch_device<T: UsbContext>(handle: &mut DeviceHandle<T>) {
    let timeout = Duration::from_secs(1);

    if let Err(e) = handle.write_control(0x40, 0xa0, 0, 0, &[], timeout) {
        // If the device reboots while the command is still executing we
        // may get a pipe error here
        if e == rusb::Error::Pipe {
            return;
        }
        panic!("Failed to send device switch control request: {0}", e)
    }
}

/// Get a handle and contet for the orbic device
///
/// If the device isn't already in command mode this function will call swtich_device to switch it into command mode
fn open_orbic<T: UsbContext>(context: &mut T) -> Option<DeviceHandle<T>> {
    // Device after initial mode switch
    if let Some(handle) = open_device(context, 0x05c6, 0xf601) {
        return Some(handle);
    }

    // Device with rndis enabled as well
    if let Some(handle) = open_device(context, 0x05c6, 0xf622) {
        return Some(handle);
    }

    // Device in out-of-the-box state, need to switch to diag mode
    match open_device(context, 0x05c6, 0xf626) {
        Some(mut handle) => switch_device(&mut handle),
        None => panic!("No Orbic device detected"),
    }

    for _ in 1..10 {
        if let Some(handle) = open_device(context, 0x05c6, 0xf601) {
            return Some(handle);
        }
        sleep(Duration::from_secs(10))
    }
    panic!("No Orbic device detected")
}

/// Generic function to open a USB device
fn open_device<T: UsbContext>(context: &mut T, vid: u16, pid: u16) -> Option<DeviceHandle<T>> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some(handle),
                Err(e) => panic!("device found but failed to open: {}", e),
            }
        }
    }

    None
}
