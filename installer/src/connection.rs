use std::future::Future;
use std::net::SocketAddr;

use anyhow::Result;

use crate::output::println;

/// Abstraction for device communication (telnet or ADB)
pub trait DeviceConnection {
    /// Run a shell command and return its output
    fn run_command(&mut self, command: &str) -> impl Future<Output = Result<String>> + Send;

    /// Write a file to the device
    fn write_file(&mut self, path: &str, content: &[u8])
    -> impl Future<Output = Result<()>> + Send;
}

/// Check if a file exists using a DeviceConnection
pub async fn file_exists<C: DeviceConnection>(conn: &mut C, path: &str) -> bool {
    conn.run_command(&format!("test -f {path} && echo exists || echo missing"))
        .await
        .map(|output| output.contains("exists"))
        .unwrap_or(false)
}

/// Shared config installation logic
pub async fn install_config<C: DeviceConnection>(
    conn: &mut C,
    config_path: &str,
    device_type: &str,
    reset_config: bool,
) -> Result<()> {
    if reset_config || !file_exists(conn, config_path).await {
        let config = crate::CONFIG_TOML.replace(
            r#"#device = "orbic""#,
            &format!(r#"device = "{device_type}""#),
        );
        conn.write_file(config_path, config.as_bytes()).await?;
    } else {
        println!("Config file already exists, skipping (use --reset-config to overwrite)");
    }
    Ok(())
}

const WIFI_CREDS_PATH: &str = "/data/rayhunter/wifi-creds.conf";

pub async fn install_wifi_creds<C: DeviceConnection>(
    conn: &mut C,
    wifi_ssid: Option<&str>,
    wifi_password: Option<&str>,
) -> Result<()> {
    match (wifi_ssid, wifi_password) {
        (Some(ssid), Some(password)) if !ssid.is_empty() && !password.is_empty() => {
            let contents = format!("ssid={ssid}\npassword={password}\n");
            conn.write_file(WIFI_CREDS_PATH, contents.as_bytes())
                .await?;
            println!("WiFi client mode credentials written");
        }
        (Some(_), None) | (None, Some(_)) => {
            println!("Both --wifi-ssid and --wifi-password are required, skipping WiFi setup");
        }
        _ => {}
    }
    Ok(())
}

/// Telnet-based connection wrapper
pub struct TelnetConnection {
    pub addr: SocketAddr,
    pub wait_for_prompt: bool,
}

impl TelnetConnection {
    pub fn new(addr: SocketAddr, wait_for_prompt: bool) -> Self {
        Self {
            addr,
            wait_for_prompt,
        }
    }
}

impl DeviceConnection for TelnetConnection {
    async fn run_command(&mut self, command: &str) -> Result<String> {
        crate::util::telnet_send_command_with_output(self.addr, command, self.wait_for_prompt).await
    }

    async fn write_file(&mut self, path: &str, content: &[u8]) -> Result<()> {
        crate::util::telnet_send_file(self.addr, path, content, self.wait_for_prompt).await
    }
}
