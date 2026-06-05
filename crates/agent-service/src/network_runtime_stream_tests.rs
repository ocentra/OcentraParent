use std::fs::remove_file;

use ocentra_parent_agent_core::{
    network_observation_event, ActivityStore, NetworkObservation, NetworkRuntimePhase,
};
use ocentra_parent_agent_protocol::{
    constants, policy_constants, ActivityCaptureCapabilityStatus, ActivityEvidenceKind,
    ActivityEvidenceRef, ActivityNetworkEndpoint, ActivityNetworkFlowCounters,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel, ActivityNetworkProtocol,
    ActivityNetworkTcpState, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    NETWORK_FLOW_SCHEMA_VERSION,
};
use serde_json::Value;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK,
    lan_pairing::LanPairingRuntime,
    network_runtime_stream_payload::{
        network_runtime_event_chain_stream_payload,
        stream_network_runtime_event_chain_for_read_model,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn service_network_runtime_streams_protocol_event_chain_entries() {
    let report =
        stream_network_runtime_event_chain_for_read_model(&read_model(vec![full_metadata_row()]))
            .await;
    let payload = network_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 1);
    assert_eq!(
        report.streamed_events,
        NetworkRuntimePhase::ordered_chain().len()
    );
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.enforcement_command_events, 1);
    assert_eq!(
        entries[0][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(
        entries[0][constants::field::EVENT_REF]
            .as_str()
            .unwrap_or_default()
            .ends_with(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED),
        true
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CLAIM_BOUNDARY]
            [constants::field::EXACT_URL_AVAILABLE],
        false
    );
    assert_eq!(
        entries[7][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
    );
    assert_eq!(
        entries[8][constants::field::PAYLOAD][constants::field::ADAPTER_ACTION_EXECUTED],
        false
    );
}

#[tokio::test]
async fn service_network_runtime_stream_skips_enforcement_for_manual_required_rows() {
    let report =
        stream_network_runtime_event_chain_for_read_model(&read_model(
            vec![partial_metadata_row()],
        ))
        .await;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload(&report));
    let event_types = entries
        .iter()
        .map(|entry| {
            entry[constants::field::EVENT_TYPE]
                .as_str()
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    assert_eq!(report.observed_rows, 1);
    assert_eq!(
        report.streamed_events,
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert!(!event_types.contains(&constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED));
    assert!(!event_types.contains(&constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED));
    assert_eq!(
        entries.last().unwrap()[constants::field::PAYLOAD]
            [constants::field::VISIBLE_MANUAL_REQUIRED],
        true
    );
}

#[tokio::test]
async fn websocket_network_runtime_stream_command_reports_store_backed_chain() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_NETWORK_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[network_activity_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let entries = stream_entries(&event.payload);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkRuntimeEventChainStreamReported
    );
    assert_eq!(entries.len(), NetworkRuntimePhase::ordered_chain().len());
    assert_eq!(
        event
            .payload
            .get(constants::field::NETWORK_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(
            NetworkRuntimePhase::ordered_chain().len() as f64
        ))
    );
    assert_eq!(
        entries[10][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
    );
}

fn read_model(rows: Vec<ActivityNetworkFlowObservation>) -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: rows.len() as u64,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        rows,
    }
}

fn full_metadata_row() -> ActivityNetworkFlowObservation {
    row(
        Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        Some(4242),
    )
}

fn partial_metadata_row() -> ActivityNetworkFlowObservation {
    row(None, None, None)
}

fn row(
    destination_domain: Option<String>,
    process_name: Option<String>,
    process_id: Option<u64>,
) -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observer: constants::activity_observer::WINDOWS_NETWORK.to_string(),
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        adapter_id: constants::activity_capture::NETWORK_ADAPTER_ID.to_string(),
        protocol: Some(constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()),
        tcp_state: Some(constants::activity_capture::TCP_STATE_ESTABLISHED.to_string()),
        local_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        },
        destination_domain,
        domain_attribution_status:
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string(),
        process_attribution_status:
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string(),
        process_id,
        process_name,
        counters: ActivityNetworkFlowCounters {
            connection_count: 1,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        },
        evidence: vec![ActivityEvidenceRef {
            evidence_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
}

fn network_activity_event() -> ocentra_parent_agent_protocol::ActivityEvent {
    network_observation_event(
        NetworkObservation {
            status: ActivityCaptureCapabilityStatus::Available,
            protocol: Some(ActivityNetworkProtocol::Tcp),
            local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
            destination_ip: Some(
                constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string(),
            ),
            destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
            destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
            tcp_state: Some(ActivityNetworkTcpState::Established),
            pid: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
            associated_pid_count: 1,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    )
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED.to_string(),
        sent_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}

fn stream_entries(payload: &LogFields) -> Vec<Value> {
    match payload.get(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
