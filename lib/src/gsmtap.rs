//! The spec for GSMTAP is here: <https://github.com/osmocom/libosmocore/blob/master/include/osmocom/core/gsmtap.h>

use deku::prelude::*;
use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GsmtapType {
    Um(UmSubtype),
    Abis,
    UmBurst,      /* raw burst bits */
    SIM,          /* ISO 7816 smart card interface */
    TetraI1,      /* tetra air interface */
    TetraI1Burst, /* tetra air interface */
    WmxBurst,     /* WiMAX burst */
    GbLlc,        /* GPRS Gb interface: LLC */
    GbSndcp,      /* GPRS Gb interface: SNDCP */
    Gmr1Um,       /* GMR-1 L2 packets */
    UmtsRlcMac,
    UmtsRrc(UmtsRrcSubtype),
    LteRrc(LteRrcSubtype), /* LTE interface */
    LteMac,                /* LTE MAC interface */
    LteMacFramed,          /* LTE MAC with context hdr */
    OsmocoreLog,           /* libosmocore logging */
    QcDiag,                /* Qualcomm DIAG frame */
    LteNas(LteNasSubtype), /* LTE Non-Access Stratum */
    E1T1,                  /* E1/T1 Lines */
    GsmRlp,                /* GSM RLP frames as per 3GPP TS 24.022 */
}

// based on https://github.com/fgsect/scat/blob/97442580e628de414c9f7c2a185f4e28d0ee7523/src/scat/parsers/qualcomm/diagltelogparser.py#L1337
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
pub enum LteNasSubtype {
    Plain = 0,
    Secure = 1,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
pub enum UmSubtype {
    Unknown = 0x00,
    Bcch = 0x01,
    Ccch = 0x02,
    Rach = 0x03,
    Agch = 0x04,
    Pch = 0x05,
    Sdcch = 0x06,
    Sdcch4 = 0x07,
    Sdcch8 = 0x08,
    TchF = 0x09,
    TchH = 0x0a,
    Pacch = 0x0b,
    Cbch52 = 0x0c,
    Pdch = 0x0d,
    Ptcch = 0x0e,
    Cbch51 = 0x0f,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
pub enum UmtsRrcSubtype {
    DlDcch = 0,
    UlDcch = 1,
    DlCcch = 2,
    UlCcch = 3,
    Pcch = 4,
    DlShcch = 5,
    UlShcch = 6,
    BcchFach = 7,
    BcchBch = 8,
    Mcch = 9,
    Msch = 10,
    HandoverToUTRANCommand = 11,
    InterRATHandoverInfo = 12,
    SystemInformationBCH = 13,
    SystemInformationContainer = 14,
    UERadioAccessCapabilityInfo = 15,
    MasterInformationBlock = 16,
    SysInfoType1 = 17,
    SysInfoType2 = 18,
    SysInfoType3 = 19,
    SysInfoType4 = 20,
    SysInfoType5 = 21,
    SysInfoType5bis = 22,
    SysInfoType6 = 23,
    SysInfoType7 = 24,
    SysInfoType8 = 25,
    SysInfoType9 = 26,
    SysInfoType10 = 27,
    SysInfoType11 = 28,
    SysInfoType11bis = 29,
    SysInfoType12 = 30,
    SysInfoType13 = 31,
    SysInfoType13_1 = 32,
    SysInfoType13_2 = 33,
    SysInfoType13_3 = 34,
    SysInfoType13_4 = 35,
    SysInfoType14 = 36,
    SysInfoType15 = 37,
    SysInfoType15bis = 38,
    SysInfoType15_1 = 39,
    SysInfoType15_1bis = 40,
    SysInfoType15_2 = 41,
    SysInfoType15_2bis = 42,
    SysInfoType15_2ter = 43,
    SysInfoType15_3 = 44,
    SysInfoType15_3bis = 45,
    SysInfoType15_4 = 46,
    SysInfoType15_5 = 47,
    SysInfoType15_6 = 48,
    SysInfoType15_7 = 49,
    SysInfoType15_8 = 50,
    SysInfoType16 = 51,
    SysInfoType17 = 52,
    SysInfoType18 = 53,
    SysInfoType19 = 54,
    SysInfoType20 = 55,
    SysInfoType21 = 56,
    SysInfoType22 = 57,
    SysInfoTypeSB1 = 58,
    SysInfoTypeSB2 = 59,
    ToTargetRNCContainer = 60,
    TargetRNCToSourceRNCContainer = 61,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
pub enum LteRrcSubtype {
    DlCcch = 0,
    DlDcch = 1,
    UlCcch = 2,
    UlDcch = 3,
    BcchBch = 4,
    BcchDlSch = 5,
    PCCH = 6,
    MCCH = 7,
    BcchBchMbms = 8,
    BcchDlSchBr = 9,
    BcchDlSchMbms = 10,
    ScMcch = 11,
    SbcchSlBch = 12,
    SbcchSlBchV2x = 13,
    DlCcchNb = 14,
    DlDcchNb = 15,
    UlCcchNb = 16,
    UlDcchNb = 17,
    BcchBchNb = 18,
    BcchBchTddNb = 19,
    BcchDlSchNb = 20,
    PcchNb = 21,
    ScMcchNb = 22,
}

#[derive(Debug)]
pub enum GsmtapTypeError {
    InvalidTypeSubtypeCombo(u8, u8),
}

impl GsmtapType {
    pub fn new(gsmtap_type: u8, gsmtap_subtype: u8) -> Result<Self, GsmtapTypeError> {
        let maybe_result = match gsmtap_type {
            0x01 => match UmSubtype::try_from(gsmtap_subtype) {
                Ok(subtype) => Some(GsmtapType::Um(subtype)),
                _ => None,
            },
            0x02 => Some(GsmtapType::Abis),
            0x03 => Some(GsmtapType::UmBurst),
            0x04 => Some(GsmtapType::SIM),
            0x05 => Some(GsmtapType::TetraI1),
            0x06 => Some(GsmtapType::TetraI1Burst),
            0x07 => Some(GsmtapType::WmxBurst),
            0x08 => Some(GsmtapType::GbLlc),
            0x09 => Some(GsmtapType::GbSndcp),
            0x0a => Some(GsmtapType::Gmr1Um),
            0x0b => Some(GsmtapType::UmtsRlcMac),
            0x0c => match UmtsRrcSubtype::try_from(gsmtap_subtype) {
                Ok(subtype) => Some(GsmtapType::UmtsRrc(subtype)),
                _ => None,
            },
            0x0d => match LteRrcSubtype::try_from(gsmtap_subtype) {
                Ok(subtype) => Some(GsmtapType::LteRrc(subtype)),
                _ => None,
            },
            0x0e => Some(GsmtapType::LteMac),
            0x0f => Some(GsmtapType::LteMacFramed),
            0x10 => Some(GsmtapType::OsmocoreLog),
            0x11 => Some(GsmtapType::QcDiag),
            0x12 => match LteNasSubtype::try_from(gsmtap_subtype) {
                Ok(subtype) => Some(GsmtapType::LteNas(subtype)),
                _ => None,
            },
            0x13 => Some(GsmtapType::E1T1),
            0x14 => Some(GsmtapType::GsmRlp),
            _ => None,
        };
        match maybe_result {
            Some(result) => Ok(result),
            None => Err(GsmtapTypeError::InvalidTypeSubtypeCombo(
                gsmtap_type,
                gsmtap_subtype,
            )),
        }
    }

    pub fn get_type(&self) -> u8 {
        match self {
            GsmtapType::Um(_) => 0x01,
            GsmtapType::Abis => 0x02,
            GsmtapType::UmBurst => 0x03,
            GsmtapType::SIM => 0x04,
            GsmtapType::TetraI1 => 0x05,
            GsmtapType::TetraI1Burst => 0x06,
            GsmtapType::WmxBurst => 0x07,
            GsmtapType::GbLlc => 0x08,
            GsmtapType::GbSndcp => 0x09,
            GsmtapType::Gmr1Um => 0x0a,
            GsmtapType::UmtsRlcMac => 0x0b,
            GsmtapType::UmtsRrc(_) => 0x0c,
            GsmtapType::LteRrc(_) => 0x0d,
            GsmtapType::LteMac => 0x0e,
            GsmtapType::LteMacFramed => 0x0f,
            GsmtapType::OsmocoreLog => 0x10,
            GsmtapType::QcDiag => 0x11,
            GsmtapType::LteNas(_) => 0x12,
            GsmtapType::E1T1 => 0x13,
            GsmtapType::GsmRlp => 0x14,
        }
    }

    pub fn get_subtype(&self) -> u8 {
        match self {
            GsmtapType::Um(subtype) => *subtype as u8,
            GsmtapType::UmtsRrc(subtype) => *subtype as u8,
            GsmtapType::LteRrc(subtype) => *subtype as u8,
            GsmtapType::LteNas(subtype) => *subtype as u8,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(endian = "big")]
pub struct GsmtapHeader {
    #[deku(skip)]
    pub gsmtap_type: GsmtapType,

    #[deku(assert_eq = "2")]
    pub version: u8,
    #[deku(assert_eq = "4")]
    pub header_len: u8, // length in 4-byte words
    #[deku(update = "self.gsmtap_type.get_type()")]
    pub packet_type: u8,
    pub timeslot: u8,
    #[deku(bits = 1)]
    pub pcs_band_indicator: bool,
    #[deku(bits = 1)]
    pub uplink: bool,
    #[deku(bits = 14)]
    pub arfcn: u16,
    pub signal_dbm: i8,
    pub signal_noise_ratio_db: u8,
    pub frame_number: u32,
    #[deku(update = "self.gsmtap_type.get_subtype()")]
    pub subtype: u8,
    pub antenna_number: u8,
    pub subslot: u8,
    #[deku(assert_eq = "0")]
    pub reserved: u8,
}

impl GsmtapHeader {
    pub fn new(gsmtap_type: GsmtapType) -> Self {
        GsmtapHeader {
            gsmtap_type,
            version: 2,
            header_len: 4,
            packet_type: gsmtap_type.get_type(),
            timeslot: 0,
            pcs_band_indicator: false,
            uplink: false,
            arfcn: 0,
            signal_dbm: 0,
            signal_noise_ratio_db: 0,
            frame_number: 0,
            subtype: gsmtap_type.get_subtype(),
            antenna_number: 0,
            subslot: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, DekuWrite)]
pub struct GsmtapMessage {
    pub header: GsmtapHeader,
    pub payload: Vec<u8>,
}
