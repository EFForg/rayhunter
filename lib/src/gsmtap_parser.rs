use crate::diag::*;
use crate::gsmtap::*;

use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GsmtapParserError {
    #[error("Invalid LteRrcOtaMessage ext header version {0}")]
    InvalidLteRrcOtaExtHeaderVersion(u8),
    #[error("Invalid LteRrcOtaMessage header/PDU number combination: {0}/{1}")]
    InvalidLteRrcOtaHeaderPduNum(u8, u8),
}

pub fn parse(msg: Message) -> Result<Option<(Timestamp, GsmtapMessage)>, GsmtapParserError> {
    let Message::Log { timestamp, body, .. } = msg else {
        return Ok(None);
    };

    match log_to_gsmtap(body)? {
        Some(msg) => Ok(Some((timestamp, msg))),
        None => Ok(None),
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
        _ => {
            error!("gsmtap_sink: ignoring unhandled log type: {value:?}");
            Ok(None)
        }
    }
}
