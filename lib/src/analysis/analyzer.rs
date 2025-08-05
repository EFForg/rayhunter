use chrono::{DateTime, FixedOffset};
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType};
use crate::util::RuntimeMetadata;
use crate::{diag::MessagesContainer, gsmtap_parser};

use super::{
    connection_redirect_downgrade::ConnectionRedirect2GDowngradeAnalyzer,
    imsi_requested::ImsiRequestedAnalyzer, incomplete_sib::IncompleteSibAnalyzer,
    information_element::InformationElement, nas_null_cipher::NasNullCipherAnalyzer,
    null_cipher::NullCipherAnalyzer, priority_2g_downgrade::LteSib6And7DowngradeAnalyzer,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AnalyzerConfig {
    pub imsi_requested: bool,
    pub connection_redirect_2g_downgrade: bool,
    pub lte_sib6_and_7_downgrade: bool,
    pub null_cipher: bool,
    pub nas_null_cipher: bool,
    pub incomplete_sib: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        AnalyzerConfig {
            imsi_requested: true,
            connection_redirect_2g_downgrade: true,
            lte_sib6_and_7_downgrade: true,
            null_cipher: true,
            nas_null_cipher: true,
            incomplete_sib: true,
        }
    }
}

pub const REPORT_VERSION: u32 = 2;

/// Qualitative measure of how severe a Warning event type is.
/// The levels should break down like this:
///   * Low: if combined with a large number of other Warnings, user should investigate
///   * Medium: if combined with a few other Warnings, user should investigate
///   * High: user should investigate
#[derive(Serialize, Debug, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
}

/// `QualitativeWarning` events will always be shown to the user in some manner,
/// while `Informational` ones may be hidden based on user settings.
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EventType {
    Informational,
    QualitativeWarning { severity: Severity },
}

/// Events are user-facing signals that can be emitted by an [Analyzer] upon a
/// message being received. They can be used to signifiy an IC detection
/// warning, or just to display some relevant information to the user.
#[derive(Serialize, Debug, Clone)]
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
    fn get_name(&self) -> Cow<str>;

    /// Returns a user-friendly description of what your heuristic looks for,
    /// the types of [Events](Event) it may return, as well as possible false-positive
    /// conditions that may trigger an [Event]. If different [Events](Event) have
    /// different false-positive conditions, consider including them in its
    /// `message` field.
    fn get_description(&self) -> Cow<str>;

    /// Analyze a single [InformationElement], possibly returning an [Event] if your
    /// heuristic deems it relevant. Again, be mindful of any state your
    /// [Analyzer] updates per message, since it may be run over hundreds or
    /// thousands of them alongside many other [Analyzers](Analyzer).
    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event>;

    /// Returns a version number for this Analyzer. This should only ever
    /// increase in value, and do so whenever substantial changes are made to
    /// the Analyzer's heuristic.
    fn get_version(&self) -> u32;
}

#[derive(Serialize, Debug)]
pub struct AnalyzerMetadata {
    pub name: String,
    pub description: String,
    pub version: u32,
}

#[derive(Serialize, Debug)]
pub struct ReportMetadata {
    pub analyzers: Vec<AnalyzerMetadata>,
    pub rayhunter: RuntimeMetadata,
    // anytime the format of the report changes, bump this by 1
    pub report_version: u32,
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
        for event in self.events.iter().flatten() {
            if matches!(event.event_type, EventType::QualitativeWarning { .. }) {
                return true;
            }
        }
        false
    }
}

pub struct Harness {
    analyzers: Vec<Box<dyn Analyzer + Send>>,
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
            harness.add_analyzer(Box::new(LteSib6And7DowngradeAnalyzer {}));
        }
        if analyzer_config.null_cipher {
            harness.add_analyzer(Box::new(NullCipherAnalyzer {}));
        }

        if analyzer_config.nas_null_cipher {
            harness.add_analyzer(Box::new(NasNullCipherAnalyzer::new()))
        }

        if analyzer_config.incomplete_sib {
            harness.add_analyzer(Box::new(IncompleteSibAnalyzer::new()))
        }

        harness
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer + Send>) {
        self.analyzers.push(analyzer);
    }

    pub fn analyze_pcap_packet(&mut self, packet: EnhancedPacketBlock) -> AnalysisRow {
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
                row.skipped_message_reason =
                    Some(format!("failed to convert gsmtap message to IE: {err:?}"));
                return row;
            }
        };
        row
    }

    pub fn analyze_qmdl_messages(&mut self, container: MessagesContainer) -> Vec<AnalysisRow> {
        let mut rows = Vec::new();
        for maybe_qmdl_message in container.into_messages() {
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

    pub fn analyze_information_element(&mut self, ie: &InformationElement) -> Vec<Option<Event>> {
        self.analyzers
            .iter_mut()
            .map(|analyzer| analyzer.analyze_information_element(ie))
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
