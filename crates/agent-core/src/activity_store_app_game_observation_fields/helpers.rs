use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(crate) fn boolean_field(fields: &LogFields, key: &str) -> Option<bool> {
    match fields.get(key) {
        Some(LogFieldValue::Boolean(value)) => Some(*value),
        _ => None,
    }
}
