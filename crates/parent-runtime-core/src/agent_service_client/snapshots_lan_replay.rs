use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventRow;
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventEnvelope};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteEventCorrelationId, ParentRouteEventId, ParentRouteEventSnapshot,
};
use serde::Deserialize;

use super::payload_fields::{log_field_string, serialized_enum_label};
use super::snapshots_lan_replay_validation::envelope::{
    validate_replay_envelope, LanReplayEnvelopeIdentity,
};
use super::snapshots_lan_replay_validation::{
    canonical_text, parse_rfc3339_timestamp, validate_optional_text, validate_report_metadata,
    LAN_REPLAY_CONTEXT,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LanRuntimeReplayEntry {
    event_type: String,
    event_ref: String,
    payload: LanDiscoveryEventRow,
}

pub(super) fn lan_runtime_replay_events_from_payload(
    response_event: &AgentEventEnvelope,
    command: &AgentCommandName,
    command_message_id: &str,
) -> Result<Vec<ParentRouteEventSnapshot>, String> {
    let identity = validate_replay_envelope(response_event, command, command_message_id)?;
    let payload = &response_event.payload;
    let stream_json = payload
        .get(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM)
        .and_then(log_field_string)
        .ok_or_else(|| {
            format!(
                "{LAN_REPLAY_CONTEXT} missing {}",
                constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM
            )
        })?;
    let entries = serde_json::from_str::<Vec<LanRuntimeReplayEntry>>(stream_json)
        .map_err(|error| format!("{LAN_REPLAY_CONTEXT} stream parse failed: {error}"))?;

    let events = entries_to_route_events(&entries, &identity)?;
    validate_report_metadata(
        payload,
        entries.len(),
        entries.last().map(|entry| entry.payload.event_id.as_str()),
        entries
            .last()
            .map(|entry| entry.payload.occurred_at.as_str()),
    )?;
    Ok(events)
}

fn entries_to_route_events(
    entries: &[LanRuntimeReplayEntry],
    identity: &LanReplayEnvelopeIdentity,
) -> Result<Vec<ParentRouteEventSnapshot>, String> {
    let mut event_ids = HashSet::new();
    let mut previous_event_id: Option<String> = None;
    let mut previous_occurred_at: Option<DateTime<FixedOffset>> = None;
    let mut events = Vec::with_capacity(entries.len());

    for entry in entries {
        let event_id = canonical_text(&entry.payload.event_id, "payload.eventId")?;
        let event_ref = canonical_text(&entry.event_ref, "eventRef")?;
        if event_ref != event_id {
            return Err(format!(
                "{LAN_REPLAY_CONTEXT} rejected eventRef that does not match payload.eventId"
            ));
        }
        if !event_ids.insert(event_id.to_string()) {
            return Err(format!(
                "{LAN_REPLAY_CONTEXT} rejected duplicate eventId {event_id}"
            ));
        }

        let event_type = canonical_text(&entry.event_type, "eventType")?;
        let canonical_event_type = serialized_enum_label(&entry.payload.event_kind);
        if event_type != canonical_event_type {
            return Err(format!(
                "{LAN_REPLAY_CONTEXT} rejected eventType that does not match payload.eventKind"
            ));
        }

        let occurred_at =
            parse_rfc3339_timestamp(&entry.payload.occurred_at, "payload.occurredAt")?;
        if previous_occurred_at
            .as_ref()
            .is_some_and(|previous| &occurred_at < previous)
        {
            return Err(format!(
                "{LAN_REPLAY_CONTEXT} rejected stale or out-of-order eventId {event_id}"
            ));
        }
        validate_optional_text(
            entry.payload.previous_event_id.as_deref(),
            "payload.previousEventId",
        )?;
        validate_optional_text(
            entry.payload.scan_session_id.as_deref(),
            "payload.scanSessionId",
        )?;
        validate_optional_text(
            entry.payload.affected_device_id.as_deref(),
            "payload.affectedDeviceId",
        )?;
        validate_optional_text(entry.payload.evidence_id.as_deref(), "payload.evidenceId")?;
        canonical_text(&entry.payload.summary, "payload.summary")?;

        if let Some(previous_event_id) = previous_event_id.as_deref() {
            if entry.payload.previous_event_id.as_deref() != Some(previous_event_id) {
                return Err(format!(
                    "{LAN_REPLAY_CONTEXT} rejected broken previousEventId chain at {event_id}"
                ));
            }
        }

        events.push(route_event_from_entry(
            entry,
            &canonical_event_type,
            identity,
        )?);
        previous_event_id = Some(event_id.to_string());
        previous_occurred_at = Some(occurred_at);
    }

    Ok(events)
}

fn route_event_from_entry(
    entry: &LanRuntimeReplayEntry,
    event_type: &str,
    identity: &LanReplayEnvelopeIdentity,
) -> Result<ParentRouteEventSnapshot, String> {
    let event_id = ParentRouteEventId::parse(entry.payload.event_id.clone())
        .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} rejected empty payload.eventId"))?;
    let correlation_id = entry
        .payload
        .scan_session_id
        .as_ref()
        .and_then(|value| ParentRouteEventCorrelationId::parse(value.clone()));
    let payload = serde_json::to_value(&entry.payload)
        .map_err(|error| format!("{LAN_REPLAY_CONTEXT} event serialization failed: {error}"))?;

    Ok(ParentRouteEventSnapshot {
        event: Some(event_type.to_string()),
        event_id: Some(event_id),
        correlation_id,
        sent_at: Some(entry.payload.occurred_at.clone()),
        source_peer_id: Some(identity.source_peer_id.clone()),
        source_role: Some(identity.source_role.clone()),
        target_peer_id: Some(identity.target_peer_id.clone()),
        target_role: Some(identity.target_role.clone()),
        severity: Some(identity.severity.clone()),
        payload: Some(payload),
        snapshot: None,
        command_result_projection: None,
    })
}
