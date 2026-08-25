use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteEventCorrelationId, ParentRouteEventId, ParentRouteEventSnapshot, ParentRoutePeerId,
};

use super::*;

pub(super) fn required_string_field(
    payload: &LogFields,
    key: &'static str,
) -> Result<String, String> {
    optional_string_field(payload, key)
        .ok_or_else(|| format!("agent-service network flow payload missing {key}"))
}

pub(super) fn optional_string_field(payload: &LogFields, key: &'static str) -> Option<String> {
    payload
        .get(key)
        .and_then(log_field_string)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn required_u64_field(payload: &LogFields, key: &'static str) -> Result<u64, String> {
    required_u64_field_with_context(payload, key, "agent-service network flow payload")
}

pub(super) fn required_u64_field_with_context(
    payload: &LogFields,
    key: &'static str,
    context: &str,
) -> Result<u64, String> {
    optional_u64_field(payload, key).ok_or_else(|| format!("{context} missing {key}"))
}

pub(super) fn optional_u64_field(payload: &LogFields, key: &'static str) -> Option<u64> {
    payload.get(key).and_then(log_field_u64)
}

pub(super) fn optional_u16_field(payload: &LogFields, key: &'static str) -> Option<u16> {
    optional_u64_field(payload, key).and_then(|value| u16::try_from(value).ok())
}

pub(super) fn optional_bool_field(payload: &LogFields, key: &'static str) -> Option<bool> {
    payload.get(key).and_then(log_field_bool)
}

fn log_field_u64(value: &LogFieldValue) -> Option<u64> {
    match value {
        LogFieldValue::Number(value)
            if value.is_finite() && *value >= 0.0 && value.fract() == 0.0 =>
        {
            Some(*value as u64)
        }
        LogFieldValue::String(value) => value.trim().parse::<u64>().ok(),
        _ => None,
    }
}

fn log_field_bool(value: &LogFieldValue) -> Option<bool> {
    match value {
        LogFieldValue::Boolean(value) => Some(*value),
        LogFieldValue::String(value) => match value.trim() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn list_field(payload: &LogFields, key: &'static str) -> Vec<String> {
    payload
        .get(key)
        .and_then(log_field_string)
        .map(|value| {
            value
                .split(constants::delimiter::LIST)
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub(super) fn parent_route_event_snapshot(event: &AgentEventEnvelope) -> ParentRouteEventSnapshot {
    ParentRouteEventSnapshot {
        event: Some(serialized_enum_label(&event.event)),
        event_id: ParentRouteEventId::parse(event.event_id.clone()),
        correlation_id: ParentRouteEventCorrelationId::parse(event.correlation_id.clone()),
        sent_at: Some(event.sent_at.clone()),
        source_peer_id: ParentRoutePeerId::parse(event.source.peer_id.clone()),
        source_role: Some(parent_route_peer_role(&event.source.role)),
        target_peer_id: ParentRoutePeerId::parse(event.target.peer_id.clone()),
        target_role: Some(parent_route_peer_role(&event.target.role)),
        severity: Some(serialized_enum_label(&event.severity)),
        payload: serde_json::to_value(&event.payload).ok(),
        snapshot: event
            .snapshot
            .as_ref()
            .and_then(|snapshot| serde_json::to_value(snapshot).ok()),
        command_result_projection: command_result_projection::command_result_projection(event),
    }
}
