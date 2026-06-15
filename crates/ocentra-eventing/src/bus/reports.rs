use serde::{Deserialize, Serialize};

use crate::envelope::{DomainEvent, EventContract, StoredEventEnvelope};
use crate::error::EventingError;
use crate::ids::{
    AggregateKey, CorrelationId, EventId, EventType, IdempotencyKey, SchemaVersion, SubscriberId,
    TargetHandler,
};
use crate::queue::policy::QueueReport;

use super::DispatchMode;

const DEAD_LETTER_RECORDED_EVENT_TYPE: &str = "eventing.dead_letter.recorded";
const DEAD_LETTER_RECORDED_SCHEMA_VERSION: u16 = 1;
const DEAD_LETTER_IDEMPOTENCY_PREFIX: &str = "dead-letter";
const DEAD_LETTER_IDEMPOTENCY_SEPARATOR: &str = "-";

pub fn dead_letter_recorded_event_type() -> Result<EventType, EventingError> {
    EventType::parse(DEAD_LETTER_RECORDED_EVENT_TYPE)
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeadLetter {
    pub envelope: StoredEventEnvelope,
    pub subscriber_id: Option<SubscriberId>,
    pub target_handler: Option<TargetHandler>,
    pub reason: DeadLetterReason,
    pub error: EventingError,
}

impl DeadLetter {
    pub(super) fn for_handler(
        stored: &StoredEventEnvelope,
        report: &HandlerReport,
    ) -> Option<Self> {
        report.error.clone().map(|error| Self {
            envelope: stored.clone(),
            subscriber_id: Some(report.subscriber_id.clone()),
            target_handler: Some(report.target_handler.clone()),
            reason: report.outcome.dead_letter_reason(),
            error,
        })
    }

    pub(super) fn for_queue(
        stored: &StoredEventEnvelope,
        reason: DeadLetterReason,
        error: EventingError,
    ) -> Self {
        Self {
            envelope: stored.clone(),
            subscriber_id: None,
            target_handler: None,
            reason,
            error,
        }
    }

    pub fn as_event(&self) -> DeadLetterEvent {
        DeadLetterEvent {
            original_event_id: self.envelope.event_id.clone(),
            original_event_type: self.envelope.contract.event_type.clone(),
            original_correlation_id: self.envelope.correlation_id.clone(),
            reason: self.reason,
            subscriber_id: self.subscriber_id.clone(),
            target_handler: self.target_handler.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeadLetterReason {
    HandlerFailed,
    HandlerTimedOut,
    HandlerDeadlineExpired,
    HandlerPanicked,
    NoSubscriber,
    QueueOverflow,
    QueueExpired,
    DeadlineExpired,
    Shutdown,
}

impl DeadLetterReason {
    pub(crate) fn idempotency_label(self) -> &'static str {
        match self {
            Self::HandlerFailed => "handler-failed",
            Self::HandlerTimedOut => "handler-timed-out",
            Self::HandlerDeadlineExpired => "handler-deadline-expired",
            Self::HandlerPanicked => "handler-panicked",
            Self::NoSubscriber => "no-subscriber",
            Self::QueueOverflow => "queue-overflow",
            Self::QueueExpired => "queue-expired",
            Self::DeadlineExpired => "deadline-expired",
            Self::Shutdown => "shutdown",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeadLetterEvent {
    pub original_event_id: EventId,
    pub original_event_type: EventType,
    pub original_correlation_id: CorrelationId,
    pub reason: DeadLetterReason,
    pub subscriber_id: Option<SubscriberId>,
    pub target_handler: Option<TargetHandler>,
}

impl DomainEvent for DeadLetterEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            dead_letter_recorded_event_type()?,
            SchemaVersion::new(DEAD_LETTER_RECORDED_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.original_event_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(DEAD_LETTER_IDEMPOTENCY_PREFIX);
        value.push_str(DEAD_LETTER_IDEMPOTENCY_SEPARATOR);
        value.push_str(self.original_event_id.as_str());
        value.push_str(DEAD_LETTER_IDEMPOTENCY_SEPARATOR);
        value.push_str(self.reason.idempotency_label());
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerOutcome {
    Handled,
    Failed,
    TimedOut,
    DeadlineExpired,
    Panicked,
}

impl HandlerOutcome {
    fn dead_letter_reason(self) -> DeadLetterReason {
        match self {
            Self::Handled => DeadLetterReason::HandlerFailed,
            Self::Failed => DeadLetterReason::HandlerFailed,
            Self::TimedOut => DeadLetterReason::HandlerTimedOut,
            Self::DeadlineExpired => DeadLetterReason::HandlerDeadlineExpired,
            Self::Panicked => DeadLetterReason::HandlerPanicked,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandlerReport {
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub outcome: HandlerOutcome,
    pub error: Option<EventingError>,
    pub attempts: usize,
    pub trace: EventTraceFields,
}

impl HandlerReport {
    pub(super) fn new(
        stored: &StoredEventEnvelope,
        subscriber_id: SubscriberId,
        target_handler: TargetHandler,
        outcome: HandlerOutcome,
        error: Option<EventingError>,
        attempts: usize,
    ) -> Self {
        let trace = EventTraceFields {
            event_id: stored.event_id.clone(),
            event_type: stored.contract.event_type.clone(),
            correlation_id: stored.correlation_id.clone(),
            subscriber_id: subscriber_id.clone(),
            target_handler: target_handler.clone(),
            outcome,
            attempts,
        };
        Self {
            subscriber_id,
            target_handler,
            outcome,
            error,
            attempts,
            trace,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventTraceFields {
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub outcome: HandlerOutcome,
    pub attempts: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishReport {
    pub event_id: EventId,
    pub event_type: EventType,
    pub dispatch_mode: DispatchMode,
    pub queue_report: QueueReport,
    pub subscriber_count: usize,
    pub handled_count: usize,
    pub dead_letter_count: usize,
    pub handler_reports: Vec<HandlerReport>,
}

impl PublishReport {
    pub fn no_subscribers(&self) -> bool {
        self.subscriber_count == 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueDrainReport {
    pub queued_before: usize,
    pub dispatched_count: usize,
    pub expired_count: usize,
    pub remaining_count: usize,
    pub dispatch_reports: Vec<PublishReport>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventMetricsSnapshot {
    pub subscription_count: usize,
    pub stored_event_count: usize,
    pub dead_letter_count: usize,
    pub queue: EventQueueMetrics,
    pub requests: EventRequestMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventQueueMetrics {
    pub queued_event_count: usize,
    pub queued_event_id_count: usize,
    pub queued_idempotency_key_count: usize,
    pub in_flight_event_id_count: usize,
    pub in_flight_idempotency_key_count: usize,
    pub completed_idempotency_key_count: usize,
    pub capacity: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventRequestMetrics {
    pub pending_request_count: usize,
    pub completed_request_count: usize,
    pub timed_out_request_count: usize,
}

pub(super) fn dead_letters_for(
    stored: &StoredEventEnvelope,
    reports: &[HandlerReport],
) -> Vec<DeadLetter> {
    reports
        .iter()
        .filter_map(|report| DeadLetter::for_handler(stored, report))
        .collect()
}

pub(super) fn empty_publish_report(
    stored: &StoredEventEnvelope,
    dispatch_mode: DispatchMode,
    queue_report: QueueReport,
    dead_letter_count: usize,
) -> PublishReport {
    PublishReport {
        event_id: stored.event_id.clone(),
        event_type: stored.contract.event_type.clone(),
        dispatch_mode,
        queue_report,
        subscriber_count: 0,
        handled_count: 0,
        dead_letter_count,
        handler_reports: Vec::new(),
    }
}
