//! Diag LTE RRC serialization/deserialization

use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "ext_header_version: u8", id = "ext_header_version")]
pub enum LteRrcOtaPacket {
    #[deku(id_pat = "0..=4")]
    V0 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u16,
        sfn_subfn: u16,
        pdu_num: u8,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "5..=7")]
    V5 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u16,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "8..=24")]
    V8 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u32,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
    #[deku(id_pat = "25..")]
    V25 {
        rrc_rel_maj: u8,
        rrc_rel_min: u8,
        nr_rrc_rel_maj: u8,
        nr_rrc_rel_min: u8,
        bearer_id: u8,
        phy_cell_id: u16,
        earfcn: u32,
        sfn_subfn: u16,
        pdu_num: u8,
        sib_mask: u32,
        len: u16,
        #[deku(count = "len")]
        packet: Vec<u8>,
    },
}

impl LteRrcOtaPacket {
    fn get_sfn_subfn(&self) -> u16 {
        match self {
            LteRrcOtaPacket::V0 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V5 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V8 { sfn_subfn, .. } => *sfn_subfn,
            LteRrcOtaPacket::V25 { sfn_subfn, .. } => *sfn_subfn,
        }
    }
    pub fn get_sfn(&self) -> u32 {
        self.get_sfn_subfn() as u32 >> 4
    }

    pub fn get_subfn(&self) -> u8 {
        (self.get_sfn_subfn() & 0xf) as u8
    }

    pub fn get_pdu_num(&self) -> u8 {
        match self {
            LteRrcOtaPacket::V0 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V5 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V8 { pdu_num, .. } => *pdu_num,
            LteRrcOtaPacket::V25 { pdu_num, .. } => *pdu_num,
        }
    }

    pub fn get_earfcn(&self) -> u32 {
        match self {
            LteRrcOtaPacket::V0 { earfcn, .. } => *earfcn as u32,
            LteRrcOtaPacket::V5 { earfcn, .. } => *earfcn as u32,
            LteRrcOtaPacket::V8 { earfcn, .. } => *earfcn,
            LteRrcOtaPacket::V25 { earfcn, .. } => *earfcn,
        }
    }

    pub fn take_payload(self) -> Vec<u8> {
        match self {
            LteRrcOtaPacket::V0 { packet, .. } => packet,
            LteRrcOtaPacket::V5 { packet, .. } => packet,
            LteRrcOtaPacket::V8 { packet, .. } => packet,
            LteRrcOtaPacket::V25 { packet, .. } => packet,
        }
    }
}
