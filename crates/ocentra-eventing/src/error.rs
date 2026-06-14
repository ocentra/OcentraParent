use std::{error::Error, fmt};

use crate::{EventId, EventType, IdempotencyKey, RequestId, SchemaVersion, SubscriberId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventingError {
    EmptyValue {
        field: &'static str,
    },
    InvalidValue {
        field: &'static str,
        value: String,
    },
    InvalidVersion,
    PayloadEncode {
        reason: String,
    },
    PayloadDecode {
        event_type: EventType,
        reason: String,
    },
    ContractMismatch {
        expected: EventType,
        received: EventType,
        expected_schema_version: SchemaVersion,
        received_schema_version: SchemaVersion,
    },
    DuplicateEventContract {
        event_type: EventType,
    },
    DuplicateSubscriber {
        subscriber_id: SubscriberId,
    },
    HandlerPanicked {
        subscriber_id: SubscriberId,
    },
    HandlerTimedOut {
        subscriber_id: SubscriberId,
    },
    InvalidHandlerPolicy {
        reason: String,
    },
    InvalidQueuePolicy {
        reason: String,
    },
    NoSubscriber {
        event_type: EventType,
    },
    QueueCapacityExceeded {
        event_type: EventType,
        capacity: usize,
    },
    EventDeadlineExpired {
        event_type: EventType,
    },
    DuplicateEventId {
        event_id: EventId,
    },
    DuplicateInFlight {
        idempotency_key: IdempotencyKey,
    },
    DuplicateIdempotencyKey {
        idempotency_key: IdempotencyKey,
    },
    InvalidRequestOptions {
        reason: String,
    },
    DuplicateRequest {
        request_id: RequestId,
    },
    RequestTimedOut {
        request_id: RequestId,
    },
    RequestResponseEncode {
        request_id: RequestId,
        reason: String,
    },
    RequestResponseDecode {
        request_id: RequestId,
        reason: String,
    },
    BusShutdown,
    JournalIo {
        path: String,
        reason: String,
    },
    JournalEncode {
        reason: String,
    },
    JournalDecode {
        reason: String,
    },
    JournalCorruptLine {
        line: usize,
        reason: String,
    },
    ReplayActionNotAllowed {
        event_type: EventType,
    },
    RegistrarDisposed,
}

impl EventingError {
    pub(crate) fn empty_value(field: &'static str) -> Self {
        Self::EmptyValue { field }
    }

    pub(crate) fn invalid_value(field: &'static str, value: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            value: value.into(),
        }
    }

    pub(crate) fn payload_encode(error: serde_json::Error) -> Self {
        Self::PayloadEncode {
            reason: error.to_string(),
        }
    }

    pub(crate) fn payload_decode(event_type: EventType, error: serde_json::Error) -> Self {
        Self::PayloadDecode {
            event_type,
            reason: error.to_string(),
        }
    }

    pub(crate) fn journal_io(path: String, error: std::io::Error) -> Self {
        Self::JournalIo {
            path,
            reason: error.to_string(),
        }
    }

    pub(crate) fn journal_encode(error: serde_json::Error) -> Self {
        Self::JournalEncode {
            reason: error.to_string(),
        }
    }
}

impl fmt::Display for EventingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { .. }
            | Self::InvalidValue { .. }
            | Self::InvalidVersion
            | Self::PayloadEncode { .. }
            | Self::PayloadDecode { .. }
            | Self::ContractMismatch { .. }
            | Self::DuplicateEventContract { .. }
            | Self::DuplicateSubscriber { .. }
            | Self::HandlerPanicked { .. }
            | Self::HandlerTimedOut { .. }
            | Self::InvalidHandlerPolicy { .. }
            | Self::InvalidQueuePolicy { .. }
            | Self::NoSubscriber { .. }
            | Self::QueueCapacityExceeded { .. }
            | Self::EventDeadlineExpired { .. }
            | Self::DuplicateEventId { .. }
            | Self::DuplicateInFlight { .. }
            | Self::DuplicateIdempotencyKey { .. } => fmt_core_error(self, formatter),
            Self::InvalidRequestOptions { .. }
            | Self::DuplicateRequest { .. }
            | Self::RequestTimedOut { .. }
            | Self::RequestResponseEncode { .. }
            | Self::RequestResponseDecode { .. }
            | Self::BusShutdown => fmt_request_error(self, formatter),
            Self::JournalIo { .. }
            | Self::JournalEncode { .. }
            | Self::JournalDecode { .. }
            | Self::JournalCorruptLine { .. }
            | Self::ReplayActionNotAllowed { .. } => fmt_journal_error(self, formatter),
            Self::RegistrarDisposed => formatter.write_str("event registrar is disposed"),
        }
    }
}

fn fmt_core_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::EmptyValue { .. }
        | EventingError::InvalidValue { .. }
        | EventingError::InvalidVersion
        | EventingError::PayloadEncode { .. }
        | EventingError::InvalidHandlerPolicy { .. }
        | EventingError::InvalidQueuePolicy { .. } => fmt_core_config_error(error, formatter),
        EventingError::PayloadDecode { .. }
        | EventingError::ContractMismatch { .. }
        | EventingError::DuplicateEventContract { .. } => fmt_contract_error(error, formatter),
        EventingError::DuplicateSubscriber { .. }
        | EventingError::HandlerPanicked { .. }
        | EventingError::HandlerTimedOut { .. } => fmt_subscriber_error(error, formatter),
        EventingError::NoSubscriber { .. }
        | EventingError::QueueCapacityExceeded { .. }
        | EventingError::EventDeadlineExpired { .. }
        | EventingError::DuplicateEventId { .. }
        | EventingError::DuplicateInFlight { .. }
        | EventingError::DuplicateIdempotencyKey { .. } => fmt_queue_error(error, formatter),
        _ => unreachable!("core eventing error formatter received non-core error"),
    }
}

fn fmt_core_config_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::EmptyValue { field } => write!(formatter, "empty eventing value: {field}"),
        EventingError::InvalidValue { field, value } => {
            write!(formatter, "invalid eventing value for {field}: {value}")
        }
        EventingError::InvalidVersion => {
            formatter.write_str("event schema version must be nonzero")
        }
        EventingError::PayloadEncode { reason } => {
            write!(formatter, "payload encode failed: {reason}")
        }
        EventingError::InvalidHandlerPolicy { reason } => {
            write!(formatter, "invalid event handler policy: {reason}")
        }
        EventingError::InvalidQueuePolicy { reason } => {
            write!(formatter, "invalid event queue policy: {reason}")
        }
        _ => unreachable!("core config formatter received non-config error"),
    }
}

fn fmt_contract_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::PayloadDecode { event_type, reason } => {
            write!(
                formatter,
                "payload decode failed for {}: {reason}",
                event_type.as_str()
            )
        }
        EventingError::ContractMismatch {
            expected,
            received,
            expected_schema_version,
            received_schema_version,
        } => write!(
            formatter,
            "event contract mismatch: expected {}@{}, received {}@{}",
            expected.as_str(),
            expected_schema_version.value(),
            received.as_str(),
            received_schema_version.value()
        ),
        EventingError::DuplicateEventContract { event_type } => {
            write!(
                formatter,
                "duplicate event contract: {}",
                event_type.as_str()
            )
        }
        _ => unreachable!("contract formatter received non-contract error"),
    }
}

fn fmt_subscriber_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::DuplicateSubscriber { subscriber_id } => {
            write!(
                formatter,
                "duplicate subscriber: {}",
                subscriber_id.as_str()
            )
        }
        EventingError::HandlerPanicked { subscriber_id } => {
            write!(
                formatter,
                "event handler panicked: {}",
                subscriber_id.as_str()
            )
        }
        EventingError::HandlerTimedOut { subscriber_id } => {
            write!(
                formatter,
                "event handler timed out: {}",
                subscriber_id.as_str()
            )
        }
        _ => unreachable!("subscriber formatter received non-subscriber error"),
    }
}

fn fmt_queue_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::NoSubscriber { event_type } => {
            write!(
                formatter,
                "no subscriber for event type: {}",
                event_type.as_str()
            )
        }
        EventingError::QueueCapacityExceeded {
            event_type,
            capacity,
        } => write!(
            formatter,
            "event queue capacity exceeded for {}: {capacity}",
            event_type.as_str()
        ),
        EventingError::EventDeadlineExpired { event_type } => {
            write!(
                formatter,
                "event deadline expired for {}",
                event_type.as_str()
            )
        }
        EventingError::DuplicateEventId { event_id } => {
            write!(formatter, "duplicate event id: {}", event_id.as_str())
        }
        EventingError::DuplicateInFlight { idempotency_key } => {
            write!(
                formatter,
                "duplicate in-flight event: {}",
                idempotency_key.as_str()
            )
        }
        EventingError::DuplicateIdempotencyKey { idempotency_key } => {
            write!(
                formatter,
                "duplicate idempotency key: {}",
                idempotency_key.as_str()
            )
        }
        _ => unreachable!("queue formatter received non-queue error"),
    }
}

fn fmt_request_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::InvalidRequestOptions { reason } => {
            write!(formatter, "invalid event request options: {reason}")
        }
        EventingError::DuplicateRequest { request_id } => {
            write!(formatter, "duplicate request id: {}", request_id.as_str())
        }
        EventingError::RequestTimedOut { request_id } => {
            write!(
                formatter,
                "event request timed out: {}",
                request_id.as_str()
            )
        }
        EventingError::RequestResponseEncode { request_id, reason } => {
            write!(
                formatter,
                "event request response encode failed for {}: {reason}",
                request_id.as_str()
            )
        }
        EventingError::RequestResponseDecode { request_id, reason } => {
            write!(
                formatter,
                "event request response decode failed for {}: {reason}",
                request_id.as_str()
            )
        }
        EventingError::BusShutdown => formatter.write_str("event bus is shut down"),
        _ => unreachable!("request error formatter received non-request error"),
    }
}

fn fmt_journal_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::JournalIo { path, reason } => {
            write!(formatter, "event journal io failed for {path}: {reason}")
        }
        EventingError::JournalEncode { reason } => {
            write!(formatter, "event journal encode failed: {reason}")
        }
        EventingError::JournalDecode { reason } => {
            write!(formatter, "event journal decode failed: {reason}")
        }
        EventingError::JournalCorruptLine { line, reason } => {
            write!(formatter, "event journal corrupt line {line}: {reason}")
        }
        EventingError::ReplayActionNotAllowed { event_type } => {
            write!(
                formatter,
                "event replay action handlers are not allowed for {}",
                event_type.as_str()
            )
        }
        _ => unreachable!("journal error formatter received non-journal error"),
    }
}

impl Error for EventingError {}
