use std::collections::BTreeMap;
use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use ocentra_schema::parent_ui_bridge::{ParentRouteId, ParentRoutePeerRole};
use serde_json::{json, Value};

use super::common::helpers::{
    sample_lan_read_model_with_explicit_history, sample_lan_read_model_with_history_state,
};
use super::load_parent_subscription_event;
use super::tests_support::{
    lan_event, require_ok, start_local_server_with_capture_responses, with_agent_addr,
};

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
        );
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|event| {
            event
                .event_id
                .as_ref()
                .is_none_or(|event_id| !event_id.as_str().starts_with("lan-history-"))
        }));
    }
}

#[test]
fn parent_subscription_event_fails_closed_on_malformed_lan_replay_payload() {
    let read_model = sample_lan_read_model_with_explicit_history();
    let malformed_event = replay_event_with_stream(
        "{not-json".to_string(),
        2,
        LanDiscoveryEventHistoryState::Ready,
        false,
    );

    let events = subscription_events_for_stream(&read_model, malformed_event);

    assert_eq!(events.len(), 2);
    assert!(events.iter().all(|event| {
        event
            .event_id
            .as_ref()
            .is_none_or(|event_id| !event_id.as_str().starts_with("lan-history-"))
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
) -> Vec<ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot> {
    let (address, _requests) = start_local_server_with_capture_responses(vec![
        lan_event(AgentEventName::AgentLanPairingStatusReported, read_model),
        stream_event,
    ]);
    with_agent_addr(&address, || {
        load_parent_subscription_event(ParentRouteId::Devices, None)
            .events
            .unwrap_or_default()
    })
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
    replay_event_with_stream(
        require_ok(serde_json::to_string(&entries), "replay entries serialize"),
        count,
        history_state,
        manual_required,
    )
}

fn replay_event_with_stream(
    stream: String,
    count: usize,
    history_state: LanDiscoveryEventHistoryState,
    manual_required: bool,
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
        LogFieldValue::String(String::new()),
    );
    payload.insert(
        constants::field::LATEST_OBSERVED_AT.to_string(),
        LogFieldValue::String(String::new()),
    );
    payload.insert(
        constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM.to_string(),
        LogFieldValue::String(stream),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.lan.runtime.event-chain.stream.reported-1".to_string(),
        correlation_id: "lan-runtime-stream".to_string(),
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
