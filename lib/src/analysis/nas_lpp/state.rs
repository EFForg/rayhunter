use chrono::{DateTime, Duration, FixedOffset};

use telcom_parser::ParsingError;
use telcom_parser::lte_lpp::{
    AbortCriticalExtensions, AbortCriticalExtensions_c1, CommonIEsAbortAbortCause as AbortCause,
    CommonIEsErrorErrorCause as ErrorCause, Error::Error_r9, Initiator, LPP_MessageBody_c1,
    LPP_TransactionID,
};

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

use crate::analysis::analyzer::{Event, EventType};

use self::Stage::*;

/// Keeps analyzer cost down by rate-limiting processing that runs across all transactions
pub(crate) const INTERVAL_SWEEP: Duration = Duration::seconds(10);
/// How long to wait before removing a transaction, ending it with [Stage::Timeout]
pub(crate) const TIMEOUT_TRANSACTION: Duration = Duration::minutes(10);
/// Prevents [Event] noise around message spikes by delaying reporting of a new transaction
pub(crate) const TIMEOUT_FIRST_REPORT: Duration = Duration::seconds(5);
/// A cooldown on reporting periodic messages within long-running transactions, to reduce noise
pub(crate) const TIMEOUT_UPDATE: Duration = Duration::minutes(30);
/// A reasonable cap on the max number of LPP transactions to track.
/// At capacity, the most stale transaction is evicted to make room for new transactions.
pub(crate) const MAX_TRANSACTIONS: usize = 64;

/// rules:
/// - a "summary" event fires when a transaction finishes
/// - a "progress" event fires once a transaction is older than [TIMEOUT_FIRST_REPORT]
///   (checked every [INTERVAL_SWEEP]), then on every new message received, provided
///   [TIMEOUT_UPDATE] has passed since an event last fired. Messages received within a
///   [TIMEOUT_UPDATE] period are squelched until the next event.
/// - incoming messages are grouped into transactions by their [TransactionKey], deduplicated by
///   [Stage]
///
///   when a transaction finishes:
///   - a message with [Stage::Abort] or [Stage::Error] (TS 37.355, 5.5.3 and 5.4.4)
///   - a message with endTransaction set (TS 37.355 4.2)
///   - no message was received since [TIMEOUT_TRANSACTION], ends with [Stage::Timeout]
///   - it's reached [Stage::Evicted] after becoming the transaction with the oldest
///     [TransactionState::last_message_recv], in a transaction collection that has reached
///     [MAX_TRANSACTIONS] and a new transaction is received.
///
/// limitations:
/// - [telcom_parser::lte_lpp::LPP_Message::sequence_number] is not tracked.
///   It is possible duplicate messages (such as retries) are accounted to a transaction.
/// - Clock jumps violate assumptions about timeout rules. A backward clock jump causes progress
///   and timeout reporting to pause until the clock catches up, and transactions spanning a clock
///   jump may time out early, and be split into multiple reported transactions.
#[derive(Default)]
pub(crate) struct State {
    pub(crate) transactions: HashMap<TransactionKey, TransactionState>,
    pub(crate) pending: VecDeque<Event>,
    /// See [INTERVAL_SWEEP], [State::sweep]
    pub(crate) last_sweep: Option<DateTime<FixedOffset>>,
}

pub(crate) struct TransactionState {
    pub(crate) first_seen: DateTime<FixedOffset>,
    pub(crate) last_event_pushed: Option<DateTime<FixedOffset>>,
    pub(crate) last_message_recv: DateTime<FixedOffset>,
    pub(crate) lifecycle: Lifecycle,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Stage {
    RequestAssistanceData,
    ProvideAssistanceData,
    RequestCapabilities,
    ProvideCapabilities,
    RequestLocationInformation,
    ProvideLocationInformation,
    Abort(Option<u8>),
    Error(Option<u8>),
    Spare,
    Timeout,
    Evicted,
}

pub(crate) struct Lifecycle(Vec<(Stage, u32)>);

impl TransactionState {
    pub(crate) fn new(timestamp: DateTime<FixedOffset>) -> Self {
        Self {
            first_seen: timestamp,
            last_message_recv: timestamp,
            last_event_pushed: None,
            lifecycle: Lifecycle(vec![]),
        }
    }

    pub(crate) fn accept(&mut self, timestamp: DateTime<FixedOffset>, stage: Stage) {
        self.last_message_recv = timestamp;
        let prev = self.lifecycle.0.last_mut();
        // if the incoming message's stage matches the stage of the previous message
        // increment that stage's count instead of appending a new stage
        match prev {
            Some(p) if p.0 == stage => p.1 += 1,
            _ => self.lifecycle.0.push((stage, 1)),
        }
    }
}

impl State {
    /// Accepts a message, representing either a new transaction, or a message onto an existing one.
    /// This may push a progress or summary event onto [State::pending]. See [State] for rules
    pub(crate) fn accept(
        &mut self,
        timestamp: DateTime<FixedOffset>,
        k: TransactionKey,
        stage: Stage,
        end: bool,
    ) {
        let end = end || matches!(stage, Abort(_) | Error(_));
        let mut state = self
            .transactions
            .remove(&k)
            .unwrap_or_else(|| TransactionState::new(timestamp));
        state.accept(timestamp, stage);
        if end {
            self.pending.push_back(into_summary(k, state));
            return;
        } else if self.pending.len() < MAX_TRANSACTIONS
            && state
                .last_event_pushed
                .is_some_and(|last| timestamp - last > TIMEOUT_UPDATE)
        {
            // report progress only if we don't have a large backlog of events
            self.pending
                .push_back(to_progress(&k, &mut state, timestamp));
        }

        if self.transactions.len() >= MAX_TRANSACTIONS {
            let oldest = self
                .transactions
                .values()
                .map(|v| v.last_message_recv)
                .min();
            let evicted = self
                .transactions
                .extract_if(|_, v| Some(v.last_message_recv) == oldest)
                .map(|(k, mut state)| {
                    state.accept(timestamp, Evicted);
                    into_summary(k, state)
                })
                .next();
            self.pending.extend(evicted);
        }
        self.transactions.insert(k, state);
    }

    /// Accept a parsing error onto the event queue
    pub(crate) fn accept_err(&mut self, route_id: Option<Vec<u8>>, e: ParsingError) {
        let key = TransactionKey::new(route_id, None);
        let decode_err = Event {
            event_type: EventType::Informational,
            message: format!("LPP::Transaction[{key}]::Error(error={e})"),
        };
        self.pending.push_back(decode_err);
    }

    /// Emits the next pending event
    pub(crate) fn next(&mut self, timestamp: DateTime<FixedOffset>) -> Option<Event> {
        self.sweep(timestamp);
        self.pending.pop_front()
    }

    /// Sweeps the full set of transactions and enqueues Events onto [State::pending].
    /// Removes finished transactions. See [State] for rules.
    fn sweep(&mut self, timestamp: DateTime<FixedOffset>) {
        if self.transactions.is_empty()
            || self
                .last_sweep
                .is_some_and(|last| timestamp - last < INTERVAL_SWEEP)
        {
            return;
        }

        let timed_out = self
            .transactions
            .extract_if(|_, v| (timestamp - v.last_message_recv) > TIMEOUT_TRANSACTION)
            .map(|(k, mut state)| {
                state.lifecycle.0.push((Stage::Timeout, 1));
                into_summary(k, state)
            });
        self.pending.extend(timed_out);

        // order: unreported should be placed after all other sweeps that remove events from state
        // so as not to double-report
        let unreported = self
            .transactions
            .iter_mut()
            .filter(|(_, v)| {
                v.last_event_pushed.is_none() && timestamp - v.first_seen > TIMEOUT_FIRST_REPORT
            })
            // defer progress if we have a large backlog of events
            .take(MAX_TRANSACTIONS.saturating_sub(self.pending.len()))
            .map(|(k, v)| to_progress(k, v, timestamp));
        self.pending.extend(unreported);
        self.last_sweep = Some(timestamp);
    }
}

/// A unique identifier for an LPP transaction
#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct TransactionKey {
    route_id: Option<Vec<u8>>,
    /// initiator and transaction_number
    /// A decomposition of [LPP_TransactionID] for the purpose of implementing Eq, Hash, PartialEq
    /// hampi can generate derives for these, but derives would be generated for all eligible types
    /// instead of the only one we need.
    transaction_id: Option<(u8, u8)>,
}

impl TransactionKey {
    pub(crate) fn new(
        route_id: Option<Vec<u8>>,
        transaction_id: Option<LPP_TransactionID>,
    ) -> Self {
        let transaction_id = transaction_id.map(|t| (t.initiator.0, t.transaction_number.0));
        Self {
            route_id,
            transaction_id,
        }
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abort(cause) => {
                let cause = match cause {
                    Some(AbortCause::UNDEFINED) => "UNDEFINED",
                    Some(AbortCause::TARGET_DEVICE_ABORT) => "TARGET_DEVICE_ABORT",
                    Some(AbortCause::STOP_PERIODIC_REPORTING) => "STOP_PERIODIC_REPORTING",
                    Some(AbortCause::NETWORK_ABORT) => "NETWORK_ABORT",
                    Some(_) => "UNKNOWN",
                    None => "NONE",
                };
                write!(f, "Abort({cause})")
            }
            Self::Error(cause) => {
                let cause = match cause {
                    Some(ErrorCause::UNDEFINED) => "UNDEFINED",
                    Some(ErrorCause::EPDU_ERROR) => "EPDU_ERROR",
                    Some(ErrorCause::LPP_MESSAGE_BODY_ERROR) => "LPP_MESSAGE_BODY_ERROR",
                    Some(ErrorCause::LPP_MESSAGE_HEADER_ERROR) => "LPP_MESSAGE_HEADER_ERROR",
                    Some(ErrorCause::INCORRECT_DATA_VALUE) => "INCORRECT_DATA_VALUE",
                    Some(_) => "UNKNOWN",
                    None => "NONE",
                };
                write!(f, "Error({cause})")
            }
            other => write!(f, "{other:?}"),
        }
    }
}

/// A printable transaction_id: {route_id}/{initiator}{transaction_number}
/// E.g. `deadbeef/t001`
impl Display for TransactionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref route_id) = self.route_id {
            for b in route_id {
                write!(f, "{b:02x}")?;
            }
        } else {
            write!(f, "--------")?;
        }
        match self.transaction_id {
            Some((Initiator::LOCATION_SERVER, tx_num)) => write!(f, "/s{tx_num:03}"),
            Some((Initiator::TARGET_DEVICE, tx_num)) => write!(f, "/t{tx_num:03}"),
            Some((i, tx_num)) => write!(f, "/?[{i}]{tx_num:03}"),
            None => write!(f, "/----"),
        }
    }
}

/// Displays a [Lifecycle] as a progression of [Stage] elements, including their counts when greater
/// than 1. Example: `RequestLocationInformation -> ProvideLocationInformation x3 -> Timeout`
impl Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stages: Vec<String> = self
            .0
            .iter()
            .map(|(s, n)| {
                if *n > 1 {
                    format!("{s} x{n}")
                } else {
                    s.to_string()
                }
            })
            .collect();
        let progression = stages.join(" -> ");
        write!(f, "{}", progression)
    }
}

impl From<LPP_MessageBody_c1> for Stage {
    fn from(value: LPP_MessageBody_c1) -> Self {
        match value {
            LPP_MessageBody_c1::RequestAssistanceData(_) => RequestAssistanceData,
            LPP_MessageBody_c1::ProvideAssistanceData(_) => ProvideAssistanceData,
            LPP_MessageBody_c1::RequestCapabilities(_) => RequestCapabilities,
            LPP_MessageBody_c1::ProvideCapabilities(_) => ProvideCapabilities,
            LPP_MessageBody_c1::RequestLocationInformation(_) => RequestLocationInformation,
            LPP_MessageBody_c1::ProvideLocationInformation(_) => ProvideLocationInformation,
            LPP_MessageBody_c1::Abort(abort) => Abort(into_abort_cause(abort)),
            LPP_MessageBody_c1::Error(e) => Error(into_error_cause(e)),
            _ => Spare,
        }
    }
}

/// Updates [TransactionState::last_event_pushed] and produces an informational [Event]
/// describing this transaction's stage progress.
fn to_progress(
    key: &TransactionKey,
    state: &mut TransactionState,
    timestamp: DateTime<FixedOffset>,
) -> Event {
    state.last_event_pushed = Some(timestamp);
    let lifecycle = &state.lifecycle;
    let message = format!("LPP::Transaction[{key}]::Progress[{lifecycle}]");
    Event {
        event_type: EventType::Informational,
        message,
    }
}

/// Consumes the [TransactionState], reporting an informational [Event] describing its full lifecycle
fn into_summary(key: TransactionKey, state: TransactionState) -> Event {
    let lifecycle = state.lifecycle;
    let message = format!("LPP::Transaction[{key}]::Summary[{lifecycle}]");
    Event {
        event_type: EventType::Informational,
        message,
    }
}

fn into_abort_cause(value: telcom_parser::lte_lpp::Abort) -> Option<u8> {
    match value.critical_extensions {
        AbortCriticalExtensions::C1(AbortCriticalExtensions_c1::Abort_r9(abort)) => {
            abort.common_i_es_abort.as_ref().map(|i| i.abort_cause.0)
        }
        _ => None,
    }
}

fn into_error_cause(value: telcom_parser::lte_lpp::Error) -> Option<u8> {
    match value {
        Error_r9(inner) => inner.common_i_es_error.as_ref().map(|i| i.error_cause.0),
        _ => None,
    }
}

#[cfg(test)]
mod tst {
    use chrono::{DateTime, Duration, FixedOffset, TimeZone};
    use telcom_parser::lte_lpp::TransactionNumber;

    use super::*;

    const OFFSET: FixedOffset = FixedOffset::east_opt(0).unwrap();
    const DURATION_SHORT: Duration = Duration::milliseconds(10);

    #[test]
    fn progress() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);
        let t1 = t0 + INTERVAL_SWEEP;

        let k = key(0xde, 0xad, 0xbe, 0xef, 0, 0);
        s.accept(t0, k, RequestLocationInformation, false);
        let event = s.next(t0);
        assert!(event.is_none(), "progress is not emitted immediately");

        let event = s.next(t1);
        assert!(
            event.is_some(),
            "progress is emitted after duration: {INTERVAL_SWEEP}"
        );
        assert_eq!(
            "LPP::Transaction[deadbeef/s000]::Progress[RequestLocationInformation]",
            event.unwrap().message
        );
        assert!(
            !s.transactions.is_empty(),
            "active transaction is not removed"
        );
    }

    #[test]
    fn summary() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);
        let t1 = t0 + DURATION_SHORT;
        let t2 = t1 + DURATION_SHORT;

        let k = key(0xde, 0xad, 0xbe, 0xef, 0, 0);
        s.accept(t0, k.clone(), RequestLocationInformation, false);
        s.accept(t1, k.clone(), ProvideLocationInformation, false);
        let event = s.next(t2);
        assert!(
            event.is_none(),
            "progress deferred before: {TIMEOUT_FIRST_REPORT}"
        );

        s.accept(t2, k, ProvideLocationInformation, true);
        let event = s.next(t2);
        assert!(event.is_some(), "summary is emitted immediately after end");
        assert_eq!(
            "LPP::Transaction[deadbeef/s000]::Summary[RequestLocationInformation -> ProvideLocationInformation x2]",
            event.unwrap().message
        );
        assert!(s.transactions.is_empty(), "finished transaction is removed");
    }

    #[test]
    fn abort() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);

        let k = key(0xde, 0xad, 0xbe, 0xef, 0, 0);
        s.accept(t0, k.clone(), RequestLocationInformation, false);
        s.accept(t0, k.clone(), Abort(None), false);
        let event = s.next(t0);
        assert!(event.is_some(), "summary emitted immediately");
        assert!(s.transactions.is_empty(), "transaction popped");
        let message = event.unwrap().message;
        assert_eq!(
            "LPP::Transaction[deadbeef/s000]::Summary[RequestLocationInformation -> Abort(NONE)]",
            message
        );
    }

    #[test]
    fn error() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);

        let k = key(0xde, 0xad, 0xbe, 0xef, 0, 0);
        s.accept(t0, k.clone(), RequestLocationInformation, false);
        s.accept(t0, k.clone(), Error(None), false);
        let event = s.next(t0);
        assert!(event.is_some(), "summary emitted immediately");
        assert!(s.transactions.is_empty(), "transaction popped");
        let message = event.unwrap().message;
        assert_eq!(
            "LPP::Transaction[deadbeef/s000]::Summary[RequestLocationInformation -> Error(NONE)]",
            message
        );
    }

    #[test]
    fn evicted() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);

        let k_evict = key(0xff, 0, 0, 0, 0, 0);
        s.accept(t0, k_evict, RequestLocationInformation, false);
        assert!(s.pending.is_empty(), "no event after t_evict");

        for i in 0..MAX_TRANSACTIONS - 1 {
            let ki = key(0, 0, 0, 0, 0, i as u8);
            let di = DURATION_SHORT * (i as i32 + 1);
            let ti = t0 + di;
            s.accept(ti, ki.clone(), RequestLocationInformation, false);
            // Another message on an existing transaction doesn't evict when full
            s.accept(ti, ki, RequestLocationInformation, false);
            assert!(s.pending.is_empty(), "no event after t{i} ({di})");
        }
        assert_eq!(MAX_TRANSACTIONS, s.transactions.len(), "at max capacity");

        let k_mx = key(0, 0, 0, 0, 0, MAX_TRANSACTIONS as u8);
        let d_mx = DURATION_SHORT * (MAX_TRANSACTIONS as i32 + 1);
        let t_mx = t0 + d_mx;
        s.accept(t_mx, k_mx, RequestLocationInformation, false);
        assert_eq!(
            MAX_TRANSACTIONS,
            s.transactions.len(),
            "still at max capacity"
        );
        assert_eq!(1, s.pending.len(), "one evicted");

        let event = s.next(t_mx).unwrap().message;
        assert_eq!(
            "LPP::Transaction[ff000000/s000]::Summary[RequestLocationInformation -> Evicted]",
            event
        );
    }

    #[test]
    fn timeout() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);
        let t_timeout = t0 + TIMEOUT_TRANSACTION + DURATION_SHORT;

        let k = key(0xde, 0xad, 0xbe, 0xef, 0, 0);
        s.accept(t0, k, RequestLocationInformation, false);
        assert_eq!(1, s.transactions.len());
        assert!(s.pending.is_empty(), "no event emitted on intake");

        let event = s.next(t_timeout).unwrap().message;
        assert!(s.transactions.is_empty(), "transaction is removed");
        assert_eq!(
            "LPP::Transaction[deadbeef/s000]::Summary[RequestLocationInformation -> Timeout]",
            event
        );
    }

    #[test]
    fn sweep() {
        let mut s = State::default();
        let t0: DateTime<FixedOffset> = ts(2026, 10, 11, 14, 0, 0);
        let t_sweep0 = t0 + INTERVAL_SWEEP;
        let t_sweep1 = t_sweep0 + INTERVAL_SWEEP;
        let t_sweep2 = t_sweep1 + INTERVAL_SWEEP;

        let k0 = key(0, 0, 0, 0, 0, 0);
        let k1 = key(0, 0, 0, 1, 0, 0);
        s.accept(t0, k0, RequestLocationInformation, false);
        s.accept(
            t0 + TIMEOUT_FIRST_REPORT,
            k1,
            RequestLocationInformation,
            false,
        );
        assert_eq!(2, s.transactions.len());
        assert!(s.pending.is_empty(), "no event emitted on intake");

        assert!(s.next(t_sweep0).is_some(), "one event popped");
        assert!(s.pending.is_empty(), "sweep pushed one event");
        assert!(
            s.next(t_sweep0 + TIMEOUT_FIRST_REPORT).is_none(),
            "sweep rate limited"
        );

        assert!(s.next(t_sweep1).is_some(), "remaining event popped");
        assert!(s.pending.is_empty(), "no more events");
        assert!(s.next(t_sweep2).is_none(), "no more events");
    }

    fn ts(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<FixedOffset> {
        OFFSET
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .unwrap()
    }

    fn key(r0: u8, r1: u8, r2: u8, r3: u8, tx_init: u8, tx_num: u8) -> TransactionKey {
        let tx_id = LPP_TransactionID {
            initiator: Initiator(tx_init),
            transaction_number: TransactionNumber(tx_num),
        };
        let route_id = vec![r0, r1, r2, r3];
        TransactionKey::new(Some(route_id), Some(tx_id))
    }
}
