use serde_json::Value;

use super::super::super::network_identity_support::push_unique_string;

pub(super) fn record_value<'a>(record: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    keys.iter()
        .find_map(|key| record.get(*key))
        .or_else(|| {
            let object = record.as_object()?;
            keys.iter().find_map(|key| {
                object
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(key))
                    .map(|(_, value)| value)
            })
        })
}

fn value_text_values(value: &Value) -> Vec<String> {
    match value {
        Value::Array(values) => values.iter().flat_map(value_text_values).collect(),
        Value::Object(values) => values.values().flat_map(value_text_values).collect(),
        _ => crate::network_inventory_command::value_text(value)
            .into_iter()
            .collect(),
    }
}

pub(super) fn record_text_values_any(record: &Value, keys: &[&str]) -> Vec<String> {
    let mut values = Vec::new();
    for key in keys {
        if let Some(value) = record_value(record, &[*key]) {
            for text in value_text_values(value) {
                push_unique_string(&mut values, text);
            }
        }
    }
    values
}

pub(super) fn record_text_any(record: &Value, keys: &[&str]) -> Option<String> {
    record_text_values_any(record, keys).into_iter().next()
}

pub(super) fn record_u64_any(record: &Value, keys: &[&str]) -> Option<u64> {
    record_text_any(record, keys)?.parse::<u64>().ok()
}

pub(super) fn record_bool_any(record: &Value, keys: &[&str]) -> Option<bool> {
    let value = record_value(record, keys)?;
    value.as_bool().or_else(|| {
        value_text_values(value)
            .into_iter()
            .next()
            .and_then(|text| match text.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "on" | "up" | "connected" => Some(true),
                "0" | "false" | "no" | "off" | "down" | "disconnected" => Some(false),
                _ => None,
            })
    })
}

pub(super) fn record_percent(record: &Value, keys: &[&str]) -> Option<u8> {
    let text = record_text_any(record, keys)?;
    let value = text.trim().trim_end_matches('%').parse::<u16>().ok()?;
    (value <= 100).then_some(value as u8)
}
