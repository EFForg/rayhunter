use std::borrow::Cow;

use pycrate_rs::nas::emm::EMMMessage;
use pycrate_rs::nas::generated::emm::emm_security_mode_command::NASSecAlgoCiphAlgo::EPSEncryptionAlgorithmEEA0Null;
use pycrate_rs::nas::NASMessage;

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};

pub struct NasNullCipherAnalyzer {
    packet_num: usize,
}

impl Default for NasNullCipherAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl NasNullCipherAnalyzer {
    pub fn new() -> Self {
        Self { packet_num: 0 }
    }
}

impl Analyzer for NasNullCipherAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("NAS Null Cipher Requested")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Tests whether the MME requests to use a null cipher in the security mode command")
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;
        let payload = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::NAS(payload) => payload,
                _ => return None,
            },
            _ => return None,
        };

        if let NASMessage::EMMMessage(EMMMessage::EMMSecurityModeCommand(req)) = payload {
            if req.nas_sec_algo.inner.ciph_algo == EPSEncryptionAlgorithmEEA0Null {
                    return Some(Event {
                        event_type: EventType::QualitativeWarning {
                            severity: Severity::High,
                        },
                        message: format!(
                            "NAS Security mode command requested null cipher(packet {})",
                            self.packet_num
                        ),
                    });
            }
        }
        None
    }
}
