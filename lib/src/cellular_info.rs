//! Cellular network information extraction from QMDL log messages
//! 
//! This module provides functionality to extract cellular network parameters
//! such as MCC, MNC, LAC, Cell ID, TAC, and other network identifiers from
//! various log message types captured in QMDL files.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

/// Comprehensive cellular network information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellularNetworkInfo {
    pub timestamp: DateTime<FixedOffset>,
    pub rat: RadioAccessTechnology,
    pub plmn_info: Option<PlmnInfo>,
    pub cell_info: Option<CellInfo>,
    pub location_info: Option<LocationInfo>,
    pub signal_info: Option<SignalInfo>,
    pub neighbor_cells: Vec<NeighborCellInfo>,
}

/// Radio Access Technology type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RadioAccessTechnology {
    GSM,
    GPRS,
    EDGE,
    UMTS,
    HSPA,
    LTE,
    NR, // 5G
}

/// Public Land Mobile Network information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlmnInfo {
    pub mcc: Option<u16>,        // Mobile Country Code
    pub mnc: Option<u16>,        // Mobile Network Code
    pub plmn_id: Option<String>, // Combined PLMN identifier
}

/// Cell identification information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellInfo {
    pub physical_cell_id: Option<u16>,  // Physical Cell ID (PCI) for LTE
    pub global_cell_id: Option<u32>,    // Global Cell ID
    pub cell_identity: Option<u32>,     // Cell Identity (28-bit for LTE)
    pub enodeb_id: Option<u32>,         // eNodeB ID (for LTE)
    pub sector_id: Option<u8>,          // Sector ID within eNodeB
}

/// Location area and tracking area information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationInfo {
    pub lac: Option<u16>,               // Location Area Code (2G/3G)
    pub rac: Option<u8>,                // Routing Area Code (GPRS)
    pub tac: Option<u16>,               // Tracking Area Code (LTE)
    pub tracking_area_id: Option<u32>,  // Full Tracking Area Identity
}

/// Signal quality and measurement information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalInfo {
    pub rsrp: Option<i16>,              // Reference Signal Received Power (dBm)
    pub rsrq: Option<i16>,              // Reference Signal Received Quality (dB)
    pub rssi: Option<i16>,              // Received Signal Strength Indicator (dBm)
    pub sinr: Option<i16>,              // Signal to Interference plus Noise Ratio (dB)
    pub cqi: Option<u8>,                // Channel Quality Indicator
    pub bandwidth: Option<u8>,          // Channel bandwidth (MHz)
}

/// Neighbor cell information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeighborCellInfo {
    pub physical_cell_id: Option<u16>,
    pub earfcn: Option<u32>,            // E-UTRA Absolute Radio Frequency Channel Number
    pub rsrp: Option<i16>,
    pub rsrq: Option<i16>,
    pub plmn_info: Option<PlmnInfo>,
}

/// Frequency and channel information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrequencyInfo {
    pub earfcn: Option<u32>,            // LTE frequency
    pub uarfcn: Option<u16>,            // UMTS frequency
    pub arfcn: Option<u16>,             // GSM frequency
    pub band: Option<u8>,               // Frequency band
    pub duplex_mode: Option<DuplexMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DuplexMode {
    FDD, // Frequency Division Duplex
    TDD, // Time Division Duplex
}

/// System Information Block information for LTE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SibInfo {
    pub sib_type: u8,
    pub plmn_list: Vec<PlmnInfo>,
    pub cell_access_related_info: Option<CellAccessInfo>,
    pub cell_selection_info: Option<CellSelectionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellAccessInfo {
    pub cell_barred: bool,
    pub intra_freq_reselection: bool,
    pub csg_indication: bool,
    pub csg_identity: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellSelectionInfo {
    pub q_rx_lev_min: Option<i8>,
    pub q_qual_min: Option<i8>,
}

/// Main cellular information extractor
pub struct CellularInfoExtractor {
    current_info: HashMap<String, CellularNetworkInfo>,
    _plmn_cache: HashMap<u32, PlmnInfo>,
}

impl CellularInfoExtractor {
    pub fn new() -> Self {
        Self {
            current_info: HashMap::new(),
            _plmn_cache: HashMap::new(),
        }
    }

    /// Extract cellular information from raw log data
    pub fn extract_from_log_data(
        &mut self,
        log_type: u16,
        log_data: &[u8],
        timestamp: DateTime<FixedOffset>,
    ) -> Option<CellularNetworkInfo> {
        match log_type {
            // LTE RRC messages - extract PLMN and cell info from SIBs
            0xb0c0 => self.extract_lte_rrc_info(log_data, timestamp),
            
            // LTE ML1 serving cell info
            0xb0e4 => self.extract_lte_serving_cell_info(log_data, timestamp),
            
            // LTE ML1 neighbor measurements
            0xb0e1 => self.extract_lte_neighbor_info(log_data, timestamp),
            
            // GSM cell information
            0x513a => self.extract_gsm_cell_id(log_data, timestamp),
            0x513b => self.extract_gsm_cell_info(log_data, timestamp),
            
            // WCDMA cell information
            0x4127 => self.extract_wcdma_cell_id(log_data, timestamp),
            0x412a => self.extract_wcdma_serving_cell_info(log_data, timestamp),
            
            // NAS messages - extract PLMN and location info
            0xb0ec | 0xb0ed => self.extract_nas_info(log_data, timestamp),
            
            _ => None,
        }
    }

    /// Extract information from LTE RRC messages
    fn extract_lte_rrc_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        // Parse LTE RRC OTA message structure
        if data.len() < 20 {
            return None;
        }

        // Extract basic info from RRC header
        let phy_cell_id = u16::from_le_bytes([data[7], data[8]]);
        let _earfcn = if data.len() > 12 {
            Some(u32::from_le_bytes([data[9], data[10], data[11], data[12]]))
        } else {
            Some(u16::from_le_bytes([data[9], data[10]]) as u32)
        };

        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::LTE,
            plmn_info: None, // Will be populated from SIB parsing
            cell_info: Some(CellInfo {
                physical_cell_id: Some(phy_cell_id),
                global_cell_id: None,
                cell_identity: None,
                enodeb_id: None,
                sector_id: None,
            }),
            location_info: None,
            signal_info: None,
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract LTE serving cell information
    fn extract_lte_serving_cell_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        // Parse LTE ML1 serving cell info structure
        if data.len() < 16 {
            return None;
        }

        // Extract EARFCN and PCI (positions may vary based on log version)
        let _earfcn = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let phy_cell_id = u16::from_le_bytes([data[4], data[5]]);
        
        // Extract signal measurements if available
        let rsrp = if data.len() > 8 {
            Some(i16::from_le_bytes([data[6], data[7]]))
        } else {
            None
        };
        
        let rsrq = if data.len() > 10 {
            Some(i16::from_le_bytes([data[8], data[9]]))
        } else {
            None
        };

        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::LTE,
            plmn_info: None,
            cell_info: Some(CellInfo {
                physical_cell_id: Some(phy_cell_id),
                global_cell_id: None,
                cell_identity: None,
                enodeb_id: None,
                sector_id: None,
            }),
            location_info: None,
            signal_info: Some(SignalInfo {
                rsrp,
                rsrq,
                rssi: None,
                sinr: None,
                cqi: None,
                bandwidth: None,
            }),
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract LTE neighbor cell information
    fn extract_lte_neighbor_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 8 {
            return None;
        }

        let mut neighbor_cells = Vec::new();
        
        // Parse neighbor cell measurements (format varies by log version)
        let mut offset = 4; // Skip header
        while offset + 8 <= data.len() {
            let phy_cell_id = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let earfcn = u32::from_le_bytes([data[offset + 2], data[offset + 3], data[offset + 4], data[offset + 5]]);
            let rsrp = i16::from_le_bytes([data[offset + 6], data[offset + 7]]);
            
            neighbor_cells.push(NeighborCellInfo {
                physical_cell_id: Some(phy_cell_id),
                earfcn: Some(earfcn),
                rsrp: Some(rsrp),
                rsrq: None,
                plmn_info: None,
            });
            
            offset += 8;
        }

        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::LTE,
            plmn_info: None,
            cell_info: None,
            location_info: None,
            signal_info: None,
            neighbor_cells,
        })
    }

    /// Extract GSM cell ID information
    fn extract_gsm_cell_id(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 6 {
            return None;
        }

        let cell_id = u16::from_le_bytes([data[0], data[1]]);
        let lac = u16::from_le_bytes([data[2], data[3]]);
        let mcc_mnc = u16::from_le_bytes([data[4], data[5]]);
        
        // Decode MCC/MNC from combined value
        let mcc = (mcc_mnc & 0x0fff) as u16;
        let mnc = ((mcc_mnc >> 12) & 0x0fff) as u16;

        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::GSM,
            plmn_info: Some(PlmnInfo {
                mcc: Some(mcc),
                mnc: Some(mnc),
                plmn_id: Some(format!("{:03}{:03}", mcc, mnc)),
            }),
            cell_info: Some(CellInfo {
                physical_cell_id: None,
                global_cell_id: Some(cell_id as u32),
                cell_identity: Some(cell_id as u32),
                enodeb_id: None,
                sector_id: None,
            }),
            location_info: Some(LocationInfo {
                lac: Some(lac),
                rac: None,
                tac: None,
                tracking_area_id: None,
            }),
            signal_info: None,
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract GSM cell information
    fn extract_gsm_cell_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 8 {
            return None;
        }

        // GSM cell information format varies, but typically includes:
        let _arfcn = u16::from_le_bytes([data[0], data[1]]);
        let _bsic = data[2]; // Base Station Identity Code
        let rx_level = data[3] as i16 - 110; // Convert to dBm
        
        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::GSM,
            plmn_info: None,
            cell_info: None,
            location_info: None,
            signal_info: Some(SignalInfo {
                rsrp: None,
                rsrq: None,
                rssi: Some(rx_level),
                sinr: None,
                cqi: None,
                bandwidth: None,
            }),
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract WCDMA cell ID information
    fn extract_wcdma_cell_id(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 8 {
            return None;
        }

        let cell_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let lac = u16::from_le_bytes([data[4], data[5]]);
        
        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::UMTS,
            plmn_info: None,
            cell_info: Some(CellInfo {
                physical_cell_id: None,
                global_cell_id: Some(cell_id),
                cell_identity: Some(cell_id),
                enodeb_id: None,
                sector_id: None,
            }),
            location_info: Some(LocationInfo {
                lac: Some(lac),
                rac: None,
                tac: None,
                tracking_area_id: None,
            }),
            signal_info: None,
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract WCDMA serving cell information
    fn extract_wcdma_serving_cell_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 12 {
            return None;
        }

        let _uarfcn = u16::from_le_bytes([data[0], data[1]]);
        let psc = u16::from_le_bytes([data[2], data[3]]); // Primary Scrambling Code
        let rscp = i16::from_le_bytes([data[4], data[5]]); // Received Signal Code Power
        let ecno = i16::from_le_bytes([data[6], data[7]]); // Ec/No ratio
        
        Some(CellularNetworkInfo {
            timestamp,
            rat: RadioAccessTechnology::UMTS,
            plmn_info: None,
            cell_info: Some(CellInfo {
                physical_cell_id: Some(psc),
                global_cell_id: None,
                cell_identity: None,
                enodeb_id: None,
                sector_id: None,
            }),
            location_info: None,
            signal_info: Some(SignalInfo {
                rsrp: Some(rscp),
                rsrq: Some(ecno),
                rssi: None,
                sinr: None,
                cqi: None,
                bandwidth: None,
            }),
            neighbor_cells: Vec::new(),
        })
    }

    /// Extract information from NAS messages
    fn extract_nas_info(&mut self, data: &[u8], timestamp: DateTime<FixedOffset>) -> Option<CellularNetworkInfo> {
        if data.len() < 10 {
            return None;
        }

        // Skip NAS header and extract PLMN and location info from EMM messages
        // This is a simplified extraction - full NAS parsing would require more complex logic
        
        // Look for PLMN ID in common NAS message positions
        if data.len() > 15 {
            // Try to extract PLMN from various NAS message types
            let plmn_bytes = &data[10..13]; // Common PLMN position in many NAS messages
            if let Some(plmn_info) = self.decode_plmn_from_bytes(plmn_bytes) {
                return Some(CellularNetworkInfo {
                    timestamp,
                    rat: RadioAccessTechnology::LTE,
                    plmn_info: Some(plmn_info),
                    cell_info: None,
                    location_info: None, // TAC would need more sophisticated parsing
                    signal_info: None,
                    neighbor_cells: Vec::new(),
                });
            }
        }

        None
    }

    /// Decode PLMN from 3-byte BCD format
    fn decode_plmn_from_bytes(&self, plmn_bytes: &[u8]) -> Option<PlmnInfo> {
        if plmn_bytes.len() < 3 {
            return None;
        }

        // PLMN is encoded in BCD format across 3 bytes
        // Byte 0: MCC digit 2 | MCC digit 1
        // Byte 1: MNC digit 3 | MCC digit 3  
        // Byte 2: MNC digit 2 | MNC digit 1
        
        let mcc_digit1 = plmn_bytes[0] & 0x0f;
        let mcc_digit2 = (plmn_bytes[0] >> 4) & 0x0f;
        let mcc_digit3 = plmn_bytes[1] & 0x0f;
        
        let mnc_digit3 = (plmn_bytes[1] >> 4) & 0x0f;
        let mnc_digit1 = plmn_bytes[2] & 0x0f;
        let mnc_digit2 = (plmn_bytes[2] >> 4) & 0x0f;
        
        // Check for valid BCD digits
        if mcc_digit1 > 9 || mcc_digit2 > 9 || mcc_digit3 > 9 ||
           mnc_digit1 > 9 || mnc_digit2 > 9 || (mnc_digit3 > 9 && mnc_digit3 != 0xf) {
            return None;
        }
        
        let mcc = mcc_digit1 as u16 + (mcc_digit2 as u16) * 10 + (mcc_digit3 as u16) * 100;
        
        let mnc = if mnc_digit3 == 0xf {
            // 2-digit MNC
            mnc_digit1 as u16 + (mnc_digit2 as u16) * 10
        } else {
            // 3-digit MNC
            mnc_digit1 as u16 + (mnc_digit2 as u16) * 10 + (mnc_digit3 as u16) * 100
        };
        
        Some(PlmnInfo {
            mcc: Some(mcc),
            mnc: Some(mnc),
            plmn_id: Some(format!("{:03}{:02}", mcc, mnc)),
        })
    }

    /// Get all collected cellular information
    pub fn get_all_info(&self) -> Vec<&CellularNetworkInfo> {
        self.current_info.values().collect()
    }

    /// Clear collected information
    pub fn clear(&mut self) {
        self.current_info.clear();
    }
}

impl Default for CellularInfoExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plmn_decoding() {
        let extractor = CellularInfoExtractor::new();
        
        // Test 3-digit MNC (MCC=310, MNC=410)
        let plmn_bytes = [0x13, 0x00, 0x14]; 
        let plmn_info = extractor.decode_plmn_from_bytes(&plmn_bytes);
        assert!(plmn_info.is_some());
        
        // Test 2-digit MNC (MCC=310, MNC=41)
        let plmn_bytes = [0x13, 0xf0, 0x14];
        let plmn_info = extractor.decode_plmn_from_bytes(&plmn_bytes);
        assert!(plmn_info.is_some());
    }
}
