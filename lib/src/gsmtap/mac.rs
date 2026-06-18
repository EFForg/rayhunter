use deku::prelude::*;

use crate::{
    diag::diaglog::mac::SubpacketBody,
    gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType},
};
use deku::{DekuContainerWrite, DekuError};

// based primarily off of SCAT's gsmtap responses and https://www.sharetechnote.com/html/MAC_LTE.html#MAC_PDU_Structure_RAR
#[derive(DekuRead, DekuWrite)]
pub struct Header {
    pub radio_type: RadioType,
    pub direction: Direction,
    pub rnti_type: RntiType,
}

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum RadioType {
    #[deku(id = "1")]
    Fdd,
    #[deku(id = "2")]
    Tdd,
}

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Direction {
    #[deku(id = "0")]
    Uplink,
    #[deku(id = "1")]
    Downlink,
}

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum RntiType {
    #[deku(id = "0")]
    NO,
    #[deku(id = "1")]
    P,
    #[deku(id = "2")]
    RA,
    #[deku(id = "3")]
    C,
    #[deku(id = "4")]
    SI,
    #[deku(id = "5")]
    SPS,
    #[deku(id = "6")]
    M,
    #[deku(id = "7")]
    SL,
    #[deku(id = "9")]
    SC,
    #[deku(id = "10")]
    G,
}

// defined in 6.5.1 of 3GPP TS 36.321
#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ETRAPIDSubheader {
    #[deku(bits = 1)]
    pub extended: bool,
    #[deku(bits = 1)]
    pub type_field: bool,
    #[deku(bits = 6)]
    pub rapid: u8,
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RACHResponse {
    #[deku(pad_bits_before = "1", bits = 11)]
    pub tac: u16,
    #[deku(bits = 20)]
    pub ul_grant: u32,
    pub tc_rnti: u16,
}

pub fn mac_subpacket_to_gsmtap(
    subpacket: &SubpacketBody,
) -> Result<Option<GsmtapMessage>, DekuError> {
    match subpacket {
        SubpacketBody::RachAttempt(attempt) => {
            let (Some(msg1), Some(msg2), Some(msg3)) =
                (attempt.get_msg1(), attempt.get_msg2(), attempt.get_msg3())
            else {
                return Ok(None);
            };
            let mut payload = Vec::new();
            payload.extend(
                Header {
                    radio_type: RadioType::Fdd,
                    direction: Direction::Downlink,
                    rnti_type: RntiType::RA,
                }
                .to_bytes()?,
            );
            payload.push(0x01); // MAC Payload Tag
            payload.extend(
                ETRAPIDSubheader {
                    extended: false,
                    type_field: true,
                    rapid: msg1.get_preamble_index(),
                }
                .to_bytes()?,
            );
            payload.extend(
                RACHResponse {
                    tac: msg2.ta,
                    ul_grant: msg3.get_grant(),
                    tc_rnti: msg2.tc_rnti,
                }
                .to_bytes()?,
            );
            Ok(Some(GsmtapMessage {
                header: GsmtapHeader::new(GsmtapType::LteMacFramed),
                payload,
            }))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::diag::diaglog::mac::Packet;
    use crate::diag::diaglog::mac::test::mac_rach_test_packets_from_scat;
    use crate::test_util::unhexlify;

    use super::*;

    fn assert_mac_gsmtap(packet: &Packet, expected_hexstr: Option<&str>) {
        assert_eq!(packet.subpackets.len(), 1);
        let subpacket = &packet.subpackets[0];
        let result = mac_subpacket_to_gsmtap(&subpacket.body).unwrap();
        match (result, expected_hexstr) {
            (Some(msg), Some(hexstr)) => {
                let (_, data) = unhexlify(hexstr);
                // SCAT's test cases use GSMTAP v3, but we're on V2, so skip
                // their GSMTAP header
                let expected_bytes = &data.into_inner().into_inner()[34..];
                assert_eq!(&msg.payload, expected_bytes);
            }
            (Some(msg), None) => panic!("expected no GSMTAP message, got {msg:?}"),
            (None, Some(_)) => panic!("expected GSMTAP message, got None"),
            _ => {}
        }
    }

    #[test]
    fn test_mac_rach() {
        // test data from SCAT unit tests: https://github.com/fgsect/scat/blob/9763cb5b1dcd5ee980f5b0ead9a8d520c8c51a51/tests/test_diagltelogparser.py#L129
        let test_packets = mac_rach_test_packets_from_scat();
        assert_mac_gsmtap(
            &test_packets[0],
            Some(
                "03000009040000000000000c0000000012d53d80000000000002000400000000fffe010102015b00411c181a23",
            ),
        );
        assert_mac_gsmtap(
            &test_packets[1],
            Some(
                "03000009040000000000000c0000000012d53d80000000000002000400000000fffe010102015800b0a2b461c6",
            ),
        );
        assert_mac_gsmtap(&test_packets[2], None);
        assert_mac_gsmtap(
            &test_packets[3],
            Some(
                "03000009040000000000000c0000000012d53d80000000000002000400000ea5fffe010102014a0070e218481c",
            ),
        );
        assert_mac_gsmtap(&test_packets[4], None);
        assert_mac_gsmtap(
            &test_packets[5],
            Some(
                "03000009040000000000000c0000000012d53d80000000000002000400000d16fffe0101020153005146b45aad",
            ),
        );
    }
}
