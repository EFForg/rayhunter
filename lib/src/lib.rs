use std::collections::BTreeSet;

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
pub mod hdlc;
pub mod log_codes;
pub mod pcap;
pub mod plmn;
pub mod qmdl;
#[cfg(test)]
mod test_util;
pub mod util;

// bin/check.rs may target windows and does not use these mods
#[cfg(target_family = "unix")]
pub mod diag_device;
#[cfg(target_family = "unix")]
pub mod sim;

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

/// Facts about the device rayhunter is running on, gathered at runtime and
/// made available to analyzers.
#[derive(PartialEq, Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct DeviceMetadata {
    /// The subscriber's home PLMNs as `"MCC-MNC"`, read from EF_HPLMNwAcT
    /// (`6F62`) on the SIM. Empty when unknown.
    pub home_plmn: BTreeSet<String>,
}
