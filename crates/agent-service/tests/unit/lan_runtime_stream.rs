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
use std::{
    fmt::Display,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::{
    app::{
        lan_pairing::LanPairingRuntime,
        lan_runtime_stream_payload::{
            LAN_RUNTIME_EVENT_CHAIN_STREAM_PAYLOAD, STREAM_LAN_RUNTIME_EVENT_CHAIN_FOR_HISTORY,
        },
        websocket::handle_command_text_for_test,
    },
    test_require_ok::require_ok,
    test_require_some::require_some,
    test_serialize_json::serialize_test_json,
    test_text::TestText,
};

#[test]
fn lan_runtime_stream_payload_serializes_replayable_discovery_event_rows() {
    let history = discovery_event_history();
    let report =
        STREAM_LAN_RUNTIME_EVENT_CHAIN_FOR_HISTORY(&history, history.generated_at.clone().into());
    let payload = LAN_RUNTIME_EVENT_CHAIN_STREAM_PAYLOAD(&report);
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

#[test]
fn stream_report_uses_current_observation_time_without_rewriting_cached_history_rows() {
    let history = discovery_event_history();
    let observed_at = "2026-06-29T00:00:00.000Z".to_string();
    let report = STREAM_LAN_RUNTIME_EVENT_CHAIN_FOR_HISTORY(&history, observed_at.clone().into());
    let entries = stream_entries(&LAN_RUNTIME_EVENT_CHAIN_STREAM_PAYLOAD(&report));
    let history_rows = history
        .rows
        .iter()
        .map(|row| require_ok(serde_json::to_value(row), "cached history row serializes"))
        .collect::<Vec<_>>();

    assert_eq!(report.generated_at, observed_at);
    assert_ne!(report.generated_at, history.generated_at);
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry[constants::field::PAYLOAD].clone())
            .collect::<Vec<_>>(),
        history_rows
    );
}

#[tokio::test]
async fn websocket_lan_runtime_stream_command_does_not_fabricate_scan_delta_events_from_persisted_cache(
) {
    let temp_dir = std::env::temp_dir().join(format!(
        "lan-runtime-stream-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    require_ok(
        fs::create_dir_all(&temp_dir),
        "temporary LAN stream directory is created",
    );
    let registry_path = temp_dir.join("registry.json");
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    assert!(runtime.durable_pairing_registry_available());
    crate::lan_pairing_browser_add_device_state::scan_history::save_scan_history(
        &runtime,
        &[persisted_network_device()],
        None,
    );
    let restarted_runtime = LanPairingRuntime::persistent_json(&registry_path);
    let body = TestText::from_display(serialize_test_json(&command_envelope()));
    let event = handle_command_text_for_test(body, restarted_runtime, None).await;
    let entries = stream_entries(&event.payload);
    require_ok(
        fs::remove_dir_all(&temp_dir),
        "temporary LAN stream directory is removed",
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentLanRuntimeEventChainStreamReported
    );
    assert!(event
        .payload
        .get(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM)
        .is_some());
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_RUNTIME_FAILED_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert!(
        entries.is_empty(),
        "a persisted inventory without a persisted canonical replay projection must not fabricate replay rows"
    );
    assert!(entries.iter().all(|entry| {
        !matches!(
            entry[constants::field::EVENT_TYPE].as_str(),
            Some(
                "scan-started"
                    | "scan-finished"
                    | "device-found"
                    | "device-updated"
                    | "device-online"
                    | "device-offline"
            )
        )
    }));
}

#[tokio::test]
async fn persisted_cached_status_and_stream_report_identical_history_binding() {
    let temp_dir = std::env::temp_dir().join(format!(
        "lan-runtime-status-stream-parity-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    require_ok(
        fs::create_dir_all(&temp_dir),
        "temporary LAN status-stream parity directory is created",
    );
    let registry_path = temp_dir.join("registry.json");
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    crate::lan_pairing_browser_add_device_state::scan_history::save_scan_history(
        &runtime,
        &[persisted_network_device()],
        None,
    );
    let restarted_runtime = LanPairingRuntime::persistent_json(&registry_path);
    let status_body = TestText::from_display(serialize_test_json(&status_command_envelope()));
    let _projection_seed_event =
        handle_command_text_for_test(status_body.clone(), restarted_runtime.clone(), None).await;
    let status_event =
        handle_command_text_for_test(status_body, restarted_runtime.clone(), None).await;
    let stream_body = TestText::from_display(serialize_test_json(&command_envelope()));
    let stream_event = handle_command_text_for_test(stream_body, restarted_runtime, None).await;
    let status_read_model = require_some(
        match status_event
            .payload
            .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        {
            Some(LogFieldValue::String(read_model)) => Some(read_model.as_str()),
            _ => None,
        },
        "status event includes the LAN add-device read model",
    );
    let status_history = require_ok(
        serde_json::from_str::<
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
        >(status_read_model),
        "status LAN read model parses",
    )
    .discovery_event_history;
    let replay_generated_at = require_ok(
        chrono::DateTime::parse_from_rfc3339(&status_history.generated_at),
        "first persisted replay seed uses a portal-compatible RFC3339 timestamp",
    );
    assert_eq!(
        replay_generated_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        status_history.generated_at,
        "portal replay timestamp remains canonical after first legacy seed"
    );
    let stream_entries = stream_entries(&stream_event.payload);
    let status_rows = status_history
        .rows
        .iter()
        .map(|row| require_ok(serde_json::to_value(row), "status history row serializes"))
        .collect::<Vec<_>>();
    let streamed_rows = stream_entries
        .iter()
        .map(|entry| entry[constants::field::PAYLOAD].clone())
        .collect::<Vec<_>>();
    require_ok(
        fs::remove_dir_all(&temp_dir),
        "temporary LAN status-stream parity directory is removed",
    );

    assert_eq!(streamed_rows, status_rows);
    assert_ne!(
        payload_string(&stream_event.payload, constants::field::GENERATED_AT),
        Some(status_history.generated_at.as_str()),
        "stream report envelope is current while replay rows retain persisted history time"
    );
    assert_eq!(
        payload_string(&stream_event.payload, constants::field::LATEST_EVENT_ID),
        status_history.latest_event_id.as_deref()
    );
    assert_eq!(
        payload_string(&stream_event.payload, constants::field::LATEST_OBSERVED_AT),
        status_history.latest_observed_at.as_deref()
    );
    assert_eq!(
        payload_string(
            &stream_event.payload,
            constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE
        ),
        require_ok(
            serde_json::to_value(&status_history.state),
            "status history state serializes"
        )
        .as_str()
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
                TestText::from_display("lan-discovery-scan-started-1"),
                LanDiscoveryEventKind::ScanStarted,
                None,
            ),
            discovery_event_row(
                TestText::from_display("lan-discovery-device-found-1"),
                LanDiscoveryEventKind::DeviceFound,
                Some(TestText::from_display("lan-discovery-scan-started-1")),
            ),
        ],
    }
}

fn discovery_event_row(
    event_id: impl Display,
    event_kind: LanDiscoveryEventKind,
    previous_event_id: Option<TestText>,
) -> LanDiscoveryEventRow {
    let event_id = TestText::from_display(event_id);
    LanDiscoveryEventRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        event_id: event_id.0,
        event_kind,
        occurred_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        previous_event_id: previous_event_id.map(|value| value.0),
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
            route: AgentRoute::LocalNetwork,
        },
        command: AgentCommandName::AgentLanRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}

fn status_command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        command: AgentCommandName::AgentLanPairingStatusGet,
        target: AgentMessageTarget {
            route: AgentRoute::Localhost,
            ..command_envelope().target
        },
        ..command_envelope()
    }
}

fn persisted_network_device() -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: "persisted-lan-device".to_string(),
        label: "Persisted LAN Device".to_string(),
        platform: "windows".to_string(),
        ip_address: "192.168.1.25".to_string(),
        mac_address: "00-11-22-33-44-55".to_string(),
        hostname: Some("persisted-device".to_string()),
        network_interface: Some("Ethernet".to_string()),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: true,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn stream_entries(payload: &LogFields) -> Vec<Value> {
    match payload.get(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn payload_string<'a>(payload: &'a LogFields, field: &str) -> Option<&'a str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) => Some(value.as_str()),
        _ => None,
    }
}
