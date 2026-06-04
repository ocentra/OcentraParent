use std::{error::Error, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventingError {
    EmptyValue { field: &'static str },
    InvalidVersion,
    PayloadEncode { reason: String },
    PayloadDecode { event_type: String, reason: String },
    ContractMismatch { expected: String, received: String },
    DuplicateSubscriber { subscriber_id: String },
    HandlerPanicked { subscriber_id: String },
    HandlerTimedOut { subscriber_id: String },
    InvalidHandlerPolicy { reason: String },
    InvalidQueuePolicy { reason: String },
    NoSubscriber { event_type: String },
    QueueCapacityExceeded { event_type: String, capacity: usize },
    DuplicateInFlight { idempotency_key: String },
    DuplicateIdempotencyKey { idempotency_key: String },
    RegistrarDisposed,
}

impl EventingError {
    pub(crate) fn empty_value(field: &'static str) -> Self {
        Self::EmptyValue { field }
    }

    pub(crate) fn payload_encode(error: serde_json::Error) -> Self {
        Self::PayloadEncode {
            reason: error.to_string(),
        }
    }

    pub(crate) fn payload_decode(event_type: String, error: serde_json::Error) -> Self {
        Self::PayloadDecode {
            event_type,
            reason: error.to_string(),
        }
    }
}

impl fmt::Display for EventingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => write!(formatter, "empty eventing value: {field}"),
            Self::InvalidVersion => formatter.write_str("event schema version must be nonzero"),
            Self::PayloadEncode { reason } => write!(formatter, "payload encode failed: {reason}"),
            Self::PayloadDecode { event_type, reason } => {
                write!(
                    formatter,
                    "payload decode failed for {event_type}: {reason}"
                )
            }
            Self::ContractMismatch { expected, received } => {
                write!(
                    formatter,
                    "event contract mismatch: expected {expected}, received {received}"
                )
            }
            Self::DuplicateSubscriber { subscriber_id } => {
                write!(formatter, "duplicate subscriber: {subscriber_id}")
            }
            Self::HandlerPanicked { subscriber_id } => {
                write!(formatter, "event handler panicked: {subscriber_id}")
            }
            Self::HandlerTimedOut { subscriber_id } => {
                write!(formatter, "event handler timed out: {subscriber_id}")
            }
            Self::InvalidHandlerPolicy { reason } => {
                write!(formatter, "invalid event handler policy: {reason}")
            }
            Self::InvalidQueuePolicy { reason } => {
                write!(formatter, "invalid event queue policy: {reason}")
            }
            Self::NoSubscriber { event_type } => {
                write!(formatter, "no subscriber for event type: {event_type}")
            }
            Self::QueueCapacityExceeded {
                event_type,
                capacity,
            } => {
                write!(
                    formatter,
                    "event queue capacity exceeded for {event_type}: {capacity}"
                )
            }
            Self::DuplicateInFlight { idempotency_key } => {
                write!(formatter, "duplicate in-flight event: {idempotency_key}")
            }
            Self::DuplicateIdempotencyKey { idempotency_key } => {
                write!(formatter, "duplicate idempotency key: {idempotency_key}")
            }
            Self::RegistrarDisposed => formatter.write_str("event registrar is disposed"),
        }
    }
}

impl Error for EventingError {}
