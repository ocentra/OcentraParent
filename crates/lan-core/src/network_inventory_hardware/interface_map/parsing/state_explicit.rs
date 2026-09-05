use serde_json::Value;

use super::super::{
    record_bool_any, record_text_any, record_u64_any, BOOLEAN_CONNECTED_KEYS, BOOLEAN_UP_KEYS,
    MEDIA_CONNECT_STATE_KEYS, NET_CONNECTION_STATUS_KEYS, STATE_KEYS,
};

pub(super) fn explicit_state(record: &Value) -> Option<(bool, bool, bool)> {
    boolean_state(record)
        .or_else(|| media_connect_state(record))
        .or_else(|| net_connection_state(record))
        .or_else(|| record_text_any(record, STATE_KEYS).map(|state| state_from_text(&state)))
}

fn boolean_state(record: &Value) -> Option<(bool, bool, bool)> {
    let is_up = record_bool_any(record, BOOLEAN_UP_KEYS);
    let is_connected = record_bool_any(record, BOOLEAN_CONNECTED_KEYS);
    is_up.or(is_connected).map(|up| {
        let connected = is_connected.unwrap_or(up);
        (up, connected, true)
    })
}

fn media_connect_state(record: &Value) -> Option<(bool, bool, bool)> {
    let state = record_u64_any(record, MEDIA_CONNECT_STATE_KEYS)?;
    Some(match state {
        1 => (true, true, true),
        2 => (false, false, true),
        _ => (true, true, false),
    })
}

fn net_connection_state(record: &Value) -> Option<(bool, bool, bool)> {
    let state = record_u64_any(record, NET_CONNECTION_STATUS_KEYS)?;
    Some(match state {
        2 => (true, true, true),
        _ => (false, false, true),
    })
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
