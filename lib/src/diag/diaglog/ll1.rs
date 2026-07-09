use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb", ctx = "_: deku::ctx::Order")]
pub struct ServingCellTiming {
    #[deku(assert_eq = "1")]
    pub version: u8,
    #[deku(bits = 5, assert = "*num_records <= 20")]
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
    #[deku(count = "*num_records")]
    pub timing_adjustment: Vec<TimingAdjustment>,
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
    #[deku(bits = 8, assert = "(-128..=127).contains(timing_advance)")]
    pub timing_advance: i8, // in 16 Ts units
}
