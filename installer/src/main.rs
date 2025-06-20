use anyhow::{Context, Error, bail};
use clap::{Parser, Subcommand};
use env_logger::Env;

mod orbic;
mod tplink;
mod util;
mod wingtech;

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
    /// Install rayhunter on the Wingtech CT2MHS01.
    Wingtech(WingtechArgs),
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

    /// For advanced users: Specify the path of the SD card to be mounted explicitly.
    ///
    /// The default (empty string) is to use whichever sdcard path the device would use natively to
    /// mount storage on. On most TP-Link this is /media/card, but on hardware versions 9+ this is
    /// /media/sdcard
    ///
    /// Only override this when the installer does not work on your hardware version, as otherwise
    /// your custom path may conflict with the builtin storage functionality.
    #[arg(long, default_value = "")]
    sdcard_path: String,
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
    /// Start an ADB shell
    Shell(Shell),
    /// Root the tplink and launch telnetd.
    TplinkStartTelnet(TplinkStartTelnet),
    /// Root the Wingtech and launch telnetd.
    WingtechStartTelnet(WingtechArgs),
    /// Root the Wingtech and launch adb.
    WingtechStartAdb(WingtechArgs),
}

#[derive(Parser, Debug)]
struct TplinkStartTelnet {
    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[derive(Parser, Debug)]
struct WingtechArgs {
    /// IP address for Wingtech admin interface, if custom.
    #[arg(long, default_value = "192.168.1.1")]
    admin_ip: String,

    /// Web portal admin password.
    #[arg(long)]
    admin_password: String,
}

#[derive(Parser, Debug)]
struct Serial {
    #[arg(long)]
    root: bool,
    command: Vec<String>,
}

#[derive(Parser, Debug)]
struct Shell {}

async fn run() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    let Args { command } = Args::parse();

    match command {
        Command::Tplink(tplink) => tplink::main_tplink(tplink).await.context("Failed to install rayhunter on the TP-Link M7350. Make sure your computer is connected to the hotspot using USB tethering or WiFi.")?,
        Command::Orbic(_) => orbic::install().await.context("\nFailed to install rayhunter on the Orbic RC400L")?,
        Command::Wingtech(args) => wingtech::install(args).await.context("\nFailed to install rayhunter on the Wingtech CT2MHS01")?,
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
            UtilSubCommand::Shell(_) => orbic::shell().await.context("\nFailed to open shell on Orbic RC400L")?,
            UtilSubCommand::TplinkStartTelnet(options) => {
                tplink::start_telnet(&options.admin_ip).await?;
            }
            UtilSubCommand::WingtechStartTelnet(args) => wingtech::start_telnet(&args.admin_ip, &args.admin_password).await.context("\nFailed to start telnet on the Wingtech CT2MHS01")?,
            UtilSubCommand::WingtechStartAdb(args) => wingtech::start_adb(&args.admin_ip, &args.admin_password).await.context("\nFailed to start adb on the Wingtech CT2MHS01")?,
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
