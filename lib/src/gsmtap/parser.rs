use crate::diag::Message;
use crate::diag::diaglog::{LogBody, Nas4GMessageDirection, Timestamp};
use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, LteNasSubtype, LteRrcSubtype};

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
            header.arfcn = (packet.get_earfcn() as u16) & 0x3FFF;
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
        LogBody::LteMl1ServingCellMeas { packet, .. } => {
            // frame_number reused for PCI (normally SFN in RRC frames) so all three
            // serving-cell fields are accessible in Wireshark as gsmtap.* columns.
            let mut header = GsmtapHeader::new(GsmtapType::QcDiag);
            header.signal_dbm = packet.get_rsrp_dbm();
            header.arfcn = packet.get_earfcn().try_into().unwrap_or(0);
            header.frame_number = packet.get_pci() as u32;
            Ok(Some(GsmtapMessage {
                header,
                payload: vec![],
            }))
        }
        LogBody::LteMacRachResponse { payload } => Ok(parse_rach_response(&payload)),
        _ => {
            error!("gsmtap_sink: ignoring unhandled log type: {value:?}");
            Ok(None)
        }
    }
}

// Parses a 0xb062 RACH response log and reconstructs a 7-byte MAC RAR PDU for Wireshark.
// Returns None if the log contains no MSG2 (no Timing Advance was received).
fn parse_rach_response(payload: &[u8]) -> Option<GsmtapMessage> {
    // Outer header: version(u8) + num_subpackets(u8) + reserved(u16)
    if *payload.first()? != 0x01 {
        return None;
    }
    let num_subpackets = *payload.get(1)? as usize;
    let mut offset = 4;

    for _ in 0..num_subpackets {
        // Subpacket header: id(u8) + version(u8) + size(u16 LE)
        let sp_hdr = payload.get(offset..offset + 4)?;
        let sp_id = sp_hdr[0];
        let sp_version = sp_hdr[1];
        let sp_size = u16::from_le_bytes([sp_hdr[2], sp_hdr[3]]) as usize;
        if sp_size < 4 {
            return None;
        }
        let sp_body = payload.get(offset + 4..offset + sp_size)?;

        if sp_id == 0x06
            && let Some(msg) = extract_rach_attempt_gsmtap(sp_body, sp_version)
        {
            return Some(msg);
        }

        offset += sp_size;
    }
    None
}

fn extract_rach_attempt_gsmtap(body: &[u8], version: u8) -> Option<GsmtapMessage> {
    // Per SCAT diagltelogparser.py, RACH Attempt subpacket layouts:
    //   v0x02:       hdr=4B, msg1=4B(BBh),   msg2=7B(HBHh)
    //   v0x03/0x31:  hdr=6B, msg1=4B(BBh),   msg2=7B(HBHh)
    //   v0x32:       hdr=6B, msg1=7B(BBhHb), msg2=7B(HBHh)
    // rapid_offset is the header byte holding preamble_index & 0x3F (the RAPID)
    let (hdr_size, msg1_size, rapid_offset, bitmask_offset) = match version {
        0x02 => (4usize, 4usize, 0usize, 3usize),
        0x03 | 0x31 => (6, 4, 2, 5),
        0x32 => (6, 7, 2, 5),
        _ => return None,
    };

    let hdr = body.get(..hdr_size)?;
    let msg_bitmask = hdr[bitmask_offset];
    let rapid = hdr[rapid_offset] & 0x3F;
    let msg1_present = msg_bitmask & 0x01 != 0;
    let msg2_present = msg_bitmask & 0x02 != 0;

    if !msg2_present {
        return None;
    }

    // MSG2: backoff(u16) + result(u8) + tc_rnti(u16) + ta(u16) = 7 bytes
    let msg2_start = hdr_size + if msg1_present { msg1_size } else { 0 };
    let msg2 = body.get(msg2_start..msg2_start + 7)?;
    let tc_rnti = u16::from_le_bytes([msg2[3], msg2[4]]);
    let ta_raw = u16::from_le_bytes([msg2[5], msg2[6]]);
    // 0xFFFF is a Qualcomm sentinel meaning the RAR was received but TA was not valid
    if ta_raw == 0xFFFF {
        return None;
    }
    let ta = ta_raw & 0x7FF;

    // Reconstruct 7-byte MAC RAR PDU (3GPP TS 36.321 §6.1.5):
    // subheader: E=0, T=0, RAPID[5:0]
    // payload:   R(1)|TA[10:3](8) | TA[2:0](3)|ULGrant[19:15](5) | ULGrant[14:7](8) |
    //            ULGrant[6:0](7)|TC-RNTI[15](1) | TC-RNTI[14:7](8) | TC-RNTI[6:0](7)|0(1)
    //
    // Use LteMacFramed (0x0f) so Wireshark's mac-lte dissector knows the RNTI type is
    // RA-RNTI (type=2) and applies the RAR PDU format. The 4-byte framing prefix is:
    //   [RadioType=1(FDD)][Direction=1(DL)][RNTIType=2(RA-RNTI)][0x01=payload-marker]
    let payload = vec![
        0x01u8,
        0x01,
        0x02,
        0x01, // framing: FDD, DL, RA-RNTI, payload-marker
        rapid & 0x3F,
        ((ta >> 3) & 0xFF) as u8,
        ((ta & 0x07) as u8) << 5,
        0u8, // UL grant zeroed; Wireshark only needs TA and TC-RNTI to decode the RAR
        ((tc_rnti >> 15) & 0x01) as u8,
        ((tc_rnti >> 7) & 0xFF) as u8,
        ((tc_rnti & 0x7F) as u8) << 1,
    ];

    let mut header = GsmtapHeader::new(GsmtapType::LteMacFramed);
    // Wireshark 4.x does not dispatch GSMTAP type 0x0f to its mac-lte dissector, so
    // mac-lte.rar.ta is unavailable. TA is also stored in frame_number (gsmtap.frame_nr).
    header.frame_number = ta as u32;
    Some(GsmtapMessage { header, payload })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gsmtap::GsmtapType;
    use deku::DekuContainerWrite;

    #[test]
    fn test_arfcn_exceeding_14_bits_does_not_panic() {
        let mut header = GsmtapHeader::new(GsmtapType::LteRrc(LteRrcSubtype::DlDcch));
        // EARFCN 54540 (band 46) exceeds 14-bit max of 16383
        let large_earfcn: u32 = 54540;
        header.arfcn = (large_earfcn as u16) & 0x3FFF;
        let msg = GsmtapMessage {
            header,
            payload: vec![0x00],
        };
        // This would panic before the fix with "bit size of input is larger than bit requested size"
        assert!(msg.to_bytes().is_ok());
    }

    // Builds a minimal 0xb062 payload: outer header + one RACH Attempt subpacket (version 0x03).
    // v0x03 body layout: hdr=6B [_, _, rapid, _, _, bitmask], then MSG2=7B [backoff(2), result(1), tc_rnti(2), ta(2)]
    fn make_rach_v03_payload(ta_raw: u16, bitmask: u8) -> Vec<u8> {
        let rapid: u8 = 43;
        let tc_rnti: u16 = 0x1234;
        let [ta_lo, ta_hi] = ta_raw.to_le_bytes();
        let [rnti_lo, rnti_hi] = tc_rnti.to_le_bytes();
        // sp_size covers the 4-byte subpacket header + 6-byte body header + 7-byte MSG2 = 17
        vec![
            0x01, 0x01, 0x00, 0x00, // outer: version=1, num_subpackets=1, reserved
            0x06, 0x03, 17, 0x00, // subpacket: id=0x06, version=0x03, size=17 LE
            0x00, 0x00, rapid, 0x00, 0x00, bitmask, // body header (6 bytes)
            0x00, 0x00, 0x01, rnti_lo, rnti_hi, ta_lo, ta_hi, // MSG2 (7 bytes)
        ]
    }

    #[test]
    fn test_rach_response_valid_ta() {
        let payload = make_rach_v03_payload(42, 0x02); // 0x02 = msg2 present, msg1 absent
        let msg = parse_rach_response(&payload).expect("expected a GsmtapMessage for valid TA");
        assert_eq!(msg.header.gsmtap_type, GsmtapType::LteMacFramed);
        // TA stored in frame_number for Wireshark compatibility (gsmtap.frame_nr)
        assert_eq!(msg.header.frame_number, 42);
        // MAC RAR PDU: 4-byte framing prefix + 7-byte RAR PDU = 11 bytes
        assert_eq!(msg.payload.len(), 11);
        // Verify TA encoding in RAR PDU bytes 5–6 (TA[10:3] and TA[2:0])
        // ta=42: ta>>3=5 in byte[5], (ta&7)<<5 = 2<<5 = 0x40 in byte[6]
        assert_eq!(msg.payload[5], 5);
        assert_eq!(msg.payload[6], 0x40);
    }

    #[test]
    fn test_rach_response_ffff_sentinel_returns_none() {
        // 0xFFFF means RAR was received but TA was not valid; must be dropped
        let payload = make_rach_v03_payload(0xFFFF, 0x02);
        assert!(parse_rach_response(&payload).is_none());
    }

    #[test]
    fn test_rach_response_no_msg2_returns_none() {
        // bitmask=0x01 means only MSG1 present; no TA available
        let payload = make_rach_v03_payload(42, 0x01);
        assert!(parse_rach_response(&payload).is_none());
    }
}
