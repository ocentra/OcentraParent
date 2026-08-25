use serde::{Deserialize, Serialize};

use crate::{
    AggregateKey, CausationId, CorrelationId, DomainEvent, EventContract, EventCustody, EventId,
    EventSource, EventType, EventingError, IdempotencyKey, SchemaVersion, StoredEventEnvelope,
    SubscriberId, TargetHandler,
};

const DEAD_LETTER_CREATED_EVENT_TYPE: &str = "eventing.dead_letter.created";
const DEAD_LETTER_CREATED_SCHEMA_VERSION: u16 = 1;
const DEAD_LETTER_IDEMPOTENCY_PREFIX: &str = "dead-letter";
const DEAD_LETTER_IDEMPOTENCY_SEPARATOR: &str = "-";

pub fn dead_letter_recorded_event_type() -> Result<EventType, EventingError> {
    EventType::parse(DEAD_LETTER_CREATED_EVENT_TYPE)
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeadLetter {
    pub envelope: StoredEventEnvelope,
    pub subscriber_id: Option<SubscriberId>,
    pub target_handler: Option<TargetHandler>,
    pub reason: DeadLetterReason,
    pub error: EventingError,
    pub retry_state: DeadLetterRetryState,
}

impl DeadLetter {
    pub(super) fn for_handler(
        stored: &StoredEventEnvelope,
        report: &super::handler::HandlerReport,
    ) -> Option<Self> {
        report.error.clone().map(|error| Self {
            envelope: stored.clone(),
            subscriber_id: Some(report.subscriber_id.clone()),
            target_handler: Some(report.target_handler.clone()),
            reason: report.outcome.dead_letter_reason(),
            error,
            retry_state: DeadLetterRetryState::for_handler(report.outcome, report.attempts),
        })
    }

    pub(crate) fn for_queue(
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
            retry_state: DeadLetterRetryState::NotAttempted,
        }
    }

    pub fn as_event(&self) -> DeadLetterEvent {
        DeadLetterEvent {
            original_event_id: self.envelope.event_id.clone(),
            original_event_type: self.envelope.contract.event_type.clone(),
            original_correlation_id: self.envelope.correlation_id.clone(),
            original_causation_id: self.envelope.causation_id.clone(),
            custody: self.envelope.source.custody.clone(),
            source: self.envelope.source.clone(),
            reason: self.reason,
            retry_state: self.retry_state,
            subscriber_id: self.subscriber_id.clone(),
            target_handler: self.target_handler.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum DeadLetterRetryState {
    NotAttempted,
    Exhausted { attempts: usize },
    DeadlineExpired { attempts: usize },
}

impl DeadLetterRetryState {
    fn for_handler(outcome: super::handler::HandlerOutcome, attempts: usize) -> Self {
        if outcome == super::handler::HandlerOutcome::DeadlineExpired {
            Self::DeadlineExpired { attempts }
        } else {
            Self::Exhausted { attempts }
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
    pub original_causation_id: Option<CausationId>,
    pub custody: EventCustody,
    pub source: EventSource,
    pub reason: DeadLetterReason,
    pub retry_state: DeadLetterRetryState,
    pub subscriber_id: Option<SubscriberId>,
    pub target_handler: Option<TargetHandler>,
}

impl DomainEvent for DeadLetterEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            dead_letter_recorded_event_type()?,
            SchemaVersion::new(DEAD_LETTER_CREATED_SCHEMA_VERSION)?,
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
