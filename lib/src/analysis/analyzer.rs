use std::borrow::Cow;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::{diag::MessagesContainer, gsmtap_parser};

use super::{information_element::InformationElement, lte_downgrade::LteSib6And7DowngradeAnalyzer};

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

/// [QualitativeWarning] events will always be shown to the user in some manner,
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
}

#[derive(Serialize, Debug)]
pub struct AnalyzerMetadata {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
pub struct ReportMetadata {
    num_packets_analyzed: usize,
    num_packets_skipped: usize,
    num_warnings: usize,
    first_packet_time: Option<DateTime<FixedOffset>>,
    last_packet_time: Option<DateTime<FixedOffset>>,
    analyzers: Vec<AnalyzerMetadata>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PacketAnalysis {
    timestamp: DateTime<FixedOffset>,
    events: Vec<Option<Event>>,
}

#[derive(Serialize, Debug)]
pub struct AnalysisReport {
    metadata: ReportMetadata,
    analysis: Vec<PacketAnalysis>,
}

pub struct Harness {
    analyzers: Vec<Box<dyn Analyzer + Send>>,
    pub num_packets_analyzed: usize,
    pub num_warnings: usize,
    pub skipped_message_reasons: Vec<String>,
    pub first_packet_time: Option<DateTime<FixedOffset>>,
    pub last_packet_time: Option<DateTime<FixedOffset>>,
    pub analysis: Vec<PacketAnalysis>,
}

impl Harness {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
            num_packets_analyzed: 0,
            skipped_message_reasons: Vec::new(),
            num_warnings: 0,
            first_packet_time: None,
            last_packet_time: None,
            analysis: Vec::new(),
        }
    }

    pub fn new_with_all_analyzers() -> Self {
        let mut harness = Harness::new();
        harness.add_analyzer(Box::new(LteSib6And7DowngradeAnalyzer{}));
        harness
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer + Send>) {
        self.analyzers.push(analyzer);
    }

    pub fn analyze_qmdl_messages(&mut self, container: MessagesContainer) {
        for maybe_qmdl_message in container.into_messages() {
            let qmdl_message = match maybe_qmdl_message {
                Ok(msg) => msg,
                Err(err) => {
                    self.skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };

            let gsmtap_message = match gsmtap_parser::parse(qmdl_message) {
                Ok(msg) => msg,
                Err(err) => {
                    self.skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };

            let Some((timestamp, gsmtap_msg)) = gsmtap_message else {
                continue;
            };

            let element = match InformationElement::try_from(&gsmtap_msg) {
                Ok(element) => element,
                Err(err) => {
                    self.skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };

            if self.first_packet_time.is_none() {
                self.first_packet_time = Some(timestamp.to_datetime());
            }

            self.last_packet_time = Some(timestamp.to_datetime());
            self.num_packets_analyzed += 1;
            let analysis_result = self.analyze_information_element(&element);
            if analysis_result.iter().any(Option::is_some) {
                self.num_warnings += analysis_result.iter()
                    .filter(|maybe_event| matches!(maybe_event, Some(Event { event_type: EventType::QualitativeWarning { .. }, .. })))
                    .count();
                self.analysis.push(PacketAnalysis {
                    timestamp: timestamp.to_datetime(),
                    events: analysis_result,
                });
            }
        }
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Vec<Option<Event>> {
        self.analyzers.iter_mut()
            .map(|analyzer| analyzer.analyze_information_element(ie))
            .collect()
    }

    pub fn get_names(&self) -> Vec<Cow<'_, str>> {
        self.analyzers.iter()
            .map(|analyzer| analyzer.get_name())
            .collect()
    }

    pub fn get_descriptions(&self) -> Vec<Cow<'_, str>> {
        self.analyzers.iter()
            .map(|analyzer| analyzer.get_description())
            .collect()
    }

    pub fn build_analysis_report(&self) -> AnalysisReport {
        let names = self.get_names();
        let descriptions = self.get_names();
        let mut analyzers = Vec::new();
        for (name, description) in names.iter().zip(descriptions.iter()) {
            analyzers.push(AnalyzerMetadata {
                name: name.to_string(),
                description: description.to_string(),
            });
        }

        AnalysisReport {
            metadata: ReportMetadata {
                num_packets_analyzed: self.num_packets_analyzed,
                num_packets_skipped: self.skipped_message_reasons.len(),
                num_warnings: self.num_warnings,
                first_packet_time: self.first_packet_time,
                last_packet_time: self.last_packet_time,
                analyzers,
            },
            analysis: self.analysis.clone(),
        }
    }
}
