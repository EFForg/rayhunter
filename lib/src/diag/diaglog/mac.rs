//! Diag MAC RACH serialization/deserialization. As with most of our diag
//! parsers, these structs were derived SCAT:
//! https://github.com/fgsect/scat/blob/9763cb5b1dcd5ee980f5b0ead9a8d520c8c51a51/src/scat/parsers/qualcomm/diagltelogparser.py#L853

use deku::prelude::*;

#[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
pub struct Packet {
    #[deku(assert_eq = "1")]
    pub version: u8,
    pub num_subpackets: u8,
    #[deku(pad_bytes_before = "2", count = "*num_subpackets")]
    pub subpackets: Vec<Subpacket>,
}

#[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
pub struct Subpacket {
    pub id: u8,
    pub version: u8,
    pub size: u16,
    // size includes the header length, so subtract that
    #[deku(ctx = "*id, *version, *size - 4")]
    pub body: SubpacketBody,
}

#[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
#[deku(ctx = "id: u8, version: u8, size: u16", id = "id")]
pub enum SubpacketBody {
    #[deku(id = 0x06)]
    RachAttempt(#[deku(ctx = "version")] rach::Attempt),
    #[deku(id_pat = "_")]
    Other {
        #[deku(count = "size")]
        data: Vec<u8>,
    },
}

pub mod rach {
    //! Derived from https://github.com/fgsect/scat/blob/9763cb5b1dcd5ee980f5b0ead9a8d520c8c51a51/src/scat/parsers/qualcomm/diagltelogparser.py#L496
    use super::*;

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    #[deku(ctx = "version: u8")]
    pub struct Attempt {
        #[deku(ctx = "version")]
        pub header: AttemptHeader,
        #[deku(ctx = "version")]
        pub msg1: Msg1,
        pub msg2: Msg2,
        #[deku(ctx = "version")]
        pub msg3: Msg3,
        #[deku(cond = "version == 0x31 || version == 0x32")]
        pub additional_info: Option<AdditionalInfo>,
    }

    impl Attempt {
        pub fn get_msg1(&self) -> Option<&Msg1> {
            if self.header.has_msg1() {
                Some(&self.msg1)
            } else {
                None
            }
        }

        pub fn get_msg2(&self) -> Option<&Msg2> {
            if self.header.has_msg2() {
                Some(&self.msg2)
            } else {
                None
            }
        }

        pub fn get_msg3(&self) -> Option<&Msg3> {
            if self.header.has_msg3() {
                Some(&self.msg3)
            } else {
                None
            }
        }
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    pub struct AdditionalInfo {
        pub ul_earfcn: u32,
        pub p_max: u8,
        pub scell_id: u8,
        pub unk1: u32,
        pub unk2: u32,
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    #[deku(ctx = "version: u8", id = "version")]
    pub enum Msg1 {
        #[deku(id = "0x02")]
        V2 {
            preamble_index: u8,
            preamble_index_mask: u8,
            preamble_power_offset: i16,
        },
        #[deku(id_pat = "0x03 | 0x31")]
        V3Or31 {
            preamble_index: u8,
            preamble_index_mask: u8,
            preamble_power_offset: i16,
        },
        #[deku(id = "0x32")]
        V32 {
            preamble_index: u8,
            preamble_index_mask: u8,
            preamble_power_offset: i16,
            unk1: u16,
            group: i8,
        },
    }

    impl Msg1 {
        pub fn get_preamble_index(&self) -> u8 {
            match self {
                Msg1::V2 { preamble_index, .. } => *preamble_index,
                Msg1::V3Or31 { preamble_index, .. } => *preamble_index,
                Msg1::V32 { preamble_index, .. } => *preamble_index,
            }
        }
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    pub struct Msg2 {
        pub backoff: u16,
        pub result: u8,
        pub tc_rnti: u16,
        pub ta: u16,
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    #[deku(ctx = "version: u8")]
    pub struct Msg3 {
        #[deku(ctx = "version")]
        pub grant: Msg3Grant,
        pub unk_grant: u16,
        pub harq_id: u8,
        pub mac_pdu: [u8; 10],
    }

    impl Msg3 {
        pub fn get_grant(&self) -> u32 {
            match &self.grant {
                Msg3Grant::V1 { grant } => *grant & 0xfffff,
                Msg3Grant::V32 { grant } => *grant & 0xfffff,
            }
        }
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    #[deku(ctx = "version: u8", id = "version")]
    pub enum Msg3Grant {
        #[deku(id_pat = "0..0x32")]
        V1 {
            #[deku(endian = "little")]
            grant: u32,
        },
        #[deku(id_pat = "0x32..")]
        V32 {
            #[deku(endian = "big")]
            grant: u32,
        },
    }

    #[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
    #[deku(ctx = "version: u8", id = "version")]
    pub enum AttemptHeader {
        #[deku(id = 0x02)]
        V2 {
            num_attempt: u8,
            rach_result: u8,
            contention: u8,
            msg_bitmask: u8,
        },
        #[deku(id_pat = "0x03 | 0x31 | 0x32")]
        V3 {
            sub_id: u8,
            cell_id: u8,
            num_attempt: u8,
            rach_result: u8,
            contention: u8,
            msg_bitmask: u8,
        },
    }

    impl AttemptHeader {
        fn get_bitmask(&self) -> u8 {
            match self {
                AttemptHeader::V2 { msg_bitmask, .. } => *msg_bitmask,
                AttemptHeader::V3 { msg_bitmask, .. } => *msg_bitmask,
            }
        }

        pub fn has_msg1(&self) -> bool {
            self.get_bitmask() & 0x01 > 0
        }

        pub fn has_msg2(&self) -> bool {
            self.get_bitmask() & 0x02 > 0
        }

        pub fn has_msg3(&self) -> bool {
            self.get_bitmask() & 0x04 > 0
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    //! These tests were adapted from SCAT's MAC RACH parser's unit tests,
    //! and the values were produced by modifying the tests to output the
    //! entire parsed struct rather than the hexlified gsmtap packets. See
    //! the changes in this commit for more info:
    //! https://github.com/wgreenberg/scat/commit/adb21575832b4f3b30c8f2aaca9ee843ef74f38b

    use super::*;
    use crate::diag::diaglog::mac::rach::{AdditionalInfo, AttemptHeader, Msg1, Msg2, Msg3};
    use crate::{diag::diaglog::mac::rach::Msg3Grant, test_util::unhexlify};

    use std::io::Seek;

    pub fn mac_rach_test_packets_from_scat() -> Vec<Packet> {
        // test data from SCAT unit tests: https://github.com/fgsect/scat/blob/9763cb5b1dcd5ee980f5b0ead9a8d520c8c51a51/tests/test_diagltelogparser.py#L129
        vec![
            parse_rach_packet(
                "0101a06906022400010001071BFF98FF000001231A0400181C010007000600465C80BD0648000000",
            ),
            parse_rach_packet(
                "0101a0690603280001000100010718ffa4ff000001c6610b00b4a2000012000120061f423f8d95075800",
            ),
            parse_rach_packet(
                "0101739e063134000100010000033f0098ff0000013c6b070058ac010007000000468f47e2d446000000644b0000180001000000d5040000",
            ),
            parse_rach_packet(
                "01010000063134000100010001070aff98ff0000011c48070018e2000007000000523b7dfd69b6000000f5540000ff0001000000d6040000",
            ),
            parse_rach_packet(
                "01010000063238000100010000032900a4ffeb000000000195b603000000a0b412000420061f425dc9be41b800885e000017000100000065050000",
            ),
            parse_rach_packet(
                "010100000632380001000100010713ffa0ffeb0000000001ad5a0500000146b412000420061f425dc9be41b400665300001800010000001a050000",
            ),
        ]
    }

    fn parse_rach_packet(bytes_str: &str) -> Packet {
        let (total_size, mut reader) = unhexlify(bytes_str);
        let packet = Packet::from_reader_with_ctx(&mut reader, ()).unwrap();
        let leftover_bits = reader.rest().len();
        let leftover_bytes = total_size - reader.stream_position().unwrap() as usize;
        assert_eq!(leftover_bytes, 0);
        assert_eq!(leftover_bits, 0);
        packet
    }

    fn assert_rach_subpacket(
        packet: &Packet,
        header: AttemptHeader,
        msg1: Option<Msg1>,
        msg2: Option<Msg2>,
        msg3: Option<Msg3>,
        additional_info: Option<AdditionalInfo>,
    ) {
        assert_eq!(packet.version, 0x01);
        assert_eq!(packet.num_subpackets, 1);
        assert_eq!(packet.subpackets.len(), 1);
        if let SubpacketBody::RachAttempt(attempt) = &packet.subpackets[0].body {
            assert_eq!(attempt.header, header);
            assert_eq!(attempt.get_msg1(), msg1.as_ref());
            assert_eq!(attempt.get_msg2(), msg2.as_ref());
            assert_eq!(attempt.get_msg3(), msg3.as_ref());
            assert_eq!(attempt.additional_info, additional_info);
        } else {
            panic!("not rach attempt {:?}", packet.subpackets[0].body);
        }
    }

    #[test]
    fn test_rach_attempt_parsing() {
        let test_packets = mac_rach_test_packets_from_scat();
        assert_rach_subpacket(
            &test_packets[0],
            rach::AttemptHeader::V2 {
                num_attempt: 1,
                rach_result: 0,
                contention: 1,
                msg_bitmask: 7,
            },
            Some(Msg1::V2 {
                preamble_index: 27,
                preamble_index_mask: 255,
                preamble_power_offset: -104,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 6691,
                ta: 4,
            }),
            Some(Msg3 {
                grant: Msg3Grant::V1 { grant: 72728 },
                unk_grant: 7,
                harq_id: 6,
                mac_pdu: [0x00, 0x46, 0x5c, 0x80, 0xbd, 0x06, 0x48, 0x00, 0x00, 0x00],
            }),
            None,
        );

        assert_rach_subpacket(
            &test_packets[1],
            rach::AttemptHeader::V3 {
                sub_id: 1,
                cell_id: 0,
                num_attempt: 1,
                rach_result: 0,
                contention: 1,
                msg_bitmask: 7,
            },
            Some(Msg1::V3Or31 {
                preamble_index: 24,
                preamble_index_mask: 255,
                preamble_power_offset: -92,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 25030,
                ta: 11,
            }),
            Some(Msg3 {
                grant: Msg3Grant::V1 { grant: 41652 },
                unk_grant: 18,
                harq_id: 1,
                mac_pdu: [0x20, 0x06, 0x1f, 0x42, 0x3f, 0x8d, 0x95, 0x07, 0x58, 0x00],
            }),
            None,
        );

        assert_rach_subpacket(
            &test_packets[2],
            rach::AttemptHeader::V3 {
                sub_id: 1,
                cell_id: 0,
                num_attempt: 1,
                rach_result: 0,
                contention: 0,
                msg_bitmask: 3,
            },
            Some(Msg1::V3Or31 {
                preamble_index: 63,
                preamble_index_mask: 0,
                preamble_power_offset: -104,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 27452,
                ta: 7,
            }),
            None,
            Some(AdditionalInfo {
                ul_earfcn: 19300,
                p_max: 24,
                scell_id: 0,
                unk1: 1,
                unk2: 1237,
            }),
        );

        assert_rach_subpacket(
            &test_packets[3],
            AttemptHeader::V3 {
                sub_id: 1,
                cell_id: 0,
                num_attempt: 1,
                rach_result: 0,
                contention: 1,
                msg_bitmask: 7,
            },
            Some(Msg1::V3Or31 {
                preamble_index: 10,
                preamble_index_mask: 255,
                preamble_power_offset: -104,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 18460,
                ta: 7,
            }),
            Some(Msg3 {
                grant: Msg3Grant::V1 { grant: 57880 },
                unk_grant: 7,
                harq_id: 0,
                mac_pdu: [0x00, 0x52, 0x3b, 0x7d, 0xfd, 0x69, 0xb6, 0x00, 0x00, 0x00],
            }),
            Some(AdditionalInfo {
                ul_earfcn: 21749,
                p_max: 255,
                scell_id: 0,
                unk1: 1,
                unk2: 1238,
            }),
        );

        assert_rach_subpacket(
            &test_packets[4],
            AttemptHeader::V3 {
                sub_id: 1,
                cell_id: 0,
                num_attempt: 1,
                rach_result: 0,
                contention: 0,
                msg_bitmask: 3,
            },
            Some(Msg1::V32 {
                preamble_index: 41,
                preamble_index_mask: 0,
                preamble_power_offset: -92,
                unk1: 235,
                group: 0,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 46741,
                ta: 3,
            }),
            None,
            Some(AdditionalInfo {
                ul_earfcn: 24200,
                p_max: 23,
                scell_id: 0,
                unk1: 1,
                unk2: 1381,
            }),
        );

        assert_rach_subpacket(
            &test_packets[5],
            AttemptHeader::V3 {
                sub_id: 1,
                cell_id: 0,
                num_attempt: 1,
                rach_result: 0,
                contention: 1,
                msg_bitmask: 7,
            },
            Some(Msg1::V32 {
                preamble_index: 19,
                preamble_index_mask: 255,
                preamble_power_offset: -96,
                unk1: 235,
                group: 0,
            }),
            Some(Msg2 {
                backoff: 0,
                result: 1,
                tc_rnti: 23213,
                ta: 5,
            }),
            Some(Msg3 {
                grant: Msg3Grant::V32 { grant: 83636 },
                unk_grant: 18,
                harq_id: 4,
                mac_pdu: [0x20, 0x06, 0x1f, 0x42, 0x5d, 0xc9, 0xbe, 0x41, 0xb4, 0x00],
            }),
            Some(AdditionalInfo {
                ul_earfcn: 21350,
                p_max: 24,
                scell_id: 0,
                unk1: 1,
                unk2: 1306,
            }),
        );
    }
}
