use std::borrow::Cow;
use serde::Serialize;

use super::information_element::InformationElement;

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

pub struct Harness {
    analyzers: Vec<Box<dyn Analyzer>>,
}

impl Harness {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) {
        self.analyzers.push(analyzer);
    }

    pub fn analyze_information_element(&mut self, ie: &InformationElement) -> Vec<Option<Event>> {
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
}
