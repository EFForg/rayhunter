use serde::{Deserialize, Serialize};
use crate::gsmtap::GsmtapMessage;
use crate::analysis::information_element::{InformationElement, LteInformationElement};
use crate::diag::{Message, LogBody, LteRrcOtaPacket};
use telcom_parser::lte_rrc;
use pycrate_rs::nas::NASMessage;

/// GPS location data for correlation with cellular events
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpsLocation {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
    pub timestamp: Option<String>,
    pub source: String, // "gps", "network", "estimated"
}

/// Security analysis results for cellular network threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    pub threat_level: ThreatLevel,
    pub attack_type: Option<AttackType>,
    pub confidence: f32, // 0.0 to 1.0
    pub indicators: Vec<String>,
    pub recommendations: Vec<String>,
    pub known_attacker: Option<bool>,
    pub historical_incidents: Vec<HistoricalIncident>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackType {
    ImsiCatcher,
    Stingray,
    ManInTheMiddle,
    DowngradeAttack,
    NullCipher,
    FakeBaseStation,
    LocationTracking,
    DataInterception,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalIncident {
    pub timestamp: String,
    pub location: GpsLocation,
    pub cell_id: String,
    pub attack_type: AttackType,
    pub description: String,
}

/// Cellular network identification data extracted from QMDL messages
/// This matches the fields that SCAT extracts from QMDL files
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CellularData {
    // GSMTAP fields (from SCAT: gsmtap.arfcn, gsmtap.mcc, gsmtap.mnc, gsmtap.lac, gsmtap.cell_id)
    pub arfcn: Option<u16>,
    pub frame_number: Option<u32>,
    pub uplink: Option<bool>,
    pub signal_dbm: Option<i8>,
    
    // LTE specific fields (from SCAT: lte_rrc.mcc, lte_rrc.mnc, lte_rrc.lac, lte_rrc.cellIdentity, lte_rrc.tac)
    pub phy_cell_id: Option<u16>,
    pub earfcn: Option<u32>,
    pub tracking_area_code: Option<u16>,
    pub cell_identity: Option<u32>,
    
    // Network identification (from SCAT: gsm_a.dtap.mcc, gsm_a.dtap.mnc, umts_rrc.mcc, umts_rrc.mnc)
    pub mcc: Option<u16>,  // Mobile Country Code
    pub mnc: Option<u16>,  // Mobile Network Code
    pub lac: Option<u16>,  // Location Area Code
    
    // Signal quality measurements (from SCAT: RSRP, RSRQ, RSSI, SINR)
    pub rsrp: Option<f32>,  // Reference Signal Received Power (dBm)
    pub rsrq: Option<f32>,  // Reference Signal Received Quality (dB)
    pub rssi: Option<f32>,  // Received Signal Strength Indicator (dBm)
    pub sinr: Option<f32>,  // Signal to Interference plus Noise Ratio (dB)
    pub snr: Option<f32>,   // Signal to Noise Ratio (dB)
    
    // Neighbor cell information
    pub neighbor_cells: Vec<NeighborCell>,
    
    // GPS location data for correlation
    pub gps_location: Option<GpsLocation>,
    
    // Security analysis results
    pub security_analysis: Option<SecurityAnalysis>,
    
    // Additional fields
    pub protocol_type: String,
    pub message_type: String,
    pub timestamp: Option<String>,
    
    // Network operator information
    pub operator_name: Option<String>,
    pub network_type: Option<String>, // "2G", "3G", "4G", "5G"
    
    // Cell tower information
    pub cell_tower_id: Option<String>,
    pub sector_id: Option<u8>,
    pub antenna_height: Option<f32>,
    pub antenna_direction: Option<u16>,
    
    // Quality metrics
    pub call_quality: Option<f32>, // 0.0 to 1.0
    pub data_rate: Option<f32>, // Mbps
    pub latency: Option<f32>, // ms
    pub packet_loss: Option<f32>, // percentage
    pub ecno: Option<f32>, // Ec/No (dB) for UMTS
    pub band: Option<u16>, // Frequency band
    pub rnc: Option<u16>,  // Radio Network Controller ID (UMTS)
}

/// Neighbor cell information extracted from QMDL messages
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeighborCell {
    pub pci: Option<u16>,           // Physical Cell ID
    pub earfcn: Option<u32>,        // EARFCN
    pub rsrp: Option<f32>,          // RSRP (dBm)
    pub rsrq: Option<f32>,          // RSRQ (dB)
    pub rssi: Option<f32>,          // RSSI (dBm)
    pub sinr: Option<f32>,          // SINR (dB)
    pub cell_type: String,          // "intra_freq", "inter_freq", "inter_rat"
    pub rank: Option<u8>,           // Cell ranking
    pub ecno: Option<f32>, // Ec/No (dB) for UMTS
    pub band: Option<u16>, // Frequency band
    pub rnc: Option<u16>,  // Radio Network Controller ID (UMTS)
}

/// Lookup table for LTE EARFCN to band mapping (partial, extend as needed)
fn earfcn_to_band(earfcn: u32) -> Option<u16> {
    match earfcn {
        0..=599 => Some(1),      // Band 1
        1200..=1949 => Some(3),  // Band 3
        1950..=2399 => Some(4),  // Band 4
        2400..=2649 => Some(5),  // Band 5
        2750..=3449 => Some(7),  // Band 7
        3450..=3799 => Some(8),  // Band 8
        6150..=6449 => Some(20), // Band 20
        65536..=66435 => Some(28), // Band 28
        // Add more bands as needed
        _ => None,
    }
}

impl CellularData {
    /// Extract cellular data from GSMTAP message and information element
    pub fn from_gsmtap_and_ie(gsmtap: &GsmtapMessage, ie: &InformationElement) -> Self {
        let mut data = CellularData {
            arfcn: Some(gsmtap.header.arfcn),
            frame_number: Some(gsmtap.header.frame_number),
            uplink: Some(gsmtap.header.uplink),
            signal_dbm: Some(gsmtap.header.signal_dbm),
            protocol_type: format!("{:?}", gsmtap.header.gsmtap_type),
            message_type: Self::get_message_type(ie),
            ..Default::default()
        };

        // ARFCN in LTE is actually EARFCN
        if gsmtap.header.arfcn > 0 {
            data.earfcn = Some(gsmtap.header.arfcn as u32);
        }

        // Extract LTE-specific cellular data
        if let InformationElement::LTE(lte_ie) = ie {
            data.extract_lte_data(lte_ie);
        }

        data
    }

    /// Extract cellular data including physical cell ID from original QMDL message
    pub fn from_qmdl_message_and_ie(msg: &Message, gsmtap: &GsmtapMessage, ie: &InformationElement) -> Self {
        let mut data = Self::from_gsmtap_and_ie(gsmtap, ie);
        
        // Extract physical cell ID and EARFCN from the original QMDL message
        if let Message::Log { body: LogBody::LteRrcOtaMessage { packet, .. }, .. } = msg {
            data.phy_cell_id = Some(packet.get_phy_cell_id());
            data.earfcn = Some(packet.get_earfcn());
        }

        data
    }

    fn get_message_type(ie: &InformationElement) -> String {
        match ie {
            InformationElement::LTE(lte_ie) => match &**lte_ie {
                LteInformationElement::DlCcch(_) => "LTE-RRC-DL-CCCH".to_string(),
                LteInformationElement::DlDcch(_) => "LTE-RRC-DL-DCCH".to_string(),
                LteInformationElement::UlCcch(_) => "LTE-RRC-UL-CCCH".to_string(),
                LteInformationElement::UlDcch(_) => "LTE-RRC-UL-DCCH".to_string(),
                LteInformationElement::BcchBch(_) => "LTE-RRC-BCCH-BCH".to_string(),
                LteInformationElement::BcchDlSch(_) => "LTE-RRC-BCCH-DL-SCH".to_string(),
                LteInformationElement::PCCH(_) => "LTE-RRC-PCCH".to_string(),
                LteInformationElement::NAS(_) => "LTE-NAS".to_string(),
                _ => "LTE-RRC-Other".to_string(),
            },
            InformationElement::GSM => "GSM".to_string(),
            InformationElement::UMTS => "UMTS".to_string(),
            InformationElement::FiveG => "5G-NR".to_string(),
        }
    }

    fn extract_lte_data(&mut self, lte_ie: &LteInformationElement) {
        match lte_ie {
            LteInformationElement::BcchDlSch(msg) => {
                self.extract_from_bcch_dl_sch(msg);
            },
            LteInformationElement::DlDcch(msg) => {
                self.extract_from_dl_dcch(msg);
            },
            LteInformationElement::NAS(nas_msg) => {
                self.extract_from_nas(nas_msg);
            },
            // Add logic for extracting band and other fields if available
            _ => {}
        }
    }

    fn extract_from_bcch_dl_sch(&mut self, msg: &lte_rrc::BCCH_DL_SCH_Message) {
        use lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1, SystemInformationCriticalExtensions};
        
        if let BCCH_DL_SCH_MessageType::C1(c1) = &msg.message {
            match c1 {
                BCCH_DL_SCH_MessageType_c1::SystemInformation(sys_info) => {
                    if let SystemInformationCriticalExtensions::SystemInformation_r8(r8) = &sys_info.critical_extensions {
                        self.extract_from_system_information(&r8.sib_type_and_info);
                    }
                },
                BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) => {
                    self.extract_from_sib1(sib1);
                }
            }
        }
        // Example: If band info is available in msg, extract it here
        // self.band = Some(...);
    }

    fn extract_from_dl_dcch(&mut self, msg: &lte_rrc::DL_DCCH_Message) {
        use lte_rrc::DL_DCCH_MessageType;
        
        // Extract signal quality measurements from measurement reports
        match &msg.message {
            DL_DCCH_MessageType::C1(c1) => {
                match c1 {
                    // lte_rrc::DL_DCCH_MessageType_c1::MeasurementReport(meas_report) => {
                    //     self.extract_from_measurement_report(meas_report);
                    // },
                    _ => {}
                }
            },
            _ => {}
        }
        // Example: If ecno or rnc info is available in msg, extract it here
        // self.ecno = Some(...);
        // self.rnc = Some(...);
    }

    fn extract_from_measurement_report(&mut self, _meas_report: &lte_rrc::MeasurementReport) {
        // Extract signal quality measurements from measurement report
        // TODO: Implement detailed measurement report parsing when we have the correct types
        // This will extract RSRP, RSRQ, and other signal quality measurements
    }

    fn extract_from_system_information(&mut self, sib_info: &lte_rrc::SystemInformation_r8_IEsSib_TypeAndInfo) {
        use lte_rrc::SystemInformation_r8_IEsSib_TypeAndInfo_Entry;
        
        for entry in &sib_info.0 {
            match entry {
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib3(sib3) => {
                    // SIB3 contains cell reselection info
                    // if let Some(cell_reselection_info) = &sib3.cell_reselection_info_common {
                    //     self.extract_from_sib3(cell_reselection_info);
                    // }
                },
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib4(sib4) => {
                    // SIB4 contains intra-frequency neighbor cell list
                    if let Some(intra_freq_neigh_cell_list) = &sib4.intra_freq_neigh_cell_list {
                        self.extract_neighbor_cells_from_sib4(intra_freq_neigh_cell_list, "intra_freq");
                    }
                },
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib5(sib5) => {
                    // SIB5 contains inter-frequency neighbor cell list
                    self.extract_neighbor_cells_from_sib5(&sib5.inter_freq_carrier_freq_list);
                },
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib6(_sib6) => {
                    // SIB6 contains UTRA neighbor cell list
                    // TODO: Implement when structure is confirmed
                },
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib7(sib7) => {
                    // SIB7 contains GERAN neighbor cell list
                    if let Some(carrier_freqs_info_list) = &sib7.carrier_freqs_info_list {
                        self.extract_neighbor_cells_from_sib7(carrier_freqs_info_list);
                    }
                },
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib8(sib8) => {
                    // SIB8 contains CDMA2000 neighbor cell list
                    // CDMA2000 neighbor extraction will be implemented when we have the correct types
                },
                _ => {}
            }
        }
    }

    fn extract_from_sib1(&mut self, _sib1: &lte_rrc::SystemInformationBlockType1) {
        // Extract Cell Identity and TAC from SIB1
        // TODO: Implement when structure is confirmed
    }

    fn extract_mcc_mnc_from_plmn(&mut self, plmn: &lte_rrc::PLMN_Identity) {
        // Extract MCC (Mobile Country Code)
        if let Some(mcc) = &plmn.mcc {
            if mcc.0.len() >= 3 {
                let mcc_val = (mcc.0[0].0 as u16) * 100 + (mcc.0[1].0 as u16) * 10 + (mcc.0[2].0 as u16);
                self.mcc = Some(mcc_val);
            }
        }

        // Extract MNC (Mobile Network Code)
        let mnc = &plmn.mnc;
        if mnc.0.len() >= 2 {
            let mnc_val = if mnc.0.len() == 2 {
                (mnc.0[0].0 as u16) * 10 + (mnc.0[1].0 as u16)
            } else {
                (mnc.0[0].0 as u16) * 100 + (mnc.0[1].0 as u16) * 10 + (mnc.0[2].0 as u16)
            };
            self.mnc = Some(mnc_val);
        }
    }

    fn extract_from_nas(&mut self, nas_msg: &NASMessage) {
        // Extract fields from NAS messages if needed
        // This is where attach requests, authentication, etc. are handled
    }

    fn extract_from_sib3(&mut self, _cell_reselection_info: &lte_rrc::SystemInformationBlockType3) {
        // Extract cell reselection information
        // TODO: Implement proper extraction when types are confirmed
    }

    fn extract_neighbor_cells_from_sib4(&mut self, intra_freq_neigh_cell_list: &lte_rrc::IntraFreqNeighCellList, cell_type: &str) {
        // Extract intra-frequency neighbor cells from SIB4
        for neighbor_cell in &intra_freq_neigh_cell_list.0 {
            let mut neighbor = NeighborCell {
                pci: Some(neighbor_cell.phys_cell_id.0),
                cell_type: cell_type.to_string(),
                ..Default::default()
            };
            
            // Extract q_offset_range if available
            // q_offset_range affects cell reselection priority
            let q_offset = neighbor_cell.q_offset_cell.0 as i8;
            if q_offset != 0 {
                // Convert q_offset to signal quality adjustment
                // q_offset is in dB, typically -24 to +24 dB
            }
            
            self.neighbor_cells.push(neighbor);
        }
    }

    fn extract_neighbor_cells_from_sib5(&mut self, inter_freq_carrier_freq_list: &lte_rrc::InterFreqCarrierFreqList) {
        // Extract inter-frequency neighbor cells from SIB5
        for carrier_freq in &inter_freq_carrier_freq_list.0 {
            let earfcn = carrier_freq.dl_carrier_freq.0;
            // Map EARFCN to band
            let band = earfcn_to_band(earfcn.into());
            if self.band.is_none() && band.is_some() {
                self.band = band;
            }
            // Extract neighbor cell list if available
            if let Some(inter_freq_neigh_cell_list) = &carrier_freq.inter_freq_neigh_cell_list {
                for neighbor_cell in &inter_freq_neigh_cell_list.0 {
                    let mut neighbor = NeighborCell {
                        pci: Some(neighbor_cell.phys_cell_id.0),
                        earfcn: Some(earfcn.into()),
                        band,
                        cell_type: "inter_freq".to_string(),
                        ..Default::default()
                    };
                    
                    // Extract q_offset_range if available
                    let q_offset = neighbor_cell.q_offset_cell.0 as i8;
                    if q_offset != 0 {
                        // Convert q_offset to signal quality adjustment
                    }
                    
                    self.neighbor_cells.push(neighbor);
                }
            }
        }
    }

    fn extract_neighbor_cells_from_sib6(&mut self, _utra_carrier_freq_list: &lte_rrc::CarrierFreqListUTRA_FDD) {
        // Extract UTRA neighbor cells from SIB6
        // TODO: Implement when structure is confirmed
    }

    fn extract_neighbor_cells_from_sib7(&mut self, _carrier_freqs_info_list: &lte_rrc::CarrierFreqsInfoListGERAN) {
        // Extract GERAN neighbor cells from SIB7
        // TODO: Implement when structure is confirmed
    }

    // TODO: Implement CDMA2000 neighbor cell extraction when we have the correct types
    // This will be added once we verify the exact CDMA2000 structure names

    /// Parse RSRP value from Qualcomm format (similar to SCAT)
    fn parse_rsrp(&self, rsrp_raw: u16) -> f32 {
        -180.0 + (rsrp_raw as f32) * 0.0625
    }

    /// Parse RSRQ value from Qualcomm format (similar to SCAT)
    fn parse_rsrq(&self, rsrq_raw: u16) -> f32 {
        -30.0 + (rsrq_raw as f32) * 0.0625
    }

    /// Parse RSSI value from Qualcomm format (similar to SCAT)
    fn parse_rssi(&self, rssi_raw: u16) -> f32 {
        -110.0 + (rssi_raw as f32) * 0.0625
    }

    /// Check if this cellular data contains useful network identification
    pub fn has_cell_identification(&self) -> bool {
        self.cell_identity.is_some() || 
        self.tracking_area_code.is_some() || 
        self.phy_cell_id.is_some() ||
        (self.mcc.is_some() && self.mnc.is_some())
    }

    /// Perform security analysis to detect known attack patterns
    pub fn analyze_security(&mut self) {
        let mut indicators = Vec::new();
        let mut threat_level = ThreatLevel::None;
        let mut attack_type = None;
        let mut confidence: f32 = 0.0;

        // Check for IMSI Catcher indicators
        if self.rsrp.is_some() && self.rsrp.unwrap() > -60.0 {
            indicators.push("Unusually strong signal strength".to_string());
            confidence += 0.3;
        }

        if self.neighbor_cells.len() < 2 {
            indicators.push("Limited neighbor cell information".to_string());
            confidence += 0.2;
        }

        // Check for downgrade attacks
        if self.network_type.as_deref() == Some("2G") {
            indicators.push("Connected to 2G network".to_string());
            attack_type = Some(AttackType::DowngradeAttack);
            confidence += 0.4;
        }

        // Check for null cipher
        if self.protocol_type.contains("NULL") {
            indicators.push("Null cipher detected".to_string());
            attack_type = Some(AttackType::NullCipher);
            confidence += 0.5;
        }

        // Determine threat level based on confidence
        threat_level = match confidence {
            c if c >= 0.7 => ThreatLevel::High,
            c if c >= 0.4 => ThreatLevel::Medium,
            c if c >= 0.2 => ThreatLevel::Low,
            _ => ThreatLevel::None,
        };

        if threat_level != ThreatLevel::None {
            self.security_analysis = Some(SecurityAnalysis {
                threat_level,
                attack_type,
                confidence: confidence.min(1.0),
                indicators,
                recommendations: vec![
                    "Monitor signal strength patterns".to_string(),
                    "Check for unusual network behavior".to_string(),
                    "Verify network operator authenticity".to_string(),
                ],
                known_attacker: None,
                historical_incidents: Vec::new(),
            });
        }
    }

    /// Add GPS location data
    pub fn add_gps_location(&mut self, lat: f64, lon: f64, alt: Option<f64>, accuracy: Option<f64>) {
        self.gps_location = Some(GpsLocation {
            latitude: Some(lat),
            longitude: Some(lon),
            altitude: alt,
            accuracy,
            timestamp: Some(chrono::Utc::now().timestamp().to_string()),
            source: "gps".to_string(),
        });
    }

    /// Generate unique cell identifier for attack correlation
    pub fn get_cell_id(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(mcc) = self.mcc {
            parts.push(format!("MCC:{}", mcc));
        }
        if let Some(mnc) = self.mnc {
            parts.push(format!("MNC:{}", mnc));
        }
        if let Some(tac) = self.tracking_area_code {
            parts.push(format!("TAC:{}", tac));
        }
        if let Some(cell_id) = self.cell_identity {
            parts.push(format!("CID:{}", cell_id));
        }
        if let Some(pci) = self.phy_cell_id {
            parts.push(format!("PCI:{}", pci));
        }
        if let Some(earfcn) = self.earfcn {
            parts.push(format!("EARFCN:{}", earfcn));
        }

        if parts.is_empty() {
            "UNKNOWN".to_string()
        } else {
            parts.join("_")
        }
    }

    /// Get a summary string of the cellular data
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if let (Some(mcc), Some(mnc)) = (self.mcc, self.mnc) {
            parts.push(format!("MCC/MNC:{}/{}", mcc, mnc));
        }

        if let Some(cell_id) = self.cell_identity {
            parts.push(format!("CellID:{}", cell_id));
        }

        if let Some(tac) = self.tracking_area_code {
            parts.push(format!("TAC:{}", tac));
        }

        if let Some(pci) = self.phy_cell_id {
            parts.push(format!("PCI:{}", pci));
        }

        if let Some(earfcn) = self.earfcn {
            parts.push(format!("EARFCN:{}", earfcn));
        }

        if parts.is_empty() {
            format!("{} - No cell data", self.protocol_type)
        } else {
            format!("{} - {}", self.protocol_type, parts.join(", "))
        }
    }

    /// Convert to comprehensive NDJSON format with Unix timestamp
    pub fn to_ndjson_format(&self) -> serde_json::Value {
        let mut json = serde_json::Map::new();
        
        // Add Unix timestamp
        json.insert("timestamp".to_string(), serde_json::Value::Number(
            serde_json::Number::from(chrono::Utc::now().timestamp())
        ));
        
        // Add cell identification
        if let Some(mcc) = self.mcc {
            json.insert("mcc".to_string(), serde_json::Value::Number(serde_json::Number::from(mcc)));
        }
        if let Some(mnc) = self.mnc {
            json.insert("mnc".to_string(), serde_json::Value::Number(serde_json::Number::from(mnc)));
        }
        if let Some(cell_id) = self.cell_identity {
            json.insert("cell_identity".to_string(), serde_json::Value::Number(serde_json::Number::from(cell_id)));
        }
        if let Some(tac) = self.tracking_area_code {
            json.insert("tracking_area_code".to_string(), serde_json::Value::Number(serde_json::Number::from(tac)));
        }
        if let Some(pci) = self.phy_cell_id {
            json.insert("phy_cell_id".to_string(), serde_json::Value::Number(serde_json::Number::from(pci)));
        }
        if let Some(earfcn) = self.earfcn {
            json.insert("earfcn".to_string(), serde_json::Value::Number(serde_json::Number::from(earfcn)));
        }
        
        // Add signal quality measurements
        if let Some(rsrp) = self.rsrp {
            json.insert("rsrp".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rsrp as f64).unwrap()));
        }
        if let Some(rsrq) = self.rsrq {
            json.insert("rsrq".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rsrq as f64).unwrap()));
        }
        if let Some(rssi) = self.rssi {
            json.insert("rssi".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rssi as f64).unwrap()));
        }
        if let Some(sinr) = self.sinr {
            json.insert("sinr".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(sinr as f64).unwrap()));
        }
        if let Some(ecno) = self.ecno {
            json.insert("ecno".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ecno as f64).unwrap()));
        }
        if let Some(band) = self.band {
            json.insert("band".to_string(), serde_json::Value::Number(serde_json::Number::from(band)));
        }
        if let Some(rnc) = self.rnc {
            json.insert("rnc".to_string(), serde_json::Value::Number(serde_json::Number::from(rnc)));
        }
        
        // Add GPS location
        if let Some(gps) = &self.gps_location {
            let mut gps_json = serde_json::Map::new();
            if let Some(lat) = gps.latitude {
                gps_json.insert("latitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(lat).unwrap()));
            }
            if let Some(lon) = gps.longitude {
                gps_json.insert("longitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(lon).unwrap()));
            }
            if let Some(alt) = gps.altitude {
                gps_json.insert("altitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(alt).unwrap()));
            }
            if let Some(acc) = gps.accuracy {
                gps_json.insert("accuracy".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(acc).unwrap()));
            }
            if let Some(ts) = &gps.timestamp {
                gps_json.insert("gps_timestamp".to_string(), serde_json::Value::String(ts.clone()));
            }
            gps_json.insert("source".to_string(), serde_json::Value::String(gps.source.clone()));
            json.insert("gps_location".to_string(), serde_json::Value::Object(gps_json));
        }
        
        // Add security analysis
        if let Some(security) = &self.security_analysis {
            let mut security_json = serde_json::Map::new();
            security_json.insert("threat_level".to_string(), serde_json::Value::String(format!("{:?}", security.threat_level)));
            if let Some(attack_type) = &security.attack_type {
                security_json.insert("attack_type".to_string(), serde_json::Value::String(format!("{:?}", attack_type)));
            }
            security_json.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(security.confidence as f64).unwrap()));
            security_json.insert("indicators".to_string(), serde_json::Value::Array(
                security.indicators.iter().map(|i| serde_json::Value::String(i.clone())).collect()
            ));
            security_json.insert("recommendations".to_string(), serde_json::Value::Array(
                security.recommendations.iter().map(|r| serde_json::Value::String(r.clone())).collect()
            ));
            json.insert("security_analysis".to_string(), serde_json::Value::Object(security_json));
        }
        
        // Add neighbor cells
        if !self.neighbor_cells.is_empty() {
            let neighbors: Vec<serde_json::Value> = self.neighbor_cells.iter().map(|n| {
                let mut neighbor_json = serde_json::Map::new();
                if let Some(pci) = n.pci {
                    neighbor_json.insert("pci".to_string(), serde_json::Value::Number(serde_json::Number::from(pci)));
                }
                if let Some(earfcn) = n.earfcn {
                    neighbor_json.insert("earfcn".to_string(), serde_json::Value::Number(serde_json::Number::from(earfcn)));
                }
                if let Some(rsrp) = n.rsrp {
                    neighbor_json.insert("rsrp".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rsrp as f64).unwrap()));
                }
                if let Some(rsrq) = n.rsrq {
                    neighbor_json.insert("rsrq".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(rsrq as f64).unwrap()));
                }
                if let Some(ecno) = n.ecno {
                    neighbor_json.insert("ecno".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ecno as f64).unwrap()));
                }
                if let Some(band) = n.band {
                    neighbor_json.insert("band".to_string(), serde_json::Value::Number(serde_json::Number::from(band)));
                }
                if let Some(rnc) = n.rnc {
                    neighbor_json.insert("rnc".to_string(), serde_json::Value::Number(serde_json::Number::from(rnc)));
                }
                neighbor_json.insert("cell_type".to_string(), serde_json::Value::String(n.cell_type.clone()));
                serde_json::Value::Object(neighbor_json)
            }).collect();
            json.insert("neighbor_cells".to_string(), serde_json::Value::Array(neighbors));
        }
        
        // Add cell identifier for attack correlation
        json.insert("cell_id".to_string(), serde_json::Value::String(self.get_cell_id()));
        
        // Add protocol information
        json.insert("protocol_type".to_string(), serde_json::Value::String(self.protocol_type.clone()));
        json.insert("message_type".to_string(), serde_json::Value::String(self.message_type.clone()));
        
        serde_json::Value::Object(json)
    }

    /// Convert to a format similar to SCAT's output
    pub fn to_scat_format(&self) -> Vec<(String, String)> {
        let mut fields = Vec::new();
        
        if let Some(arfcn) = self.arfcn {
            fields.push(("gsmtap.arfcn".to_string(), arfcn.to_string()));
        }
        
        if let Some(mcc) = self.mcc {
            fields.push(("lte_rrc.mcc".to_string(), mcc.to_string()));
        }
        
        if let Some(mnc) = self.mnc {
            fields.push(("lte_rrc.mnc".to_string(), mnc.to_string()));
        }
        
        if let Some(cell_id) = self.cell_identity {
            fields.push(("lte_rrc.cellIdentity".to_string(), cell_id.to_string()));
        }
        
        if let Some(tac) = self.tracking_area_code {
            fields.push(("lte_rrc.tac".to_string(), tac.to_string()));
        }
        
        if let Some(pci) = self.phy_cell_id {
            fields.push(("lte_rrc.phy_cell_id".to_string(), pci.to_string()));
        }
        
        if let Some(earfcn) = self.earfcn {
            fields.push(("lte_rrc.earfcn".to_string(), earfcn.to_string()));
        }
        
        // Add signal quality measurements
        if let Some(rsrp) = self.rsrp {
            fields.push(("lte_rrc.rsrp".to_string(), rsrp.to_string()));
        }
        
        if let Some(rsrq) = self.rsrq {
            fields.push(("lte_rrc.rsrq".to_string(), rsrq.to_string()));
        }
        
        if let Some(rssi) = self.rssi {
            fields.push(("lte_rrc.rssi".to_string(), rssi.to_string()));
        }
        
        if let Some(sinr) = self.sinr {
            fields.push(("lte_rrc.sinr".to_string(), sinr.to_string()));
        }
        
        // Add neighbor cell information
        for (i, neighbor) in self.neighbor_cells.iter().enumerate() {
            if let Some(pci) = neighbor.pci {
                fields.push((format!("neighbor_{}.pci", i), pci.to_string()));
            }
            if let Some(earfcn) = neighbor.earfcn {
                fields.push((format!("neighbor_{}.earfcn", i), earfcn.to_string()));
            }
            if let Some(rsrp) = neighbor.rsrp {
                fields.push((format!("neighbor_{}.rsrp", i), rsrp.to_string()));
            }
            if let Some(rsrq) = neighbor.rsrq {
                fields.push((format!("neighbor_{}.rsrq", i), rsrq.to_string()));
            }
            fields.push((format!("neighbor_{}.type", i), neighbor.cell_type.clone()));
        }
        
        fields
    }
} 