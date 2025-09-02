use std::borrow::Cow;

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;
use pycrate_rs::nas::generated::emm::emm_security_mode_command::NASSecAlgoCiphAlgo::EPSEncryptionAlgorithmEEA0Null;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};

pub struct NasNullCipherAnalyzer {}

impl Analyzer for NasNullCipherAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("NAS Null Cipher Requested")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "Tests whether the MME requests to use a null cipher in the NAS security mode command",
        )
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<Event> {
        let payload = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::NAS(payload) => payload,
                _ => return None,
            },
            _ => return None,
        };

        if let NASMessage::EMMMessage(EMMMessage::EMMSecurityModeCommand(req)) = payload
            && req.nas_sec_algo.inner.ciph_algo == EPSEncryptionAlgorithmEEA0Null
        {
            return Some(Event {
                event_type: EventType::High,
                message: "NAS Security mode command requested null cipher".to_string(),
            });
        }
        None
    }
}
