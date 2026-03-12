use anyhow::ensure;
use libtest_mimic::Trial;

use crate::capabilities::Capabilities;
use crate::context::{ctx, run};
use crate::shell::{DEVICE_CONFIG_PATH, DEVICE_LOG_PATH, DEVICE_WPA_CONF_PATH, ShellConnection};

pub fn register(caps: &Capabilities) -> Vec<Trial> {
    let shell = caps.shell;
    let wifi = caps.wifi_enabled;

    vec![
        Trial::test("shell::daemon_process_running", move || {
            run(async {
                ensure!(shell, "requires --shell");
                let ps = ctx().shell.as_ref().unwrap().run_command("ps").await?;
                ensure!(
                    ps.contains("rayhunter-daemon"),
                    "rayhunter-daemon not found in process list"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!shell),
        Trial::test("shell::config_toml_exists", move || {
            run(async {
                ensure!(shell, "requires --shell");
                let content = ctx()
                    .shell
                    .as_ref()
                    .unwrap()
                    .read_file(DEVICE_CONFIG_PATH)
                    .await?;
                ensure!(
                    content.is_some(),
                    "{DEVICE_CONFIG_PATH} does not exist on device"
                );
                let toml = content.unwrap();
                ensure!(
                    toml.contains("qmdl_store_path"),
                    "{DEVICE_CONFIG_PATH} missing qmdl_store_path key — \
                     file may be corrupt or empty"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!shell),
        Trial::test("shell::dns_resolv_conf_has_nameservers", move || {
            run(async {
                ensure!(shell && wifi, "requires --shell + wifi_enabled");
                let content = ctx()
                    .shell
                    .as_ref()
                    .unwrap()
                    .read_file("/etc/resolv.conf")
                    .await?;
                ensure!(
                    content.is_some(),
                    "/etc/resolv.conf does not exist on device"
                );
                let resolv = content.unwrap();
                ensure!(
                    resolv.contains("nameserver"),
                    "/etc/resolv.conf has no nameserver entries"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!(shell && wifi)),
        Trial::test("shell::wpa_creds_not_world_readable", move || {
            run(async {
                ensure!(shell && wifi, "requires --shell + wifi_enabled");
                let shell_conn = ctx().shell.as_ref().unwrap();
                let exists = shell_conn.read_file(DEVICE_WPA_CONF_PATH).await?.is_some();
                if !exists {
                    // No wpa_sta.conf means no creds on disk — that's fine
                    return Ok(());
                }

                let ls = shell_conn
                    .run_command(&format!("ls -la {DEVICE_WPA_CONF_PATH}"))
                    .await?;
                // perms format: -rwxrwxrwx (positions 0-9)
                // positions 1-3: owner, 4-6: group, 7-9: other
                // Expected: -rw------- (0600)
                let perms = ls.split_whitespace().next().unwrap_or("");
                ensure!(perms.len() >= 10, "could not parse permissions from: {ls}");
                let group_other = &perms[4..10];
                ensure!(
                    !group_other.contains('r') && !group_other.contains('w'),
                    "wpa_sta.conf is accessible to group/other: {perms}"
                );
                Ok(())
            })
        })
        .with_ignored_flag(!(shell && wifi)),
        Trial::test("shell::log_file_exists", move || {
            run(async {
                ensure!(shell, "requires --shell");
                let content = ctx()
                    .shell
                    .as_ref()
                    .unwrap()
                    .read_file(DEVICE_LOG_PATH)
                    .await?;
                ensure!(
                    content.is_some(),
                    "{DEVICE_LOG_PATH} does not exist on device"
                );
                ensure!(!content.unwrap().is_empty(), "log file exists but is empty");
                Ok(())
            })
        })
        .with_ignored_flag(!shell),
    ]
}
