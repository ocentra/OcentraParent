use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use serde_json::Value;

use crate::{
    app::{
        lan_pairing::LanPairingRuntime,
        lan_runtime_stream_payload::{
            lan_runtime_event_chain_stream_payload, stream_lan_runtime_event_chain_for_history,
        },
        websocket::handle_command_text_for_test,
    },
    test_invariants::serialize_test_json,
};

#[test]
fn lan_runtime_stream_payload_serializes_replayable_discovery_event_rows() {
    let history = discovery_event_history();
    let report = stream_lan_runtime_event_chain_for_history(&history);
    let payload = lan_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_events, 2);
    assert_eq!(report.streamed_events, 2);
    assert_eq!(report.failed_events, 0);
    assert!(!report.manual_required_state);
    assert_eq!(
        payload.get(constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE),
        Some(&LogFieldValue::String("ready".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::LATEST_EVENT_ID),
        Some(&LogFieldValue::String(
            "lan-discovery-device-found-1".to_string()
        ))
    );
    assert_eq!(
        entries[0][constants::field::EVENT_TYPE],
        serde_json::json!("scan-started")
    );
    assert_eq!(
        entries[1][constants::field::EVENT_TYPE],
        serde_json::json!("device-found")
    );
    assert_eq!(
        entries[1][constants::field::PAYLOAD]["previousEventId"],
        serde_json::json!("lan-discovery-scan-started-1")
    );
    assert_eq!(
        entries[1][constants::field::PAYLOAD]["affectedDeviceId"],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
}

#[tokio::test]
async fn websocket_lan_runtime_stream_command_reports_service_backed_stream() {
    let body = serialize_test_json(&command_envelope());
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanRuntimeEventChainStreamReported
    );
    assert!(event
        .payload
        .contains_key(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM));
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_RUNTIME_FAILED_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
}

fn discovery_event_history() -> LanDiscoveryEventHistory {
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        state: LanDiscoveryEventHistoryState::Ready,
        latest_event_id: Some("lan-discovery-device-found-1".to_string()),
        latest_observed_at: Some(constants::lan_pairing::OBSERVED_AT.to_string()),
        rows: vec![
            discovery_event_row(
                "lan-discovery-scan-started-1",
                LanDiscoveryEventKind::ScanStarted,
                None,
            ),
            discovery_event_row(
                "lan-discovery-device-found-1",
                LanDiscoveryEventKind::DeviceFound,
                Some("lan-discovery-scan-started-1"),
            ),
        ],
    }
}

fn discovery_event_row(
    event_id: &str,
    event_kind: LanDiscoveryEventKind,
    previous_event_id: Option<&str>,
) -> LanDiscoveryEventRow {
    LanDiscoveryEventRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        event_id: event_id.to_string(),
        event_kind,
        occurred_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        previous_event_id: previous_event_id.map(str::to_string),
        scan_session_id: Some("lan-scan-session-1".to_string()),
        affected_device_id: Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        evidence_id: Some("lan-evidence-1".to_string()),
        summary: "LAN discovery event from Rust read model".to_string(),
    }
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::lan_pairing::EVENT_RUNTIME_EVENT_CHAIN_STREAM_REPORTED.to_string(),
        sent_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentLanRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}

fn stream_entries(payload: &LogFields) -> Vec<Value> {
    match payload.get(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_default(),
        _ => Vec::new(),
    }
}
