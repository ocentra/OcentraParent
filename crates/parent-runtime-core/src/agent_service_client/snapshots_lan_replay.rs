use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventRow,
};
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
use super::types::LanRuntimeReplaySnapshot;

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
) -> Result<LanRuntimeReplaySnapshot, String> {
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

    let report_generated_at = parse_rfc3339_timestamp(
        payload
            .get(constants::field::GENERATED_AT)
            .and_then(log_field_string)
            .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} missing generatedAt"))?,
        constants::field::GENERATED_AT,
    )?;
    let envelope_sent_at = parse_rfc3339_timestamp(&response_event.sent_at, "envelope.sentAt")?;
    let events =
        entries_to_route_events(&entries, &identity, report_generated_at, envelope_sent_at)?;
    validate_report_metadata(
        payload,
        entries.len(),
        entries.iter().map(|entry| &entry.payload.event_kind),
        entries.last().map(|entry| entry.payload.event_id.as_str()),
        entries
            .last()
            .map(|entry| entry.payload.occurred_at.as_str()),
    )?;
    let history_state_label = payload
        .get(constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE)
        .and_then(log_field_string)
        .ok_or_else(|| {
            format!(
                "{LAN_REPLAY_CONTEXT} missing {}",
                constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE
            )
        })?;
    let history_state = serde_json::from_value::<LanDiscoveryEventHistoryState>(
        serde_json::Value::String(history_state_label.to_string()),
    )
    .map_err(|error| format!("{LAN_REPLAY_CONTEXT} history state parse failed: {error}"))?;
    let latest_event_id = entries.last().map(|entry| entry.payload.event_id.clone());
    let latest_observed_at = entries
        .last()
        .map(|entry| entry.payload.occurred_at.clone());

    Ok(LanRuntimeReplaySnapshot {
        events,
        history_state,
        latest_event_id,
        latest_observed_at,
    })
}

fn entries_to_route_events(
    entries: &[LanRuntimeReplayEntry],
    identity: &LanReplayEnvelopeIdentity,
    report_generated_at: DateTime<FixedOffset>,
    envelope_sent_at: DateTime<FixedOffset>,
) -> Result<Vec<ParentRouteEventSnapshot>, String> {
    let mut event_ids = HashSet::new();
    let mut previous_event_id: Option<String> = None;
    let mut previous_occurred_at: Option<DateTime<FixedOffset>> = None;
    let mut events = Vec::with_capacity(entries.len());

    for (index, entry) in entries.iter().enumerate() {
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
        validate_entry_timing(
            event_id,
            &occurred_at,
            &report_generated_at,
            &envelope_sent_at,
        )?;
        validate_event_order(event_id, &occurred_at, previous_occurred_at.as_ref())?;
        validate_optional_text(
            entry.payload.previous_event_id.as_deref(),
            "payload.previousEventId",
        )?;
        validate_first_predecessor(index, entry.payload.previous_event_id.as_deref())?;
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

fn validate_entry_timing(
    event_id: &str,
    occurred_at: &DateTime<FixedOffset>,
    report_generated_at: &DateTime<FixedOffset>,
    envelope_sent_at: &DateTime<FixedOffset>,
) -> Result<(), String> {
    if occurred_at > report_generated_at || occurred_at > envelope_sent_at {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected eventId {event_id} later than report or envelope timestamp"
        ));
    }
    Ok(())
}

fn validate_event_order(
    event_id: &str,
    occurred_at: &DateTime<FixedOffset>,
    previous: Option<&DateTime<FixedOffset>>,
) -> Result<(), String> {
    if previous.is_some_and(|value| occurred_at < value) {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected stale or out-of-order eventId {event_id}"
        ));
    }
    Ok(())
}

fn validate_first_predecessor(index: usize, previous_event_id: Option<&str>) -> Result<(), String> {
    if index == 0 && previous_event_id.is_some() {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected first stream entry with previousEventId"
        ));
    }
    Ok(())
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
