use crate::analysis::information_element::InformationElementError;
use crate::gsm::mobility_management::MobilityManagementMessage;
use crate::gsm::grps_mobility_management::GPRSMobilityManagementMessage;
use crate::gsm::radio_resource_management::RadioResourceManagementMessage;
use deku::prelude::*;

pub fn parse_l3(block: &[u8], pseudo_length: bool) -> Result<L3Frame, InformationElementError> {
    if pseudo_length {
        let (_rest, val) = PseudoLengthL3Frame::from_bytes((block.as_ref(), 0)).unwrap();
        return Ok(val.l3_frame);
    } else {
        /* return Err(InformationElementError::GsmDecodingError); */
        let (_rest, val) = L3Frame::from_bytes((block.as_ref(), 0)).unwrap();
        return Ok(val);
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct PseudoLengthL3Frame {
    #[deku(bits = 6, pad_bits_after = "2")]
    pub l2_pseudo_length: u8,
    pub l3_frame: L3Frame,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct L3Frame {
    #[deku(bits = 4)]
    pub skip_indicator: u8,
    pub protocol_discriminated_messages: ProtocolDiscrimiminatedMessage,
}

// 3GPP TS 24.007 V8.2.0 Section 11.2.3.1.1
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 4)]
pub enum ProtocolDiscrimiminatedMessage {
    #[deku(id = 0b0101)]
    MobilityManagement(MobilityManagementMessage),
    #[deku(id = 0b0110)]
    RadioResourceManagement(RadioResourceManagementMessage),
    #[deku(id = 0b1000)]
    GPRSMobilityManagement(GPRSMobilityManagementMessage),
    #[deku(id_pat = "_")]
    Unknown,
}
