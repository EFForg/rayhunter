use std::borrow::Cow;

use rayhunter::telcom_parser::lte_rrc::{PCCH_MessageType, PCCH_MessageType_c1, PagingUE_Identity};

use rayhunter::analysis::analyzer::{Analyzer, Event, EventType, Severity};
use rayhunter::analysis::information_element::{InformationElement, LteInformationElement};

pub struct TestAnalyzer{
    pub count: i32,
}

impl Analyzer for TestAnalyzer{
    fn get_name(&self) -> Cow<str> {
        Cow::from("Example Analyzer")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Always returns true, if you are seeing this you are either a developer or you are about to have problems.")
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.count += 1;
        if self.count % 100 == 0 {
            return Some(Event {
                event_type: EventType::Informational ,
                message: "multiple of 100 events processed".to_string(),
            })
        }
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
            if let PagingUE_Identity::S_TMSI(_) = record.ue_identity {
                return Some(Event {
                    event_type: EventType::QualitativeWarning { severity: Severity::Low },
                    message: "TMSI was provided to cell".to_string(),
                })
            }
        }
        None
    }
}
