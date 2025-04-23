use anyhow::{Context, Error};
use clap::{Parser, Subcommand};

mod tplink;

pub static CONFIG_TOML: &[u8] = include_bytes!("../../dist/config.toml.example");
pub static RAYHUNTER_DAEMON_INIT: &[u8] = include_bytes!("../../dist/scripts/rayhunter_daemon");

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install rayhunter on the TP-Link M7350.
    InstallTplink(InstallTpLink),
}

#[derive(Parser, Debug)]
struct InstallTpLink {
    /// Do not enforce use of SD card. All data will be stored in /mnt/card regardless, which means
    /// that if an SD card is later added, your existing installation is shadowed!
    #[arg(long)]
    skip_sdcard: bool,

    /// Username for TP-Link admin interface, if custom.
    #[arg(long, default_value = "admin")]
    username: String,

    /// Password for TP-Link admin interface, if custom.
    #[arg(long, default_value = "admin")]
    password: String,

    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args { command } = Args::parse();

    match command {
        Command::InstallTplink(tplink) => tplink::main_tplink(tplink).await.context("Failed to install rayhunter on the TP-Link M7350. Make sure your computer is connected to the hotspot using USB tethering or WiFi. Currently only Hardware Revision v3 is supported.")?,
    }

    Ok(())
}
