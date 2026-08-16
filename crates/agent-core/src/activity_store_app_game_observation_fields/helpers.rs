use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(crate) fn process_identity_from_pid(pid: u64) -> String {
    let mut identity = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    identity.push_str(&pid.to_string());
    identity
}

pub(crate) fn number_field(fields: &LogFields, key: &str) -> Option<u64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            Some(*value as u64)
        }
        _ => None,
    }
}

pub(crate) fn boolean_field(fields: &LogFields, key: &str) -> Option<bool> {
    match fields.get(key) {
        Some(LogFieldValue::Boolean(value)) => Some(*value),
        _ => None,
    }
}
