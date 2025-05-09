use anyhow::{Context, Error, bail};
use clap::{Parser, Subcommand};

mod orbic;
mod tplink;

pub static CONFIG_TOML: &str = include_str!("../../dist/config.toml.example");
pub static RAYHUNTER_DAEMON_INIT: &str = include_str!("../../dist/scripts/rayhunter_daemon");

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install rayhunter on the Orbic Orbic RC400L.
    Orbic(InstallOrbic),
    /// Install rayhunter on the TP-Link M7350.
    Tplink(InstallTpLink),
    /// Developer utilities.
    Util(Util),
}

#[derive(Parser, Debug)]
struct InstallTpLink {
    /// Do not enforce use of SD card. All data will be stored in /mnt/card regardless, which means
    /// that if an SD card is later added, your existing installation is shadowed!
    #[arg(long)]
    skip_sdcard: bool,

    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[derive(Parser, Debug)]
struct InstallOrbic {}

#[derive(Parser, Debug)]
struct Util {
    #[command(subcommand)]
    command: UtilSubCommand,
}

#[derive(Subcommand, Debug)]
enum UtilSubCommand {
    /// Send a serial command to the Orbic.
    Serial(Serial),
    /// Root the tplink and launch telnetd.
    TplinkStartTelnet(TplinkStartTelnet),
}

#[derive(Parser, Debug)]
struct TplinkStartTelnet {
    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[derive(Parser, Debug)]
struct Serial {
    #[arg(long)]
    root: bool,
    command: Vec<String>,
}

async fn run_function() -> Result<(), Error> {
    let Args { command } = Args::parse();

    match command {
        Command::Tplink(tplink) => tplink::main_tplink(tplink).await.context("Failed to install rayhunter on the TP-Link M7350. Make sure your computer is connected to the hotspot using USB tethering or WiFi.")?,
        Command::Orbic(_) => orbic::install().await.context("Failed to install rayhunter on the Orbic RC400L")?,
        Command::Util(subcommand) => match subcommand.command {
            UtilSubCommand::Serial(serial_cmd) => {
                if serial_cmd.root {
                    if !serial_cmd.command.is_empty() {
                        eprintln!("You cannot use --root and specify a command at the same time");
                        std::process::exit(64);
                    }
                    orbic::enable_command_mode()?;
                } else if serial_cmd.command.is_empty() {
                    eprintln!("Command cannot be an empty string");
                    std::process::exit(64);
                } else {
                    let cmd = serial_cmd.command.join(" ");
                    match orbic::open_orbic()? {
                        Some(interface) => orbic::send_serial_cmd(&interface, &cmd).await?,
                        None => bail!(orbic::ORBIC_NOT_FOUND),
                    }
                }
            }
            UtilSubCommand::TplinkStartTelnet(options) => {
                tplink::start_telnet(&options.admin_ip).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_function().await {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
