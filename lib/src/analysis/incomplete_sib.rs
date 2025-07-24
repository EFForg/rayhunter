use std::borrow::Cow;

use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};

use super::analyzer::{Analyzer, Event, EventType, Severity};
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
    fn get_name(&self) -> Cow<str> {
        Cow::from("Incomplete SIB")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Tests whether a SIB1 message contains a full chain of followup sibs")
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;

        let sch_msg = match ie {
            InformationElement::LTE(lte_ie) => match &**lte_ie {
                LteInformationElement::BcchDlSch(sch_msg) => sch_msg,
                _ => return None,
            },
            _ => return None,
        };
        let BCCH_DL_SCH_MessageType::C1(BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(
            sib1,
        )) = &sch_msg.message
        else {
            return None;
        };

        if sib1.scheduling_info_list.0.len() < 2 {
            return Some(Event {
                event_type: EventType::QualitativeWarning {
                    severity: Severity::Medium,
                },
                message: format!(
                    "SIB1 scheduling info list was malformed (packet {})",
                    self.packet_num
                )
                .to_string(),
            });
        }
        None
    }
}
