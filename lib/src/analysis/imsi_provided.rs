use std::borrow::Cow;

use telcom_parser::lte_rrc::{PCCH_MessageType, PCCH_MessageType_c1, PagingUE_Identity};

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};

pub struct ImsiProvidedAnalyzer {
}

impl Analyzer for ImsiProvidedAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("IMSI Provided")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Tests whether the UE's IMSI was ever provided to the cell")
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        let pcch_msg = match ie {
            InformationElement::LTE(lte_ie) => match &** lte_ie {
                LteInformationElement::PCCH(pcch_msg) => pcch_msg,
                _ => return None,
            }
            _ => return None,
        };
        let PCCH_MessageType::C1(PCCH_MessageType_c1::Paging(paging)) = &pcch_msg.message else {
            return None;
        };
        for record in &paging.paging_record_list.as_ref()?.0 {
            if let PagingUE_Identity::Imsi(_) = record.ue_identity {
                return Some(Event {
                    event_type: EventType::QualitativeWarning { severity: Severity::High },
                    message: "IMSI was provided to cell".to_string(),
                })
            }
        }
        None
    }
}
