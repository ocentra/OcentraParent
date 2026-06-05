use std::sync::atomic::{AtomicU64, Ordering};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::EventingError;

static EVENT_ID_SEQUENCE: AtomicU64 = AtomicU64::new(1);
static REQUEST_ID_SEQUENCE: AtomicU64 = AtomicU64::new(1);

const EVENT_ID_PREFIX: &str = "event-";
const REQUEST_ID_PREFIX: &str = "request-";
const EVENT_ID_SEPARATOR: &str = "-";
const EVENT_TYPE_LABEL: &str = "event_type";
const EVENT_NAMESPACE_LABEL: &str = "event_namespace";
const EVENT_ID_LABEL: &str = "event_id";
const CORRELATION_ID_LABEL: &str = "correlation_id";
const CAUSATION_ID_LABEL: &str = "causation_id";
const REQUEST_ID_LABEL: &str = "request_id";
const JOURNAL_HASH_LABEL: &str = "journal_hash";
const AGGREGATE_KEY_LABEL: &str = "aggregate_key";
const IDEMPOTENCY_KEY_LABEL: &str = "idempotency_key";
const SUBSCRIBER_ID_LABEL: &str = "subscriber_id";
const TARGET_HANDLER_LABEL: &str = "target_handler";
const EVENT_CUSTODY_LABEL: &str = "event_custody";
const RUNTIME_ROLE_LABEL: &str = "runtime_role";
const SOURCE_SERVICE_LABEL: &str = "source_service";
const SOURCE_COMPONENT_LABEL: &str = "source_component";
const RUNTIME_INSTANCE_ID_LABEL: &str = "runtime_instance_id";
const RECORDED_AT_LABEL: &str = "recorded_at";

macro_rules! text_identifier {
    ($name:ident, $label:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                validate_text($label, value.into()).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

text_identifier!(EventType, EVENT_TYPE_LABEL);
text_identifier!(EventNamespace, EVENT_NAMESPACE_LABEL);
text_identifier!(EventId, EVENT_ID_LABEL);
text_identifier!(CorrelationId, CORRELATION_ID_LABEL);
text_identifier!(CausationId, CAUSATION_ID_LABEL);
text_identifier!(RequestId, REQUEST_ID_LABEL);
text_identifier!(JournalHash, JOURNAL_HASH_LABEL);
text_identifier!(AggregateKey, AGGREGATE_KEY_LABEL);
text_identifier!(IdempotencyKey, IDEMPOTENCY_KEY_LABEL);
text_identifier!(SubscriberId, SUBSCRIBER_ID_LABEL);
text_identifier!(TargetHandler, TARGET_HANDLER_LABEL);
text_identifier!(EventCustody, EVENT_CUSTODY_LABEL);
text_identifier!(RuntimeRole, RUNTIME_ROLE_LABEL);
text_identifier!(SourceService, SOURCE_SERVICE_LABEL);
text_identifier!(SourceComponent, SOURCE_COMPONENT_LABEL);
text_identifier!(RuntimeInstanceId, RUNTIME_INSTANCE_ID_LABEL);
text_identifier!(RecordedAt, RECORDED_AT_LABEL);

impl EventId {
    pub fn generated() -> Self {
        let mut value = String::from(EVENT_ID_PREFIX);
        value.push_str(&Utc::now().timestamp_micros().to_string());
        value.push_str(EVENT_ID_SEPARATOR);
        value.push_str(
            &EVENT_ID_SEQUENCE
                .fetch_add(1, Ordering::Relaxed)
                .to_string(),
        );
        Self(value)
    }
}

impl EventNamespace {
    pub fn from_event_type(event_type: &EventType) -> Result<Self, EventingError> {
        let namespace = event_type
            .as_str()
            .split(['.', '/'])
            .next()
            .ok_or_else(|| EventingError::empty_value(EVENT_NAMESPACE_LABEL))?;
        Self::parse(namespace)
    }

    pub fn matches_event_type(&self, event_type: &EventType) -> bool {
        event_type.as_str() == self.as_str()
            || event_type
                .as_str()
                .strip_prefix(self.as_str())
                .is_some_and(|suffix| suffix.starts_with(['.', '/']))
    }
}

impl RequestId {
    pub fn generated() -> Self {
        let mut value = String::from(REQUEST_ID_PREFIX);
        value.push_str(&Utc::now().timestamp_micros().to_string());
        value.push_str(EVENT_ID_SEPARATOR);
        value.push_str(
            &REQUEST_ID_SEQUENCE
                .fetch_add(1, Ordering::Relaxed)
                .to_string(),
        );
        Self(value)
    }
}

impl RecordedAt {
    pub fn now_utc() -> Self {
        Self(Utc::now().to_rfc3339())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersion(u16);

impl SchemaVersion {
    pub fn new(value: u16) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidVersion);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

fn validate_text(field: &'static str, value: String) -> Result<String, EventingError> {
    if value.trim().is_empty() {
        return Err(EventingError::empty_value(field));
    }
    if field == EVENT_TYPE_LABEL || field == EVENT_NAMESPACE_LABEL {
        validate_event_taxonomy(field, &value)?;
    }
    Ok(value)
}

fn validate_event_taxonomy(field: &'static str, value: &str) -> Result<(), EventingError> {
    let mut previous_was_separator = false;
    for (index, character) in value.chars().enumerate() {
        let is_separator = matches!(character, '.' | '/');
        let is_valid =
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-') || is_separator;
        if !is_valid || (is_separator && (index == 0 || previous_was_separator)) {
            return Err(EventingError::invalid_value(field, value));
        }
        previous_was_separator = is_separator;
    }
    if previous_was_separator {
        return Err(EventingError::invalid_value(field, value));
    }
    Ok(())
}
