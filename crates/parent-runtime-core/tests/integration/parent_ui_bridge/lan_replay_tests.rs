use std::collections::BTreeMap;
use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind, LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use ocentra_schema::parent_ui_bridge::{ParentRouteId, ParentRoutePeerRole};
use serde_json::{json, Value};

use ocentra_parent_runtime_core::parent_ui_bridge::lan_replay_rejection_episode::ParentRouteSubscriptionLoadState;

use super::common::helpers::{
    require_some, sample_lan_read_model_with_explicit_history,
    sample_lan_read_model_with_history_state, TestContext,
};
use super::load_parent_subscription_event;
use super::tests_support::{
    lan_event, require_ok, start_local_server_with_capture_responses, with_agent_addr,
    REQUEST_MESSAGE_ID_CORRELATION,
};

const LAN_REPLAY_REJECTION_EVENT: &str = "lan-runtime-event-chain-replay-rejected";

#[test]
fn parent_subscription_event_replays_ordered_lan_stream_rows() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);
    let stream_event = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    let (address, requests) = start_local_server_with_capture_responses(vec![
        lan_event(AgentEventName::AgentLanPairingStatusReported, &read_model),
        stream_event,
    ]);

    let subscription = with_agent_addr(&address, || {
        load_parent_subscription_event(ParentRouteId::Devices, None)
    });
    let events = subscription.events.unwrap_or_default();
    let event_ids = events
        .iter()
        .filter_map(|event| event.event_id.as_ref().map(|event_id| event_id.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(
        event_ids,
        vec![
            "lan-history-1",
            "lan-history-2",
            "agent.connection.ready-1",
            "agent.lan-pairing.event-1",
        ]
    );
    assert_eq!(events[0].event.as_deref(), Some("scan-started"));
    assert_eq!(events[1].event.as_deref(), Some("device-found"));
    assert_eq!(
        events[1]
            .correlation_id
            .as_ref()
            .map(|correlation_id| correlation_id.as_str()),
        Some("lan-scan-1")
    );
    assert_eq!(
        events[1].source_role,
        Some(ParentRoutePeerRole::AgentService)
    );
    assert_eq!(events[1].target_role, Some(ParentRoutePeerRole::Portal));
    assert_eq!(
        events[1]
            .source_peer_id
            .as_ref()
            .map(|peer_id| peer_id.as_str()),
        Some(constants::peer::LOCAL_DEV_AGENT)
    );
    assert_eq!(
        events[1]
            .target_peer_id
            .as_ref()
            .map(|peer_id| peer_id.as_str()),
        Some(constants::peer::PORTAL_DEV)
    );
    assert_eq!(events[1].severity.as_deref(), Some("info"));
    assert_eq!(
        events[1]
            .payload
            .as_ref()
            .and_then(|payload| payload.get("previousEventId")),
        Some(&json!("lan-history-1"))
    );

    let _status_request = require_ok(
        requests.recv_timeout(Duration::from_secs(2)),
        "status request is captured",
    );
    let replay_request = require_ok(
        requests.recv_timeout(Duration::from_secs(2)),
        "replay request is captured",
    );
    assert_eq!(
        replay_request.command["command"],
        json!(constants::lan_pairing::COMMAND_RUNTIME_EVENT_CHAIN_STREAM_GET)
    );
    assert_eq!(
        replay_request.command["target"]["route"],
        json!("local-network")
    );
}

#[test]
fn parent_subscription_event_rejects_duplicate_and_stale_lan_replay_batches() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let valid_entries = replay_entries(&read_model.discovery_event_history.rows);
    let mut duplicate_entries = valid_entries.clone();
    duplicate_entries[1]["eventRef"] = json!("lan-history-1");
    duplicate_entries[1]["payload"]["eventId"] = json!("lan-history-1");
    let mut stale_entries = valid_entries;
    stale_entries[1]["payload"]["occurredAt"] = json!("2026-06-23T00:00:00Z");

    for entries in [duplicate_entries, stale_entries] {
        let events = subscription_events_for_stream(
            &read_model,
            replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false),
            None,
        );
        assert_redacted_replay_rejection_diagnostic(&events);
        assert!(events.iter().all(|event| {
            event
                .event_id
                .as_ref()
                .is_none_or(|event_id| !event_id.as_str().starts_with("lan-history-"))
        }));
    }
}

#[test]
fn parent_subscription_event_rejects_history_states_that_disagree_with_replay_entries() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    for (history_state, manual_required) in [
        (LanDiscoveryEventHistoryState::Empty, false),
        (LanDiscoveryEventHistoryState::ManualRequired, true),
    ] {
        assert_replay_rejected(
            &read_model,
            replay_event(&entries, history_state, manual_required),
        );
    }

    let empty_ready_model =
        sample_lan_read_model_with_history_state(LanDiscoveryEventHistoryState::Ready);
    assert_replay_rejected(
        &empty_ready_model,
        replay_event(&[], LanDiscoveryEventHistoryState::Ready, false),
    );
}

#[test]
fn parent_subscription_event_rejects_metadata_only_ready_history() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.discovery_event_history.rows[1].event_kind = LanDiscoveryEventKind::ScanFinished;
    read_model.discovery_event_history.rows[1].affected_device_id = None;
    read_model.discovery_event_history.rows[1].evidence_id = None;
    read_model.discovery_event_history.rows[1].summary =
        "LAN discovery scan finished with 0 devices".to_string();
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    assert_replay_rejected(
        &read_model,
        replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false),
    );
}

#[test]
fn parent_subscription_event_accepts_metadata_only_empty_history_per_canonical_producer() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.discovery_event_history.state = LanDiscoveryEventHistoryState::Empty;
    read_model.discovery_event_history.rows[1].event_kind = LanDiscoveryEventKind::ScanFinished;
    read_model.discovery_event_history.rows[1].affected_device_id = None;
    read_model.discovery_event_history.rows[1].evidence_id = None;
    read_model.discovery_event_history.rows[1].summary =
        "LAN discovery scan finished with 0 devices".to_string();
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let events = subscription_events_for_stream(
        &read_model,
        replay_event(&entries, LanDiscoveryEventHistoryState::Empty, false),
        None,
    );

    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event.as_deref(), Some("scan-started"));
    assert_eq!(events[1].event.as_deref(), Some("scan-finished"));
    assert!(events
        .iter()
        .all(|event| event.event.as_deref() != Some(LAN_REPLAY_REJECTION_EVENT)));
}

#[test]
fn parent_subscription_event_accepts_unavailable_replay_rows_per_canonical_precedence() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.discovery_event_history.state = LanDiscoveryEventHistoryState::Unavailable;
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let events = subscription_events_for_stream(
        &read_model,
        replay_event(&entries, LanDiscoveryEventHistoryState::Unavailable, false),
        None,
    );

    assert_eq!(events.len(), 4);
    assert!(events
        .iter()
        .all(|event| event.event.as_deref() != Some(LAN_REPLAY_REJECTION_EVENT)));
}

#[test]
fn parent_subscription_event_accepts_degraded_replay_rows_per_canonical_precedence() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.discovery_event_history.state = LanDiscoveryEventHistoryState::Degraded;
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let events = subscription_events_for_stream(
        &read_model,
        replay_event(&entries, LanDiscoveryEventHistoryState::Degraded, false),
        None,
    );

    assert_eq!(events.len(), 4);
    assert!(events
        .iter()
        .all(|event| event.event.as_deref() != Some(LAN_REPLAY_REJECTION_EVENT)));
}

#[test]
fn parent_subscription_event_accepts_agent_offline_replay_rows_per_canonical_precedence() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.discovery_event_history.state = LanDiscoveryEventHistoryState::AgentOffline;
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let events = subscription_events_for_stream(
        &read_model,
        replay_event(&entries, LanDiscoveryEventHistoryState::AgentOffline, false),
        None,
    );

    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event.as_deref(), Some("scan-started"));
    assert_eq!(events[1].event.as_deref(), Some("device-found"));
    assert!(events
        .iter()
        .all(|event| event.event.as_deref() != Some(LAN_REPLAY_REJECTION_EVENT)));
}

#[test]
fn parent_subscription_event_orders_lan_replay_rows_by_rfc3339_instant() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let mut ordered_entries = replay_entries(&read_model.discovery_event_history.rows);
    ordered_entries[0]["payload"]["occurredAt"] = json!("2026-06-23T01:00:00+02:00");
    ordered_entries[1]["payload"]["occurredAt"] = json!("2026-06-23T00:00:02Z");

    let ordered_events = subscription_events_for_stream(
        &read_model,
        replay_event(
            &ordered_entries,
            LanDiscoveryEventHistoryState::Ready,
            false,
        ),
        None,
    );
    assert!(ordered_events.iter().any(|event| {
        event
            .event_id
            .as_ref()
            .is_some_and(|event_id| event_id.as_str() == "lan-history-2")
    }));

    let mut stale_entries = replay_entries(&read_model.discovery_event_history.rows);
    stale_entries[0]["payload"]["occurredAt"] = json!("2026-06-23T00:00:02Z");
    stale_entries[1]["payload"]["occurredAt"] = json!("2026-06-23T01:00:00+02:00");
    assert_replay_rejected(
        &read_model,
        replay_event(&stale_entries, LanDiscoveryEventHistoryState::Ready, false),
    );
}

#[test]
fn parent_subscription_event_rejects_invalid_lan_replay_envelope_contract() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);
    let valid = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    let mut invalid_envelopes = Vec::new();

    let mut invalid = valid.clone();
    invalid.schema_version = 2;
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.event_id.clear();
    invalid_envelopes.push(invalid);
    for event_id in [
        "agent.lan.runtime.event-chain.stream.reported",
        "other-event-1",
        "agent.lan.runtime.event-chain.stream.reported-",
        "agent.lan.runtime.event-chain.stream.reported-1x",
        "agent.lan.runtime.event-chain.stream.reported- 1",
    ] {
        let mut invalid = valid.clone();
        invalid.event_id = event_id.to_string();
        invalid_envelopes.push(invalid);
    }
    let mut invalid = valid.clone();
    invalid.sent_at = "not-rfc3339".to_string();
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.event = AgentEventName::AgentLanPairingStatusReported;
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.source.peer_id.clear();
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.source.peer_id = "spoofed-agent".to_string();
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.source.role = AgentPeerRole::Portal;
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.target.peer_id.clear();
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.target.peer_id = "spoofed-portal".to_string();
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.target.role = AgentPeerRole::AgentService;
    invalid_envelopes.push(invalid);
    let mut invalid = valid.clone();
    invalid.severity = LogLevel::Warn;
    invalid_envelopes.push(invalid);

    for invalid in invalid_envelopes {
        assert_replay_rejected(&read_model, invalid);
    }

    let mut wrong_correlation = valid;
    wrong_correlation.correlation_id = "wrong-command-message-id".to_string();
    assert_replay_rejected(&read_model, wrong_correlation);
}

#[test]
fn parent_subscription_event_requires_exact_nonempty_lan_replay_latest_metadata() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);
    let valid = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);

    for (field, replacement) in [
        (constants::field::LATEST_EVENT_ID, None),
        (constants::field::LATEST_EVENT_ID, Some("")),
        (constants::field::LATEST_EVENT_ID, Some("wrong-event")),
        (constants::field::LATEST_OBSERVED_AT, None),
        (constants::field::LATEST_OBSERVED_AT, Some("")),
        (
            constants::field::LATEST_OBSERVED_AT,
            Some("2026-06-23T00:00:00Z"),
        ),
    ] {
        let mut invalid = valid.clone();
        if let Some(replacement) = replacement {
            invalid.payload.insert(
                field.to_string(),
                LogFieldValue::String(replacement.to_string()),
            );
        } else {
            remove_payload_field(&mut invalid, field);
        }
        assert_replay_rejected(&read_model, invalid);
    }
}

#[test]
fn parent_subscription_event_rejects_status_and_replay_history_mutated_between_reads() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let state_changed = replay_event(&entries, LanDiscoveryEventHistoryState::Degraded, false);

    let mut id_changed_entries = entries.clone();
    id_changed_entries[1]["eventRef"] = json!("lan-history-after-status");
    id_changed_entries[1]["payload"]["eventId"] = json!("lan-history-after-status");
    let id_changed = replay_event(
        &id_changed_entries,
        LanDiscoveryEventHistoryState::Ready,
        false,
    );

    let mut time_changed_entries = entries;
    time_changed_entries[1]["payload"]["occurredAt"] = json!("2026-06-23T00:00:02.500Z");
    let time_changed = replay_event(
        &time_changed_entries,
        LanDiscoveryEventHistoryState::Ready,
        false,
    );

    for independently_changed_replay in [state_changed, id_changed, time_changed] {
        assert_replay_rejected(&read_model, independently_changed_replay);
    }
}

#[test]
fn parent_subscription_event_rejects_non_rfc3339_lan_replay_payload_timestamps() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);
    let mut invalid_generated_at =
        replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    invalid_generated_at.payload.insert(
        constants::field::GENERATED_AT.to_string(),
        LogFieldValue::String("not-rfc3339".to_string()),
    );
    assert_replay_rejected(&read_model, invalid_generated_at);

    let mut invalid_entries = entries;
    invalid_entries[1]["payload"]["occurredAt"] = json!("not-rfc3339");
    assert_replay_rejected(
        &read_model,
        replay_event(
            &invalid_entries,
            LanDiscoveryEventHistoryState::Ready,
            false,
        ),
    );
}

#[test]
fn parent_subscription_event_rejects_lan_replay_rows_after_report_or_envelope_time_and_a_first_predecessor(
) {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);

    let mut after_report = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    after_report.payload.insert(
        constants::field::GENERATED_AT.to_string(),
        LogFieldValue::String("2026-06-23T00:00:01Z".to_string()),
    );
    assert_replay_rejected(&read_model, after_report);

    let mut after_envelope = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    after_envelope.sent_at = "2026-06-23T00:00:01Z".to_string();
    assert_replay_rejected(&read_model, after_envelope);

    let mut first_has_predecessor = entries;
    first_has_predecessor[0]["payload"]["previousEventId"] = json!("outside-history");
    assert_replay_rejected(
        &read_model,
        replay_event(
            &first_has_predecessor,
            LanDiscoveryEventHistoryState::Ready,
            false,
        ),
    );
}

#[test]
fn parent_subscription_event_fails_closed_on_malformed_lan_replay_payload() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let malformed_event = replay_event_with_stream(
        "{not-json".to_string(),
        2,
        LanDiscoveryEventHistoryState::Ready,
        false,
        "",
        "",
    );

    let events = subscription_events_for_stream(&read_model, malformed_event, None);

    assert_redacted_replay_rejection_diagnostic(&events);
    assert!(events.iter().all(|event| {
        event
            .event_id
            .as_ref()
            .is_none_or(|event_id| !event_id.as_str().starts_with("lan-history-"))
    }));
}

#[test]
fn parent_subscription_event_reuses_one_safe_warning_until_valid_replay_closes_the_episode() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let entries = replay_entries(&read_model.discovery_event_history.rows);
    let mut rejected = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    let rejected_identifier = "attacker-controlled-rejected-event-id";
    rejected.payload.insert(
        constants::field::LATEST_EVENT_ID.to_string(),
        LogFieldValue::String(rejected_identifier.to_string()),
    );
    let valid = replay_event(&entries, LanDiscoveryEventHistoryState::Ready, false);
    let mut state = ParentRouteSubscriptionLoadState::default();

    let first_events =
        subscription_events_for_stream(&read_model, rejected.clone(), Some(&mut state));
    let second_events =
        subscription_events_for_stream(&read_model, rejected.clone(), Some(&mut state));
    let recovery_events = subscription_events_for_stream(&read_model, valid, Some(&mut state));
    let later_events = subscription_events_for_stream(&read_model, rejected, Some(&mut state));
    assert_redacted_replay_rejection_diagnostic(&first_events);
    assert_redacted_replay_rejection_diagnostic(&second_events);
    assert_redacted_replay_rejection_diagnostic(&later_events);
    assert!(recovery_events
        .iter()
        .all(|event| event.event.as_deref() != Some(LAN_REPLAY_REJECTION_EVENT)));

    let warning = |events: &[ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot]| {
        events
            .iter()
            .find(|event| event.event.as_deref() == Some(LAN_REPLAY_REJECTION_EVENT))
            .cloned()
    };
    let first_warning = warning(&first_events);
    assert_eq!(first_warning, warning(&second_events));
    assert_ne!(first_warning, warning(&later_events));
    assert!(first_events.iter().all(|event| {
        event.event_id.as_ref().is_none_or(|event_id| {
            !event_id.as_str().contains(rejected_identifier)
                && !event_id.as_str().starts_with("lan-history-")
        })
    }));
}

#[test]
fn parent_subscription_event_preserves_offline_and_manual_required_lan_states() {
    for (state, expected_manual_required, expected_state) in [
        (
            LanDiscoveryEventHistoryState::AgentOffline,
            false,
            "agent-offline",
        ),
        (
            LanDiscoveryEventHistoryState::ManualRequired,
            true,
            "manual-required",
        ),
    ] {
        let read_model = sample_lan_read_model_with_history_state(state.clone());
        let stream_event = replay_event(&[], state.clone(), expected_manual_required);
        let (address, _requests) = start_local_server_with_capture_responses(vec![
            lan_event(AgentEventName::AgentLanPairingStatusReported, &read_model),
            stream_event,
        ]);

        let subscription = with_agent_addr(&address, || {
            load_parent_subscription_event(ParentRouteId::Devices, None)
        });
        let history_state = subscription
            .snapshot
            .live_activity
            .as_ref()
            .and_then(|activity| activity.lan_add_device_read_model.as_ref())
            .map(|model| model.discovery_event_history.state.clone());

        assert_eq!(history_state.as_deref(), Some(expected_state));
        assert_eq!(subscription.events.unwrap_or_default().len(), 2);
    }
}

fn subscription_events_for_stream(
    read_model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
    stream_event: AgentEventEnvelope,
    state: Option<&mut ParentRouteSubscriptionLoadState>,
) -> Vec<ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot> {
    let mut default_state = ParentRouteSubscriptionLoadState::default();
    let state = state.unwrap_or(&mut default_state);
    let responses = vec![
        lan_event(AgentEventName::AgentLanPairingStatusReported, read_model),
        stream_event,
    ];
    let (address, _requests) = start_local_server_with_capture_responses(responses);
    with_agent_addr(&address, || {
        state
            .load(ParentRouteId::Devices, None)
            .events
            .unwrap_or_default()
    })
}

fn assert_replay_rejected(
    read_model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
    stream_event: AgentEventEnvelope,
) {
    let events = subscription_events_for_stream(read_model, stream_event, None);
    assert_redacted_replay_rejection_diagnostic(&events);
    assert!(events.iter().all(|event| {
        event
            .event_id
            .as_ref()
            .is_none_or(|event_id| !event_id.as_str().starts_with("lan-history-"))
    }));
}

fn assert_redacted_replay_rejection_diagnostic(
    events: &[ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot],
) {
    assert_eq!(events.len(), 3);
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event.as_deref() == Some(LAN_REPLAY_REJECTION_EVENT))
            .count(),
        1
    );
    let diagnostic = require_some(
        events
            .iter()
            .find(|event| event.event.as_deref() == Some(LAN_REPLAY_REJECTION_EVENT)),
        TestContext("rejected replay emits a safe host diagnostic"),
    );
    assert_eq!(diagnostic.severity.as_deref(), Some("warn"));
    let event_id = require_some(
        diagnostic.event_id.as_ref(),
        TestContext("rejected replay warning has host event identity"),
    );
    let event_id_suffix = require_some(
        event_id
            .as_str()
            .strip_prefix(&format!("{LAN_REPLAY_REJECTION_EVENT}-")),
        TestContext("rejected replay warning uses the host-owned prefix"),
    );
    let (timestamp_micros, sequence) = require_some(
        event_id_suffix.rsplit_once('-'),
        TestContext("rejected replay warning identity has timestamp and sequence"),
    );
    let timestamp_micros = require_ok(
        timestamp_micros.parse::<i64>(),
        "rejected replay warning identity timestamp parses",
    );
    let sequence = require_ok(
        sequence.parse::<u64>(),
        "rejected replay warning identity sequence parses",
    );
    assert!(timestamp_micros > 0);
    assert!(sequence < u64::MAX);
    assert_eq!(diagnostic.correlation_id, None);
    let sent_at = require_some(
        diagnostic.sent_at.as_ref(),
        TestContext("rejected replay warning has host timestamp"),
    );
    let parsed_sent_at = require_ok(
        chrono::DateTime::parse_from_rfc3339(sent_at),
        "rejected replay warning timestamp parses",
    );
    assert_eq!(
        parsed_sent_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        sent_at.as_str()
    );
    assert_eq!(
        diagnostic
            .source_peer_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::peer::LOCAL_DEV_AGENT)
    );
    assert_eq!(
        diagnostic.source_role,
        Some(ParentRoutePeerRole::AgentService)
    );
    assert_eq!(
        diagnostic
            .target_peer_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::peer::PORTAL_DEV)
    );
    assert_eq!(diagnostic.target_role, Some(ParentRoutePeerRole::Portal));
    assert_eq!(diagnostic.payload, None);
    assert_eq!(diagnostic.snapshot, None);
    assert_eq!(diagnostic.command_result_projection, None);
}

fn remove_payload_field(event: &mut AgentEventEnvelope, field: &str) {
    event.payload = event
        .payload
        .iter()
        .filter(|(key, _value)| key.as_str() != field)
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
}

fn replay_entries(rows: &[LanDiscoveryEventRow]) -> Vec<Value> {
    rows.iter()
        .map(|row| {
            json!({
                "eventType": require_ok(
                    serde_json::to_value(&row.event_kind),
                    "event kind serializes"
                ),
                "eventRef": row.event_id,
                "payload": row,
            })
        })
        .collect()
}

fn replay_event(
    entries: &[Value],
    history_state: LanDiscoveryEventHistoryState,
    manual_required: bool,
) -> AgentEventEnvelope {
    let count = entries.len();
    let latest_event_id = entries
        .last()
        .and_then(|entry| entry["payload"]["eventId"].as_str())
        .unwrap_or_default();
    let latest_observed_at = entries
        .last()
        .and_then(|entry| entry["payload"]["occurredAt"].as_str())
        .unwrap_or_default();
    replay_event_with_stream(
        require_ok(serde_json::to_string(&entries), "replay entries serialize"),
        count,
        history_state,
        manual_required,
        latest_event_id,
        latest_observed_at,
    )
}

fn replay_event_with_stream(
    stream: String,
    count: usize,
    history_state: LanDiscoveryEventHistoryState,
    manual_required: bool,
    latest_event_id: &str,
    latest_observed_at: &str,
) -> AgentEventEnvelope {
    let history_state = require_ok(
        serde_json::to_value(history_state),
        "history state serializes",
    );
    let history_state = history_state.as_str().unwrap_or_default().to_string();
    let mut payload = BTreeMap::new();
    payload.insert(
        constants::field::GENERATED_AT.to_string(),
        LogFieldValue::String("2026-06-23T00:00:03Z".to_string()),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_OBSERVED_EVENTS.to_string(),
        LogFieldValue::Number(count as f64),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_STREAMED_EVENTS.to_string(),
        LogFieldValue::Number(count as f64),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_FAILED_EVENTS.to_string(),
        LogFieldValue::Number(0.0),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE.to_string(),
        LogFieldValue::String(history_state),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE.to_string(),
        LogFieldValue::Boolean(manual_required),
    );
    payload.insert(
        constants::field::LATEST_EVENT_ID.to_string(),
        LogFieldValue::String(latest_event_id.to_string()),
    );
    payload.insert(
        constants::field::LATEST_OBSERVED_AT.to_string(),
        LogFieldValue::String(latest_observed_at.to_string()),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM.to_string(),
        LogFieldValue::String(stream),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.lan.runtime.event-chain.stream.reported-1".to_string(),
        correlation_id: REQUEST_MESSAGE_ID_CORRELATION.to_string(),
        sent_at: "2026-06-23T00:00:03Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentLanRuntimeEventChainStreamReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}
