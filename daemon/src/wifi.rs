use log::{info, warn};

const WCNSS_INI: &str = "/lib/firmware/wlan/qca_cld/WCNSS_qcom_cfg.ini";

pub async fn apply_wifi_auto_shutdown_config(disable: bool) {
    let contents = match tokio::fs::read_to_string(WCNSS_INI).await {
        Ok(c) => c,
        Err(e) => {
            warn!("could not read {WCNSS_INI}: {e}");
            return;
        }
    };

    let target_value = if disable { "0" } else { "600" };
    let mut found = false;
    let mut changed = false;
    let new_contents: String = contents
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("gWlanAutoShutdown") {
                found = true;
                let new_line = format!("gWlanAutoShutdown={target_value}");
                if line.trim() != new_line {
                    changed = true;
                }
                new_line
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !found {
        if disable {
            info!("gWlanAutoShutdown not found in {WCNSS_INI}, skipping");
        }
        return;
    }

    if !changed {
        info!("gWlanAutoShutdown already set to {target_value}");
        return;
    }

    // Preserve trailing newline if the original had one
    let new_contents = if contents.ends_with('\n') && !new_contents.ends_with('\n') {
        new_contents + "\n"
    } else {
        new_contents
    };

    match tokio::fs::write(WCNSS_INI, &new_contents).await {
        Ok(()) => info!("set gWlanAutoShutdown={target_value} (reboot required to take effect)"),
        Err(e) => warn!("failed to write {WCNSS_INI}: {e}"),
    }
}
