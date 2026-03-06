use std::borrow::Cow;
use log::debug;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, GsmInformationElement};
use crate::gsm::grps_mobility_management::GPRSMobilityManagementMessage;
use crate::gsm::layer3::{ProtocolDiscrimiminatedMessage};
use crate::gsm::radio_resource_management::{RadioResourceManagementMessage};
use crate::gsm::information_elements::{CipherModeSetting, CipheringAlgorithm};

pub struct GsmCipheringModeAnalyzer {}


impl Analyzer for GsmCipheringModeAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("GSM Ciphering Mode")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "GSM Ciphering Mode",
        )
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<super::analyzer::Event> {
        if let InformationElement::GSM(gsm_ie) = ie
            && let GsmInformationElement::DTAP(l3_frame) = &**gsm_ie
            && let ProtocolDiscrimiminatedMessage::RadioResourceManagement(
                RadioResourceManagementMessage::CipheringModeCommand(ciphering_mode_command),
            ) = &l3_frame.protocol_discriminated_messages
        {
            debug!("CipherModeSetting");
            let event_type = match ciphering_mode_command.cipher_mode_setting
            {
                CipherModeSetting::A5_6 | CipherModeSetting::A5_7 => EventType::Informational,
                CipherModeSetting::A5_4 | CipherModeSetting::A5_5 => EventType::Low,
                CipherModeSetting::A5_3 => EventType::Medium,
                CipherModeSetting::NoCiphering| CipherModeSetting::Reserved | CipherModeSetting::A5_1 | CipherModeSetting::A5_2 => EventType::High,
            };
            return Some(Event {
                event_type: event_type,
                message: format!("GSM Ciphering Mode Setting: {:?}", ciphering_mode_command.cipher_mode_setting),
            });
        };
        if let InformationElement::GSM(gsm_ie) = ie
            && let GsmInformationElement::DTAP(l3_frame) = &**gsm_ie
            && let ProtocolDiscrimiminatedMessage::GPRSMobilityManagement(
                GPRSMobilityManagementMessage::AuthenticationAndCipheringRequest(authentication_and_ciphering_request),
            ) = &l3_frame.protocol_discriminated_messages
        {
            let event_type = match authentication_and_ciphering_request.ciphering_algorithm
            {
                CipheringAlgorithm::GEA6 | CipheringAlgorithm::GEA7  => EventType::Informational,
                CipheringAlgorithm::GEA5 => EventType::Low,
                CipheringAlgorithm::GEA3 | CipheringAlgorithm::GEA4  => EventType::Medium,
                CipheringAlgorithm::CipheringNotUsed | CipheringAlgorithm::GEA1 | CipheringAlgorithm::GEA2  => EventType::High,
            };
            return Some(Event {
                event_type: event_type,
                message: format!("GPRS Ciphering Algorithm: {:?}", authentication_and_ciphering_request.ciphering_algorithm),
            });
        };
        None
    }
}
