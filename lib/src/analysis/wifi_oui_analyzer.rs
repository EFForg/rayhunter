use log::debug;

use crate::analysis::{
    analyzer::{Analyzer, Event, EventType},
    information_element::InformationElement,
};

pub struct WifiOUIAnalyzer {
    wifi_ouis: Vec<String>,
}

impl WifiOUIAnalyzer {
    pub fn new(ouis: &[String]) -> Self {
        Self {
            wifi_ouis: ouis.to_vec(),
        }
    }
}

impl Analyzer for WifiOUIAnalyzer {
    fn get_name(&self) -> std::borrow::Cow<'_, str> {
        "WifiOUIAnalyzer".into()
    }

    fn get_description(&self) -> std::borrow::Cow<'_, str> {
        "blah blah blah".into()
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<Event> {
        if let InformationElement::WifiBSSIDList(bssids) = ie {
            debug!("WifiOUIAnalyzer got BSSIDs {:?}", bssids);
            if !self.wifi_ouis.is_empty() {
                for bssid in bssids {
                    if self
                        .wifi_ouis
                        .iter()
                        .find(|oui| bssid.to_uppercase().starts_with(&oui.to_uppercase()))
                        .is_some()
                    {
                        debug!("Found match for bssid {bssid}");
                        return Some(Event {
                            event_type: EventType::High,
                            message: "Detected possible IMSI catcher wifi endpoint".to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}
