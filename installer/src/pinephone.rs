use std::path::Path;
use std::time::Duration;

use adb_client::{ADBDeviceExt, ADBUSBDevice};
use anyhow::{Context, Result, anyhow, bail};
use md5::compute as md5_compute;
use md5crypt::md5crypt;
use nusb::Interface;
use nusb::transfer::{Control, ControlType, Recipient, RequestBuffer};
use tokio::time::sleep;

use crate::connection::DeviceConnection;
use crate::orbic::test_rayhunter;
use crate::output::{print, println};
use crate::util::open_usb_device;
use crate::{CONFIG_TOML, RAYHUNTER_DAEMON_INIT};

const USB_VENDOR_ID: u16 = 0x2C7C;
const USB_PRODUCT_ID: u16 = 0x125;
const USB_INTERFACE_NUMBER: u8 = 2;

pub async fn install() -> Result<()> {
    print!("Unlocking modem ... ");
    start_adb().await?;
    sleep(Duration::from_secs(3)).await;
    let mut adb = ADBUSBDevice::new(USB_VENDOR_ID, USB_PRODUCT_ID).unwrap();
    println!("ok");

    run_command_expect(&mut adb, "mount -o remount,rw /", "exit code 0").await?;
    run_command_expect(&mut adb, "mkdir -p /data/rayhunter", "exit code 0").await?;

    let rayhunter_daemon_bin = include_bytes!(env!("FILE_RAYHUNTER_DAEMON"));
    adb.write_file("/data/rayhunter/rayhunter-daemon", rayhunter_daemon_bin)
        .await?;
    adb.write_file(
        "/data/rayhunter/config.toml",
        CONFIG_TOML
            .replace("#device = \"orbic\"", "device = \"pinephone\"")
            .as_bytes(),
    )
    .await?;
    adb.write_file(
        "/etc/init.d/rayhunter_daemon",
        RAYHUNTER_DAEMON_INIT.as_bytes(),
    )
    .await?;
    adb.write_file(
        "/etc/init.d/misc-daemon",
        include_bytes!("../../dist/scripts/misc-daemon"),
    )
    .await?;
    run_command_expect(
        &mut adb,
        "chmod 755 /etc/init.d/rayhunter_daemon",
        "exit code 0",
    )
    .await?;
    run_command_expect(&mut adb, "chmod 755 /etc/init.d/misc-daemon", "exit code 0").await?;

    println!("Rebooting device and waiting 30 seconds for it to start up.");
    run_command_expect(&mut adb, "shutdown -r -t 1 now", "exit code 0").await?;
    sleep(Duration::from_secs(30)).await;

    print!("Unlocking modem ... ");
    start_adb().await?;
    sleep(Duration::from_secs(3)).await;
    let mut adb = ADBUSBDevice::new(USB_VENDOR_ID, USB_PRODUCT_ID).unwrap();
    println!("ok");

    print!("Testing rayhunter ... ");
    test_rayhunter(&mut adb).await?;
    println!("ok");
    println!("rayhunter is running on the modem. Use adb to access the web interface.");

    Ok(())
}

/// Helper to run a command and check for expected output
async fn run_command_expect(
    adb: &mut ADBUSBDevice,
    command: &str,
    expected_output: &str,
) -> Result<()> {
    let output = adb.run_command(command).await?;
    if !output.contains(expected_output) {
        bail!("{expected_output:?} not found in: {output}");
    }
    Ok(())
}

struct Qusbcfg {
    vendor_id: u16,
    product_id: u16,
    diag: u8,
    nmea: u8,
    at: u8,
    modem: u8,
    net: u8,
    adb: u8,
    audio: u8,
}

impl Default for Qusbcfg {
    fn default() -> Self {
        Qusbcfg {
            vendor_id: USB_VENDOR_ID,
            product_id: USB_PRODUCT_ID,
            diag: 1,
            nmea: 1,
            at: 1,
            modem: 1,
            net: 1,
            adb: 0,
            audio: 0,
        }
    }
}

impl std::fmt::Display for Qusbcfg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&format!(
            "AT+QCFG=\"usbcfg\",{:#X},{:#X},{},{},{},{},{},{},{}",
            self.vendor_id,
            self.product_id,
            self.diag,
            self.nmea,
            self.at,
            self.modem,
            self.net,
            self.adb,
            self.audio
        ))?;
        Ok(())
    }
}

/// Start the adb daemon on the Quectel modem.
/// A reimplementation of qadbkey-unlock.c by "igem, 2019 ;)"
pub async fn start_adb() -> Result<()> {
    let tty = serial_interface()?.unwrap();

    let get_qadbkey = tty
        .send_at_command("AT+QADBKEY?")
        .await
        .context("Failed to request QADBKEY")?;
    let resp = String::from_utf8_lossy(&get_qadbkey);
    if !resp.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", resp);
    }
    let salt = match resp.find("+QADBKEY: ") {
        Some(i) => &resp[i + 10..i + 18],
        None => bail!("Received unexpected response: {0}", resp),
    };

    let hashed = &md5crypt(b"SH_adb_quectel", salt.as_bytes())[12..28];
    let hashed = String::from_utf8_lossy(hashed);

    let unlock = tty
        .send_at_command(&format!("AT+QADBKEY=\"{hashed}\""))
        .await
        .context("Failed to send AT+QADBKEY")?;
    let resp = String::from_utf8_lossy(&unlock);
    if !resp.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", resp);
    }

    let adb_enable = Qusbcfg {
        adb: 1,
        ..Default::default()
    };
    let start_adb = tty
        .send_at_command(&adb_enable.to_string())
        .await
        .context("Failed to send enable adb command.")?;
    let resp = String::from_utf8_lossy(&start_adb);
    if !resp.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", resp);
    }

    Ok(())
}

/// Stop the adb daemon on the Quectel modem.
pub async fn stop_adb() -> Result<()> {
    let tty = serial_interface()?.unwrap();
    let adb_disable = Qusbcfg::default();
    let stop_adb = tty
        .send_at_command(&adb_disable.to_string())
        .await
        .context("Failed to disable adb.")?;
    let resp = String::from_utf8_lossy(&stop_adb);
    if !resp.contains("\r\nOK\r\n") {
        bail!("Received unexpected response: {0}", resp);
    }
    Ok(())
}

impl DeviceConnection for ADBUSBDevice {
    /// Run an adb shell command, append '; echo exit code $?' to the command and return output.
    async fn run_command(&mut self, command: &str) -> Result<String> {
        let mut buf = Vec::<u8>::new();
        let cmd = ["sh", "-c", &format!("{command}; echo exit code $?")];
        self.shell_command(&cmd, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    /// Transfer a file to the modem's filesystem with adb push.
    /// Validates the file sends successfully to /tmp before overwriting the destination.
    async fn write_file(&mut self, dest: &str, mut payload: &[u8]) -> Result<()> {
        print!("Sending file {dest} ... ");
        let file_name = Path::new(dest)
            .file_name()
            .ok_or_else(|| anyhow!("{dest} does not have a file name"))?
            .to_str()
            .ok_or_else(|| anyhow!("{dest}'s file name is not UTF8"))?
            .to_owned();
        let push_tmp_path = format!("/tmp/{file_name}");
        let file_hash = md5_compute(payload);
        self.push(&mut payload, &push_tmp_path)?;
        let output = self.run_command(&format!("md5sum {push_tmp_path}")).await?;
        if !output.contains(&format!("{file_hash:x}")) {
            bail!("{:x} not found in: {output}", file_hash);
        }
        let output = self
            .run_command(&format!("mv {push_tmp_path} {dest}"))
            .await?;
        if !output.contains("exit code 0") {
            bail!("exit code 0 not found in: {output}");
        }
        println!("ok");
        Ok(())
    }
}

/// Claim the modem's USB interface for sending AT commands.
fn serial_interface() -> Result<Option<Interface>> {
    if let Some(device) = open_usb_device(USB_VENDOR_ID, USB_PRODUCT_ID)? {
        let interface = device
            .detach_and_claim_interface(USB_INTERFACE_NUMBER)
            .context("detach_and_claim_interface({USB_INTERFACE_NUMBER}) failed")?;
        return Ok(Some(interface));
    }
    Ok(None)
}

trait AT {
    async fn send_at_command(&self, command: &str) -> Result<Vec<u8>>;
}

impl AT for Interface {
    /// Send an AT command to the Quectel modem.
    async fn send_at_command(&self, command: &str) -> Result<Vec<u8>> {
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
            index: USB_INTERFACE_NUMBER as u16,
        };

        self.control_out_blocking(enable_serial_port, &[], timeout)
            .context("Failed to send control request")?;

        tokio::time::timeout(timeout, self.bulk_out(0x3, data.as_bytes().to_vec()))
            .await
            .context("Timed out writing command")?
            .into_result()
            .context("Failed to write command")?;

        let response = tokio::time::timeout(timeout, self.bulk_in(0x84, RequestBuffer::new(256)))
            .await
            .context("Timed out reading response")?
            .into_result()
            .context("Failed to read response")?;

        Ok(response)
    }
}

#[test]
fn test_qadbcfg_fmt() {
    assert_eq!(
        Qusbcfg::default().to_string(),
        "AT+QCFG=\"usbcfg\",0x2C7C,0x125,1,1,1,1,1,0,0"
    );
}
