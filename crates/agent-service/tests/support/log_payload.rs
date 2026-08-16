use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use serde::de::DeserializeOwned;

use crate::test_text::TestText;

pub(crate) fn payload_json<T>(payload: &LogFields, field: impl std::fmt::Display) -> T
where
    T: DeserializeOwned,
{
    let field_name = field.to_string();
    match payload.get(field_name.as_str()) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::process::abort(),
    }
}

pub(crate) fn payload_number(payload: &LogFields, field: impl std::fmt::Display) -> f64 {
    let field_name = field.to_string();
    match payload.get(field_name.as_str()) {
        Some(LogFieldValue::Number(value)) => *value,
        _ => std::process::abort(),
    }
}

pub(crate) fn payload_text(payload: &LogFields, field: impl std::fmt::Display) -> TestText {
    let field_name = field.to_string();
    match payload.get(field_name.as_str()) {
        Some(LogFieldValue::String(value)) => TestText::from_display(value),
        _ => std::process::abort(),
    }
}
