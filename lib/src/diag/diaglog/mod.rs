//! Diag LogBody serialization/deserialization

use chrono::{DateTime, FixedOffset};
use deku::prelude::*;

pub mod mac;
pub mod measurement;
pub mod rrc;
#[cfg(test)]
mod test_util;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "log_type: u16, hdr_len: u16", id = "log_type")]
pub enum LogBody {
    #[deku(id = "0x412f")]
    WcdmaSignallingMessage {
        channel_type: u8,
        radio_bearer: u8,
        length: u16,
        #[deku(count = "length")]
        msg: Vec<u8>,
    },
    #[deku(id = "0x512f")]
    GsmRrSignallingMessage {
        channel_type: u8,
        message_type: u8,
        length: u8,
        #[deku(count = "length")]
        msg: Vec<u8>,
    },
    #[deku(id = "0x5226")]
    GprsMacSignallingMessage {
        channel_type: u8,
        message_type: u8,
        length: u8,
        #[deku(count = "length")]
        msg: Vec<u8>,
    },
    #[deku(id = "0xb0c0")]
    LteRrcOtaMessage {
        ext_header_version: u8,
        #[deku(ctx = "*ext_header_version")]
        packet: rrc::LteRrcOtaPacket,
    },
    // the four NAS command opcodes refer to:
    // * 0xb0e2: plain ESM NAS message (incoming)
    // * 0xb0e3: plain ESM NAS message (outgoing)
    // * 0xb0ec: plain EMM NAS message (incoming)
    // * 0xb0ed: plain EMM NAS message (outgoing)
    #[deku(id_pat = "0xb0e2 | 0xb0e3 | 0xb0ec | 0xb0ed")]
    Nas4GMessage {
        #[deku(skip, default = "log_type")]
        log_type: u16,
        #[deku(ctx = "*log_type")]
        direction: Nas4GMessageDirection,
        ext_header_version: u8,
        rrc_rel: u8,
        rrc_version_minor: u8,
        rrc_version_major: u8,
        // message length = hdr_len - (sizeof(ext_header_version) + sizeof(rrc_rel) + sizeof(rrc_version_minor) + sizeof(rrc_version_major))
        #[deku(count = "hdr_len.saturating_sub(4)")]
        msg: Vec<u8>,
    },
    #[deku(id = "0x11eb")]
    IpTraffic {
        // is this right?? based on https://github.com/P1sec/QCSuper/blob/81dbaeee15ec7747e899daa8e3495e27cdcc1264/src/modules/pcap_dump.py#L378
        #[deku(count = "hdr_len.saturating_sub(8)")]
        msg: Vec<u8>,
    },
    #[deku(id = "0x713a")]
    UmtsNasOtaMessage {
        is_uplink: u8,
        length: u32,
        #[deku(count = "length")]
        msg: Vec<u8>,
    },
    #[deku(id = "0xb821")]
    NrRrcOtaMessage {
        #[deku(count = "hdr_len")]
        msg: Vec<u8>,
    },
    #[deku(id = "0xb17f")]
    LteMl1ServingCellMeasurementAndEvaluation {
        data: measurement::serving_cell::MeasurementAndEvaluation,
    },
    #[deku(id = "0xb180")]
    LteMl1NeighborCellsMeasurements {
        data: measurement::neighbor_cells::Measurements,
    },
    // Raw bytes; subpacket parsing happens in gsmtap_parser to extract Timing Advance
    #[deku(id = "0xb062")]
    LteMacRachResponse {
        #[deku(count = "hdr_len")]
        payload: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "log_type: u16", id = "log_type")]
pub enum Nas4GMessageDirection {
    // * 0xb0e2: plain ESM NAS message (incoming)
    // * 0xb0e3: plain ESM NAS message (outgoing)
    // * 0xb0ec: plain EMM NAS message (incoming)
    // * 0xb0ed: plain EMM NAS message (outgoing)
    #[deku(id_pat = "0xb0e2 | 0xb0ec")]
    Downlink,
    #[deku(id_pat = "0xb0e3 | 0xb0ed")]
    Uplink,
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct Timestamp {
    pub ts: u64,
}

impl Timestamp {
    pub fn to_datetime(&self) -> DateTime<FixedOffset> {
        // Upper 48 bits: epoch at 1980-01-06 00:00:00, incremented by 1 for 1/800s
        // Lower 16 bits: time since last 1/800s tick in 1/32 chip units
        let ts_upper = self.ts >> 16;
        let ts_lower = self.ts & 0xffff;
        let epoch = chrono::DateTime::parse_from_rfc3339("1980-01-06T00:00:00-00:00").unwrap();
        let mut delta_seconds = ts_upper as f64 * 1.25;
        delta_seconds += ts_lower as f64 / 40960.0;
        let ts_delta = chrono::Duration::milliseconds(delta_seconds as i64);
        epoch + ts_delta
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::{diag::*, hdlc};

    #[test]
    fn test_logs() {
        let data = vec![
            16, 0, 38, 0, 38, 0, 192, 176, 26, 165, 245, 135, 118, 35, 2, 1, 20, 14, 48, 0, 160, 0,
            2, 8, 0, 0, 217, 15, 5, 0, 0, 0, 0, 7, 0, 64, 1, 238, 173, 213, 77, 208,
        ];
        let msg = Message::from_bytes((&data, 0)).unwrap().1;
        assert_eq!(
            msg,
            Message::Log {
                pending_msgs: 0,
                outer_length: 38,
                inner_length: 38,
                log_type: 0xb0c0,
                timestamp: Timestamp {
                    ts: 72659535985485082
                },
                body: LogBody::LteRrcOtaMessage {
                    ext_header_version: 20,
                    packet: rrc::LteRrcOtaPacket::V8 {
                        rrc_rel_maj: 14,
                        rrc_rel_min: 48,
                        bearer_id: 0,
                        phy_cell_id: 160,
                        earfcn: 2050,
                        sfn_subfn: 4057,
                        pdu_num: 5,
                        sib_mask: 0,
                        len: 7,
                        packet: vec![0x40, 0x1, 0xee, 0xad, 0xd5, 0x4d, 0xd0],
                    },
                },
            }
        );
    }

    #[test]
    fn test_fuzz_crash_inner_length_underflow() {
        // Regression test: inner_length < 12 previously caused panic.
        // Fixed by using saturating_sub in Message::Log body length calculation.
        let fuzz_data = b"\x10\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let _ = Message::from_bytes((fuzz_data, 0));
    }

    #[test]
    fn test_fuzz_crash_nas_hdr_len_underflow() {
        // Regression test for two things:
        // - hdr_len < 4 previously caused panic in Nas4GMessage.
        // - Upgrading to deku 0.20 caused incorrect parsing behavior (double-read of discriminant)
        let nas_msg =
            b"\x10\x00\x14\x00\x02\x00\xe2\xb0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00";

        let ((rest, _), msg) = Message::from_bytes((nas_msg, 0)).unwrap();

        assert_eq!(rest.len(), 0);
        assert!(
            matches!(
                msg,
                Message::Log {
                    log_type: 0xb0e2,
                    body: LogBody::Nas4GMessage {
                        direction: Nas4GMessageDirection::Downlink,
                        ..
                    },
                    ..
                }
            ),
            "Unexpected message: {:?}",
            msg
        );
    }

    #[test]
    fn test_fuzz_crash_ip_traffic_hdr_len_underflow() {
        // Regression test: hdr_len < 8 previously caused panic in IpTraffic.
        // Fixed by using saturating_sub for msg length calculation.
        let ip_msg = b"\x10\x00\x14\x00\x02\x00\xeb\x11\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00";
        let _ = Message::from_bytes((ip_msg, 0));
    }

    // Just about all of these test cases from manually parsing diag packets w/ QCSuper
    #[test]
    fn test_request_serialization() {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        assert_eq!(req.to_bytes().unwrap(), vec![115, 0, 0, 0, 1, 0, 0, 0]);

        let req = Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 0,
            log_mask: vec![],
        });
        assert_eq!(
            req.to_bytes().unwrap(),
            vec![115, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_build_log_mask_request() {
        let log_type = 11;
        let bitsize = 513;
        let req = build_log_mask_request(
            log_type,
            bitsize,
            &crate::diag_device::LOG_CODES_FOR_RAW_PACKET_LOGGING,
        );
        assert_eq!(
            req,
            Request::LogConfig(LogConfigRequest::SetMask {
                log_type,
                log_mask_bitsize: bitsize,
                log_mask: vec![
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xc, 0x30, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x80, 0x1, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0,
                ],
            })
        );
    }

    #[test]
    fn test_request_container() {
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: false,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(req.to_bytes().unwrap(), vec![32, 0, 0, 0, 1, 2, 3, 4,]);
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: true,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(
            req.to_bytes().unwrap(),
            vec![32, 0, 0, 0, 255, 255, 255, 255, 1, 2, 3, 4,]
        );
    }

    fn make_container(data_type: DataType, message: HdlcEncapsulatedMessage) -> MessagesContainer {
        MessagesContainer {
            data_type,
            num_messages: 1,
            messages: vec![message],
        }
    }

    // this log is based on one captured on a real device -- if it fails to
    // serialize or deserialize, that's probably a problem with this mock, not
    // the DiagReader implementation
    pub fn get_test_message(payload: &[u8]) -> (HdlcEncapsulatedMessage, Message) {
        let length_with_payload = 31 + payload.len() as u16;
        let message = Message::Log {
            pending_msgs: 0,
            outer_length: length_with_payload,
            inner_length: length_with_payload,
            log_type: 0xb0c0,
            timestamp: Timestamp {
                ts: 72659535985485082,
            },
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 20,
                packet: diaglog::rrc::LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 14,
                    rrc_rel_min: 48,
                    bearer_id: 0,
                    phy_cell_id: 160,
                    earfcn: 2050,
                    sfn_subfn: 4057,
                    pdu_num: 5,
                    sib_mask: 0,
                    len: payload.len() as u16,
                    packet: payload.to_vec(),
                },
            },
        };
        let serialized = message
            .to_bytes()
            .expect("failed to serialize test message");
        let encapsulated_data = hdlc::hdlc_encapsulate(&serialized, &CRC_CCITT);
        let encapsulated = HdlcEncapsulatedMessage {
            len: encapsulated_data.len() as u32,
            data: encapsulated_data,
        };
        // sanity check
        assert_eq!(&Message::from_hdlc(&encapsulated.data).unwrap(), &message);
        (encapsulated, message)
    }

    #[test]
    fn test_containers_with_multiple_messages() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(encapsulated2);
        container.num_messages += 1;
        assert_eq!(container.messages(), vec![Ok(message1), Ok(message2)]);
    }

    #[test]
    fn test_containers_with_concatenated_message() {
        let (mut encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        encapsulated1.data.extend(encapsulated2.data);
        encapsulated1.len += encapsulated2.len;
        let container = make_container(DataType::UserSpace, encapsulated1);
        assert_eq!(container.messages(), vec![Ok(message1), Ok(message2)]);
    }

    #[test]
    fn test_handles_parsing_errors() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let bad_message = hdlc::hdlc_encapsulate(&[0x01, 0x02, 0x03, 0x04], &CRC_CCITT);
        let encapsulated2 = HdlcEncapsulatedMessage {
            len: bad_message.len() as u32,
            data: bad_message,
        };
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(encapsulated2);
        container.num_messages += 1;
        let result = container.messages();
        assert_eq!(result[0], Ok(message1));
        assert!(matches!(
            result[1],
            Err(DiagParsingError::MessageParsingError(_, _))
        ));
    }

    #[test]
    fn test_handles_encapsulation_errors() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let bad_encapsulation = HdlcEncapsulatedMessage {
            len: 4,
            data: vec![0x01, 0x02, 0x03, 0x04],
        };
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(bad_encapsulation);
        container.num_messages += 1;
        let result = container.messages();
        assert_eq!(result[0], Ok(message1));
        assert!(matches!(
            result[1],
            Err(DiagParsingError::HdlcDecapsulationError(_, _))
        ));
    }

    #[test]
    fn test_fuzz_crash_response_opcode_parsing() {
        // Regression test: Upgrading to deku 0.20 caused incorrect parsing of Response messages.
        // The issue was that deku 0.20 requires an `id` field for `id_pat = "_"` variants,
        // but in deku 0.18 the discriminant was NOT consumed from the stream.
        // This caused a 1-byte offset, making opcode and all subsequent fields misaligned.
        // Fixed by splitting the opcode into 4 separate u8 fields so the discriminant byte
        // becomes the first byte of the opcode, matching the old deku 0.18 behavior.
        let response_msg = b"\x73\x00\x00\x00\x03\x00\x00\x00\x0a\x00\xec\xb0\x8e\x51\x02\x6f\x2a\xc5\x0b\x01\x01\x09\x05\x00\x07\x45\x8e\x14\x7d";

        let ((rest, _), msg) = Message::from_bytes((response_msg, 0)).unwrap();

        // Verify the opcode is correctly parsed as 115 (0x73 in first byte)
        // In little-endian: [0x73, 0x00, 0x00, 0x00] = 0x00000073 = 115
        assert!(
            matches!(
                msg,
                Message::Response {
                    opcode1: 0x73,
                    opcode2: 0x00,
                    opcode3: 0x00,
                    opcode4: 0x00,
                    subopcode: 3,
                    status: 2968256522, // [0x0a, 0x00, 0xec, 0xb0] in LE
                    payload: ResponsePayload::LogConfig(LogConfigResponse::SetMask),
                }
            ),
            "Unexpected message: {:?}",
            msg
        );

        // Verify we consumed the expected number of bytes
        assert_eq!(rest.len(), 17);
    }
}
