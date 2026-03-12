use deku::prelude::*;
use crate::gsm::information_elements::*;

// 3GPP TS 24.008 Table 10.4
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum GPRSMobilityManagementMessage {
    #[deku(id = 0b00010101)]
    IdentityRequest(GMMIdentityRequest),
    #[deku(id = 0b00010010)]
    AuthenticationAndCipheringRequest(GMMAuthenticationAndCipheringRequest),
    #[deku(id_pat = "_")]
    Unknown,
}

// 3GPP TS 24.008 Section 9.4.12
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct GMMIdentityRequest {
    #[deku(pad_bits_before = "1")]
    pub force_to_standby: ForceToStandby,
    #[deku(pad_bits_before = "1")]
    pub identity_type: IdentityType2,
}

// 3GPP TS 24.008 Section 9.4.9
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct GMMAuthenticationAndCipheringRequest {
    #[deku(pad_bits_before = "1")]
    pub imeisv_request: IMEISVRequest,
    #[deku(pad_bits_before = "1")]
    pub ciphering_algorithm: CipheringAlgorithm,
}
