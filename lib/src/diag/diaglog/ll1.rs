use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", ctx = "body_len: u16")]
pub enum ServingCellTiming {
    #[deku(id = "1")]
    V1 { data: ServingCellTimingV1 },
    #[deku(id = "0x7a")]
    V122 {
        #[deku(assert = "body_len == 16 + 8 * u16::from(data.num_records)")]
        data: ServingCellTimingV122,
    },
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct ServingCellTimingV1 {
    #[deku(bits = 5, assert = "(1..=20).contains(num_records)")]
    pub num_records: u8,
    #[deku(bits = 4, assert = "*starting_sub_fn <= 9")]
    pub starting_sub_fn: u8,
    #[deku(
        bits = 10,
        pad_bits_after = "5",
        assert = "*starting_system_fn <= 1023"
    )]
    pub starting_system_fn: u16,
    #[deku(
        bits = 19,
        pad_bits_after = "13",
        assert = "*starting_dl_frame_timing_offs <= 307200"
    )]
    pub starting_dl_frame_timing_offs: u32, // in Ts units
    #[deku(bits = 19, assert = "*starting_ul_frame_timing_offs <= 307200")]
    pub starting_ul_frame_timing_offs: u32, // in Ts units
    #[deku(bits = 11, pad_bits_after = "2")]
    pub starting_ul_timing_advance: u16, // in 16 Ts units
    #[deku(count = "num_records")]
    pub timing_adjustment: Vec<TimingAdjustment>,
}

/// The MR1100 envelope is verified; the timing fields inside these bytes are not.
/// Do not interpret these blocks using the version-1 timing units or bit layout.
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct ServingCellTimingV122 {
    #[deku(bits = 5, assert = "(1..=20).contains(num_records)")]
    pub num_records: u8,
    #[deku(bits = 3)]
    pub unknown_flags: u8,
    pub header_tail: [u8; 14],
    #[deku(count = "num_records")]
    pub record_bytes: Vec<[u8; 8]>,
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb", ctx = "_: deku::ctx::Order")]
pub struct TimingAdjustment {
    #[deku(
        bits = 11,
        assert = "(-512..=511).contains(dl_frame_timing_adjustment)"
    )]
    pub dl_frame_timing_adjustment: i16, // in Ts units
    #[deku(bits = 5, assert = "(-16..=15).contains(ul_frame_timing_adjustment)")]
    pub ul_frame_timing_adjustment: i8, // in Ts units
    #[deku(assert = "(-128..=127).contains(timing_advance)")]
    pub timing_advance: i8, // in 16 Ts units
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::diaglog::LogBody;
    use crate::diag::{CRC_CCITT, Message};
    use crate::hdlc::hdlc_encapsulate;

    // Synthetic fixtures: preserve the observed framing, not captured radio data.
    fn fixture(version: u8, count: u8) -> Vec<u8> {
        let (header, stride) = if version == 1 { (12, 3) } else { (16, 8) };
        let mut body = vec![0; header + stride * usize::from(count)];
        body[0] = version;
        body[1] = count;
        if version == 0x7a {
            body[1] |= 0b101 << 5;
            for (i, byte) in body[2..].iter_mut().enumerate() {
                *byte = (i as u8).wrapping_add(0x70);
            }
        }
        body
    }

    fn wire(body: &[u8]) -> Vec<u8> {
        let length = (body.len() as u16 + 12).to_le_bytes();
        let mut message = vec![0x10, 0];
        message.extend(length);
        message.extend(length);
        message.extend(0xb114u16.to_le_bytes());
        message.extend([0; 8]);
        message.extend(body);
        message
    }

    fn decode(body: &[u8]) -> Result<Message, crate::diag::DiagParsingError> {
        Message::from_hdlc(&hdlc_encapsulate(&wire(body), &CRC_CCITT))
    }

    #[test]
    fn version_one_still_decodes_and_round_trips() {
        for count in [1, 20] {
            let body = fixture(1, count);
            let message = decode(&body).unwrap();
            assert_eq!(message.to_bytes().unwrap(), wire(&body));
            let Message::Log {
                body:
                    LogBody::LteLl1ServingCellTiming {
                        data: ServingCellTiming::V1 { data },
                    },
                ..
            } = message
            else {
                panic!("wrong timing format")
            };
            assert_eq!(data.num_records, count);
            assert_eq!(data.starting_ul_timing_advance, 0);
            assert_eq!(data.timing_adjustment.len(), usize::from(count));
        }
    }

    #[test]
    fn mr1100_preserves_every_unknown_byte() {
        for count in 1..=20 {
            let body = fixture(0x7a, count);
            let message = decode(&body).unwrap();
            assert_eq!(message.to_bytes().unwrap(), wire(&body));
            let Message::Log {
                body:
                    LogBody::LteLl1ServingCellTiming {
                        data: ServingCellTiming::V122 { data },
                    },
                ..
            } = message
            else {
                panic!("wrong timing format")
            };
            assert_eq!(data.num_records, count);
            assert_eq!(data.unknown_flags, 0b101);
            assert_eq!(&data.header_tail, &body[2..16]);
            assert_eq!(data.record_bytes.concat(), body[16..]);
        }
    }

    #[test]
    fn invalid_counts_lengths_and_versions_still_fail() {
        for version in [1, 0x7a] {
            for count in [0, 21, 31] {
                assert!(decode(&fixture(version, count)).is_err());
            }
            let body = fixture(version, 2);
            for cut in 0..body.len() {
                assert!(
                    decode(&body[..cut]).is_err(),
                    "accepted truncated format {version}, length {cut}"
                );
            }
            if version == 0x7a {
                let mut extra = body;
                extra.push(0);
                assert!(decode(&extra).is_err());
            }
        }
        assert!(decode(&fixture(2, 1)).is_err());
        assert!(decode(&fixture(0xff, 1)).is_err());
    }

    #[test]
    fn version_one_field_validation_is_unchanged() {
        let mut body = fixture(1, 1);
        // Four-bit subframe crosses bytes 1 and 2. A subframe of 15 is invalid.
        body[1] |= 0b111 << 5;
        body[2] |= 1;
        assert!(decode(&body).is_err());
    }

    #[test]
    fn valid_mr1100_records_do_not_become_analysis_errors() {
        use crate::analysis::analyzer::{AnalyzerConfig, Harness};
        let mut harness = Harness::new_with_config(&AnalyzerConfig::default(), &Default::default());
        let row = harness.analyze_qmdl_message(decode(&fixture(0x7a, 1)));
        assert!(row.skipped_message_reason.is_none());
        assert!(!row.contains_warnings());
        let malformed = harness.analyze_qmdl_message(decode(&fixture(0x7a, 0)));
        assert!(malformed.skipped_message_reason.is_some());
    }

    #[test]
    fn mr1100_does_not_invent_a_gsmtap_timing_message() {
        let message = decode(&fixture(0x7a, 1)).unwrap();
        assert!(crate::gsmtap::parser::parse(message).unwrap().is_none());
    }
}
