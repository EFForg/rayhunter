use serde::{Deserialize, Serialize};

/// Initialize logging with the given default level, suppressing noisy warnings
/// from hampi about undecoded ASN1 extensions. Respects `RUST_LOG` overrides.
pub fn init_logging(default_level: log::LevelFilter) {
    env_logger::Builder::new()
        .filter_level(default_level)
        //Filter out a stupid massive amount of uneccessary warnings from hampi about undecoded extensions
        .filter_module("asn1_codecs", log::LevelFilter::Error)
        .parse_default_env()
        .init();
}

pub mod analysis;
pub mod clock;
pub mod diag;
pub mod gsmtap;
pub mod gsmtap_parser;
pub mod hdlc;
pub mod log_codes;
pub mod pcap;
pub mod qmdl;
pub mod util;

// bin/check.rs may target windows and does not use this mod
#[cfg(target_family = "unix")]
pub mod diag_device;

// re-export telcom_parser, since we use its types in our API
pub use telcom_parser;

/// A list of the internal names of currently implemented devices
#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub enum Device {
    Orbic,
    Tplink,
    Tmobile,
    Wingtech,
    Pinephone,
    Uz801,
    Moxee,
}

/// Generate a wpa_supplicant configuration file from an SSID and password.
/// Escapes backslashes and double quotes in both fields.
pub fn format_wpa_conf(ssid: &str, password: &str) -> String {
    let ssid = ssid.replace('\\', "\\\\").replace('"', "\\\"");
    let password = password.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "ctrl_interface=/var/run/wpa_supplicant\nnetwork={{\n    ssid=\"{ssid}\"\n    psk=\"{password}\"\n    key_mgmt=WPA-PSK\n}}\n"
    )
}

/// Read the SSID from a wpa_supplicant configuration file.
/// Returns None if the file doesn't exist or has no ssid line.
pub fn read_ssid_from_wpa_conf(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    content.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("ssid=\"")
            .and_then(|s| s.strip_suffix('"'))
            .map(|s| s.replace("\\\"", "\"").replace("\\\\", "\\"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_wpa_conf_basic() {
        let conf = format_wpa_conf("MyNetwork", "mypassword");
        assert!(conf.contains("ssid=\"MyNetwork\""));
        assert!(conf.contains("psk=\"mypassword\""));
        assert!(conf.contains("key_mgmt=WPA-PSK"));
        assert!(conf.starts_with("ctrl_interface=/var/run/wpa_supplicant\n"));
    }

    #[test]
    fn test_format_wpa_conf_escapes_quotes() {
        let conf = format_wpa_conf("My\"Net", "pass\"word");
        assert!(conf.contains("ssid=\"My\\\"Net\""));
        assert!(conf.contains("psk=\"pass\\\"word\""));
    }

    #[test]
    fn test_format_wpa_conf_escapes_backslashes() {
        let conf = format_wpa_conf("Net\\work", "pass\\word");
        assert!(conf.contains("ssid=\"Net\\\\work\""));
        assert!(conf.contains("psk=\"pass\\\\word\""));
    }

    #[test]
    fn test_read_ssid_from_wpa_conf() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("TestSSID", "password123");
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("TestSSID".to_string()));
    }

    #[test]
    fn test_read_ssid_roundtrips_special_chars() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wpa.conf");
        let conf = format_wpa_conf("My\"Net\\work", "pass");
        std::fs::write(&path, conf).unwrap();

        let ssid = read_ssid_from_wpa_conf(path.to_str().unwrap());
        assert_eq!(ssid, Some("My\"Net\\work".to_string()));
    }

    #[test]
    fn test_read_ssid_missing_file() {
        assert_eq!(read_ssid_from_wpa_conf("/nonexistent/path"), None);
    }
}
