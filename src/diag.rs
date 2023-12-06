//! Diag protocol serialization/deserialization

use chrono::{DateTime, FixedOffset};
use deku::{prelude::*, bitvec::{BitSlice, Msb0}};

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
#[deku(type = "u32")]
pub enum Request {
    #[deku(id = "115")]
    LogConfig(LogConfigRequest),
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(type = "u32", endian = "little")]
pub enum LogConfigRequest {
    #[deku(id = "1")]
    RetrieveIdRanges,

    #[deku(id = "3")]
    SetMask {
        log_type: u32,
        log_mask_bitsize: u32,
        log_mask: Vec<u8>,
    }
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "little")]
pub enum DataType {
    #[deku(id = "32")]
    UserSpace,
    #[deku(id_pat = "_")]
    Other(u32),
}

#[derive(Debug, Clone, DekuRead)]
pub struct MessagesContainer {
    pub data_type: DataType,
    pub num_responses: u32,
    #[deku(count = "num_responses")]
    pub messages: Vec<HdlcEncapsulatedMessage>,
}

#[derive(Debug, Clone, DekuRead)]
pub struct HdlcEncapsulatedMessage {
    pub len: u32,
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, DekuRead)]
#[deku(type = "u8")]
pub enum Message {
    #[deku(id = "16")]
    Log {
        pending_msgs: u8,
        outer_length: u16,
        inner_length: u16,
        log_type: u16,
        timestamp: Timestamp,
        //#[deku(count = "inner_length - 12")]
        #[deku(ctx = "*log_type, *inner_length - 12")]
        body: LogBody,
    },

    // kinda unpleasant deku hackery here. deku expects an enum's variant to be
    // right before its data, but in this case, a status value comes between the
    // variants and the data. so we need to use deku's context (ctx) feature to
    // pass those opcodes down to their respective parsers.
    #[deku(id_pat = "_")]
    Response {
        opcode: u32,
        subopcode: u32,
        status: u32,
        #[deku(ctx = "*opcode, *subopcode")]
        payload: ResponsePayload,
    },
}

#[derive(Debug, Clone, PartialEq, DekuRead)]
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
        rrc_rel: u8,
        rrc_version: u8,
        #[deku(skip, cond = "*ext_header_version < 25")] // handle post-NR releases
        nc_rrc_rel: Option<u16>,
        bearer_id: u8,
        phy_cell_id: u16,

        // extended header. some of these fields need manual parsing based on
        // header version
        #[deku(reader = "read_lte_rrc_ota_message_log_freq(deku::rest, *ext_header_version)")]
        freq: u32,
        sfn: u16,
        channel_type: u8,
        #[deku(reader = "read_lte_rrc_ota_message_log_msg(deku::rest)")]
        msg: Vec<u8>,
    },
    #[deku(id_pat = "0xb0e2 | 0xb0e3 | 0xb0ec | 0xb0ed")]
    Nas4GMessage {
        ext_header_version: u8,
        rrc_rel: u8,
        rrc_version_minor: u8,
        rrc_version_major: u8,
        #[deku(count = "hdr_len - 4")] // is this right??
        msg: Vec<u8>,
    },
    #[deku(id = "0x11eb")]
    IpTraffic {
        #[deku(count = "hdr_len - 8")] // is this right???
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
    }
}

fn read_lte_rrc_ota_message_log_freq(rest: &BitSlice<u8, Msb0>, ext_header_version: u8) -> Result<(&BitSlice<u8, Msb0>, u32), DekuError> {
    if ext_header_version < 8 {
        let (rest, freq) = u16::read(rest, ())?;
        Ok((rest, freq as u32))
    } else {
        let (rest, freq) = u32::read(rest, ())?;
        Ok((rest, freq))
    }
}

fn read_lte_rrc_ota_message_log_msg(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, Vec<u8>), DekuError> {
    let (mut rest, length) = u16::read(rest, ())?;
    if length != rest.len() as u16 / 8 {
        let (new_rest, _) = u16::read(rest, ())?;
        rest = new_rest;
    }
    let (new_rest, length) = u16::read(rest, ())?;
    rest = new_rest;
    if length != rest.len() as u16 / 8 {
        return Err(DekuError::Incomplete(NeedSize::new(length as usize * 8)));
    }
    let mut result = Vec::new();
    for _ in 0..length {
        let (new_rest, byte) = u8::read(rest, ())?;
        rest = new_rest;
        result.push(byte);
    }
    Ok((rest, result))
}

#[derive(Debug, Clone, PartialEq, DekuRead)]
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

#[derive(Debug, Clone, PartialEq, DekuRead)]
#[deku(ctx = "opcode: u32, subopcode: u32", id = "opcode")]
pub enum ResponsePayload {
    #[deku(id = "115")]
    LogConfig(#[deku(ctx = "subopcode")] LogConfigResponse),
}

#[derive(Debug, Clone, PartialEq, DekuRead)]
#[deku(ctx = "subopcode: u32", id = "subopcode")]
pub enum LogConfigResponse {
    #[deku(id = "1")]
    RetrieveIdRanges {
        log_mask_sizes: [u32; 16],
    },

    #[deku(id = "3")]
    SetMask,
}

pub fn build_log_mask_request(log_type: u32, log_mask_bitsize: u32, accepted_log_codes: &[u32]) -> Request {
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
        assert_eq!(req.to_bytes().unwrap(), vec![
            115, 0, 0, 0,
            3, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ]);
    }

    #[test]
    fn test_build_log_mask_request() {
        let log_type = 11;
        let bitsize = 513;
        let req = build_log_mask_request(log_type, bitsize, &crate::diag_device::LOG_CODES_FOR_RAW_PACKET_LOGGING);
        assert_eq!(req, Request::LogConfig(LogConfigRequest::SetMask {
            log_type: log_type,
            log_mask_bitsize: bitsize,
            log_mask: vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
                0x0, 0x0, 0xc, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0,
            ],
        }));
    }

    #[test]
    fn test_request_container() {
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: false,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(req.to_bytes().unwrap(), vec![
            32, 0, 0, 0,
            1, 2, 3, 4,
        ]);
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: true,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(req.to_bytes().unwrap(), vec![
            32, 0, 0, 0,
            255, 255, 255, 255,
            1, 2, 3, 4,
        ]);
    }

    #[test]
    fn test_logs() {
        env_logger::init();
        let data = vec![
            16, 0, 38, 0, 38, 0, 192, 176, 26, 165, 245, 135, 118, 35, 2, 1, 20,
            14, 48, 0, 160, 0, 2, 8, 0, 0, 217, 15, 5, 0, 0, 0, 0, 7, 0, 64, 1,
            238, 173, 213, 77, 208
        ];
        let msg = Message::from_bytes((&data, 0)).unwrap().1;
        assert_eq!(msg, Message::Log {
            pending_msgs: 0,
            outer_length: 38,
            inner_length: 38,
            log_type: 0xb0c0,
            timestamp: Timestamp { ts: 72659535985485082 },
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 20,
                rrc_rel: 14,
                rrc_version: 48,
                nc_rrc_rel: None,
                bearer_id: 0,
                phy_cell_id: 160,
                freq: 2050,
                sfn: 4057,
                channel_type: 5,
                msg: vec![0x40, 0x1, 0xee, 0xad, 0xd5, 0x4d, 0xd0],
            },
        });
    }
}
