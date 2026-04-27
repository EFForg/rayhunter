use deku::prelude::*;
use crate::gsm::information_elements::*;

// 3GPP TS 44.018 Table 10.4.1
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum RadioResourceManagementMessage {
    #[deku(id = 0b00110101)]
    CipheringModeCommand(CipheringModeCommand),
    #[deku(id = 0b00011011)]
    SystemInformationType3(SystemInformationType3),
    #[deku(id_pat = "_")]
    Unknown,
}

// 3GPP TS 44.018 Section 9.1.9
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CipheringModeCommand {
    #[deku(pad_bits_before = "3")]
    pub cipher_response: CipherResponse,
    pub cipher_mode_setting: CipherModeSetting,
}

// 3GPP TS 44.018 Section 9.1.35
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct SystemInformationType3 {
    pub cell_identity: u16,
    pub lai: LocationAreaIdentification,
    pub control_channel_desc: ControlChannelDescription,
    pub cell_options: CellOptionsBcch,
    pub cell_selection_params: CellSelectionParams,
    pub rach_control: RachControlParams,
    pub si3_rest_octets: SI3RestOctets,
}
