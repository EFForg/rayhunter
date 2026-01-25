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

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Device {
    Orbic,
    Tplink,
    Tmobile,
    Wingtech,
    Pinephone,
    Uz801,
}
