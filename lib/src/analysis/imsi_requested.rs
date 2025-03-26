use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};

const PACKET_THRESHHOLD: usize = 150;

pub struct ImsiRequestedAnalyzer {
    packet_num: usize,
}

impl Default for ImsiRequestedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsiRequestedAnalyzer {
    pub fn new() -> Self {
        Self { packet_num: 0 }
    }
}

impl Analyzer for ImsiRequestedAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("IMSI Requested")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Tests whether the ME sends an IMSI Identity Request NAS message")
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;
        let payload = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::NAS(payload) => payload,
                _ => return None,
            }
            _ => return None,
        };

        // NAS identity request, ID type IMSI
        if payload == &[0x07, 0x55, 0x01] {
            if self.packet_num < PACKET_THRESHHOLD {
                return Some(Event {
                    event_type: EventType::QualitativeWarning {
                        severity: Severity::Medium
                    },
                    message: format!(
                        "NAS IMSI identity request detected, however it was within \
                        the first {} packets of this analysis. If you just \
                        turned your device on, this is likely a \
                        false-positive.",
                        PACKET_THRESHHOLD
                    )
                })
            } else {
                return Some(Event {
                    event_type: EventType::QualitativeWarning {
                        severity: Severity::High
                    },
                    message: "NAS IMSI identity request detected".to_owned(),
                })
            }
        }
        None
    }
}
