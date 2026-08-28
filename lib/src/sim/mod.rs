use std::collections::BTreeSet;

use log::{info, warn};
use thiserror::Error;

use crate::Device;

pub mod generic_at;

#[derive(Error, Debug)]
pub enum SimError {
    #[error("Reading the SIM is not supported for this device")]
    UnsupportedDevice,
    #[error("AT command error: {0}")]
    AtCommandError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Read the subscriber's home PLMNs from the SIM, as `"MCC-MNC"`. Empty on any
/// failure, which callers must treat as "unknown".
pub async fn home_plmn(device: &Device) -> BTreeSet<String> {
    let result = match device {
        Device::Tplink | Device::Orbic => generic_at::get_home_plmn("/dev/smd7").await,
        _ => Err(SimError::UnsupportedDevice),
    };

    match result {
        Ok(plmns) => {
            info!("read home PLMNs {plmns:?} from SIM");
            plmns
        }
        Err(SimError::UnsupportedDevice) => {
            info!("reading the home PLMN from the SIM isn't supported for this device");
            BTreeSet::new()
        }
        Err(e) => {
            warn!("couldn't read the home PLMN from the SIM: {e}");
            BTreeSet::new()
        }
    }
}
