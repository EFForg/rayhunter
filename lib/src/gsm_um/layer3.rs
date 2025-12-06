use crate::analysis::information_element::InformationElementError;
use crate::gsm_um::messages::*;
use deku::prelude::*;

pub fn parse_l3(block: &[u8]) -> Result<L3Frame, InformationElementError> {
    let (_rest, val) = L3Frame::from_bytes((block.as_ref(), 0)).unwrap();
    return Ok(val);
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct L3Frame {
    #[deku(bits = 6, pad_bits_after = "2")]
    pub l2_pseudo_length: u8,
    #[deku(bits = 4)]
    pub skip_indicator: u8,
    #[deku(bits = 4)]
    pub protocol_discriminator: u8,
    pub message: L3Message,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum L3Message {
    #[deku(id = 0x1b)]
    SystemInformationType3(SystemInformationType3),
    #[deku(id_pat = "_")]
    Unknown,
}
