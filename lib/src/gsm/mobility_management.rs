use deku::prelude::*;
use crate::gsm::information_elements::*;

// 3GPP TS 24.008 Table 10.2
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum MobilityManagementMessage {
    #[deku(id = 0b00011000)]
    IdentityRequest(MMIdentityRequest),
    #[deku(id_pat = "_")]
    Unknown,
}

// 3GPP TS 24.008 Section 9.2.10
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MMIdentityRequest {
    #[deku(pad_bits_before = "1", pad_bits_after = "4")]
    pub identity_type: IdentityType,
}
