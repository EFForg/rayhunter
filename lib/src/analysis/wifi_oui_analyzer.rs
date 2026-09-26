use chrono::{DateTime, FixedOffset};
use log::{debug, info};

use crate::analysis::{
    analyzer::{Analyzer, AnalyzerMetadata, Event, EventType},
    information_element::InformationElement,
};

pub struct WifiOUIAnalyzer {
    wifi_ouis: Vec<String>,
}

impl WifiOUIAnalyzer {
    pub fn new(wifi_ouis: &[String]) -> Self {
        Self {
            wifi_ouis: wifi_ouis.to_owned(),
        }
    }
}

impl Analyzer for WifiOUIAnalyzer {
    fn metadata() -> AnalyzerMetadata {
        AnalyzerMetadata {
            key: "connection_redirect_2g_downgrade".into(),
            default_enabled: true,
            name: "WifiOUIAnalyzer".into(),
            description: "Scans wifi channels looking for OUIs of known IMSI catchers".into(),
            version: 1,
        }
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
        _timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        if let InformationElement::WifiNetwork(bssid) = ie {
            debug!("WifiOUIAnalyzer got BSSIDs {:?}", bssid);
            if !self.wifi_ouis.is_empty()
                && self
                    .wifi_ouis
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

        None
    }
}
