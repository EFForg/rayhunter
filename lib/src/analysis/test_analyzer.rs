use std::borrow::Cow;

use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};

pub struct TestAnalyzer {
    packet_num: usize,
}

impl Default for TestAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TestAnalyzer {
    pub fn new() -> Self {
        Self { packet_num: 0 }
    }
}

impl Analyzer for TestAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Test Analyzer")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from("This is an analyzer which can be used to test that your rayhunter is working. It will generate an alert for every SIB1 message (a beacon from the cell tower) that it sees. Do not leave this on when you are hunting or it will be very noisy.")
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;

        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(sch_msg) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(c1) = &sch_msg.message
            && let BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) = c1
        {
            return Some(Event {
                event_type: EventType::QualitativeWarning {
                    severity: Severity::Low,
                },
                message: format!(
                    "SIB1 received (packet {}) CID: {}, PLMN: {:?}",
                    self.packet_num,
                    sib1.cell_access_related_info.cell_identity.0,
                    sib1.cell_access_related_info.plmn_identity_list.0
                ),
            });
        }
        None
    }
}
