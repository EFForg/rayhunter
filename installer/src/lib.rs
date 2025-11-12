use anyhow::{Context, Error, bail};
use clap::{Parser, Subcommand};
use env_logger::Env;

mod orbic;
mod orbic_auth;
mod orbic_network;
mod pinephone;
mod tmobile;
mod tplink;
mod util;
mod uz801;
mod wingtech;

pub static CONFIG_TOML: &str = include_str!("../../dist/config.toml.in");
pub static RAYHUNTER_DAEMON_INIT: &str = include_str!("../../dist/scripts/rayhunter_daemon");

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

// A note on stylisation of device names: strip special characters and spell like This regardless
// of the manufacturer's capitalisation.
#[derive(Subcommand, Debug)]
enum Command {
    /// Install rayhunter on the Orbic RC400L using the legacy USB+ADB-based installer.
    OrbicUsb(InstallOrbic),
    /// Install rayhunter on the Orbic RC400L or Moxee Hotspot via network.
    #[clap(alias = "orbic-network")]
    Orbic(OrbicNetworkArgs),
    /// Install rayhunter on the TMobile TMOHS1.
    Tmobile(TmobileArgs),
    /// Install rayhunter on the Uz801.
    Uz801(Uz801Args),
    /// Install rayhunter on a PinePhone's Quectel modem.
    Pinephone(InstallPinephone),
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
struct OrbicNetworkArgs {
    /// IP address for Orbic admin interface, if custom.
    #[arg(long, default_value = "192.168.1.1")]
    admin_ip: String,

    /// Admin username for authentication.
    #[arg(long, default_value = "admin")]
    admin_username: String,

    /// Admin password for authentication.
    #[arg(long)]
    admin_password: Option<String>,
}

#[derive(Parser, Debug)]
struct InstallPinephone {}

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
    Shell,
    /// Root the Tmobile and launch adb.
    TmobileStartAdb(TmobileArgs),
    /// Root the Tmobile and launch telnetd.
    TmobileStartTelnet(TmobileArgs),
    /// Root the Uz801 and launch adb.
    Uz801StartAdb(Uz801Args),
    /// Root the tplink and launch telnetd.
    TplinkStartTelnet(TplinkStartTelnet),
    /// Root the Wingtech and launch telnetd.
    WingtechStartTelnet(WingtechArgs),
    /// Root the Wingtech and launch adb.
    WingtechStartAdb(WingtechArgs),
    /// Unlock the Pinephone's modem and start adb.
    PinephoneStartAdb,
    /// Lock the Pinephone's modem and stop adb.
    PinephoneStopAdb,
    /// Root the Orbic and launch telnetd.
    OrbicStartTelnet(OrbicNetworkArgs),
    /// Send a file to the TP-Link device over telnet.
    ///
    /// Before running this utility, you need to make telnet accessible with `installer util
    /// tplink-start-telnet`.
    TplinkSendFile(TplinkSendFile),
    /// Send a file to the Wingtech device over telnet.
    ///
    /// Before running this utility, you need to make telnet accessible with `installer util
    /// wingtech-start-telnet`.
    WingtechSendFile(WingtechSendFile),
}

#[derive(Parser, Debug)]
struct TmobileArgs {
    /// IP address for Tmobile admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,

    /// Web portal admin password.
    #[arg(long)]
    admin_password: String,
}

#[derive(Parser, Debug)]
struct Uz801Args {
    /// IP address for Uz801 admin interface, if custom.
    #[arg(long, default_value = "192.168.100.1")]
    admin_ip: String,
}

#[derive(Parser, Debug)]
struct TplinkStartTelnet {
    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
}

#[derive(Parser, Debug)]
struct TplinkSendFile {
    /// IP address for TP-Link admin interface, if custom.
    #[arg(long, default_value = "192.168.0.1")]
    admin_ip: String,
    /// Local path to the file to send.
    local_path: String,
    /// Remote path where the file should be stored on the device.
    remote_path: String,
}

#[derive(Parser, Debug)]
struct WingtechSendFile {
    /// IP address for Wingtech admin interface, if custom.
    #[arg(long, default_value = "192.168.1.1")]
    admin_ip: String,
    /// Local path to the file to send.
    local_path: String,
    /// Remote path where the file should be stored on the device.
    remote_path: String,
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

async fn run() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    let Args { command } = Args::parse();

    match command {
        Command::Tmobile(args) => tmobile::install(args).await.context("Failed to install rayhunter on the Tmobile TMOHS1. Make sure your computer is connected to the hotspot using USB tethering or WiFi.")?,
        Command::Uz801(args) => uz801::install(args).await.context("Failed to install rayhunter on the Uz801. Make sure your computer is connected to the hotspot using USB.")?,
        Command::Tplink(tplink) => tplink::main_tplink(tplink).await.context("Failed to install rayhunter on the TP-Link M7350. Make sure your computer is connected to the hotspot using USB tethering or WiFi.")?,
        Command::Pinephone(_) => pinephone::install().await
            .context("Failed to install rayhunter on the Pinephone's Quectel modem")?,
        Command::OrbicUsb(_) => orbic::install().await.context("\nFailed to install rayhunter on the Orbic RC400L (USB installer)")?,
        Command::Orbic(args) => orbic_network::install(args.admin_ip, args.admin_username, args.admin_password).await.context("\nFailed to install rayhunter on the Orbic RC400L")?,
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
            UtilSubCommand::Shell => orbic::shell().await.context("\nFailed to open shell on Orbic RC400L")?,
            UtilSubCommand::TmobileStartTelnet(args) => wingtech::start_telnet(&args.admin_ip, &args.admin_password).await.context("\nFailed to start telnet on the Tmobile TMOHS1")?,
            UtilSubCommand::TmobileStartAdb(args) => wingtech::start_adb(&args.admin_ip, &args.admin_password).await.context("\nFailed to start adb on the Tmobile TMOHS1")?,
            UtilSubCommand::Uz801StartAdb(args) => uz801::activate_usb_debug(&args.admin_ip).await.context("\nFailed to activate USB debug on the Uz801")?,
            UtilSubCommand::TplinkStartTelnet(options) => {
                tplink::start_telnet(&options.admin_ip).await?;
            }
            UtilSubCommand::TplinkSendFile(options) => {
                util::send_file(&options.admin_ip, &options.local_path, &options.remote_path).await?;
            }
            UtilSubCommand::WingtechSendFile(options) => {
                util::send_file(&options.admin_ip, &options.local_path, &options.remote_path).await?;
            }
            UtilSubCommand::WingtechStartTelnet(args) => wingtech::start_telnet(&args.admin_ip, &args.admin_password).await.context("\nFailed to start telnet on the Wingtech CT2MHS01")?,
            UtilSubCommand::WingtechStartAdb(args) => wingtech::start_adb(&args.admin_ip, &args.admin_password).await.context("\nFailed to start adb on the Wingtech CT2MHS01")?,
            UtilSubCommand::PinephoneStartAdb => pinephone::start_adb().await.context("\nFailed to start adb on the PinePhone's modem")?,
            UtilSubCommand::PinephoneStopAdb => pinephone::stop_adb().await.context("\nFailed to stop adb on the PinePhone's modem")?,
            UtilSubCommand::OrbicStartTelnet(args) => orbic_network::start_telnet(&args.admin_ip, &args.admin_username, args.admin_password.as_deref()).await.context("\\nFailed to start telnet on the Orbic RC400L")?,
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
