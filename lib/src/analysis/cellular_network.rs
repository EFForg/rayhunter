//! Cellular Network Information Analyzer
//!
//! This analyzer extracts and tracks cellular network parameters from QMDL messages,
//! including MCC/MNC/LAC/Cell ID and other network identifiers. It provides detailed
//! information about the cellular environment and enriches it with data from local
//! OpenCellID CSV files.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

use super::analyzer::{QmdlAnalyzer, Event, EventType};
use crate::cellular_info::{
    CellularNetworkInfo, LocationInfo, PlmnInfo, RadioAccessTechnology, SignalInfo,
    NeighborCellInfo, CellularInfoExtractor,
};

/// OpenCellID CSV record structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenCellIdRecord {
    pub radio: String,        // GSM, UMTS, LTE, NR
    pub mcc: u16,            // Mobile Country Code
    pub net: u16,            // Mobile Network Code (MNC)
    pub area: u32,           // Location Area Code (LAC) or Tracking Area Code (TAC)
    pub cell: u64,           // Cell ID
    pub unit: Option<u32>,   // For UMTS, this is the RNC-ID
    pub lon: Option<f64>,    // Longitude
    pub lat: Option<f64>,    // Latitude
    pub range: Option<u32>,  // Cell coverage range in meters
    pub samples: Option<u32>, // Number of measurements
    pub changeable: Option<u8>, // 1 if position is exact, 0 if approximate
    pub created: Option<u64>,   // Unix timestamp of creation
    pub updated: Option<u64>,   // Unix timestamp of last update
    pub average_signal: Option<i16>, // Average signal strength
}

/// Local cell database loaded from OpenCellID CSV files
pub struct CellDatabase {
    cells: HashMap<CellKey, OpenCellIdRecord>,
    loaded: bool,
}

/// Key for looking up cells in the database
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CellKey {
    radio: String,
    mcc: u16,
    mnc: u16,
    area: u32,
    cell: u64,
}

impl CellDatabase {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            loaded: false,
        }
    }

    /// Load cell data from CSV files in the specified directory
    pub fn load_from_directory<P: AsRef<Path>>(&mut self, csv_dir: P) -> Result<usize, Box<dyn std::error::Error>> {
        let dir = csv_dir.as_ref();
        if !dir.exists() {
            return Err(format!("Directory does not exist: {}", dir.display()).into());
        }

        let mut total_loaded = 0;
        
        // Look for CSV files in the directory
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                match self.load_csv_file(&path) {
                    Ok(count) => {
                        total_loaded += count;
                        log::info!("Loaded {} cells from {}", count, path.display());
                    }
                    Err(e) => {
                        log::warn!("Failed to load CSV file {}: {}", path.display(), e);
                    }
                }
            }
        }

        self.loaded = total_loaded > 0;
        Ok(total_loaded)
    }

    /// Load a single CSV file
    fn load_csv_file<P: AsRef<Path>>(&mut self, csv_path: P) -> Result<usize, Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_path(csv_path)?;
        let mut count = 0;

        for result in reader.deserialize() {
            let record: OpenCellIdRecord = result?;
            
            let key = CellKey {
                radio: record.radio.clone(),
                mcc: record.mcc,
                mnc: record.net,
                area: record.area,
                cell: record.cell,
            };

            self.cells.insert(key, record);
            count += 1;
        }

        Ok(count)
    }

    /// Look up cell information by identifiers
    pub fn lookup_cell(&self, radio: &str, mcc: u16, mnc: u16, area: u32, cell: u64) -> Option<&OpenCellIdRecord> {
        if !self.loaded {
            return None;
        }

        let key = CellKey {
            radio: radio.to_string(),
            mcc,
            mnc,
            area,
            cell,
        };

        self.cells.get(&key)
    }

    /// Get statistics about the loaded database
    pub fn get_stats(&self) -> CellDatabaseStats {
        let mut by_radio = HashMap::new();
        for record in self.cells.values() {
            *by_radio.entry(record.radio.clone()).or_insert(0) += 1;
        }

        CellDatabaseStats {
            total_cells: self.cells.len(),
            by_radio,
            loaded: self.loaded,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CellDatabaseStats {
    pub total_cells: usize,
    pub by_radio: HashMap<String, usize>,
    pub loaded: bool,
}

/// Analyzer for extracting cellular network information with local database lookup
pub struct CellularNetworkAnalyzer {
    // Cellular info extractor for parsing QMDL messages
    extractor: CellularInfoExtractor,
    
    // Local cell database
    cell_db: CellDatabase,
    
    // Track current serving cell information
    current_serving_cell: Option<CellularNetworkInfo>,
    
    // Track neighbor cells
    #[allow(dead_code)]
    neighbor_cells: HashMap<u16, NeighborCellInfo>, // keyed by physical_cell_id
    
    // Track PLMN information
    #[allow(dead_code)]
    available_plmns: Vec<PlmnInfo>,
    
    // Track location information
    #[allow(dead_code)]
    current_location: Option<LocationInfo>,
    
    // Counter for information events
    info_count: usize,
    
    // Path to CSV directory (configurable)
    csv_directory: Option<String>,
}

impl Default for CellularNetworkAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CellularNetworkAnalyzer {
    pub fn new() -> Self {
        // Optimized for resource-constrained devices
        // No OpenCellID database loading - saves memory
        // All data collection for offline post-processing
        Self {
            extractor: CellularInfoExtractor::new(),
            cell_db: CellDatabase::new(),  // Empty database to save memory
            current_serving_cell: None,
            neighbor_cells: HashMap::new(),
            available_plmns: Vec::new(),
            current_location: None,
            info_count: 0,
            csv_directory: Some("/data/rayhunter/captures".to_string()), // Save for offline analysis
        }
    }

    /// Set the directory containing OpenCellID CSV files
    pub fn set_csv_directory<P: AsRef<Path>>(&mut self, csv_dir: P) -> Result<(), Box<dyn std::error::Error>> {
        let dir_str = csv_dir.as_ref().to_string_lossy().to_string();
        self.csv_directory = Some(dir_str.clone());
        
        match self.cell_db.load_from_directory(csv_dir) {
            Ok(count) => {
                log::info!("Loaded {} cells from OpenCellID CSV files in {}", count, dir_str);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to load OpenCellID CSV files from {}: {}", dir_str, e);
                Err(e)
            }
        }
    }

    /// Process extracted cellular information and enrich with database data
    fn process_cellular_info(&mut self, cellular_info: &CellularNetworkInfo) -> Option<Event> {
        // Update current serving cell
        self.current_serving_cell = Some(cellular_info.clone());
        
        // Extract identifiers for database lookup
        let mut enriched_info = cellular_info.clone();
        
        if let (Some(plmn), Some(cell_info)) = (&cellular_info.plmn_info, &cellular_info.cell_info) {
            // Determine radio type
            let radio = match cellular_info.rat {
                RadioAccessTechnology::GSM => "GSM",
                RadioAccessTechnology::UMTS => "UMTS", 
                RadioAccessTechnology::LTE => "LTE",
                RadioAccessTechnology::NR => "NR",
                _ => return None, // Skip unknown radio types
            };

            // Look up cell in database
            if let Some(db_record) = self.cell_db.lookup_cell(
                radio,
                plmn.mcc.unwrap_or(0),
                plmn.mnc.unwrap_or(0),
                cellular_info.location_info.as_ref().map(|l| l.tac.unwrap_or(l.lac.unwrap_or(0)) as u32).unwrap_or(0),
                cell_info.global_cell_id.unwrap_or(cell_info.cell_identity.unwrap_or(0)) as u64
            ) {
                // Enrich with database information
                if let (Some(_lat), Some(_lon)) = (db_record.lat, db_record.lon) {
                    // Note: LocationInfo doesn't have lat/lon fields, but we can report it in the event
                    if enriched_info.location_info.is_none() {
                        enriched_info.location_info = Some(LocationInfo {
                            tac: Some(db_record.area as u16),
                            lac: Some(db_record.area as u16),
                            rac: None,
                            tracking_area_id: Some(db_record.area),
                        });
                    }
                }

                // Add signal information from database if available
                if let Some(avg_signal) = db_record.average_signal {
                    if enriched_info.signal_info.is_none() {
                        enriched_info.signal_info = Some(SignalInfo {
                            rssi: Some(avg_signal),
                            rsrp: None,
                            rsrq: None,
                            sinr: None,
                            cqi: None,
                            bandwidth: None,
                        });
                    } else if let Some(ref mut signal) = enriched_info.signal_info {
                        // Only update if we don't have live signal data
                        if signal.rssi.is_none() {
                            signal.rssi = Some(avg_signal);
                        }
                    }
                }

                self.info_count += 1;

                return Some(Event {
                    event_type: EventType::Informational,
                    message: format!(
                        "Cell identified: {} MCC:{} MNC:{} CellID:{} {}{}{}",
                        radio,
                        plmn.mcc.unwrap_or(0),
                        plmn.mnc.unwrap_or(0),
                        cell_info.global_cell_id.unwrap_or(cell_info.cell_identity.unwrap_or(0)),
                        if let (Some(lat), Some(lon)) = (db_record.lat, db_record.lon) {
                            format!(" Location:{:.4},{:.4}", lat, lon)
                        } else {
                            String::new()
                        },
                        if let Some(ref signal) = enriched_info.signal_info {
                            if let Some(rssi) = signal.rssi {
                                format!(" RSSI:{}dBm", rssi)
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        },
                        if let Some(range) = db_record.range {
                            format!(" Range:{}m", range)
                        } else {
                            String::new()
                        }
                    ),
                });
            }
        }

        // Even without database match, report the basic cellular info
        if let Some(plmn) = &cellular_info.plmn_info {
            self.info_count += 1;
            return Some(Event {
                event_type: EventType::Informational,
                message: format!(
                    "Cellular network detected: MCC:{} MNC:{} RAT:{:?}{}",
                    plmn.mcc.unwrap_or(0),
                    plmn.mnc.unwrap_or(0),
                    cellular_info.rat,
                    if let Some(cell_info) = &cellular_info.cell_info {
                        format!(" CellID:{}", cell_info.global_cell_id.unwrap_or(cell_info.cell_identity.unwrap_or(0)))
                    } else {
                        String::new()
                    }
                ),
            });
        }

        None
    }

    /// Get current database statistics
    pub fn get_database_stats(&self) -> CellDatabaseStats {
        self.cell_db.get_stats()
    }
}

impl QmdlAnalyzer for CellularNetworkAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("Cellular Network Information")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from(
            "Extracts and tracks cellular network parameters including MCC/MNC, Cell ID, \
            TAC/LAC, and neighbor cell information from QMDL messages. Enriches data with \
            location and coverage information from local OpenCellID CSV files."
        )
    }

    fn analyze_qmdl_message(&mut self, qmdl_message: &crate::diag::Message) -> Option<Event> {
        // Use the enhanced parser to extract both GSMTAP and cellular info
        if let Ok(Some((_timestamp, _gsmtap_msg, cellular_info))) = crate::gsmtap_parser::parse_with_cellular_info(qmdl_message.clone(), &mut self.extractor) {
            if let Some(cellular_info) = cellular_info {
                self.process_cellular_info(&cellular_info)
            } else {
                None
            }
        } else {
            None
        }
    }
}
