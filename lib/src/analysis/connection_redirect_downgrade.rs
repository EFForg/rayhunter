use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use telcom_parser::lte_rrc::{
    DL_DCCH_MessageType, DL_DCCH_MessageType_c1, RRCConnectionReleaseCriticalExtensions,
    RRCConnectionReleaseCriticalExtensions_c1, RedirectedCarrierInfo,
};

// Based on HITBSecConf presentation "Forcing a targeted LTE cellphone into an
// eavesdropping network" by Lin Huang
pub struct ConnectionRedirect2GDowngradeAnalyzer {}

// TODO: keep track of SIB state to compare LTE reselection blocks w/ 2g/3g ones
impl Analyzer for ConnectionRedirect2GDowngradeAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Connection Release/Redirected Carrier 2G Downgrade")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from("Tests if a cell releases our connection and redirects us to a 2G cell.")
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<Event> {
        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::DlDcch(msg_cont) = &**lte_ie
            && let DL_DCCH_MessageType::C1(c1) = &msg_cont.message
            && let DL_DCCH_MessageType_c1::RrcConnectionRelease(release) = c1
            && let RRCConnectionReleaseCriticalExtensions::C1(c1) = &release.critical_extensions
            && let RRCConnectionReleaseCriticalExtensions_c1::RrcConnectionRelease_r8(r8_ies) = c1
            && let Some(carrier_info) = &r8_ies.redirected_carrier_info
        {
            match carrier_info {
                RedirectedCarrierInfo::Geran(_carrier_freqs_geran) => Some(Event {
                    event_type: EventType::High,
                    message: "Detected 2G downgrade".to_owned(),
                }),
                _ => Some(Event {
                    event_type: EventType::Informational,
                    message: format!("RRCConnectionRelease CarrierInfo: {carrier_info:?}"),
                }),
            }
        } else {
            None
        }
    }
}
