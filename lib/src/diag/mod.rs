//! Diag protocol serialization/deserialization

use crc::{Algorithm, Crc};
use deku::prelude::*;

use crate::hdlc::{self, hdlc_decapsulate};
use log::warn;
use thiserror::Error;

pub mod diaglog;

use diaglog::{LogBody, Timestamp};

pub const MESSAGE_TERMINATOR: u8 = 0x7e;
pub const MESSAGE_ESCAPE_CHAR: u8 = 0x7d;

pub const ESCAPED_MESSAGE_TERMINATOR: u8 = 0x5e;
pub const ESCAPED_MESSAGE_ESCAPE_CHAR: u8 = 0x5d;

#[derive(Debug, Clone, DekuWrite)]
pub struct RequestContainer {
    pub data_type: DataType,
    #[deku(skip)]
    pub use_mdm: bool,
    #[deku(skip, cond = "!*use_mdm")]
    pub mdm_field: i32,
    pub hdlc_encapsulated_request: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(id_type = "u32")]
pub enum Request {
    #[deku(id = "115")]
    LogConfig(LogConfigRequest),
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(id_type = "u32", endian = "little")]
pub enum LogConfigRequest {
    #[deku(id = "1")]
    RetrieveIdRanges,

    #[deku(id = "3")]
    SetMask {
        log_type: u32,
        log_mask_bitsize: u32,
        log_mask: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u32", endian = "little")]
pub enum DataType {
    #[deku(id = "32")]
    UserSpace,
    #[deku(id_pat = "_")]
    Other(u32),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DiagParsingError {
    #[error("Failed to parse Message: {0}, data: {1:?}")]
    MessageParsingError(deku::DekuError, Vec<u8>),
    #[error("HDLC decapsulation of message failed: {0}, data: {1:?}")]
    HdlcDecapsulationError(hdlc::HdlcError, Vec<u8>),
}

// this is sorta based on the params qcsuper uses, plus what seems to be used in
// https://github.com/fgsect/scat/blob/f1538b397721df3ab8ba12acd26716abcf21f78b/util.py#L47
pub const CRC_CCITT_ALG: Algorithm<u16> = Algorithm {
    poly: 0x1021,
    init: 0xffff,
    refin: true,
    refout: true,
    width: 16,
    xorout: 0xffff,
    check: 0x2189,
    residue: 0x0000,
};

pub const CRC_CCITT: Crc<u16> = Crc::<u16>::new(&CRC_CCITT_ALG);
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct MessagesContainer {
    pub data_type: DataType,
    pub num_messages: u32,
    #[deku(count = "num_messages")]
    pub messages: Vec<HdlcEncapsulatedMessage>,
}

impl MessagesContainer {
    pub fn into_messages(self) -> Vec<Result<Message, DiagParsingError>> {
        let mut result = Vec::new();
        for msg in self.messages {
            for sub_msg in msg.data.split_inclusive(|&b| b == MESSAGE_TERMINATOR) {
                match hdlc_decapsulate(sub_msg, &CRC_CCITT) {
                    Ok(data) => match Message::from_bytes((&data, 0)) {
                        Ok(((leftover_bytes, _), res)) => {
                            if !leftover_bytes.is_empty() {
                                warn!(
                                    "warning: {} leftover bytes when parsing Message",
                                    leftover_bytes.len()
                                );
                            }
                            result.push(Ok(res));
                        }
                        Err(e) => result.push(Err(DiagParsingError::MessageParsingError(e, data))),
                    },
                    Err(err) => result.push(Err(DiagParsingError::HdlcDecapsulationError(
                        err,
                        sub_msg.to_vec(),
                    ))),
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct HdlcEncapsulatedMessage {
    pub len: u32,
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Message {
    #[deku(id = "16")]
    Log {
        pending_msgs: u8,
        outer_length: u16,
        inner_length: u16,
        log_type: u16,
        timestamp: Timestamp,
        // pass the log type and log length (inner_length - (sizeof(log_type) + sizeof(timestamp)))
        #[deku(ctx = "*log_type, inner_length.saturating_sub(12)")]
        body: LogBody,
    },

    // kinda unpleasant deku hackery here. deku expects an enum's variant to be
    // right before its data, but in this case, a status value comes between the
    // variants and the data. so we need to use deku's context (ctx) feature to
    // pass those opcodes down to their respective parsers.
    #[deku(id_pat = "_")]
    Response {
        opcode1: u8, // the "id" (from deku's POV) gets parsed into this field
        opcode2: u8,
        opcode3: u8,
        opcode4: u8,
        subopcode: u32,
        status: u32,
        #[deku(ctx = "u32::from_le_bytes([*opcode1, *opcode2, *opcode3, *opcode4]), *subopcode")]
        payload: ResponsePayload,
    },
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "opcode: u32, subopcode: u32", id = "opcode")]
pub enum ResponsePayload {
    #[deku(id = "115")]
    LogConfig(#[deku(ctx = "subopcode")] LogConfigResponse),
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "subopcode: u32", id = "subopcode")]
pub enum LogConfigResponse {
    #[deku(id = "1")]
    RetrieveIdRanges { log_mask_sizes: [u32; 16] },

    #[deku(id = "3")]
    SetMask,
}

pub fn build_log_mask_request(
    log_type: u32,
    log_mask_bitsize: u32,
    accepted_log_codes: &[u32],
) -> Request {
    let mut current_byte: u8 = 0;
    let mut num_bits_written: u8 = 0;
    let mut log_mask: Vec<u8> = vec![];
    for i in 0..log_mask_bitsize {
        let log_code: u32 = (log_type << 12) | i;
        if accepted_log_codes.contains(&log_code) {
            current_byte |= 1 << num_bits_written;
        }
        num_bits_written += 1;

        if num_bits_written == 8 || i == log_mask_bitsize - 1 {
            log_mask.push(current_byte);
            current_byte = 0;
            num_bits_written = 0;
        }
    }

    Request::LogConfig(LogConfigRequest::SetMask {
        log_type,
        log_mask_bitsize,
        log_mask,
    })
}

#[cfg(test)]
mod test {
    use super::*;

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
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xc, 0x30, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0,
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
    fn get_test_message(payload: &[u8]) -> (HdlcEncapsulatedMessage, Message) {
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
        (encapsulated, message)
    }

    #[test]
    fn test_containers_with_multiple_messages() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(encapsulated2);
        container.num_messages += 1;
        assert_eq!(container.into_messages(), vec![Ok(message1), Ok(message2)]);
    }

    #[test]
    fn test_containers_with_concatenated_message() {
        let (mut encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        encapsulated1.data.extend(encapsulated2.data);
        encapsulated1.len += encapsulated2.len;
        let container = make_container(DataType::UserSpace, encapsulated1);
        assert_eq!(container.into_messages(), vec![Ok(message1), Ok(message2)]);
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
        let result = container.into_messages();
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
        let result = container.into_messages();
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
