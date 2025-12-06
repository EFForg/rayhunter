use deku::prelude::*;
use crate::gsm_um::information_elements::*;
// see GSM 04.08 version 5.0.0

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
