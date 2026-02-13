//! Diag ML1 measurement log serialization/deserialization. These are pretty
//! much entirely based on Shinjo Park's work in scat, since we couldn't find
//! any other documentation for the logs' structure.

use deku::prelude::*;
use deku::ctx::Order;

fn decode_rsrp(rsrp: u16) -> f32 {
    rsrp as f32 / 16.0 - 180.0
}

fn decode_rssi(rssi: u16) -> f32 {
    rssi as f32 / 16.0 - 110.0
}

fn decode_rsrq(rsrq: u16) -> f32 {
    rsrq as f32 / 16.0 - 30.0
}

pub mod serving_cell {
    use super::*;

    #[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
    #[deku(bit_order = "lsb")]
    pub struct MeasurementAndEvaluation {
        pub header: MeasurementAndEvaluationHeader,
        #[deku(bits = 12, pad_bits_after = "20")]
        meas_rsrp: u16,
        avg_rsrp: u32,
        #[deku(bits = 10, pad_bits_after = "22")]
        meas_rsrq: u16,
        #[deku(pad_bits_before = "10", bits = 11, pad_bits_after = "11")]
        meas_rssi: u16,
        rxlev: u32,
        s_search: u32,
        #[deku(cond = "header.get_rrc_rel() == 0x01")]
        r9_data: Option<u32>,
    }

    #[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
    #[deku(ctx = "_: Order", id_type = "u8", bit_order = "lsb")]
    pub enum MeasurementAndEvaluationHeader {
        #[deku(id = "4")]
        V4 {
            rrc_rel: u8,
            _reserved: u16,
            earfcn: u16,
            #[deku(bits = 9)]
            pci: u16,
            #[deku(bits = 7)]
            serv_layer_priority: u8,
        },
        #[deku(id = "5")]
        V5 {
            rrc_rel: u8,
            _reserved: u16,
            earfcn: u32,
            #[deku(bits = 9)]
            pci: u16,
            #[deku(bits = 7, pad_bytes_after = "2")]
            serv_layer_priority: u8,
        },
    }

    impl MeasurementAndEvaluationHeader {
        fn get_rrc_rel(&self) -> u8 {
            match self {
                MeasurementAndEvaluationHeader::V4 { rrc_rel, .. } => *rrc_rel,
                MeasurementAndEvaluationHeader::V5 { rrc_rel, .. } => *rrc_rel,
            }
        }
    }

    impl MeasurementAndEvaluation {
        pub fn get_pci(&self) -> u16 {
            match &self.header {
                MeasurementAndEvaluationHeader::V4 { pci, .. } => *pci,
                MeasurementAndEvaluationHeader::V5 { pci, .. } => *pci,
            }
        }

        pub fn get_earfcn(&self) -> u32 {
            match &self.header {
                MeasurementAndEvaluationHeader::V4 { earfcn, .. } => *earfcn as u32,
                MeasurementAndEvaluationHeader::V5 { earfcn, .. } => *earfcn,
            }
        }

        pub fn get_meas_rsrp(&self) -> f32 {
            decode_rsrp(self.meas_rsrp)
        }

        pub fn get_meas_rssi(&self) -> f32 {
            decode_rssi(self.meas_rssi)
        }

        pub fn get_meas_rsrq(&self) -> f32 {
            decode_rsrq(self.meas_rsrq)
        }
    }
}

pub mod neighbor_cells {
    use super::*;

    #[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq)]
    #[deku(id_type = "u8", bit_order = "lsb")]
    pub enum MeasurementsHeader {
        #[deku(id = "4")]
        V4 {
            rrc_rel: u8,
            _reserved1: u16,
            earfcn: u16,
            #[deku(bits = 6)]
            q_rxlevmin: u8,
            #[deku(bits = 10)]
            n_cells: u16,
        },
        #[deku(id = "5")]
        V5 {
            rrc_rel: u8,
            _reserved1: u16,
            earfcn: u32,
            #[deku(bits = 6)]
            q_rxlevmin: u8,
            #[deku(bits = 26)]
            n_cells: u32,
        },
    }

    impl MeasurementsHeader {
        fn get_n_cells(&self) -> usize {
            match self {
                MeasurementsHeader::V4 { n_cells, .. } => *n_cells as usize,
                MeasurementsHeader::V5 { n_cells, .. } => *n_cells as usize,
            }
        }
    }

    #[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq)]
    pub struct Measurements {
        pub header: MeasurementsHeader,
        #[deku(count = "header.get_n_cells()")]
        pub cells: Vec<MeasurementsCell>
    }

    impl Measurements {
        pub fn get_earfcn(&self) -> u32 {
            match &self.header {
                MeasurementsHeader::V4 { earfcn, .. } => *earfcn as u32,
                MeasurementsHeader::V5 { earfcn, .. } => *earfcn,
            }
        }
    }


    #[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq)]
    #[deku(bit_order = "lsb")]
    pub struct MeasurementsCell {
        #[deku(bits = 9)]
        pub pci: u16,
        #[deku(bits = 11)]
        meas_rssi: u16,
        #[deku(bits = 12)]
        meas_rsrp: u16,
        #[deku(pad_bits_before = "12", bits = 12, pad_bits_after = "8")]
        avg_rsrp: u16,
        #[deku(pad_bits_before = "12", bits = 10, pad_bits_after = "10")]
        meas_rsrq: u16,
        #[deku(bits = 10, pad_bits_after = "10")]
        avg_rsrq: u16,
        #[deku(bits = 6, pad_bits_after = "6")]
        s_rxlev: u16,
        n_freq_offset: u16,
        val5: u16,
        ant0_offset: u32,
        ant1_offset: u32,
        unk1: u32,
    }

    impl MeasurementsCell {
        pub fn get_meas_rsrp(&self) -> f32 {
            decode_rsrp(self.meas_rsrp)
        }

        pub fn get_meas_rssi(&self) -> f32 {
            decode_rssi(self.meas_rssi)
        }

        pub fn get_meas_rsrq(&self) -> f32 {
            decode_rsrq(self.meas_rsrq)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::diag::diaglog::LogBody;
    use crate::log_codes::{LOG_LTE_ML1_NEIGHBOR_MEAS, LOG_LTE_ML1_SERVING_CELL_MEAS_RESP_AND_EVAL};
    use std::io::{Cursor, Seek};

    fn unhexlify(hexlified_bytes: &str) -> (usize, Reader<Cursor<Vec<u8>>>) {
        let byte_len = hexlified_bytes.len() / 2;
        let bytes = (0..hexlified_bytes.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hexlified_bytes[i..i+2], 16).unwrap())
            .collect();
        (byte_len, Reader::new(Cursor::new(bytes)))
    }

    fn parse_ncell_measurements(hexlified_bytes: &str) -> (u8, neighbor_cells::Measurements) {
        let (total_size, mut reader) = unhexlify(hexlified_bytes);
        match LogBody::from_reader_with_ctx(&mut reader, (LOG_LTE_ML1_NEIGHBOR_MEAS as u16, 0)) {
            Ok(LogBody::LteMl1NeighborCellsMeasurements { data }) => {
                if !reader.end() {
                    let leftover_bits = reader.rest();
                    let leftover_bytes = total_size - reader.stream_position().unwrap() as usize;
                    panic!("failed to read entire buffer ({} bytes, {} bits left)", leftover_bytes, leftover_bits.len());
                }
                let pkt_version = match data.header {
                    neighbor_cells::MeasurementsHeader::V4 { .. } => 4,
                    neighbor_cells::MeasurementsHeader::V5 { .. } => 5,
                };
                (pkt_version, data)
            },
            Ok(x) => panic!("expected MeasurementAndEvaluation, but parsed {:?}", x),
            Err(x) => panic!("failed to parse MeasurementAndEvaluation {:?}", x),
        }
    }

    fn parse_meas_eval(hexlified_bytes: &str) -> (u8, serving_cell::MeasurementAndEvaluation) {
        let (total_size, mut reader) = unhexlify(hexlified_bytes);
        match LogBody::from_reader_with_ctx(&mut reader, (LOG_LTE_ML1_SERVING_CELL_MEAS_RESP_AND_EVAL as u16, 0)) {
            Ok(LogBody::LteMl1ServingCellMeasurementAndEvaluation { data }) => {
                if !reader.end() {
                    let leftover_bits = reader.rest();
                    let leftover_bytes = total_size - reader.stream_position().unwrap() as usize;
                    panic!("failed to read entire buffer ({} bytes, {} bits left)", leftover_bytes, leftover_bits.len());
                }
                let pkt_version = match data.header {
                    serving_cell::MeasurementAndEvaluationHeader::V4 { .. } => 4,
                    serving_cell::MeasurementAndEvaluationHeader::V5 { .. } => 5,
                };
                (pkt_version, data)
            },
            Ok(x) => panic!("expected MeasurementAndEvaluation, but parsed {:?}", x),
            Err(x) => panic!("failed to parse MeasurementAndEvaluation {:?}", x),
        }
    }

    fn scell_meas_and_eval_case(
        hexlified_bytes: &str,
        pkt_version: u8,
        pci: u16,
        earfcn: u32,
        rsrp: f32,
        rsrq: f32,
        rssi: f32
    ) {
        let (parsed_pkt_version, data) = parse_meas_eval(hexlified_bytes);
        assert_eq!(parsed_pkt_version, pkt_version);
        assert_eq!(data.get_pci(), pci, "incorrect pci");
        assert_eq!(data.get_earfcn(), earfcn, "incorrect earfcn");
        assert_eq!(data.get_meas_rsrp(), rsrp, "incorrect rsrp");
        assert_eq!(data.get_meas_rsrq(), rsrq, "incorrect rsrq");
        assert_eq!(data.get_meas_rssi(), rssi, "incorrect rssi");
    }

    // Adapted from scat's TestDiagLteLogParser::test_parse_lte_ml1_scell_meas,
    // but edited to print full-precision floats
    #[test]
    fn test_scell_meas() {
        scell_meas_and_eval_case(
            "040100009C18D60AECC44E00E2244E00FFFCE30FFED80A0047AD56021D310100A2624100",
            4,
            214,
            6300,
            -101.25,
            -14.0625,
            -66.625
        );
        scell_meas_and_eval_case(
            "05010000160d0000d40e00004bb444005444450039e514133149070048adfe019f310100a23f0000",
            5,
            212,
            3350,
            -111.3125,
            -10.4375,
            -80.875,
        );
        scell_meas_and_eval_case(
            "05010000f424000a4d43434d4e434d41524b45527c307c3236327c317c34323330333233347c7c4d43434d4e434d41524b45520a0a434f504d41524b45527c434f504552524f5232363230317c434f504d41524b45520a006306000057755500577555001d75d4111d290b0048ad7e02dd370100a27f4100",
            5,
            333,
            167781620,
            -127.125,
            -22.25,
            2.75,
        );
        scell_meas_and_eval_case(
            "0501000000190000a90d0000d9944d00d9944d006081d5d55d2568bc48ad3e027f314fe0891900e0",
            5,
            425,
            6400,
            -102.4375,
            -8.0,
            -77.4375,
        );
    }

    fn ncell_meas_case(
        hexlified_bytes: &str,
        pkt_version: u8,
        earfcn: u32,
        cells: Vec<(u16, f32, f32, f32)>,
    ) {
        let (parsed_pkt_version, data) = parse_ncell_measurements(hexlified_bytes);
        assert_eq!(parsed_pkt_version, pkt_version, "incorrect pkt_version");
        assert_eq!(data.cells.len(), cells.len(), "incorrect number of cells");
        assert_eq!(data.get_earfcn(), earfcn, "incorrect earfcn");
        for (parsed, (pci, rsrp, rssi, rsrq)) in data.cells.iter().zip(cells) {
            assert_eq!(parsed.pci, pci, "incorrect pci");
            assert_eq!(parsed.get_meas_rsrp(), rsrp, "incorrect rsrp");
            assert_eq!(parsed.get_meas_rssi(), rssi, "incorrect rssi");
            assert_eq!(parsed.get_meas_rsrq(), rsrq, "incorrect rsrq");
        }
    }

    // Adapted from scat's TestDiagLteLogParser::test_parse_lte_ml1_ncell_meas,
    // but edited to print full-precision floats
    #[test]
    fn test_ncell_meas() {
        ncell_meas_case(
            "040100009C1847008348E44DDEA44C00CAB4CC32B6D8420300000000FF773301FF77330122020100",
            4,
            6300,
            vec![
                (131, -102.125, -75.75, -17.3125),
            ]
        );
         ncell_meas_case(
             "05010000160d0000480000006cea413bb4433b00b4f3cc33cf3c130200000000ffefc00fffefc00f45081600",
             5,
             3350,
             vec![
                 (108, -120.75, -94.6875, -17.0625),
             ]
         );
    }
}
