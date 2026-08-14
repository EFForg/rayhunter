use std::borrow::Cow;
use std::time::Duration;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};

const NAS_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// Warns if no NAS messages have been observed for [NAS_TIMEOUT], which usually
/// means the SIM card isn't working (or isn't inserted).
///
/// Only one warning is emitted per recording.
#[derive(Default)]
pub struct NoNasMessagesAnalyzer {
    /// Time since the last NAS message, or since the start of the recording if we
    /// haven't seen one yet.
    since_last_nas: Duration,
    warned: bool,
}

impl NoNasMessagesAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }
}

fn is_nas(ie: &InformationElement) -> bool {
    match ie {
        InformationElement::LTE(inner) => matches!(**inner, LteInformationElement::NAS(_)),
        _ => false,
    }
}

impl Analyzer for NoNasMessagesAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("No NAS Messages")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "Warns if no NAS messages have been seen for 5 minutes, which usually means the SIM card is not working and the recording is not usable for detecting IMSI catchers.",
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
        if is_nas(ie) {
            self.since_last_nas = Duration::ZERO;
        }
        None
    }

    fn poll(&mut self, elapsed: Duration) -> Option<Event> {
        if self.warned {
            return None;
        }
        self.since_last_nas += elapsed;
        if self.since_last_nas < NAS_TIMEOUT {
            return None;
        }
        self.warned = true;
        Some(Event {
            event_type: EventType::Low,
            message: "No NAS messages seen in 5 minutes, SIM possibly not working".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, LteNasSubtype, LteRrcSubtype};

    fn information_element(gsmtap_type: GsmtapType, payload: Vec<u8>) -> InformationElement {
        InformationElement::try_from(&GsmtapMessage {
            header: GsmtapHeader::new(gsmtap_type),
            payload,
        })
        .expect("failed to build test information element")
    }

    /// An EMM Identity Request, i.e. a NAS message.
    fn nas_information_element() -> InformationElement {
        let ie = information_element(
            GsmtapType::LteNas(LteNasSubtype::Plain),
            vec![0x07, 0x55, 0x01],
        );
        assert!(is_nas(&ie));
        ie
    }

    /// An LTE RRC UL-CCCH RRCConnectionRequest, i.e. not a NAS message.
    fn rrc_information_element() -> InformationElement {
        let ie = information_element(
            GsmtapType::LteRrc(LteRrcSubtype::UlCcch),
            vec![0x40, 0x0c, 0x8e, 0xc9, 0x42, 0x89, 0xe0],
        );
        assert!(!is_nas(&ie));
        ie
    }

    const MINUTE: Duration = Duration::from_secs(60);

    #[test]
    fn test_warns_once_after_timeout() {
        let mut analyzer = NoNasMessagesAnalyzer::new();

        for _ in 0..4 {
            assert!(analyzer.poll(MINUTE).is_none());
        }

        let event = analyzer.poll(MINUTE).expect("expected a warning");
        assert_eq!(event.event_type, EventType::Low);
        assert_eq!(
            event.message,
            "No NAS messages seen in 5 minutes, SIM possibly not working"
        );

        // ...but only once
        assert!(analyzer.poll(MINUTE).is_none());
    }

    /// The whole point of polling: we warn even if the modem never produces a single
    /// message we can decode.
    #[test]
    fn test_warns_without_any_packets() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        assert!(analyzer.poll(NAS_TIMEOUT).is_some());
    }

    #[test]
    fn test_nas_messages_reset_the_timer() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        let nas = nas_information_element();

        // keep resetting the timer just before it expires
        for _ in 0..3 {
            assert!(analyzer.poll(NAS_TIMEOUT - MINUTE).is_none());
            assert!(analyzer.analyze_information_element(&nas, 1).is_none());
        }

        // ...but once they stop, we warn
        assert!(analyzer.poll(NAS_TIMEOUT).is_some());
    }

    #[test]
    fn test_non_nas_messages_do_not_reset_the_timer() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        let rrc = rrc_information_element();

        assert!(analyzer.poll(NAS_TIMEOUT - MINUTE).is_none());
        assert!(analyzer.analyze_information_element(&rrc, 1).is_none());
        assert!(analyzer.poll(MINUTE).is_some());
    }
}
