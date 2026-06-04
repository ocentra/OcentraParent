use std::{error::Error, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventingError {
    EmptyValue { field: &'static str },
    InvalidVersion,
    PayloadEncode { reason: String },
    PayloadDecode { event_type: String, reason: String },
    ContractMismatch { expected: String, received: String },
    DuplicateEventContract { event_type: String },
    DuplicateSubscriber { subscriber_id: String },
    HandlerPanicked { subscriber_id: String },
    HandlerTimedOut { subscriber_id: String },
    InvalidHandlerPolicy { reason: String },
    InvalidQueuePolicy { reason: String },
    NoSubscriber { event_type: String },
    QueueCapacityExceeded { event_type: String, capacity: usize },
    EventDeadlineExpired { event_type: String },
    DuplicateInFlight { idempotency_key: String },
    DuplicateIdempotencyKey { idempotency_key: String },
    InvalidRequestOptions { reason: String },
    DuplicateRequest { request_id: String },
    RequestTimedOut { request_id: String },
    RequestResponseEncode { request_id: String, reason: String },
    RequestResponseDecode { request_id: String, reason: String },
    JournalIo { path: String, reason: String },
    JournalEncode { reason: String },
    JournalDecode { reason: String },
    JournalCorruptLine { line: usize, reason: String },
    ReplayActionNotAllowed { event_type: String },
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
            | Self::DuplicateInFlight { .. }
            | Self::DuplicateIdempotencyKey { .. } => fmt_core_error(self, formatter),
            Self::InvalidRequestOptions { .. }
            | Self::DuplicateRequest { .. }
            | Self::RequestTimedOut { .. }
            | Self::RequestResponseEncode { .. }
            | Self::RequestResponseDecode { .. } => fmt_request_error(self, formatter),
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
        EventingError::EmptyValue { field } => {
            write!(formatter, "empty eventing value: {field}")
        }
        EventingError::InvalidVersion => {
            formatter.write_str("event schema version must be nonzero")
        }
        EventingError::PayloadEncode { reason } => {
            write!(formatter, "payload encode failed: {reason}")
        }
        EventingError::PayloadDecode { event_type, reason } => {
            write!(
                formatter,
                "payload decode failed for {event_type}: {reason}"
            )
        }
        EventingError::ContractMismatch { expected, received } => {
            write!(
                formatter,
                "event contract mismatch: expected {expected}, received {received}"
            )
        }
        EventingError::DuplicateEventContract { event_type } => {
            write!(formatter, "duplicate event contract: {event_type}")
        }
        EventingError::DuplicateSubscriber { subscriber_id } => {
            write!(formatter, "duplicate subscriber: {subscriber_id}")
        }
        EventingError::HandlerPanicked { subscriber_id } => {
            write!(formatter, "event handler panicked: {subscriber_id}")
        }
        EventingError::HandlerTimedOut { subscriber_id } => {
            write!(formatter, "event handler timed out: {subscriber_id}")
        }
        EventingError::InvalidHandlerPolicy { reason } => {
            write!(formatter, "invalid event handler policy: {reason}")
        }
        EventingError::InvalidQueuePolicy { reason } => {
            write!(formatter, "invalid event queue policy: {reason}")
        }
        EventingError::NoSubscriber { event_type } => {
            write!(formatter, "no subscriber for event type: {event_type}")
        }
        EventingError::QueueCapacityExceeded {
            event_type,
            capacity,
        } => write!(
            formatter,
            "event queue capacity exceeded for {event_type}: {capacity}"
        ),
        EventingError::EventDeadlineExpired { event_type } => {
            write!(formatter, "event deadline expired for {event_type}")
        }
        EventingError::DuplicateInFlight { idempotency_key } => {
            write!(formatter, "duplicate in-flight event: {idempotency_key}")
        }
        EventingError::DuplicateIdempotencyKey { idempotency_key } => {
            write!(formatter, "duplicate idempotency key: {idempotency_key}")
        }
        _ => unreachable!("core eventing error formatter received non-core error"),
    }
}

fn fmt_request_error(error: &EventingError, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match error {
        EventingError::InvalidRequestOptions { reason } => {
            write!(formatter, "invalid event request options: {reason}")
        }
        EventingError::DuplicateRequest { request_id } => {
            write!(formatter, "duplicate request id: {request_id}")
        }
        EventingError::RequestTimedOut { request_id } => {
            write!(formatter, "event request timed out: {request_id}")
        }
        EventingError::RequestResponseEncode { request_id, reason } => {
            write!(
                formatter,
                "event request response encode failed for {request_id}: {reason}"
            )
        }
        EventingError::RequestResponseDecode { request_id, reason } => {
            write!(
                formatter,
                "event request response decode failed for {request_id}: {reason}"
            )
        }
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
                "event replay action handlers are not allowed for {event_type}"
            )
        }
        _ => unreachable!("journal error formatter received non-journal error"),
    }
}

impl Error for EventingError {}
