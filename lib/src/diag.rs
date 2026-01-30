//! Diag protocol serialization/deserialization

use chrono::{DateTime, FixedOffset};
use crc::{Algorithm, Crc};
use deku::prelude::*;

use crate::hdlc::{self, hdlc_decapsulate};
use log::warn;
use thiserror::Error;

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
        packet: LteRrcOtaPacket,
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
    /// LTE ML1 Serving Cell Measurement Response (0xB193)
    /// Contains RSRP, RSRQ, and RSSI measurements for the serving cell.
    /// This is used to populate signal strength in GSMTAP headers.
    #[deku(id = "0xb193")]
    LteMl1ServingCellMeas { meas: LteMl1ServingCellMeasData },
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
#[deku(ctx = "ext_header_version: u8", id = "ext_header_version")]
pub enum LteRrcOtaPacket {
    #[deku(id_pat = "0..=4")]
    V0 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u16,
        sfn_subfn: u16,
        pdu_num: u8,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "5..=7")]
    V5 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u16,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "8..=24")]
    V8 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u32,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "25..")]
    V25 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        nr_rrc_rel_maj: u8,
        nr_rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u32,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
}

impl LteRrcOtaPacket {
    fn get_sfn_subfn(&self) -> u16 {
        match self {
            LteRrcOtaPacket::V0 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V5 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V8 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V25 { sfn_subfn, .. } => *sfn_subfn,
        }
    }
    pub fn get_sfn(&self) -> u32 {
        self.get_sfn_subfn() as u32 >> 4
    }

    pub fn get_subfn(&self) -> u8 {
        (self.get_sfn_subfn() & 0xf) as u8
    }

    pub fn get_pdu_num(&self) -> u8 {
        match self {
            LteRrcOtaPacket::V0 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V5 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V8 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V25 { pdu_num, .. } => *pdu_num,
        }
    }

    pub fn get_earfcn(&self) -> u32 {
        match self {
            LteRrcOtaPacket::V0 { earfcn, .. } => *earfcn as u32,
            LteRrcOtaPacket::V5 { earfcn, .. } => *earfcn as u32,
            LteRrcOtaPacket::V8 { earfcn, .. } => *earfcn,
            LteRrcOtaPacket::V25 { earfcn, .. } => *earfcn,
        }
    }

    pub fn take_payload(self) -> Vec<u8> {
        match self {
            LteRrcOtaPacket::V0 { packet, .. } => packet,
            LteRrcOtaPacket::V5 { packet, .. } => packet,
            LteRrcOtaPacket::V8 { packet, .. } => packet,
            LteRrcOtaPacket::V25 { packet, .. } => packet,
        }
    }
}

/// LTE ML1 Serving Cell Measurement (0xB193) packet structure.
/// Uses subpacket architecture per Mobile Insight / Qualcomm DIAG format.
///
/// Packet layout:
/// - Main Header: version (1) + num_subpackets (1) + reserved (2) = 4 bytes
/// - SubPacket Header: id (1) + version (1) + size (2) = 4 bytes
/// - SubPacket Data: varies by subpacket version (v4, v7, v18, v19, v22, v24, v35, v36, v40)
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct LteMl1ServingCellMeasData {
    pub main_version: u8,
    pub num_subpackets: u8,
    pub reserved: u16,
    // SubPacket header
    pub subpacket_id: u8,
    pub subpacket_version: u8,
    pub subpacket_size: u16,
    // SubPacket data - we read enough to get RSRP/RSRQ/RSSI
    // The actual layout depends on subpacket_version, but EARFCN and PCI are always first
    #[deku(count = "subpacket_size.saturating_sub(4).min(128)")]
    pub subpacket_data: Vec<u8>,
}

impl LteMl1ServingCellMeasData {
    /// Helper to read a u16 from subpacket data at given offset
    fn read_u16(&self, offset: usize) -> Option<u16> {
        if offset + 2 <= self.subpacket_data.len() {
            Some(u16::from_le_bytes([
                self.subpacket_data[offset],
                self.subpacket_data[offset + 1],
            ]))
        } else {
            None
        }
    }

    /// Helper to read a u32 from subpacket data at given offset
    fn read_u32(&self, offset: usize) -> Option<u32> {
        if offset + 4 <= self.subpacket_data.len() {
            Some(u32::from_le_bytes([
                self.subpacket_data[offset],
                self.subpacket_data[offset + 1],
                self.subpacket_data[offset + 2],
                self.subpacket_data[offset + 3],
            ]))
        } else {
            None
        }
    }

    /// Get the RSRP field offset based on subpacket version
    /// Returns (earfcn_offset, earfcn_size, rsrp_offset)
    fn get_offsets(&self) -> (usize, usize, usize) {
        match self.subpacket_version {
            // v4: EARFCN(2) + PCI(2) + SFN(2) + skip(6) = offset 12 for RSRP
            4 => (0, 2, 12),
            // v7: EARFCN(4) + PCI(2) + SFN(2) + skip(6) = offset 14 for RSRP
            7 => (0, 4, 14),
            // v18+: more complex, estimate based on structure
            // EARFCN(4) + PCI(2) + ... + skip = ~24-34 for RSRP
            18..=24 => (0, 4, 24),
            // v35+: 4-antenna support, larger structure
            35..=40 => (0, 4, 28),
            // Unknown version, try v7 offsets
            _ => (0, 4, 14),
        }
    }

    /// Get Physical Cell ID from measurement
    pub fn get_pci(&self) -> Option<u16> {
        let (earfcn_offset, earfcn_size, _) = self.get_offsets();
        let pci_offset = earfcn_offset + earfcn_size;
        self.read_u16(pci_offset).map(|v| v & 0x1FF)
    }

    /// Get EARFCN from measurement
    pub fn get_earfcn(&self) -> Option<u32> {
        let (earfcn_offset, earfcn_size, _) = self.get_offsets();
        if earfcn_size == 2 {
            self.read_u16(earfcn_offset).map(|v| v as u32)
        } else {
            self.read_u32(earfcn_offset)
        }
    }

    /// Get RSRP (Reference Signal Received Power) in dBm.
    /// Formula: -180 + raw_value * 0.0625
    pub fn get_rsrp_dbm(&self) -> Option<f32> {
        let (_, _, rsrp_offset) = self.get_offsets();
        self.read_u32(rsrp_offset).map(|raw| {
            let rsrp_raw = raw & 0xFFF;
            -180.0 + (rsrp_raw as f32) * 0.0625
        })
    }

    /// Get RSSI (Received Signal Strength Indicator) in dBm.
    /// Formula: -110 + raw_value * 0.0625
    /// RSSI is typically 12 bytes after RSRP (RSRP + avg_RSRP + RSRQ)
    pub fn get_rssi_dbm(&self) -> Option<f32> {
        let (_, _, rsrp_offset) = self.get_offsets();
        let rssi_offset = rsrp_offset + 12; // Skip RSRP(4) + avg_RSRP(4) + RSRQ(4)
        self.read_u32(rssi_offset).map(|raw| {
            let rssi_raw = (raw >> 10) & 0x7FF;
            -110.0 + (rssi_raw as f32) * 0.0625
        })
    }

    /// Get RSRQ (Reference Signal Received Quality) in dB.
    /// Formula: -30 + raw_value * 0.0625
    pub fn get_rsrq_db(&self) -> Option<f32> {
        let (_, _, rsrp_offset) = self.get_offsets();
        let rsrq_offset = rsrp_offset + 8; // Skip RSRP(4) + avg_RSRP(4)
        self.read_u32(rsrq_offset).map(|raw| {
            let rsrq_raw = raw & 0x3FF;
            -30.0 + (rsrq_raw as f32) * 0.0625
        })
    }

    /// Get signal strength as i8 for GSMTAP header (clamped to valid range).
    /// Uses RSRP as the primary signal indicator.
    pub fn get_signal_dbm_i8(&self) -> Option<i8> {
        self.get_rsrp_dbm()
            .map(|rsrp| rsrp.clamp(-128.0, 127.0) as i8)
    }
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
        // Expected mask includes:
        // - 0xB0C0 (LTE RRC): byte 24 = 0x01
        // - 0xB0E2, 0xB0E3, 0xB0EC, 0xB0ED (NAS): bytes 28-29 = 0x0C, 0x30
        // - 0xB193 (ML1 Serving Cell Meas): byte 50 = 0x08 (bit 3 for code 0x193 = 403)
        assert_eq!(
            req,
            Request::LogConfig(LogConfigRequest::SetMask {
                log_type,
                log_mask_bitsize: bitsize,
                log_mask: vec![
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xc, 0x30, 0x0,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                    0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
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
                    packet: LteRrcOtaPacket::V8 {
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
                packet: LteRrcOtaPacket::V8 {
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

    #[test]
    fn test_lte_ml1_serving_cell_meas_parsing() {
        // Test parsing of 0xB193 LTE ML1 Serving Cell Measurement log
        // with subpacket version 18 (common on Orbic RC400L)
        //
        // Structure:
        // - Log message header (type=16, log_type=0xB193)
        // - LteMl1ServingCellMeasData with v18 subpacket containing RSRP=-95dBm
        //
        // RSRP calculation: -180 + (raw & 0xFFF) * 0.0625
        // For -95 dBm: raw = (-95 + 180) / 0.0625 = 1360 = 0x550

        let mut msg_bytes: Vec<u8> = vec![
            // Log message header
            0x10, // Message type: Log (16)
            0x00, // pending_msgs
            0x38, 0x00, // outer_length: 56
            0x34, 0x00, // inner_length: 52
            0x93, 0xB1, // log_type: 0xB193 (LTE ML1 Serving Cell Meas)
            // timestamp (8 bytes, arbitrary)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // LteMl1ServingCellMeasData
            0x01, // main_version
            0x01, // num_subpackets
            0x00, 0x00, // reserved
            0x00, // subpacket_id
            0x12, // subpacket_version: 18
            0x28, 0x00, // subpacket_size: 40 bytes (including header)
        ];

        // Subpacket data (36 bytes = 40 - 4 for header)
        // For v18: EARFCN at offset 0 (4 bytes), PCI at offset 4 (2 bytes), RSRP at offset 24
        let mut subpacket_data = vec![0u8; 36];
        // EARFCN = 975 at offset 0 (u32 LE)
        subpacket_data[0..4].copy_from_slice(&975u32.to_le_bytes());
        // PCI = 446 at offset 4 (u16 LE, only lower 9 bits used)
        subpacket_data[4..6].copy_from_slice(&446u16.to_le_bytes());
        // RSRP raw = 1360 (0x550) at offset 24 (u32 LE)
        // This gives RSRP = -180 + 1360 * 0.0625 = -95 dBm
        subpacket_data[24..28].copy_from_slice(&1360u32.to_le_bytes());

        msg_bytes.extend(subpacket_data);

        let ((rest, _), msg) = Message::from_bytes((&msg_bytes, 0)).unwrap();

        assert_eq!(rest.len(), 0, "Should consume all bytes");

        if let Message::Log {
            log_type,
            body: LogBody::LteMl1ServingCellMeas { meas },
            ..
        } = msg
        {
            assert_eq!(log_type, 0xB193);
            assert_eq!(meas.subpacket_version, 18);

            // Verify RSRP extraction
            let rsrp = meas.get_rsrp_dbm().expect("Should extract RSRP");
            assert!(
                (rsrp - (-95.0)).abs() < 0.1,
                "RSRP should be -95 dBm, got {}",
                rsrp
            );

            // Verify PCI extraction
            let pci = meas.get_pci().expect("Should extract PCI");
            assert_eq!(pci, 446);

            // Verify EARFCN extraction
            let earfcn = meas.get_earfcn().expect("Should extract EARFCN");
            assert_eq!(earfcn, 975);

            // Verify i8 conversion for GSMTAP header
            let signal_dbm = meas.get_signal_dbm_i8().expect("Should get signal_dbm");
            assert_eq!(signal_dbm, -95);
        } else {
            panic!("Expected LteMl1ServingCellMeas message, got {:?}", msg);
        }
    }
}
