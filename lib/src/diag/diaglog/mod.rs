//! Diag LogBody serialization/deserialization

use chrono::{DateTime, FixedOffset};
use deku::prelude::*;

pub mod measurement;
pub mod rrc;

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
mod test {
    use super::*;
    use crate::diag::Message;

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
}
