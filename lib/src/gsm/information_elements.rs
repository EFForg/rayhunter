use deku::prelude::*;
// 3GPP TS 44.018 V8.17.0
// 3GPP TS 24.008 V8.20.0

// 10.5.1.3
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct LocationAreaIdentification {
    #[deku(bits = 4)]
    pub mcc_digit2: u8,
    #[deku(bits = 4)]
    pub mcc_digit1: u8,
    #[deku(bits = 4, pad_bits_before = "4")]
    pub mcc_digit3: u8,
    #[deku(bits = 4)]
    pub mnc_digit2: u8,
    #[deku(bits = 4)]
    pub mnc_digit1: u8,
    pub lac: u16,
}

// 10.5.2.3
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CellOptionsBcch {
    #[deku(bits = 1, pad_bits_before = "1")] pub pwrc: u8,
    #[deku(bits = 2)] pub dtx: u8,
    #[deku(bits = 4)] pub radio_link_timeout: u8,
}

// 10.5.2.4
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CellSelectionParams {
    #[deku(bits = 3)] pub cell_resel_hysteresis: u8,
    #[deku(bits = 5)] pub ms_txpwr_max_cch: u8,
    #[deku(bits = 1)] pub acs: u8,
    #[deku(bits = 1)] pub neci: u8,
    #[deku(bits = 6)] pub rxlev_access_min: u8,
}

// 10.5.2.11
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ControlChannelDescription {
    #[deku(bits = 1, pad_bits_before = "1")]
    pub att: u8,
    #[deku(bits = 3)]
    pub bs_ag_blks_res: u8,
    #[deku(bits = 3)]
    pub ccch_conf: u8,
    #[deku(bits = 3, pad_bits_before = "5")]
    pub bs_pa_mfrms: u8,
}

// 10.5.2.29
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct RachControlParams {
    #[deku(bits = 2)] pub max_retrans: u8,
    #[deku(bits = 4)] pub tx_integer: u8,
    #[deku(bits = 1)] pub cell_bar_access: u8,
    #[deku(bits = 1)] pub reestablishment: u8,
    #[deku(bits = 1)] pub ac_c15: u8,
    #[deku(bits = 1)] pub ac_c14: u8,
    #[deku(bits = 1)] pub ac_c13: u8,
    #[deku(bits = 1)] pub ac_c12: u8,
    #[deku(bits = 1)] pub ac_c11: u8,
    #[deku(bits = 1)] pub ec_c10: u8,
    #[deku(bits = 1)] pub ac_c09: u8,
    #[deku(bits = 1)] pub ac_c08: u8,
    #[deku(bits = 1)] pub ac_c07: u8,
    #[deku(bits = 1)] pub ac_c06: u8,
    #[deku(bits = 1)] pub ac_c05: u8,
    #[deku(bits = 1)] pub ac_c04: u8,
    #[deku(bits = 1)] pub ac_c03: u8,
    #[deku(bits = 1)] pub ac_c02: u8,
    #[deku(bits = 1)] pub ac_c01: u8,
    #[deku(bits = 1)] pub ac_c00: u8,
}

// Optional data in rest octets
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum OptionalSelectionParameters {
    #[deku(id = 0b1)]
    Present(SelectionParameters),
    #[deku(id = 0b0)]
    NotPresent
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct SelectionParameters {
    #[deku(bits = 1)] pub cbq: u8,
    #[deku(bits = 6)] pub cell_reselect_offset: u8,
    #[deku(bits = 3)] pub temporary_offset: u8,
    #[deku(bits = 5)] pub penalty_time: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum OptionalPowerOffset {
    #[deku(id = 0b1)]
    Present(PowerOffset),
    #[deku(id = 0b0)]
    NotPresent
}
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct PowerOffset {
    #[deku(bits = 2)] pub power_offset: u8,
}

// 10.5.2.9
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 4)]
pub enum CipherModeSetting {
    #[deku(id = 0b0001)]
    A5_1,
    #[deku(id = 0b0011)]
    A5_2,
    #[deku(id = 0b0101)]
    A5_3,
    #[deku(id = 0b0111)]
    A5_4,
    #[deku(id = 0b1001)]
    A5_5,
    #[deku(id = 0b1011)]
    A5_6,
    #[deku(id = 0b1101)]
    A5_7,
    #[deku(id = 0b1111)]
    Reserved,
    #[deku(id_pat = "_")]
    NoCiphering
}

// 10.5.2.10
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum CipherResponse {
    #[deku(id = 0b0)]
    IncludeIMEISV,
    #[deku(id = 0b1)]
    DoNotIncludeIMEISV,
}

// 10.5.2.34
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct SI3RestOctets {
    pub optional_selection_parameters: OptionalSelectionParameters,
    pub option_power_offset: OptionalPowerOffset,
    #[deku(bits = 1)]
    pub system_information_2ter_indicator: u8,
    #[deku(bits = 1)]
    pub early_classmark_sending_control: u8,
}

// 10.5.3.4
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum IdentityType {
    #[deku(id = 0b001)]
    IMSI,
    #[deku(id = 0b010)]
    IMEI,
    #[deku(id = 0b011)]
    IMEISV,
    #[deku(id = 0b100)]
    TMSI
}

// 10.5.5.3
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum CipheringAlgorithm {
    #[deku(id = 0b000)]
    CipheringNotUsed,
    #[deku(id = 0b001)]
    GEA1,
    #[deku(id = 0b010)]
    GEA2,
    #[deku(id = 0b011)]
    GEA3,
    #[deku(id = 0b100)]
    GEA4,
    #[deku(id = 0b101)]
    GEA5,
    #[deku(id = 0b110)]
    GEA6,
    #[deku(id = 0b111)]
    GEA7,
}

// 10.5.5.7
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum ForceToStandby {
    #[deku(id = 0b000)]
    NotIndicated,
    #[deku(id = 0b001)]
    Indicated
}

// 10.5.5.9
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum IdentityType2 {
    #[deku(id = 0b001)]
    IMSI,
    #[deku(id = 0b010)]
    IMEI,
    #[deku(id = 0b011)]
    IMEISV,
    #[deku(id = 0b100)]
    TMSI
}

// 10.5.5.10
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum IMEISVRequest {
    #[deku(id = 0b000)]
    IMEINotRequested,
    #[deku(id = 0b001)]
    IMEIRequested
}
