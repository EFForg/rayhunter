use deku::prelude::*;

// Qualcomm ML1 (physical layer) serving cell measurement log (0xb17f).
// Format from SCAT: https://github.com/fgsect/scat/blob/master/src/scat/parsers/qualcomm/diagltelogparser.py
// V5 format string (after version byte): '<BHLH2xLLLLLL'
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "version: u8", id = "version")]
pub enum LteMl1ServingCellMeasPacket {
    #[deku(id = "4")]
    V4 {
        rrc_release: u16,
        reserved: u16,
        earfcn: u16,
        pci_serv_layer: u16,
        meas_rsrp: u32,
        avg_rsrp: u32,
        rsrq: u32,
        rssi: u32,
        rxlev: u32,
        search_threshold: u32,
    },
    // V5 expanded earfcn to u32; rrc_release shrunk to u8 with a reserved u16 before earfcn;
    // 2-byte padding follows pci_serv_layer (SCAT: 2x)
    #[deku(id_pat = "5..=255")]
    V5 {
        rrc_release: u8,
        reserved: u16,
        earfcn: u32,
        #[deku(pad_bytes_after = "2")]
        pci_serv_layer: u16,
        meas_rsrp: u32,
        avg_rsrp: u32,
        rsrq: u32,
        rssi: u32,
        rxlev: u32,
        search_threshold: u32,
    },
}

impl LteMl1ServingCellMeasPacket {
    pub fn get_earfcn(&self) -> u32 {
        match self {
            Self::V4 { earfcn, .. } => *earfcn as u32,
            Self::V5 { earfcn, .. } => *earfcn,
        }
    }

    // Lower 9 bits are the Physical Cell ID (0–503); upper bits encode serving layer.
    pub fn get_pci(&self) -> u16 {
        let raw = match self {
            Self::V4 { pci_serv_layer, .. } => *pci_serv_layer,
            Self::V5 { pci_serv_layer, .. } => *pci_serv_layer,
        };
        raw & 0x1FF
    }

    // RSRP lower 12 bits, 1/16 dB steps, -180 dBm base.
    // Returns whole dBm clamped to i8 for the GSMTAP signal_dbm header field.
    pub fn get_rsrp_dbm(&self) -> i8 {
        let raw = match self {
            Self::V4 { meas_rsrp, .. } => *meas_rsrp,
            Self::V5 { meas_rsrp, .. } => *meas_rsrp,
        };
        let sixteenth_db = -2880_i32 + (raw & 0x0FFF) as i32;
        (sixteenth_db / 16).clamp(i8::MIN as i32, i8::MAX as i32) as i8
    }
}

#[cfg(test)]
mod test {
    use crate::diag::{Message, diaglog::LogBody};
    use super::*;

    #[test]
    fn test_lte_ml1_v5_rsrp() {
        // Probe capture: full diag Message wrapping a 0xb17f log (Version 5, Band 3 / EARFCN 1849).
        // Constructed as: opcode(1) + pending(1) + outer_len(2) + inner_len(2) +
        //                 log_type(2=0xb17f LE) + timestamp(8) + body(40) = 56 bytes total
        let mut msg_bytes: Vec<u8> = vec![
            0x10, 0x00,             // opcode=Log, pending=0
            56, 0, 56, 0,           // outer_length=56, inner_length=56
            0x7f, 0xb1,             // log_type = 0xb17f (LE)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp
        ];
        msg_bytes.extend_from_slice(&[
            0x05, // version=5
            0x01, 0x00, 0x00, 0x39, 0x07, 0x00, 0x00, 0x89, 0x00, 0x00, 0x00,
            0xab, 0xb5, 0x5a, 0x00, 0xab, 0xb5, 0x5a, 0x00,
            0x1a, 0x69, 0xa4, 0x11, 0x1a, 0x45, 0x0d, 0x00, 0x86, 0xa7, 0xae, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x80, 0x1c, 0x00, 0x00,
        ]);
        let msg = Message::from_bytes((&msg_bytes, 0)).expect("Message parse failed").1;
        if let Message::Log { body: LogBody::LteMl1ServingCellMeas { packet, .. }, .. } = msg {
            assert_eq!(packet.get_earfcn(), 1849);
            let rsrp = packet.get_rsrp_dbm();
            assert!(rsrp <= -44 && rsrp >= -120, "RSRP {rsrp} dBm outside valid LTE range");
        } else {
            panic!("unexpected message variant");
        }
    }
}
