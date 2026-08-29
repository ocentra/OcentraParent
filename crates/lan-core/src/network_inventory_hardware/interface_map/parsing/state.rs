use serde_json::Value;

use super::{
    record_bool_any, record_text_any, record_text_values_any, record_u64_any,
    BOOLEAN_CONNECTED_KEYS, BOOLEAN_UP_KEYS, LOOPBACK_KEYS, MEDIA_CONNECT_STATE_KEYS,
    NET_CONNECTION_STATUS_KEYS, STATE_KEYS,
};

pub(super) fn interface_state(record: &Value) -> (bool, bool, bool) {
    let is_up = record_bool_any(record, BOOLEAN_UP_KEYS);
    let is_connected = record_bool_any(record, BOOLEAN_CONNECTED_KEYS);
    if is_up.is_some() || is_connected.is_some() {
        let up = is_up.or(is_connected).unwrap_or(true);
        return (up, is_connected.unwrap_or(up), true);
    }
    if let Some(state) = record_u64_any(record, MEDIA_CONNECT_STATE_KEYS) {
        return match state {
            1 => (true, true, true),
            2 => (false, false, true),
            _ => (true, true, false),
        };
    }
    if let Some(state) = record_u64_any(record, NET_CONNECTION_STATUS_KEYS) {
        return match state {
            2 => (true, true, true),
            _ => (false, false, true),
        };
    }
    if let Some(state) = record_text_any(record, STATE_KEYS) {
        return state_from_text(&state);
    }
    let flags = record_text_values_any(record, &["flags", "Flags"]);
    let normalized_flags = flags
        .iter()
        .map(|flag| flag.to_ascii_lowercase())
        .collect::<Vec<_>>();
    if normalized_flags.iter().any(|flag| {
        matches!(
            flag.as_str(),
            "down" | "no-carrier" | "lowerlayerdown" | "disconnected"
        )
    }) {
        return (false, false, true);
    }
    if normalized_flags
        .iter()
        .any(|flag| matches!(flag.as_str(), "up" | "lower_up" | "lower-up" | "connected"))
    {
        return (true, true, true);
    }
    (true, true, false)
}

fn state_from_text(value: &str) -> (bool, bool, bool) {
    let normalized = value.trim().to_ascii_lowercase().replace(['_', ' '], "-");
    if matches!(
        normalized.as_str(),
        "connected" | "up" | "active" | "running" | "online" | "enabled" | "ready"
    ) {
        return (true, true, true);
    }
    if normalized == "down" {
        return (false, true, true);
    }
    if matches!(
        normalized.as_str(),
        "disconnected"
            | "not-present"
            | "media-disconnected"
            | "no-carrier"
            | "lowerlayerdown"
            | "lower-layer-down"
    ) {
        return (false, false, true);
    }
    (true, true, false)
}

pub(super) fn interface_is_loopback(record: &Value, interface_name: &str) -> bool {
    record_bool_any(record, LOOPBACK_KEYS).unwrap_or(false)
        || record_text_values_any(record, &["flags", "Flags"])
            .iter()
            .any(|flag| flag.eq_ignore_ascii_case("loopback"))
        || interface_name.trim().eq_ignore_ascii_case("lo")
        || interface_name.to_ascii_lowercase().contains("loopback")
}
