use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, GsmInformationElement};
use crate::gsm::layer3::{ProtocolDiscrimiminatedMessage};
use crate::gsm::radio_resource_management::{RadioResourceManagementMessage};

pub struct GsmCellReselectionHysteresisAnalyzer {}


impl Analyzer for GsmCellReselectionHysteresisAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("GSM Reselection Hysteresis")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "The GSM Reselection Hysteresis",
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
            && let GsmInformationElement::Ccch(l3_frame) = &**gsm_ie
            && let ProtocolDiscrimiminatedMessage::RadioResourceManagement(
                RadioResourceManagementMessage::SystemInformationType3(si3),
            ) = &l3_frame.protocol_discriminated_messages
        {
            let hysteresis = si3.cell_selection_params.cell_resel_hysteresis * 2;
            let event_type = match hysteresis {
                0 ..= 6 => EventType::Informational,
                7 ..= 9 => EventType::Medium,
                10 .. => EventType::High,
            };
            return Some(Event {
                event_type: event_type,
                message: format!("Cell Reselection Hysteresis: {}", hysteresis),
            });
        };
        None
    }
}
