use std::borrow::Cow;

use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};

pub struct IncompleteSibAnalyzer {
    packet_num: usize,
}

impl Default for IncompleteSibAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl IncompleteSibAnalyzer {
    pub fn new() -> Self {
        Self { packet_num: 0 }
    }
}

impl Analyzer for IncompleteSibAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Incomplete SIB")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from("Tests whether a SIB1 message contains a full chain of followup sibs")
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
            && sib1.scheduling_info_list.0.len() < 2
        {
            return Some(Event {
                event_type: EventType::Medium,
                message: format!(
                    "SIB1 scheduling info list was malformed (packet {})",
                    self.packet_num
                ),
            });
        }
        None
    }
}
