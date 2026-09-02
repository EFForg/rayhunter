use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};

const NAS_TIMEOUT: chrono::TimeDelta = chrono::TimeDelta::minutes(5);

#[derive(Default)]
pub struct NoNasMessagesAnalyzer {
    first_timestamp: Option<DateTime<FixedOffset>>,
    latest_timestamp: Option<DateTime<FixedOffset>>,
    finished: bool,
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
            "Warns if a recording contains diagnostic traffic spanning 5 minutes but no NAS messages, which usually means the SIM card is not working and the recording is not usable for detecting IMSI catchers.",
        )
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
        timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        if is_nas(ie) {
            self.finished = true;
        }
        self.report_skipped_packet(timestamp)
    }

    fn report_skipped_packet(&mut self, timestamp: DateTime<FixedOffset>) -> Option<Event> {
        if self.finished {
            return None;
        }
        // A jump backwards, or a single forward jump as large as the timeout
        // itself, indicates a clock discontinuity (e.g. a NITZ/GPS resync)
        // rather than real elapsed monitoring time. Restart the window
        // instead of letting it count toward (or immediately satisfy) the
        // timeout, since we only want to warn once 5 real minutes of
        // diagnostic traffic have gone by without a NAS message.
        let is_discontinuous = self.latest_timestamp.is_some_and(|latest_timestamp| {
            timestamp < latest_timestamp || timestamp - latest_timestamp >= NAS_TIMEOUT
        });
        if is_discontinuous {
            self.first_timestamp = Some(timestamp);
        }
        self.latest_timestamp = Some(timestamp);
        let first_timestamp = *self.first_timestamp.get_or_insert(timestamp);

        if timestamp - first_timestamp < NAS_TIMEOUT {
            return None;
        }

        self.finished = true;
        Some(Event {
            event_type: EventType::Low,
            message: "No NAS messages seen in 5 minutes, SIM possibly not working".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, LteNasSubtype};

    fn packet_time(seconds: i64) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2025-01-01T00:00:00+00:00").unwrap()
            + chrono::TimeDelta::seconds(seconds)
    }

    fn nas_information_element() -> InformationElement {
        let ie = InformationElement::try_from(&GsmtapMessage {
            header: GsmtapHeader::new(GsmtapType::LteNas(LteNasSubtype::Plain)),
            payload: vec![0x07, 0x55, 0x01],
        })
        .expect("failed to build test information element");
        assert!(is_nas(&ie));
        ie
    }

    #[test]
    fn test_warns_once_after_timeout() {
        let mut analyzer = NoNasMessagesAnalyzer::new();

        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(299)).is_none());
        let event = analyzer
            .report_skipped_packet(packet_time(300))
            .expect("expected a warning");
        assert_eq!(event.event_type, EventType::Low);
        assert!(analyzer.report_skipped_packet(packet_time(600)).is_none());
    }

    #[test]
    fn test_nas_message_disables_analyzer() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        let nas = nas_information_element();

        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        assert!(
            analyzer
                .analyze_information_element(&nas, 1, packet_time(0))
                .is_none()
        );
        assert!(analyzer.report_skipped_packet(packet_time(600)).is_none());
    }

    #[test]
    fn test_backwards_timestamp_restarts_window() {
        let mut analyzer = NoNasMessagesAnalyzer::new();

        assert!(analyzer.report_skipped_packet(packet_time(200)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(299)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(300)).is_some());
    }

    #[test]
    fn test_forward_jump_restarts_window() {
        let mut analyzer = NoNasMessagesAnalyzer::new();

        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        // A single large forward jump (e.g. a clock resync) shouldn't by
        // itself be treated as 5 minutes of monitored diagnostic traffic.
        assert!(
            analyzer
                .report_skipped_packet(packet_time(10_000))
                .is_none()
        );
        assert!(
            analyzer
                .report_skipped_packet(packet_time(10_000 + 299))
                .is_none()
        );
        assert!(
            analyzer
                .report_skipped_packet(packet_time(10_000 + 300))
                .is_some()
        );
    }

    #[test]
    fn test_small_forward_gap_does_not_restart_window() {
        let mut analyzer = NoNasMessagesAnalyzer::new();

        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(299)).is_none());
        // A sub-timeout gap between packets is normal and should still
        // count toward the window.
        assert!(
            analyzer
                .report_skipped_packet(packet_time(299 + 250))
                .is_some()
        );
    }

    #[test]
    fn test_parsed_nas_with_timestamp_suppresses_warning() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        let nas = nas_information_element();

        assert!(analyzer.report_skipped_packet(packet_time(0)).is_none());
        assert!(analyzer.report_skipped_packet(packet_time(299)).is_none());
        assert!(
            analyzer
                .analyze_information_element(&nas, 3, packet_time(300))
                .is_none()
        );
        assert!(analyzer.report_skipped_packet(packet_time(600)).is_none());
    }

    #[test]
    fn test_parsed_non_nas_element_advances_clock_and_warns() {
        let mut analyzer = NoNasMessagesAnalyzer::new();
        let non_nas = InformationElement::GSM;

        assert!(
            analyzer
                .analyze_information_element(&non_nas, 1, packet_time(0))
                .is_none()
        );
        assert!(
            analyzer
                .analyze_information_element(&non_nas, 2, packet_time(299))
                .is_none()
        );
        let event = analyzer
            .analyze_information_element(&non_nas, 3, packet_time(300))
            .expect("expected a warning");
        assert_eq!(event.event_type, EventType::Low);
    }
}
