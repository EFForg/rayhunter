mod capabilities;
mod client;
mod context;
mod shell;
mod tests;
#[allow(dead_code)] // fields exist for serde deserialization
mod types;

use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;

use capabilities::Capabilities;
use client::RayhunterClient;
use context::TestContext;

#[derive(Parser)]
#[command(
    name = "rayhunter-test",
    about = "Acceptance tests for a live Rayhunter device"
)]
struct Cli {
    /// Device address as host:port (required unless --list)
    #[arg(long)]
    host: Option<String>,

    /// Shell access method for shell-level tests
    #[arg(long, value_parser = ["adb"])]
    shell: Option<String>,
}

async fn setup(host: &str, cli: &Cli) -> Result<Arc<TestContext>> {
    let client = RayhunterClient::new(host);
    let config = client
        .get_config()
        .await
        .context("initial connection to device failed")?;

    let shell = match cli.shell.as_deref() {
        Some("adb") => Some(shell::AdbShell),
        _ => None,
    };

    let capabilities = Capabilities::from_config(&config, shell.is_some());

    eprintln!("Connected to device at {host}");
    eprintln!(
        "  device={}, wifi_capable={}, wifi_enabled={}, recording={}, shell={}",
        config.device,
        capabilities.wifi_capable,
        capabilities.wifi_enabled,
        capabilities.recording,
        capabilities.shell,
    );

    Ok(Arc::new(TestContext {
        client,
        capabilities,
        shell,
    }))
}

fn main() {
    let all_args: Vec<String> = std::env::args().collect();

    // clap handles --host/--shell, libtest-mimic handles --list/filters/etc.
    let mut our_args = vec![all_args[0].clone()];
    let mut test_args = vec![all_args[0].clone()];
    let mut i = 1;
    while i < all_args.len() {
        match all_args[i].as_str() {
            "--host" => {
                our_args.push(all_args[i].clone());
                if i + 1 < all_args.len() {
                    i += 1;
                    our_args.push(all_args[i].clone());
                }
            }
            "--shell" => {
                our_args.push(all_args[i].clone());
                if i + 1 < all_args.len() {
                    i += 1;
                    our_args.push(all_args[i].clone());
                }
            }
            _ if all_args[i].starts_with("--host=") || all_args[i].starts_with("--shell=") => {
                our_args.push(all_args[i].clone());
            }
            _ => {
                test_args.push(all_args[i].clone());
            }
        }
        i += 1;
    }

    let cli = Cli::parse_from(&our_args);
    let mimic_args = libtest_mimic::Arguments::from_iter(test_args);

    if mimic_args.list {
        let dummy_caps = Capabilities {
            http: true,
            shell: cli.shell.is_some(),
            wifi_enabled: true,
            wifi_capable: true,
            recording: true,
        };
        let mut tests = Vec::new();
        tests.extend(tests::config::register(&dummy_caps));
        tests.extend(tests::system::register(&dummy_caps));
        tests.extend(tests::recording::register(&dummy_caps));
        tests.extend(tests::download::register(&dummy_caps));
        tests.extend(tests::analysis::register(&dummy_caps));
        tests.extend(tests::wifi::register(&dummy_caps));
        tests.extend(tests::shell_tests::register(&dummy_caps));
        tests.extend(tests::security::register(&dummy_caps));
        libtest_mimic::run(&mimic_args, tests).exit();
    }

    let host = cli.host.as_deref().unwrap_or_else(|| {
        eprintln!("error: --host is required when running tests");
        eprintln!("usage: rayhunter-test --host <ADDRESS:PORT>");
        std::process::exit(1);
    });

    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    let ctx = rt.block_on(setup(host, &cli)).unwrap_or_else(|e| {
        eprintln!("Failed to connect to device: {e:#}");
        std::process::exit(1);
    });

    let caps = &ctx.capabilities;
    let mut tests = Vec::new();
    tests.extend(tests::config::register(caps));
    tests.extend(tests::system::register(caps));
    tests.extend(tests::recording::register(caps));
    tests.extend(tests::download::register(caps));
    tests.extend(tests::analysis::register(caps));
    tests.extend(tests::wifi::register(caps));
    tests.extend(tests::shell_tests::register(caps));
    tests.extend(tests::security::register(caps));

    context::set_context(ctx);
    libtest_mimic::run(&mimic_args, tests).exit();
}
