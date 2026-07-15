use crate::mac_identity::normalize_scan_mac_address;

pub(super) fn record_text(record: &serde_json::Value, field_name: &str) -> Option<String> {
    record.get(field_name).and_then(value_text)
}

pub(super) fn value_text(value: &serde_json::Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_string)
        .or_else(|| value.as_number().map(ToString::to_string))
        .and_then(|value| clean_string(&value))
}

pub(super) fn record_u64(record: &serde_json::Value, field_name: &str) -> Option<u64> {
    record.get(field_name).and_then(|value| {
        value
            .as_u64()
            .or_else(|| value_text(value).and_then(|value| value.parse().ok()))
    })
}

pub(super) fn normalize_mac_address(value: &str) -> Option<String> {
    normalize_scan_mac_address(value)
}

pub(super) fn clean_string(value: &str) -> Option<String> {
    let value = value.trim().to_string();
    (!value.is_empty()).then_some(value)
}
