use std::future::Future;
use std::net::SocketAddr;

use anyhow::{Result, bail};

use crate::output::{print, println};

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

/// Shared config installation logic. Installs to /data/rayhunter/config.toml which resolves
/// through the symlink to the actual data directory.
pub async fn install_config<C: DeviceConnection>(
    conn: &mut C,
    device_type: &str,
    reset_config: bool,
) -> Result<()> {
    let config_path = "/data/rayhunter/config.toml";
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

/// Check if a directory exists using a DeviceConnection
pub async fn dir_exists<C: DeviceConnection>(conn: &mut C, path: &str) -> bool {
    conn.run_command(&format!("test -d {path} && echo exists || echo missing"))
        .await
        .map(|output| output.contains("exists"))
        .unwrap_or(false)
}

/// Check if a path is a symlink using a DeviceConnection
pub async fn is_symlink<C: DeviceConnection>(conn: &mut C, path: &str) -> bool {
    conn.run_command(&format!("test -L {path} && echo yes || echo no"))
        .await
        .map(|output| output.contains("yes"))
        .unwrap_or(false)
}

/// Read the target of a symlink using a DeviceConnection
pub async fn readlink<C: DeviceConnection>(conn: &mut C, path: &str) -> Result<String> {
    // Use a prefix marker to find the actual output line, since some shells (TP-Link) echo
    // back the command and run_command appends protocol lines.
    let output = conn
        .run_command(&format!("echo RL:$(readlink {path})"))
        .await?;

    for line in output.lines() {
        if let Some(target) = line.trim().strip_prefix("RL:") {
            return Ok(target.to_string());
        }
    }

    bail!("unexpected readlink output: {output:?}");
}

/// Set up the data directory at `data_dir` and create a symlink from `/data/rayhunter` to it.
///
/// Handles migration from old locations:
/// - If `/data/rayhunter` is a real directory, moves its contents to `data_dir`
/// - If `/data/rayhunter` is a symlink to a different location, moves from the old target
/// - If `/data/rayhunter` doesn't exist, just creates the symlink
/// - If `/data/rayhunter` is a symlink to `data_dir`, does nothing
pub async fn setup_data_directory<C: DeviceConnection>(conn: &mut C, data_dir: &str) -> Result<()> {
    if data_dir == "/data/rayhunter" {
        bail!("data_dir must not be /data/rayhunter");
    }

    // Determine where old data lives, if anywhere
    let old_data_source = if is_symlink(conn, "/data/rayhunter").await {
        let current_target = readlink(conn, "/data/rayhunter").await?;
        if current_target == data_dir {
            println!("Data directory already configured at {data_dir}");
            return Ok(());
        }
        conn.run_command("rm -f /data/rayhunter").await?;
        // The old symlink target is where data actually lives
        if dir_exists(conn, &current_target).await {
            Some(current_target)
        } else {
            None
        }
    } else if dir_exists(conn, "/data/rayhunter").await {
        if dir_exists(conn, data_dir).await {
            bail!("Both /data/rayhunter and {data_dir} exist and are directories.");
        }
        // Real directory (pre-migration Orbic state)
        Some("/data/rayhunter".to_string())
    } else {
        None
    };

    // Migrate old data if present
    if let Some(old_source) = &old_data_source {
        // Stop rayhunter-daemon so it doesn't write during migration.
        // The device will be rebooted at the end of installation anyway.
        print!("Stopping rayhunter-daemon ... ");
        let _ = conn
            .run_command("/etc/init.d/rayhunter_daemon stop 2>/dev/null; true")
            .await;
        println!("ok");

        print!("Migrating data from {old_source} to {data_dir} ... ");

        // mv old data into its place. If source and destination are on the same filesystem,
        // this is an instant rename.
        // XXX: DeviceConnection::run_command does not expose the exit code of the ran command. It
        // probably should, or a utility for it should exist?
        let mv_output = conn
            .run_command(&format!("mv {old_source} {data_dir} && echo MV_OK"))
            .await?;
        if mv_output.contains("MV_OK") {
            println!("ok");
        } else {
            bail!("Failed to move data from {old_source} to {data_dir}:\n{mv_output}");
        }
    } else {
        // No migration needed, just ensure the target directory exists
        conn.run_command(&format!("mkdir -p {data_dir}")).await?;
    }

    // Create the symlink
    print!("Creating symlink /data/rayhunter -> {data_dir} ... ");
    conn.run_command("mkdir -p /data").await?;
    conn.run_command(&format!("ln -sf {data_dir} /data/rayhunter"))
        .await?;
    println!("ok");

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
