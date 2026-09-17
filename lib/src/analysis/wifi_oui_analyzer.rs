use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use log::{debug, info};

use crate::analysis::{
    analyzer::{Analyzer, Event, EventType},
    information_element::InformationElement,
};

pub struct WifiOUIAnalyzer {
    wifi_ouis: Option<Vec<String>>,
}

impl WifiOUIAnalyzer {
    pub fn new(wifi_ouis: Option<Vec<String>>) -> Self {
        Self { wifi_ouis }
    }
}

impl Analyzer for WifiOUIAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        "WifiOUIAnalyzer".into()
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from("Scans wifi channels looking for OUIs of known IMSI catchers")
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
        _timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        if let InformationElement::WifiNetwork(bssid) = ie {
            debug!("WifiOUIAnalyzer got BSSIDs {:?}", bssid);
            if let Some(ouis) = &self.wifi_ouis {
                if !ouis.is_empty() {
                    if ouis
                        .iter()
                        .find(|oui| bssid.to_uppercase().starts_with(&oui.to_uppercase()))
                        .is_some()
                    {
                        info!("Found match for bssid {bssid}");
                        return Some(Event {
                            event_type: EventType::Informational,
                            message: format!("Found suspicious wifi network {bssid}"),
                        });
                    }
                }
            }
        }

        None
    }
}
