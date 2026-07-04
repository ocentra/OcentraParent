use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(crate) fn number_field(fields: &LogFields, key: &str) -> Option<f64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) => Some(*value),
        _ => None,
    }
}

pub(crate) fn bool_field(fields: &LogFields, key: &str) -> Option<bool> {
    match fields.get(key) {
        Some(LogFieldValue::Boolean(value)) => Some(*value),
        _ => None,
    }
}

pub(crate) fn string_list_field(fields: &LogFields, key: &str) -> Vec<String> {
    non_empty_string_list_field(fields, key).unwrap_or_default()
}

pub(crate) fn non_empty_string_list_field(fields: &LogFields, key: &str) -> Option<Vec<String>> {
    let values = string_field(fields, key)?
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}
