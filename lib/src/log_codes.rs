//! Enumerates some relevant diag log codes. Copied from QCSuper

// These are 2G-related log types.

pub const LOG_GSM_RR_SIGNALING_MESSAGE_C: u32 = 0x512f;

pub const DCCH: u32 = 0x00;
pub const BCCH: u32 = 0x01;
pub const L2_RACH: u32 = 0x02;
pub const CCCH: u32 = 0x03;
pub const SACCH: u32 = 0x04;
pub const SDCCH: u32 = 0x05;
pub const FACCH_F: u32 = 0x06;
pub const FACCH_H: u32 = 0x07;
pub const L2_RACH_WITH_NO_DELAY: u32 = 0x08;

// These are GPRS-related log types.

pub const LOG_GPRS_MAC_SIGNALLING_MESSAGE_C: u32 = 0x5226;

pub const PACCH_RRBP_CHANNEL: u32 = 0x03;
pub const UL_PACCH_CHANNEL: u32 = 0x04;
pub const DL_PACCH_CHANNEL: u32 = 0x83;

pub const PACKET_CHANNEL_REQUEST: u32 = 0x20;

// These are 5G-related log types.

pub const LOG_NR_RRC_OTA_MSG_LOG_C: u32 = 0xb821;

// These are 4G-related log types.

pub const LOG_LTE_RRC_OTA_MSG_LOG_C: u32 = 0xb0c0;
pub const LOG_LTE_NAS_ESM_OTA_IN_MSG_LOG_C: u32 = 0xb0e2;
pub const LOG_LTE_NAS_ESM_OTA_OUT_MSG_LOG_C: u32 = 0xb0e3;
pub const LOG_LTE_NAS_EMM_OTA_IN_MSG_LOG_C: u32 = 0xb0ec;
pub const LOG_LTE_NAS_EMM_OTA_OUT_MSG_LOG_C: u32 = 0xb0ed;

pub const LOG_LTE_MAC_DL: u32 = 0xb063;
pub const LOG_LTE_MAC_UL: u32 = 0xb064;

pub const LTE_BCCH_BCH_V0: u32 = 1;
pub const LTE_BCCH_DL_SCH_V0: u32 = 2;
pub const LTE_MCCH_V0: u32 = 3;
pub const LTE_PCCH_V0: u32 = 4;
pub const LTE_DL_CCCH_V0: u32 = 5;
pub const LTE_DL_DCCH_V0: u32 = 6;
pub const LTE_UL_CCCH_V0: u32 = 7;
pub const LTE_UL_DCCH_V0: u32 = 8;

pub const LTE_BCCH_BCH_V14: u32 = 1;
pub const LTE_BCCH_DL_SCH_V14: u32 = 2;
pub const LTE_MCCH_V14: u32 = 4;
pub const LTE_PCCH_V14: u32 = 5;
pub const LTE_DL_CCCH_V14: u32 = 6;
pub const LTE_DL_DCCH_V14: u32 = 7;
pub const LTE_UL_CCCH_V14: u32 = 8;
pub const LTE_UL_DCCH_V14: u32 = 9;

pub const LTE_BCCH_BCH_V9: u32 = 8;
pub const LTE_BCCH_DL_SCH_V9: u32 = 9;
pub const LTE_MCCH_V9: u32 = 10;
pub const LTE_PCCH_V9: u32 = 11;
pub const LTE_DL_CCCH_V9: u32 = 12;
pub const LTE_DL_DCCH_V9: u32 = 13;
pub const LTE_UL_CCCH_V9: u32 = 14;
pub const LTE_UL_DCCH_V9: u32 = 15;

pub const LTE_BCCH_BCH_V19: u32 = 1;
pub const LTE_BCCH_DL_SCH_V19: u32 = 3;
pub const LTE_MCCH_V19: u32 = 6;
pub const LTE_PCCH_V19: u32 = 7;
pub const LTE_DL_CCCH_V19: u32 = 8;
pub const LTE_DL_DCCH_V19: u32 = 9;
pub const LTE_UL_CCCH_V19: u32 = 10;
pub const LTE_UL_DCCH_V19: u32 = 11;

pub const LTE_BCCH_BCH_NB: u32 = 45;
pub const LTE_BCCH_DL_SCH_NB: u32 = 46;
pub const LTE_PCCH_NB: u32 = 47;
pub const LTE_DL_CCCH_NB: u32 = 48;
pub const LTE_DL_DCCH_NB: u32 = 49;
pub const LTE_UL_CCCH_NB: u32 = 50;
pub const LTE_UL_DCCH_NB: u32 = 52;

// These are 3G-related log types.

pub const RRCLOG_SIG_UL_CCCH: u32 = 0;
pub const RRCLOG_SIG_UL_DCCH: u32 = 1;
pub const RRCLOG_SIG_DL_CCCH: u32 = 2;
pub const RRCLOG_SIG_DL_DCCH: u32 = 3;
pub const RRCLOG_SIG_DL_BCCH_BCH: u32 = 4;
pub const RRCLOG_SIG_DL_BCCH_FACH: u32 = 5;
pub const RRCLOG_SIG_DL_PCCH: u32 = 6;
pub const RRCLOG_SIG_DL_MCCH: u32 = 7;
pub const RRCLOG_SIG_DL_MSCH: u32 = 8;
pub const RRCLOG_EXTENSION_SIB: u32 = 9;
pub const RRCLOG_SIB_CONTAINER: u32 = 10;

// 3G layer 3 packets:

pub const WCDMA_SIGNALLING_MESSAGE: u32 = 0x412f;

// Upper layers

pub const LOG_DATA_PROTOCOL_LOGGING_C: u32 = 0x11eb;

pub const LOG_UMTS_NAS_OTA_MESSAGE_LOG_PACKET_C: u32 = 0x713a;
