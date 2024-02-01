use crate::lte_rrc::Message as LteRrcMessage;

/// Qualitative measure of how severe a Warning event type is.
/// The levels should break down like this:
///   * Low: if combined with a large number of other Warnings, user should investigate
///   * Medium: if combined with a few other Warnings, user should investigate
///   * High: user should investigate
pub enum Severity {
    Low,
    Medium,
    High,
}

/// `QualitativeWarning` events will always be shown to the user in some manner,
/// while `Informational` ones may be hidden based on user settings.
pub enum EventType {
    Informational,
    QualitativeWarning(Severity),
}

/// Events are user-facing signals that can be emitted by an `Analyzer` upon a
/// message being received. They can be used to signifiy an IC detection
/// warning, or just to display some relevant information to the user.
pub struct Event {
    pub event_type: EventType,
    pub message: String,
}

/// An `Analyzer` represents one type of heuristic for detecting an IMSI Catcher
/// (IC). While maintaining some amount of state is useful, be mindful of how
/// much memory your `Analyzer` uses at runtime, since rayhunter may run for
/// many hours at a time with dozens of `Analyzer`s working in parallel.
trait Analyzer {
    /// Returns a user-friendly, concise name for your heuristic.
    fn get_name(&self) -> String;

    /// Returns a user-friendly description of what your heuristic looks for,
    /// the types of `Event`s it may return, as well as possible false-positive
    /// conditions that may trigger an `Event`. If different `Event`s have
    /// different false-positive conditions, consider including them in its
    /// `message` field.
    fn get_description(&self) -> String;

    /// Processes a single `LteRrcMessage`, possibly returning an `Event` if your
    /// heuristic deems it relevant. Again, be mindful of any state your
    /// `Analyzer` updates per message, since it may be run over hundreds or
    /// thousands of them alongside many other `Analyzer`s.
    fn process_lte_rrc_message(&mut self, message: &LteRrcMessage) -> Option<Event>;
}
