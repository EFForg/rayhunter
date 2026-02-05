use chrono::{DateTime, FixedOffset};
use log::debug;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::analysis::diagnostic::DiagnosticAnalyzer;
use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType};
use crate::util::RuntimeMetadata;
use crate::{diag::MessagesContainer, gsmtap_parser};

use super::{
    connection_redirect_downgrade::ConnectionRedirect2GDowngradeAnalyzer,
    imsi_requested::ImsiRequestedAnalyzer, incomplete_sib::IncompleteSibAnalyzer,
    information_element::InformationElement, nas_null_cipher::NasNullCipherAnalyzer,
    null_cipher::NullCipherAnalyzer, priority_2g_downgrade::LteSib6And7DowngradeAnalyzer,
    test_analyzer::TestAnalyzer,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AnalyzerConfig {
    pub diagnostic_analyzer: bool,
    pub connection_redirect_2g_downgrade: bool,
    pub lte_sib6_and_7_downgrade: bool,
    pub null_cipher: bool,
    pub nas_null_cipher: bool,
    pub incomplete_sib: bool,
    pub test_analyzer: bool,
    pub imsi_requested: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        AnalyzerConfig {
            imsi_requested: true,
            diagnostic_analyzer: true,
            connection_redirect_2g_downgrade: true,
            lte_sib6_and_7_downgrade: true,
            null_cipher: true,
            nas_null_cipher: true,
            incomplete_sib: true,
            test_analyzer: false,
        }
    }
}

pub const REPORT_VERSION: u32 = 2;

/// The severity level of an event.
///
/// Informational does not result in any alert on the display.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub message: String,
}

/// An [Analyzer] represents one type of heuristic for detecting an IMSI Catcher
/// (IC). While maintaining some amount of state is useful, be mindful of how
/// much memory your [Analyzer] uses at runtime, since rayhunter may run for
/// many hours at a time with dozens of [Analyzers](Analyzer) working in parallel.
pub trait Analyzer {
    /// Returns a user-friendly, concise name for your heuristic.
    fn get_name(&self) -> Cow<'_, str>;

    /// Returns a user-friendly description of what your heuristic looks for,
    /// the types of [Events](Event) it may return, as well as possible false-positive
    /// conditions that may trigger an [Event]. If different [Events](Event) have
    /// different false-positive conditions, consider including them in its
    /// `message` field.
    fn get_description(&self) -> Cow<'_, str>;

    /// Analyze a single [InformationElement], possibly returning an [Event] if your
    /// heuristic deems it relevant. Again, be mindful of any state your
    /// [Analyzer] updates per message, since it may be run over hundreds or
    /// thousands of them alongside many other [Analyzers](Analyzer).
    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
    ) -> Option<Event>;

    /// Returns a version number for this Analyzer. This should only ever
    /// increase in value, and do so whenever substantial changes are made to
    /// the Analyzer's heuristic.
    fn get_version(&self) -> u32;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalyzerMetadata {
    pub name: String,
    pub description: String,
    pub version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
#[derive(Default)]
pub struct ReportMetadata {
    pub analyzers: Vec<AnalyzerMetadata>,
    pub rayhunter: RuntimeMetadata,

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

#[derive(Serialize, Debug)]
pub struct AnalysisRow {
    pub packet_timestamp: Option<DateTime<FixedOffset>>,
    pub skipped_message_reason: Option<String>,
    pub events: Vec<Option<Event>>,
}

impl AnalysisRow {
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
            packet_num: 0,
        }
    }

    pub fn new_with_config(analyzer_config: &AnalyzerConfig) -> Self {
        let mut harness = Harness::new();

        if analyzer_config.imsi_requested {
            harness.add_analyzer(Box::new(ImsiRequestedAnalyzer::new()));
        }
        if analyzer_config.connection_redirect_2g_downgrade {
            harness.add_analyzer(Box::new(ConnectionRedirect2GDowngradeAnalyzer {}));
        }
        if analyzer_config.lte_sib6_and_7_downgrade {
            harness.add_analyzer(Box::new(LteSib6And7DowngradeAnalyzer::new()));
        }
        if analyzer_config.null_cipher {
            harness.add_analyzer(Box::new(NullCipherAnalyzer {}));
        }

        if analyzer_config.nas_null_cipher {
            harness.add_analyzer(Box::new(NasNullCipherAnalyzer {}))
        }

        if analyzer_config.incomplete_sib {
            harness.add_analyzer(Box::new(IncompleteSibAnalyzer {}))
        }

        if analyzer_config.test_analyzer {
            harness.add_analyzer(Box::new(TestAnalyzer {}))
        }

        if analyzer_config.diagnostic_analyzer {
            harness.add_analyzer(Box::new(DiagnosticAnalyzer {}));
        }

        harness
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer + Send>) {
        self.analyzers.push(analyzer);
    }

    pub fn analyze_pcap_packet(&mut self, packet: EnhancedPacketBlock) -> AnalysisRow {
        self.packet_num += 1;

        let epoch = DateTime::parse_from_rfc3339("1980-01-06T00:00:00-00:00").unwrap();
        let mut row = AnalysisRow {
            packet_timestamp: Some(epoch + packet.timestamp),
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
                return row;
            }
        };
        let packet_offset = gsmtap_offset + 16;
        let packet_data = &packet.data[packet_offset..];
        let gsmtap_message = GsmtapMessage {
            header: gsmtap_header,
            payload: packet_data.to_vec(),
        };
        row.events = match InformationElement::try_from(&gsmtap_message) {
            Ok(element) => self.analyze_information_element(&element),
            Err(err) => {
                let msg = format!(
                    "in packet {}, failed to convert gsmtap message to IE: {err:?}",
                    self.packet_num
                );
                debug!("{msg}");
                row.skipped_message_reason = Some(msg);
                return row;
            }
        };
        row
    }

    pub fn analyze_qmdl_messages(&mut self, container: MessagesContainer) -> Vec<AnalysisRow> {
        let mut rows = Vec::new();
        for maybe_qmdl_message in container.into_messages() {
            self.packet_num += 1;

            rows.push(AnalysisRow {
                packet_timestamp: None,
                skipped_message_reason: None,
                events: Vec::new(),
            });
            // unwrap is safe here since we just pushed a value
            let row = rows.last_mut().unwrap();
            let qmdl_message = match maybe_qmdl_message {
                Ok(msg) => msg,
                Err(err) => {
                    row.skipped_message_reason = Some(format!("{err:?}"));
                    continue;
                }
            };

            let gsmtap_message = match gsmtap_parser::parse(qmdl_message) {
                Ok(msg) => msg,
                Err(err) => {
                    row.skipped_message_reason = Some(format!("{err:?}"));
                    continue;
                }
            };

            let Some((timestamp, gsmtap_msg)) = gsmtap_message else {
                continue;
            };
            row.packet_timestamp = Some(timestamp.to_datetime());

            let element = match InformationElement::try_from(&gsmtap_msg) {
                Ok(element) => element,
                Err(err) => {
                    row.skipped_message_reason = Some(format!("{err:?}"));
                    continue;
                }
            };

            row.events = self.analyze_information_element(&element);
        }
        rows
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Vec<Option<Event>> {
        // This method is private because incrementing packet_num is currently handled entirely by the other
        // methods that call this one. This could be changed with some careful refactoring, but
        // while this method is only used by other Harness methods, let's keep it private to help
        // ensure we always bump packet_num exactly once for each processed packet.
        let packet_str = format!(" (packet {})", self.packet_num);
        self.analyzers
            .iter_mut()
            .map(|analyzer| {
                let mut maybe_event = analyzer.analyze_information_element(ie, self.packet_num);
                if let Some(ref mut event) = maybe_event {
                    event.message.push_str(&packet_str);
                }
                maybe_event
            })
            .collect()
    }

    pub fn get_metadata(&self) -> ReportMetadata {
        let mut analyzers = Vec::new();
        for analyzer in &self.analyzers {
            analyzers.push(AnalyzerMetadata {
                name: analyzer.get_name().to_string(),
                description: analyzer.get_description().to_string(),
                version: analyzer.get_version(),
            });
        }

        let rayhunter = RuntimeMetadata::new();

        ReportMetadata {
            analyzers,
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
}
