use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, GsmInformationElement};
use crate::gsm::layer3::{ProtocolDiscrimiminatedMessage};
use crate::gsm::radio_resource_management::{RadioResourceManagementMessage};
use crate::gsm::information_elements::{OptionalSelectionParameters};

pub struct GsmCellReselectionOffsetAnalyzer {}

// See TS GSM 05.08 Section 6.4 Criteria for cell selection and reselection

impl Analyzer for GsmCellReselectionOffsetAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("GSM Reselection Offset")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "GSM Reselection Offset",
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
            if let OptionalSelectionParameters::Present(selection_parameters) = &si3.si3_rest_octets.optional_selection_parameters {
                let event_type = match selection_parameters.cell_reselect_offset {
                    0 => EventType::Informational,
                    1 .. => EventType::Medium,
                };
                return Some(Event {
                    event_type: event_type,
                    message: format!("Cell Reselection Offset: {}", selection_parameters.cell_reselect_offset),
                });
            };
        };
        None
    }
}
