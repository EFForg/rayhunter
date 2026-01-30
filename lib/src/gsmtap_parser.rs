use crate::diag::*;
use crate::gsmtap::*;

use log::{debug, error};
use serde::Serialize;
use std::sync::RwLock;
use thiserror::Error;

/// Cached LTE cell information from ML1 measurements.
/// Contains signal strength and cell identity information.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CellInfo {
    /// Reference Signal Received Power in dBm (typical range: -140 to -44)
    pub rsrp_dbm: Option<f32>,
    /// Reference Signal Received Quality in dB (typical range: -20 to -3)
    pub rsrq_db: Option<f32>,
    /// Received Signal Strength Indicator in dBm
    pub rssi_dbm: Option<f32>,
    /// Physical Cell ID (0-503)
    pub pci: Option<u16>,
    /// E-UTRA Absolute Radio Frequency Channel Number
    pub earfcn: Option<u32>,
}

/// Global cache for the most recent cell/signal measurement.
/// This is populated by LteMl1ServingCellMeas messages and can be used
/// to add signal strength to GSMTAP headers and display in the UI.
///
/// Uses RwLock for consistent multi-field updates. Reads >> writes so this is efficient.
static CACHED_CELL_INFO: RwLock<CellInfo> = RwLock::new(CellInfo {
    rsrp_dbm: None,
    rsrq_db: None,
    rssi_dbm: None,
    pci: None,
    earfcn: None,
});

/// Get the cached cell information.
/// Returns a clone of the current cell info state.
pub fn get_cached_cell_info() -> CellInfo {
    CACHED_CELL_INFO
        .read()
        .expect("cell info lock poisoned")
        .clone()
}

/// Get the cached signal strength (RSRP) in dBm as i8 for GSMTAP header compatibility.
/// Returns 0 if no measurement has been received yet.
pub fn get_cached_signal_dbm() -> i8 {
    CACHED_CELL_INFO
        .read()
        .expect("cell info lock poisoned")
        .rsrp_dbm
        .map(|rsrp| rsrp.clamp(-128.0, 127.0) as i8)
        .unwrap_or(0)
}

/// Update the cached cell info from a measurement.
fn update_cell_info_cache(meas: &LteMl1ServingCellMeasData) {
    let mut cache = CACHED_CELL_INFO.write().expect("cell info lock poisoned");
    cache.rsrp_dbm = meas.get_rsrp_dbm();
    cache.rsrq_db = meas.get_rsrq_db();
    cache.rssi_dbm = meas.get_rssi_dbm();
    cache.pci = meas.get_pci();
    cache.earfcn = meas.get_earfcn();
}

#[derive(Debug, Error)]
pub enum GsmtapParserError {
    #[error("Invalid LteRrcOtaMessage ext header version {0}")]
    InvalidLteRrcOtaExtHeaderVersion(u8),
    #[error("Invalid LteRrcOtaMessage header/PDU number combination: {0}/{1}")]
    InvalidLteRrcOtaHeaderPduNum(u8, u8),
}

pub fn parse(msg: Message) -> Result<Option<(Timestamp, GsmtapMessage)>, GsmtapParserError> {
    if let Message::Log {
        timestamp, body, ..
    } = msg
    {
        match log_to_gsmtap(body)? {
            Some(msg) => Ok(Some((timestamp, msg))),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn log_to_gsmtap(value: LogBody) -> Result<Option<GsmtapMessage>, GsmtapParserError> {
    match value {
        LogBody::LteRrcOtaMessage {
            ext_header_version,
            packet,
        } => {
            let gsmtap_type = match ext_header_version {
                0x02 | 0x03 | 0x04 | 0x06 | 0x07 | 0x08 | 0x0d | 0x16 => match packet.get_pdu_num()
                {
                    1 => GsmtapType::LteRrc(LteRrcSubtype::BcchBch),
                    2 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSch),
                    3 => GsmtapType::LteRrc(LteRrcSubtype::MCCH),
                    4 => GsmtapType::LteRrc(LteRrcSubtype::PCCH),
                    5 => GsmtapType::LteRrc(LteRrcSubtype::DlCcch),
                    6 => GsmtapType::LteRrc(LteRrcSubtype::DlDcch),
                    7 => GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
                    8 => GsmtapType::LteRrc(LteRrcSubtype::UlDcch),
                    pdu => {
                        return Err(GsmtapParserError::InvalidLteRrcOtaHeaderPduNum(
                            ext_header_version,
                            pdu,
                        ));
                    }
                },
                0x09 | 0x0c => match packet.get_pdu_num() {
                    8 => GsmtapType::LteRrc(LteRrcSubtype::BcchBch),
                    9 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSch),
                    10 => GsmtapType::LteRrc(LteRrcSubtype::MCCH),
                    11 => GsmtapType::LteRrc(LteRrcSubtype::PCCH),
                    12 => GsmtapType::LteRrc(LteRrcSubtype::DlCcch),
                    13 => GsmtapType::LteRrc(LteRrcSubtype::DlDcch),
                    14 => GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
                    15 => GsmtapType::LteRrc(LteRrcSubtype::UlDcch),
                    pdu => {
                        return Err(GsmtapParserError::InvalidLteRrcOtaHeaderPduNum(
                            ext_header_version,
                            pdu,
                        ));
                    }
                },
                0x0e..=0x10 => match packet.get_pdu_num() {
                    1 => GsmtapType::LteRrc(LteRrcSubtype::BcchBch),
                    2 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSch),
                    4 => GsmtapType::LteRrc(LteRrcSubtype::MCCH),
                    5 => GsmtapType::LteRrc(LteRrcSubtype::PCCH),
                    6 => GsmtapType::LteRrc(LteRrcSubtype::DlCcch),
                    7 => GsmtapType::LteRrc(LteRrcSubtype::DlDcch),
                    8 => GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
                    9 => GsmtapType::LteRrc(LteRrcSubtype::UlDcch),
                    pdu => {
                        return Err(GsmtapParserError::InvalidLteRrcOtaHeaderPduNum(
                            ext_header_version,
                            pdu,
                        ));
                    }
                },
                0x13 | 0x1a | 0x1b => match packet.get_pdu_num() {
                    1 => GsmtapType::LteRrc(LteRrcSubtype::BcchBch),
                    3 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSch),
                    6 => GsmtapType::LteRrc(LteRrcSubtype::MCCH),
                    7 => GsmtapType::LteRrc(LteRrcSubtype::PCCH),
                    8 => GsmtapType::LteRrc(LteRrcSubtype::DlCcch),
                    9 => GsmtapType::LteRrc(LteRrcSubtype::DlDcch),
                    10 => GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
                    11 => GsmtapType::LteRrc(LteRrcSubtype::UlDcch),
                    45 => GsmtapType::LteRrc(LteRrcSubtype::BcchBchNb),
                    46 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSchNb),
                    47 => GsmtapType::LteRrc(LteRrcSubtype::PcchNb),
                    48 => GsmtapType::LteRrc(LteRrcSubtype::DlCcchNb),
                    49 => GsmtapType::LteRrc(LteRrcSubtype::DlDcchNb),
                    50 => GsmtapType::LteRrc(LteRrcSubtype::UlCcchNb),
                    52 => GsmtapType::LteRrc(LteRrcSubtype::UlDcchNb),
                    pdu => {
                        return Err(GsmtapParserError::InvalidLteRrcOtaHeaderPduNum(
                            ext_header_version,
                            pdu,
                        ));
                    }
                },
                0x14 | 0x18 | 0x19 => match packet.get_pdu_num() {
                    1 => GsmtapType::LteRrc(LteRrcSubtype::BcchBch),
                    2 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSch),
                    4 => GsmtapType::LteRrc(LteRrcSubtype::MCCH),
                    5 => GsmtapType::LteRrc(LteRrcSubtype::PCCH),
                    6 => GsmtapType::LteRrc(LteRrcSubtype::DlCcch),
                    7 => GsmtapType::LteRrc(LteRrcSubtype::DlDcch),
                    8 => GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
                    9 => GsmtapType::LteRrc(LteRrcSubtype::UlDcch),
                    54 => GsmtapType::LteRrc(LteRrcSubtype::BcchBchNb),
                    55 => GsmtapType::LteRrc(LteRrcSubtype::BcchDlSchNb),
                    56 => GsmtapType::LteRrc(LteRrcSubtype::PcchNb),
                    57 => GsmtapType::LteRrc(LteRrcSubtype::DlCcchNb),
                    58 => GsmtapType::LteRrc(LteRrcSubtype::DlDcchNb),
                    59 => GsmtapType::LteRrc(LteRrcSubtype::UlCcchNb),
                    61 => GsmtapType::LteRrc(LteRrcSubtype::UlDcchNb),
                    pdu => {
                        return Err(GsmtapParserError::InvalidLteRrcOtaHeaderPduNum(
                            ext_header_version,
                            pdu,
                        ));
                    }
                },
                _ => {
                    return Err(GsmtapParserError::InvalidLteRrcOtaExtHeaderVersion(
                        ext_header_version,
                    ));
                }
            };
            let mut header = GsmtapHeader::new(gsmtap_type);
            header.arfcn = packet.get_earfcn().try_into().unwrap_or(0);
            header.frame_number = packet.get_sfn();
            header.subslot = packet.get_subfn();
            // Apply cached signal strength from ML1 measurements
            header.signal_dbm = get_cached_signal_dbm();
            Ok(Some(GsmtapMessage {
                header,
                payload: packet.take_payload(),
            }))
        }
        LogBody::Nas4GMessage { msg, direction, .. } => {
            // currently we only handle "plain" (i.e. non-secure) NAS messages
            let mut header = GsmtapHeader::new(GsmtapType::LteNas(LteNasSubtype::Plain));
            header.uplink = matches!(direction, Nas4GMessageDirection::Uplink);
            Ok(Some(GsmtapMessage {
                header,
                payload: msg,
            }))
        }
        LogBody::LteMl1ServingCellMeas { meas, .. } => {
            // Update the cell info cache with measurement data
            update_cell_info_cache(&meas);
            debug!(
                "ML1 0xB193 v{}: RSRP={:?}dBm, RSRQ={:?}dB, RSSI={:?}dBm, PCI={:?}, EARFCN={:?}",
                meas.subpacket_version,
                meas.get_rsrp_dbm(),
                meas.get_rsrq_db(),
                meas.get_rssi_dbm(),
                meas.get_pci(),
                meas.get_earfcn()
            );
            // Measurement messages don't produce GSMTAP output themselves,
            // they just update the cell info cache for subsequent messages.
            Ok(None)
        }
        _ => {
            error!("gsmtap_sink: ignoring unhandled log type: {value:?}");
            Ok(None)
        }
    }
}
