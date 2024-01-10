use std::str;
use std::thread::sleep;
use std::time::Duration;

use rusb::{
    Context, DeviceHandle, UsbContext,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: {0} <command>", args[0]);
        return;
    }

    match Context::new() {
	Ok(mut context) => match open_orbic(&mut context) {
	    Some(mut handle) => {
		send_command(&mut handle, &args[1])
	    },
	    None => panic!("No Orbic device found"),
	},
	Err(e) => panic!("Failed to initialize libusb: {0}", e),
    }
}

fn send_command<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    command: &String,
) {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    let timeout = Duration::from_secs(1);
    let mut response = [0; 256];
    match handle.write_control(0x21, 0x22, 3, 1, &[], timeout) {
	Ok(_) => match handle.write_bulk(0x2, data.as_bytes(), timeout) {
	    // Consume the echoed back command
	    Ok(_) => match handle.read_bulk(0x82, &mut response, timeout) {
		// Read the actual response
		Ok(_) => match handle.read_bulk(0x82, &mut response, timeout) {
		    Ok(_) => {
			let responsestr = str::from_utf8(&response).unwrap();
			if !responsestr.starts_with("\r\nOK\r\n") {
			    println!("Received unexpected response{0}", responsestr);
			}
		    },
		    Err(e) => panic!("Failed to read response: {0}", e),
		},
		Err(e) => panic!("Failed to read submitted command: {0}", e),
	    }
	    Err(e) => panic!("Failed to write command: {0}", e),
	},
	Err(e) => panic!("Failed to send control request: {0}", e),
    }
}

fn switch_device<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
) {
    // Send a command to switch the device into generic mode, exposing serial
    let timeout = Duration::from_secs(1);
    match handle.write_control(0x40, 0xa0, 0, 0, &[], timeout) {
	Ok(_) => (),
	Err(e) => {
	    // If the device reboots while the command is still executing we
	    // may get a pipe error here
	    if e == rusb::Error::Pipe {
		return;
	    }
	    panic!("Failed to send device switch control request: {0}", e);
	},
    }
}

fn open_orbic<T: UsbContext>(
    context: &mut T,
) -> Option<DeviceHandle<T>> {
    // Device after initial mode switch
    match open_device(context, 0x05c6, 0xf601) {
	Some(handle) => return Some(handle),
	None => (),
    }
    // Device with rndis enabled as well
    match open_device(context, 0x05c6, 0xf622) {
	Some(handle) => return Some(handle),
	None => (),
    }

    // Device in out-of-the-box state, need to switch to diag mode
    match open_device(context, 0x05c6, 0xf626) {
	Some(mut handle) => switch_device(&mut handle),
	None => panic!("No Orbic device detected")
    }

    for _ in 1..10 {
	match open_device(context, 0x05c6, 0xf601) {
	    Some(handle) => return Some(handle),
	    None => (),
	}
	sleep(Duration::from_secs(10))
    }
    panic!("No Orbic device detected")
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<DeviceHandle<T>> {
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
