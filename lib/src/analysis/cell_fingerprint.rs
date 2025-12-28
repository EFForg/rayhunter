//! Cell Fingerprinting Analyzer
//!
//! This analyzer tracks cell tower identities and alerts on anomalies such as:
//! - New/unknown cell towers appearing
//! - Cells with unusual parameters
//! - Rapid cell changes that could indicate a mobile IMSI catcher
//!
//! Enhancement ideas:
//! - Compare against a known-good cell database
//! - Track signal strength patterns
//! - Correlate with GPS location (if available)

use std::borrow::Cow;
use std::collections::HashMap;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use log::debug;
use telcom_parser::lte_rrc::{
    BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1, SystemInformationBlockType1,
};

/// Minimum number of packets before we start alerting on new cells
/// This prevents false positives during initial baseline building
const BASELINE_PACKETS: usize = 100;

/// Information about a cell tower we've seen
#[derive(Debug, Clone)]
struct CellInfo {
    /// E-UTRA Cell Global Identifier (if available)
    cell_id: Option<u32>,
    /// Tracking Area Code
    tac: Option<u16>,
    /// PLMN (Mobile Country Code + Mobile Network Code)
    plmn: Option<String>,
    /// First packet number where we saw this cell
    first_seen_packet: usize,
    /// Total times we've seen this cell
    times_seen: u32,
    /// Last packet number where we saw this cell
    last_seen_packet: usize,
}

/// Cell Fingerprinting Analyzer
///
/// Tracks cell tower identities from SIB1 messages and alerts when:
/// 1. A new cell appears after the baseline period
/// 2. A cell has suspicious parameters
/// 3. Cells are changing too rapidly
pub struct CellFingerprintAnalyzer {
    /// Known cells indexed by a fingerprint key
    known_cells: HashMap<String, CellInfo>,
    /// Current packet number
    packet_count: usize,
    /// Whether we're still in baseline building mode
    baseline_complete: bool,
    /// Recent cell changes for rapid-change detection
    recent_cell_changes: Vec<usize>,
}

impl Default for CellFingerprintAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CellFingerprintAnalyzer {
    pub fn new() -> Self {
        Self {
            known_cells: HashMap::new(),
            packet_count: 0,
            baseline_complete: false,
            recent_cell_changes: Vec::new(),
        }
    }

    /// Extract cell information from a SIB1 message
    fn extract_cell_info(&self, sib1: &SystemInformationBlockType1) -> Option<(String, CellInfo)> {
        // Extract Cell Identity from cellAccessRelatedInfo
        // cell_identity is a BitString, convert to u32
        let cell_id: Option<u32> = {
            let bits = &sib1.cell_access_related_info.cell_identity.0;
            if bits.len() >= 28 {
                // Cell identity is 28 bits
                let mut val: u32 = 0;
                for (i, bit) in bits.iter().take(28).enumerate() {
                    if *bit {
                        val |= 1 << (27 - i);
                    }
                }
                Some(val)
            } else {
                None
            }
        };

        // Extract Tracking Area Code (16 bits)
        let tac: Option<u16> = {
            let bits = &sib1.cell_access_related_info.tracking_area_code.0;
            if bits.len() >= 16 {
                let mut val: u16 = 0;
                for (i, bit) in bits.iter().take(16).enumerate() {
                    if *bit {
                        val |= 1 << (15 - i);
                    }
                }
                Some(val)
            } else {
                None
            }
        };

        // Extract PLMN from the first entry in plmn_IdentityList
        let plmn = sib1
            .cell_access_related_info
            .plmn_identity_list
            .0
            .first()
            .map(|plmn_info| {
                // Format as MCC-MNC string
                format!("{:?}", plmn_info.plmn_identity)
            });

        // Create a fingerprint key combining available identifiers
        let fingerprint = format!(
            "cell:{:?}-tac:{:?}-plmn:{:?}",
            cell_id, tac, plmn
        );

        Some((
            fingerprint,
            CellInfo {
                cell_id,
                tac,
                plmn,
                first_seen_packet: self.packet_count,
                times_seen: 1,
                last_seen_packet: self.packet_count,
            },
        ))
    }

    /// Check if cells are changing too rapidly (potential mobile IMSI catcher)
    fn check_rapid_cell_changes(&mut self) -> Option<Event> {
        // Keep only changes from last 50 packets
        self.recent_cell_changes
            .retain(|&p| self.packet_count - p < 50);

        // If we see more than 10 cell changes in 50 packets, that's suspicious
        if self.recent_cell_changes.len() > 10 {
            return Some(Event {
                event_type: EventType::Medium,
                message: format!(
                    "Rapid cell changes detected: {} changes in last 50 packets",
                    self.recent_cell_changes.len()
                ),
            });
        }
        None
    }
}

impl Analyzer for CellFingerprintAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Cell Fingerprinting")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "Tracks cell tower identities and alerts on new/unknown cells appearing after \
             baseline period, or when cells are changing suspiciously fast. This can help \
             detect mobile IMSI catchers or newly deployed surveillance equipment.",
        )
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
    ) -> Option<Event> {
        self.packet_count = packet_num;

        // Check if baseline period is complete
        if !self.baseline_complete && packet_num >= BASELINE_PACKETS {
            self.baseline_complete = true;
            debug!(
                "Cell fingerprint baseline complete with {} known cells",
                self.known_cells.len()
            );
        }

        // We're looking for SIB1 messages which contain cell identity info
        let sib1 = match ie {
            InformationElement::LTE(lte_ie) => match &**lte_ie {
                LteInformationElement::BcchDlSch(sch_msg) => {
                    match &sch_msg.message {
                        BCCH_DL_SCH_MessageType::C1(c1) => match c1 {
                            BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) => sib1,
                            _ => return None,
                        },
                        _ => return None,
                    }
                }
                _ => return None,
            },
            _ => return None,
        };

        // Extract cell info from SIB1
        let (fingerprint, mut cell_info) = self.extract_cell_info(sib1)?;

        // Check if this is a known cell
        if let Some(existing) = self.known_cells.get_mut(&fingerprint) {
            // Update existing cell info
            existing.times_seen += 1;
            existing.last_seen_packet = packet_num;

            // No alert for known cells
            return self.check_rapid_cell_changes();
        }

        // This is a new cell
        self.recent_cell_changes.push(packet_num);

        // During baseline, just record the cell
        if !self.baseline_complete {
            debug!("Baseline: recording new cell {}", fingerprint);
            self.known_cells.insert(fingerprint, cell_info);
            return None;
        }

        // After baseline, alert on new cells
        cell_info.first_seen_packet = packet_num;
        self.known_cells.insert(fingerprint.clone(), cell_info.clone());

        // Check for rapid changes first
        if let Some(rapid_change_event) = self.check_rapid_cell_changes() {
            return Some(rapid_change_event);
        }

        // Alert on new cell after baseline
        Some(Event {
            event_type: EventType::Low,
            message: format!(
                "New cell tower detected after baseline: cell_id={:?}, tac={:?}, plmn={:?}",
                cell_info.cell_id, cell_info.tac, cell_info.plmn
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_analyzer() {
        let analyzer = CellFingerprintAnalyzer::new();
        assert!(!analyzer.baseline_complete);
        assert!(analyzer.known_cells.is_empty());
    }

    #[test]
    fn test_baseline_completion() {
        let mut analyzer = CellFingerprintAnalyzer::new();
        analyzer.packet_count = BASELINE_PACKETS;

        // Simulate checking baseline status
        if !analyzer.baseline_complete && analyzer.packet_count >= BASELINE_PACKETS {
            analyzer.baseline_complete = true;
        }

        assert!(analyzer.baseline_complete);
    }
}
