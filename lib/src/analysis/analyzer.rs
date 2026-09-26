use chrono::{DateTime, FixedOffset};
use log::debug;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::DeviceMetadata;
use crate::diag::{DiagParsingError, Message, MessagesContainer};
use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, parser as gsmtap_parser};
use crate::util::RuntimeMetadata;

use super::{
    connection_redirect_downgrade::ConnectionRedirect2GDowngradeAnalyzer,
    diagnostic::DiagnosticAnalyzer, imsi_requested::ImsiRequestedAnalyzer,
    incomplete_sib::IncompleteSibAnalyzer, information_element::InformationElement,
    nas_null_cipher::NasNullCipherAnalyzer, no_nas_messages::NoNasMessagesAnalyzer,
    null_cipher::NullCipherAnalyzer, priority_2g_downgrade::LteSib6And7DowngradeAnalyzer,
    test_analyzer::TestAnalyzer,
};

fn boxed(analyzer: impl Analyzer + Send + 'static) -> Box<dyn Analyzer + Send> {
    Box::new(analyzer)
}

fn all_analyzers(
    device_metadata: &DeviceMetadata,
) -> impl Iterator<Item = (AnalyzerMetadata, Box<dyn Analyzer + Send>)> {
    [
        (
            ImsiRequestedAnalyzer::metadata(),
            boxed(ImsiRequestedAnalyzer::new(
                device_metadata.home_plmn.clone(),
            )),
        ),
        (
            ConnectionRedirect2GDowngradeAnalyzer::metadata(),
            boxed(ConnectionRedirect2GDowngradeAnalyzer {}),
        ),
        (
            LteSib6And7DowngradeAnalyzer::metadata(),
            boxed(LteSib6And7DowngradeAnalyzer::new()),
        ),
        (NullCipherAnalyzer::metadata(), boxed(NullCipherAnalyzer {})),
        (
            NasNullCipherAnalyzer::metadata(),
            boxed(NasNullCipherAnalyzer {}),
        ),
        (
            IncompleteSibAnalyzer::metadata(),
            boxed(IncompleteSibAnalyzer {}),
        ),
        (TestAnalyzer::metadata(), boxed(TestAnalyzer {})),
        (
            NoNasMessagesAnalyzer::metadata(),
            boxed(NoNasMessagesAnalyzer::new()),
        ),
        (DiagnosticAnalyzer::metadata(), boxed(DiagnosticAnalyzer {})),
    ]
    .into_iter()
}

/// Stores whether an analyzer is enabled, keyed by [AnalyzerMetadata::key].
/// Missing keys use the analyzer default; unknown keys are retained and
/// produce a warning when deserialized.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
#[serde(transparent)]
pub struct AnalyzerConfig(BTreeMap<String, bool>);

impl Default for AnalyzerConfig {
    fn default() -> Self {
        AnalyzerConfig(
            get_analyzers_metadata()
                .into_iter()
                .map(|metadata| (metadata.key.to_string(), metadata.default_enabled))
                .collect(),
        )
    }
}

impl AnalyzerConfig {
    /// Returns the configured override for `key`, or `None` when there is no
    /// config entry for it.
    fn is_enabled(&self, key: &str) -> Option<bool> {
        self.0.get(key).copied()
    }
}

impl<'de> Deserialize<'de> for AnalyzerConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = <BTreeMap<String, bool>>::deserialize(deserializer)?;
        let known_keys: Vec<String> = get_analyzers_metadata()
            .into_iter()
            .map(|metadata| metadata.key.to_string())
            .collect();
        let mut unknown_keys: Vec<&String> =
            map.keys().filter(|key| !known_keys.contains(key)).collect();
        unknown_keys.sort_unstable();
        if !unknown_keys.is_empty() {
            log::warn!(
                "config contains unknown analyzer(s) {unknown_keys:?}; valid keys are {known_keys:?}"
            );
        }
        Ok(AnalyzerConfig(map))
    }
}

pub fn get_analyzers_metadata() -> Vec<AnalyzerMetadata> {
    all_analyzers(&DeviceMetadata::default())
        .map(|(metadata, _)| metadata)
        .collect()
}

pub const REPORT_VERSION: u32 = 2;

/// The severity level of an event.
///
/// Informational does not result in any alert on the display.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub enum EventType {
    Informational = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum OldEventType {
            QualitativeWarning { severity: String },
            Informational,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum EventTypeHelper {
            New(String),
            Old(OldEventType),
        }

        match EventTypeHelper::deserialize(deserializer)? {
            EventTypeHelper::New(s) => match s.as_str() {
                "Informational" => Ok(EventType::Informational),
                "Low" => Ok(EventType::Low),
                "Medium" => Ok(EventType::Medium),
                "High" => Ok(EventType::High),
                _ => Err(D::Error::custom(format!("unknown EventType: {s}"))),
            },
            EventTypeHelper::Old(old) => match old {
                OldEventType::Informational => Ok(EventType::Informational),
                OldEventType::QualitativeWarning { severity } => match severity.as_str() {
                    "Low" => Ok(EventType::Low),
                    "Medium" => Ok(EventType::Medium),
                    "High" => Ok(EventType::High),
                    _ => Err(D::Error::custom(format!("unknown severity: {severity}"))),
                },
            },
        }
    }
}

/// Events are user-facing signals that can be emitted by an [Analyzer] upon a
/// message being received. They can be used to signifiy an IC detection
/// warning, or just to display some relevant information to the user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub message: String,
}

/// An [Analyzer] represents one type of heuristic for detecting an IMSI Catcher
/// (IC). While maintaining some amount of state is useful, be mindful of how
/// much memory your [Analyzer] uses at runtime, since rayhunter may run for
/// many hours at a time with dozens of [Analyzers](Analyzer) working in parallel.
pub trait Analyzer {
    /// Returns static metadata for this analyzer type; it must not depend on
    /// instance state.
    fn metadata() -> AnalyzerMetadata
    where
        Self: Sized;

    /// Analyze a single [InformationElement], possibly returning an [Event] if your
    /// heuristic deems it relevant. `timestamp` is the timestamp of the packet
    /// that produced this element. Again, be mindful of any state your
    /// [Analyzer] updates per message, since it may be run over hundreds or
    /// thousands of them alongside many other [Analyzers](Analyzer).
    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
        timestamp: DateTime<FixedOffset>,
    ) -> Option<Event>;

    /// Reports a skipped, unhandled, or otherwise unparsed packet to the analyzer.
    /// Time-based analyzers can also call this from [Self::analyze_information_element]
    /// to process the timestamp of a parsed packet. The default implementation does
    /// nothing.
    fn report_skipped_packet(&mut self, _timestamp: DateTime<FixedOffset>) -> Option<Event> {
        None
    }
}

/// Static analyzer metadata exposed in configuration and analysis reports.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct AnalyzerMetadata {
    /// Unique configuration key. Should remain the same even if `name` changes to maintain config stability.
    #[cfg_attr(feature = "apidocs", schema(value_type = String))]
    pub key: Cow<'static, str>,
    pub default_enabled: bool,
    #[cfg_attr(feature = "apidocs", schema(value_type = String))]
    pub name: Cow<'static, str>,
    /// User-facing summary of the heuristic, relevant events, and notable
    /// false-positive conditions.
    #[cfg_attr(feature = "apidocs", schema(value_type = String))]
    pub description: Cow<'static, str>,
    /// Monotonically increasing heuristic version; bump for substantial
    /// behavior changes.
    pub version: u32,
}

/// The metadata for an analyzed report
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
#[derive(Default)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct ReportMetadata {
    /// A vector array of which analyzers were in use for the analysis
    pub analyzers: Vec<AnalyzerMetadata>,
    /// The runtime metadata for rayhunter during the recording and analysis
    pub rayhunter: RuntimeMetadata,
    /// The version of the reporting format used
    // anytime the format of the report changes, bump this by 1
    //
    // the default is 0. we consider our legacy (unversioned) heuristics to be v0 -- this'll let us
    // clearly differentiate some known false-positive-results from the pre-versioned era from v1
    // heuristics
    pub report_version: u32,
}

impl ReportMetadata {
    /// Normalize the report metadata to the current version
    pub fn normalize(&mut self) {
        self.report_version = REPORT_VERSION;
    }
}

/// Normalizer for analysis report lines that maintains state internally.
/// The first line is expected to be ReportMetadata, and subsequent lines
/// are expected to be AnalysisRow entries.
pub struct AnalysisLineNormalizer {
    is_first: bool,
}

impl Default for AnalysisLineNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisLineNormalizer {
    pub fn new() -> Self {
        Self { is_first: true }
    }

    /// Normalize a single line from an analysis report.
    /// Returns the normalized JSON string with a newline appended.
    pub fn normalize_line(&mut self, line: String) -> String {
        if self.is_first {
            self.is_first = false;
            // the first line is the report metadata. we overwrite the report version there to
            // latest, because the output of the remaining lines will follow latest versions
            if let Ok(mut metadata) = serde_json::from_str::<ReportMetadata>(&line) {
                metadata.normalize();
                serde_json::to_string(&metadata).unwrap_or(line) + "\n"
            } else {
                line + "\n"
            }
        } else {
            // Remaining lines are AnalysisRow, roundtrip them through serde to normalize them.
            if let Ok(row) = serde_json::from_str::<AnalysisRow>(&line) {
                serde_json::to_string(&row).unwrap_or(line) + "\n"
            } else {
                line + "\n"
            }
        }
    }
}

#[derive(Serialize, Debug, Default)]
pub struct AnalysisRow {
    pub packet_timestamp: Option<DateTime<FixedOffset>>,
    pub skipped_message_reason: Option<String>,
    pub events: Vec<Option<Event>>,
}

impl AnalysisRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.skipped_message_reason.is_none() && !self.contains_warnings()
    }

    pub fn contains_warnings(&self) -> bool {
        self.get_max_event_type() != EventType::Informational
    }

    pub fn get_max_event_type(&self) -> EventType {
        self.events
            .iter()
            .flatten()
            .map(|event| event.event_type)
            .max()
            .unwrap_or(EventType::Informational)
    }
}

impl<'de> Deserialize<'de> for AnalysisRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        struct V1AnalysisEntry {
            timestamp: DateTime<FixedOffset>,
            events: Vec<Option<Event>>,
        }

        #[derive(Deserialize)]
        struct V1Format {
            timestamp: DateTime<FixedOffset>,
            skipped_message_reasons: Vec<String>,
            analysis: Vec<V1AnalysisEntry>,
        }

        #[derive(Deserialize)]
        struct V2Format {
            packet_timestamp: Option<DateTime<FixedOffset>>,
            skipped_message_reason: Option<String>,
            events: Vec<Option<Event>>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RowFormat {
            V1(V1Format),
            V2(V2Format),
        }

        match RowFormat::deserialize(deserializer)? {
            RowFormat::V1(v1) => {
                // For v1 format, we can only deserialize the first non-skipped analysis entry
                // The caller needs to handle multiple rows differently for v1
                if let Some(first_analysis) = v1.analysis.first() {
                    Ok(AnalysisRow {
                        packet_timestamp: Some(first_analysis.timestamp),
                        skipped_message_reason: None,
                        events: first_analysis.events.clone(),
                    })
                } else if let Some(first_reason) = v1.skipped_message_reasons.first() {
                    Ok(AnalysisRow {
                        packet_timestamp: Some(v1.timestamp),
                        skipped_message_reason: Some(first_reason.clone()),
                        events: Vec::new(),
                    })
                } else {
                    Err(D::Error::custom(
                        "V1 format has no analysis entries or skipped reasons",
                    ))
                }
            }
            RowFormat::V2(v2) => Ok(AnalysisRow {
                packet_timestamp: v2.packet_timestamp,
                skipped_message_reason: v2.skipped_message_reason,
                events: v2.events,
            }),
        }
    }
}

pub struct Harness {
    analyzers: Vec<Box<dyn Analyzer + Send>>,
    metadata: Vec<AnalyzerMetadata>,
    packet_num: usize,
}

impl Default for Harness {
    fn default() -> Self {
        Self::new()
    }
}

impl Harness {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
            metadata: Vec::new(),
            packet_num: 0,
        }
    }

    pub fn new_with_config(
        analyzer_config: &AnalyzerConfig,
        device_metadata: &DeviceMetadata,
    ) -> Self {
        let mut harness = Harness::new();

        for (metadata, analyzer) in all_analyzers(device_metadata) {
            if analyzer_config
                .is_enabled(metadata.key.as_ref())
                .unwrap_or(metadata.default_enabled)
            {
                harness.add_analyzer(metadata, analyzer);
            }
        }

        harness
    }

    pub fn add_analyzer(&mut self, metadata: AnalyzerMetadata, analyzer: Box<dyn Analyzer + Send>) {
        self.metadata.push(metadata);
        self.analyzers.push(analyzer);
    }

    pub fn analyze_wifi_network(
        &mut self,
        bssid: &str,
        timestamp: DateTime<FixedOffset>,
    ) -> AnalysisRow {
        let mut analysis_row = AnalysisRow::new();
        let ie = InformationElement::WifiNetwork(bssid.to_string());
        for analyzer in &mut self.analyzers {
            if let Some(event) = analyzer.analyze_information_element(&ie, 0, timestamp) {
                analysis_row.events.push(Some(event));
            }
        }

        analysis_row
    }

    pub fn analyze_pcap_packet(&mut self, packet: EnhancedPacketBlock) -> AnalysisRow {
        self.packet_num += 1;

        let epoch = DateTime::parse_from_rfc3339("1980-01-06T00:00:00-00:00").unwrap();
        let packet_timestamp = epoch + packet.timestamp;
        let mut row = AnalysisRow {
            packet_timestamp: Some(packet_timestamp),
            skipped_message_reason: None,
            events: Vec::new(),
        };
        let gsmtap_offset = 20 + 8;
        let gsmtap_data = &packet.data[gsmtap_offset..];
        // the type and subtype are at byte offsets 3 and 13, respectively
        let gsmtap_header = match GsmtapType::new(gsmtap_data[2], gsmtap_data[12]) {
            Ok(gsmtap_type) => GsmtapHeader::new(gsmtap_type),
            Err(err) => {
                row.skipped_message_reason = Some(format!("failed to read GsmtapHeader: {err:?}"));
                row.events = self.report_skipped_packet(packet_timestamp);
                self.assert_events_match_analyzers(&row.events);
                return row;
            }
        };
        let packet_offset = gsmtap_offset + 16;
        let packet_data = &packet.data[packet_offset..];
        let gsmtap_message = GsmtapMessage {
            header: gsmtap_header,
            payload: packet_data.to_vec(),
        };
        let element = match InformationElement::try_from(&gsmtap_message) {
            Ok(element) => element,
            Err(err) => {
                let msg = format!(
                    "in packet {}, failed to convert gsmtap message to IE: {err:?}",
                    self.packet_num
                );
                debug!("{msg}");
                row.skipped_message_reason = Some(msg);
                row.events = self.report_skipped_packet(packet_timestamp);
                self.assert_events_match_analyzers(&row.events);
                return row;
            }
        };
        row.events = self.analyze_information_element(&element, packet_timestamp);
        self.assert_events_match_analyzers(&row.events);
        row
    }

    pub fn analyze_qmdl_message(
        &mut self,
        maybe_qmdl_message: Result<Message, DiagParsingError>,
    ) -> AnalysisRow {
        let mut row = AnalysisRow::new();
        self.packet_num += 1;

        let qmdl_message = match maybe_qmdl_message {
            Ok(msg) => msg,
            Err(err) => {
                row.skipped_message_reason = Some(format!("{err:?}"));
                return row;
            }
        };
        let packet_timestamp = if let Message::Log { timestamp, .. } = &qmdl_message {
            Some(timestamp.to_datetime())
        } else {
            None
        };
        row.packet_timestamp = packet_timestamp;

        let (timestamp, gsmtap_msg) = match gsmtap_parser::parse(qmdl_message) {
            Ok(Some((timestamp, msg))) => (timestamp.to_datetime(), msg),
            Ok(None) => {
                if let Some(timestamp) = packet_timestamp {
                    row.events = self.report_skipped_packet(timestamp);
                    self.assert_events_match_analyzers(&row.events);
                }
                return row;
            }
            Err(err) => {
                row.skipped_message_reason = Some(format!("{err:?}"));
                if let Some(timestamp) = packet_timestamp {
                    row.events = self.report_skipped_packet(timestamp);
                    self.assert_events_match_analyzers(&row.events);
                }
                return row;
            }
        };

        let element = match InformationElement::try_from(&gsmtap_msg) {
            Ok(element) => element,
            Err(err) => {
                row.skipped_message_reason = Some(format!("{err:?}"));
                row.events = self.report_skipped_packet(timestamp);
                self.assert_events_match_analyzers(&row.events);
                return row;
            }
        };

        row.events = self.analyze_information_element(&element, timestamp);
        self.assert_events_match_analyzers(&row.events);
        row
    }

    pub fn analyze_qmdl_messages(&mut self, container: MessagesContainer) -> Vec<AnalysisRow> {
        container
            .messages()
            .drain(..)
            .map(|maybe_message| self.analyze_qmdl_message(maybe_message))
            .collect()
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        timestamp: DateTime<FixedOffset>,
    ) -> Vec<Option<Event>> {
        // This method is private because incrementing packet_num is currently handled entirely by the other
        // methods that call this one. This could be changed with some careful refactoring, but
        // while this method is only used by other Harness methods, let's keep it private to help
        // ensure we always bump packet_num exactly once for each processed packet.
        let packet_str = self.packet_suffix();
        self.analyzers
            .iter_mut()
            .map(|analyzer| {
                let mut maybe_event =
                    analyzer.analyze_information_element(ie, self.packet_num, timestamp);
                if let Some(ref mut event) = maybe_event {
                    event.message.push_str(&packet_str);
                }
                maybe_event
            })
            .collect()
    }

    fn report_skipped_packet(&mut self, timestamp: DateTime<FixedOffset>) -> Vec<Option<Event>> {
        let packet_str = self.packet_suffix();
        self.analyzers
            .iter_mut()
            .map(|analyzer| {
                let mut maybe_event = analyzer.report_skipped_packet(timestamp);
                if let Some(ref mut event) = maybe_event {
                    event.message.push_str(&packet_str);
                }
                maybe_event
            })
            .collect()
    }

    fn packet_suffix(&self) -> String {
        format!(" (packet {})", self.packet_num)
    }

    fn assert_events_match_analyzers(&self, events: &[Option<Event>]) {
        assert_eq!(events.len(), self.analyzers.len());
    }

    pub fn get_metadata(&self) -> ReportMetadata {
        let rayhunter = RuntimeMetadata::new();

        ReportMetadata {
            analyzers: self.metadata.clone(),
            rayhunter,
            report_version: REPORT_VERSION,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_analyzer_keys_are_unique() {
        let metadata = get_analyzers_metadata();
        let mut keys: Vec<&str> = metadata.iter().map(|m| m.key.as_ref()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), metadata.len(), "duplicate analyzer keys");
    }

    #[test]
    fn test_analysis_row_deserialize_old_format() {
        let row: AnalysisRow = serde_json::from_value(json!({
            "packet_timestamp": "2023-01-01T00:00:00+00:00",
            "skipped_message_reason": null,
            "events": [
                {
                    "event_type": { "type": "QualitativeWarning", "severity": "High" },
                    "message": "Test warning"
                },
                {
                    "event_type": { "type": "Informational" },
                    "message": "Test info"
                },
                null
            ]
        }))
        .unwrap();

        assert_eq!(row.events[0].as_ref().unwrap().event_type, EventType::High);
        assert_eq!(
            row.events[1].as_ref().unwrap().event_type,
            EventType::Informational
        );
        assert!(row.events[2].is_none());
    }

    #[test]
    fn test_analysis_row_deserialize_new_format() {
        let row: AnalysisRow = serde_json::from_value(json!({
            "packet_timestamp": "2023-01-01T00:00:00+00:00",
            "skipped_message_reason": null,
            "events": [
                { "event_type": "High", "message": "Test warning" },
                { "event_type": "Informational", "message": "Test info" },
                null
            ]
        }))
        .unwrap();

        assert_eq!(row.events[0].as_ref().unwrap().event_type, EventType::High);
        assert_eq!(
            row.events[1].as_ref().unwrap().event_type,
            EventType::Informational
        );
        assert!(row.events[2].is_none());
    }

    use crate::analysis::no_nas_messages::NoNasMessagesAnalyzer;
    use crate::diag::diaglog::{LogBody, Nas4GMessageDirection, Timestamp};

    fn log_message(ts: u64, body: LogBody) -> Message {
        Message::Log {
            pending_msgs: 0,
            outer_length: 0,
            inner_length: 0,
            log_type: 0,
            timestamp: Timestamp { ts },
            body,
        }
    }

    // The upper 48 bits of a Timestamp tick every 1.25ms (see
    // Timestamp::to_datetime), so N * 800 ticks == N seconds.
    const FIVE_MINUTES_TS: u64 = 240_000 << 16;
    // Just under 5 minutes: close enough to the boundary that the final,
    // sub-threshold hop won't be treated as a forward-jump discontinuity.
    const ALMOST_FIVE_MINUTES_TS: u64 = 239_200 << 16;

    #[test]
    fn test_parsed_message_updates_timestamp_without_changing_event_count() {
        let mut harness = Harness::new();
        harness.add_analyzer(
            NoNasMessagesAnalyzer::metadata(),
            Box::new(NoNasMessagesAnalyzer::new()),
        );

        let row =
            harness.analyze_qmdl_message(Ok(log_message(0, LogBody::IpTraffic { msg: vec![] })));
        assert!(row.events.iter().all(|event| event.is_none()));

        let row = harness.analyze_qmdl_message(Ok(log_message(
            ALMOST_FIVE_MINUTES_TS,
            LogBody::IpTraffic { msg: vec![] },
        )));
        assert!(row.events.iter().all(|event| event.is_none()));

        let nas = log_message(
            FIVE_MINUTES_TS,
            LogBody::Nas4GMessage {
                log_type: 0xb0ec,
                direction: Nas4GMessageDirection::Downlink,
                ext_header_version: 0,
                rrc_rel: 0,
                rrc_version_minor: 0,
                rrc_version_major: 0,
                msg: vec![0x07, 0x55, 0x01],
            },
        );
        let row = harness.analyze_qmdl_message(Ok(nas));
        assert_eq!(row.events.len(), 1);
        assert!(row.events.iter().all(|event| event.is_none()));
    }

    #[test]
    fn test_skipped_packet_timestamp_event_includes_packet_suffix() {
        let mut harness = Harness::new();
        harness.add_analyzer(
            NoNasMessagesAnalyzer::metadata(),
            Box::new(NoNasMessagesAnalyzer::new()),
        );

        let row =
            harness.analyze_qmdl_message(Ok(log_message(0, LogBody::IpTraffic { msg: vec![] })));
        assert!(row.events.iter().all(|event| event.is_none()));

        let row = harness.analyze_qmdl_message(Ok(log_message(
            ALMOST_FIVE_MINUTES_TS,
            LogBody::IpTraffic { msg: vec![] },
        )));
        assert!(row.events.iter().all(|event| event.is_none()));

        let row = harness.analyze_qmdl_message(Ok(log_message(
            FIVE_MINUTES_TS,
            LogBody::IpTraffic { msg: vec![] },
        )));
        let event = row.events[0].as_ref().expect("expected a warning event");
        assert!(event.message.ends_with(" (packet 3)"));
    }
}
