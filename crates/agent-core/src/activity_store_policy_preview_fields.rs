use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(crate) fn number_field(fields: &LogFields, key: &str) -> Option<u64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            (*value < u64::MAX as f64 && value.fract() == 0.0).then_some(*value as u64)
        }
        Some(LogFieldValue::String(value)) => value.parse::<u64>().ok(),
        _ => None,
    }
}

pub(crate) fn protocol_field<T>(
    fields: &LogFields,
    key: &str,
    parse: impl for<'a> Fn(&'a str) -> Option<T>,
) -> Option<T> {
    string_field(fields, key).and_then(|value| parse(&value))
}
