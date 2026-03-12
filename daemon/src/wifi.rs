use log::{info, warn};

const WCNSS_INI: &str = "/lib/firmware/wlan/qca_cld/WCNSS_qcom_cfg.ini";
const WCNSS_INI_BACKUP: &str = "/data/rayhunter/WCNSS_qcom_cfg.ini.bak";

pub async fn apply_wifi_auto_shutdown_config(disable: bool) {
    let contents = match tokio::fs::read_to_string(WCNSS_INI).await {
        Ok(c) => c,
        Err(e) => {
            warn!("could not read {WCNSS_INI}: {e}");
            return;
        }
    };

    if !contents.contains("gWlanAutoShutdown") {
        if disable {
            info!("gWlanAutoShutdown not found in {WCNSS_INI}, skipping");
        }
        return;
    }

    if disable {
        if tokio::fs::metadata(WCNSS_INI_BACKUP).await.is_err()
            && let Err(e) = tokio::fs::copy(WCNSS_INI, WCNSS_INI_BACKUP).await
        {
            warn!("could not back up {WCNSS_INI}: {e}");
            return;
        }
        let new_contents = replace_shutdown_value(&contents, "0");
        if new_contents == contents {
            info!("gWlanAutoShutdown already disabled");
            return;
        }
        match tokio::fs::write(WCNSS_INI, &new_contents).await {
            Ok(()) => info!("disabled gWlanAutoShutdown (reboot required to take effect)"),
            Err(e) => warn!("failed to write {WCNSS_INI}: {e}"),
        }
    } else {
        match tokio::fs::read_to_string(WCNSS_INI_BACKUP).await {
            Ok(backup) => match tokio::fs::write(WCNSS_INI, &backup).await {
                Ok(()) => {
                    info!("restored {WCNSS_INI} from backup (reboot required to take effect)")
                }
                Err(e) => warn!("failed to restore {WCNSS_INI}: {e}"),
            },
            Err(_) => info!("no backup of {WCNSS_INI} to restore"),
        }
    }
}

fn replace_shutdown_value(contents: &str, value: &str) -> String {
    let mut result = String::with_capacity(contents.len());
    for chunk in contents.split_inclusive('\n') {
        if chunk.trim_start().starts_with("gWlanAutoShutdown") {
            let trailing = &chunk[chunk.trim_end().len()..];
            result.push_str(&format!("gWlanAutoShutdown={value}"));
            result.push_str(trailing);
        } else {
            result.push_str(chunk);
        }
    }
    result
}
