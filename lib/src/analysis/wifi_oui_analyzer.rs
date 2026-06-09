use std::borrow::Cow;

use log::{debug, info, LevelFilter, Log};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config, Logger,
};

use crate::analysis::{
    analyzer::{Analyzer, Event, EventType},
    information_element::InformationElement,
};

pub struct WifiOUIAnalyzer {
    wifi_ouis: Vec<String>,
    logger: Box<dyn Log>,
}

impl WifiOUIAnalyzer {
    pub fn new(ouis: &[String]) -> Self {
        Self {
            wifi_ouis: ouis.to_vec(),
            logger: Box::new(Self::init_logger()),
        }
    }

    fn init_logger() -> impl Log {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("/data/rayhunter/wifi.log")
            .expect("Error creating FileAppender for wifi logs");

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))
            .expect("Error creating config for wifi logs Logger");

        Logger::new(config)
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
                        info!(logger: self.logger, "Found match for bssid {bssid}");
                        return Some(Event {
                            event_type: EventType::Informational,
                            message: "Detected possible IMSI catcher wifi endpoint".to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}
