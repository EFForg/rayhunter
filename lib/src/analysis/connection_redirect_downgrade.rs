use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};
use telcom_parser::lte_rrc::{DL_DCCH_MessageType, DL_DCCH_MessageType_c1, RRCConnectionReleaseCriticalExtensions, RRCConnectionReleaseCriticalExtensions_c1, RedirectedCarrierInfo};
use super::util::unpack;

// Based on HITBSecConf presentation "Forcing a targeted LTE cellphone into an
// eavesdropping network" by Lin Huang
pub struct ConnectionRedirect2GDowngradeAnalyzer {
}

// TODO: keep track of SIB state to compare LTE reselection blocks w/ 2g/3g ones
impl Analyzer for ConnectionRedirect2GDowngradeAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("Connection Release/Redirected Carrier 2G Downgrade")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from("Tests if a cell releases our connection and redirects us to a 2G cell.")
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        unpack!(InformationElement::LTE(lte_ie) = ie);
        let message = match &**lte_ie {
            LteInformationElement::DlDcch(msg_cont) => &msg_cont.message,
            _ => return None,
        };
        unpack!(DL_DCCH_MessageType::C1(c1) = message);
        unpack!(DL_DCCH_MessageType_c1::RrcConnectionRelease(release) = c1);
        unpack!(RRCConnectionReleaseCriticalExtensions::C1(c1) = &release.critical_extensions);
        unpack!(RRCConnectionReleaseCriticalExtensions_c1::RrcConnectionRelease_r8(r8_ies) = c1);
        unpack!(Some(carrier_info) = &r8_ies.redirected_carrier_info);
        match carrier_info {
            RedirectedCarrierInfo::Geran(_carrier_freqs_geran) => Some(Event {
                event_type: EventType::QualitativeWarning { severity: Severity::High },
                message: "Detected 2G downgrade".to_owned(),
            }),
            _ => Some(Event {
                event_type: EventType::Informational,
                message: format!("RRCConnectionRelease CarrierInfo: {:?}", carrier_info),
            }),
        }
    }
}
