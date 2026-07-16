use crate::network_inventory_command::value_text;

use super::push_unique_string;

pub(super) fn record_text_values(record: &serde_json::Value, field_name: &str) -> Vec<String> {
    record
        .get(field_name)
        .map(value_text_values)
        .unwrap_or_default()
}

fn value_text_values(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(values) => {
            values
                .iter()
                .filter_map(value_text)
                .fold(Vec::new(), |mut texts, text| {
                    push_unique_string(&mut texts, text);
                    texts
                })
        }
        _ => value_text(value).into_iter().collect(),
    }
}
