use std::fs::remove_file;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use crate::test_text::TestText;
use ocentra_parent_agent_core::{
    activity_store::ActivityStore, network_capture::NetworkObservation,
    network_capture_event::network_observation_event,
};
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NetworkEvidenceGrade, NetworkFlowObservedEvent,
    NetworkPolicyDecisionAction, NetworkPolicyDecisionCompletedEvent, NetworkRuntimePhase,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
    NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS, NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::{
    ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION, NETWORK_FLOW_SCHEMA_VERSION,
};
use ocentra_parent_agent_service::test_support::{
    handle_local_command_text_for_test, lock_activity_report_env_for_test,
    network_runtime_event_chain_stream_payload_for_test,
    stream_network_runtime_event_chain_for_read_model_for_test,
};
use serde_json::Value;

#[tokio::test]
async fn service_network_runtime_streams_protocol_event_chain_entries(
) -> Result<(), Box<dyn std::error::Error>> {
    let report = stream_network_runtime_event_chain_for_read_model_for_test(&read_model(vec![
        full_metadata_row(),
    ]))
    .await;
    let payload = network_runtime_event_chain_stream_payload_for_test(&report);
    let entries = stream_entries(&payload);

    assert_eq!(report.observed_rows, 1);
    assert_eq!(
        report.streamed_events,
        NetworkRuntimePhase::ordered_chain().len() - 4
    );
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(report.active_rows, 1);
    assert_eq!(report.exportable_rows, 1);
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        entries[0][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert!(entries[0][constants::field::EVENT_REF]
        .as_str()
        .unwrap_or_default()
        .ends_with(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED));
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CLAIM_BOUNDARY]
            [constants::field::EXACT_URL_AVAILABLE],
        false
    );
    assert_eq!(
        entries[5][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED
    );
    let flow_event: NetworkFlowObservedEvent =
        serde_json::from_value(entries[0][constants::field::PAYLOAD].clone())?;
    assert_eq!(flow_event.evidence_grade, NetworkEvidenceGrade::B);
    let policy_event: NetworkPolicyDecisionCompletedEvent =
        serde_json::from_value(entries[4][constants::field::PAYLOAD].clone())?;
    assert_eq!(
        policy_event.decision_action,
        NetworkPolicyDecisionAction::Observe
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CLAIM_BOUNDARY]
            [constants::field::ADAPTER_ACTION_EXECUTED],
        false
    );
    assert_eq!(
        entries
            .iter()
            .filter(|entry| {
                matches!(
                    entry[constants::field::EVENT_TYPE].as_str(),
                    Some(constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED)
                        | Some(constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED)
                )
            })
            .count(),
        0
    );
    Ok(())
}

#[tokio::test]
async fn service_network_runtime_stream_skips_enforcement_for_manual_required_rows(
) -> Result<(), Box<dyn std::error::Error>> {
    let report = stream_network_runtime_event_chain_for_read_model_for_test(&read_model(vec![
        partial_metadata_row(),
    ]))
    .await;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload_for_test(
        &report,
    ));
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
        NetworkRuntimePhase::ordered_chain().len() - 4
    );
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(
        event_types,
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
            constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED,
            constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED,
            constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
            constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED,
        ]
    );
    assert_eq!(
        entries[6][constants::field::PAYLOAD][constants::field::VISIBLE_MANUAL_REQUIRED],
        true
    );
    Ok(())
}

#[tokio::test]
async fn websocket_network_runtime_stream_command_reports_store_backed_chain(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let store_path = temp_path(&TestText::from_display(
        constants::activity_store::TEST_NETWORK_STORE_SUFFIX,
    ));
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path)
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    store
        .ingest_events(&[network_activity_event()])
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    let body = serde_json::to_string(&command_envelope())?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let entries = stream_entries(&event.payload);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkRuntimeEventChainStreamReported
    );
    assert_eq!(
        entries.len(),
        NetworkRuntimePhase::ordered_chain().len() - 4
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::NETWORK_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(
            (NetworkRuntimePhase::ordered_chain().len() - 4) as f64
        ))
    );
    let event_types = entries
        .iter()
        .map(|entry| {
            entry[constants::field::EVENT_TYPE]
                .as_str()
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
            constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED,
            constants::network_flow::EVENT_POLICY_DECISION_COMPLETED,
            constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
            constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED,
        ]
    );
    assert_eq!(
        entries[6][constants::field::EVENT_TYPE],
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
    );
    let flow_event: NetworkFlowObservedEvent =
        serde_json::from_value(entries[0][constants::field::PAYLOAD].clone())?;
    assert_eq!(flow_event.evidence_grade, NetworkEvidenceGrade::B);
    let policy_event: NetworkPolicyDecisionCompletedEvent =
        serde_json::from_value(entries[4][constants::field::PAYLOAD].clone())?;
    assert_eq!(
        policy_event.decision_action,
        NetworkPolicyDecisionAction::Observe
    );
    Ok(())
}

#[tokio::test]
async fn websocket_network_runtime_stream_reports_tombstone_without_streaming_deleted_row(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let store_path = temp_path(&TestText::from_display(
        constants::activity_store::TEST_NETWORK_STORE_SUFFIX,
    ));
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path)
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    let network_event = network_activity_event();
    let deleted_event_id = network_event.event_id.clone();
    store
        .ingest_events(&[
            network_event,
            network_retention_deleted_event(&deleted_event_id),
        ])
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    let body = serde_json::to_string(&command_envelope())?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let entries = stream_entries(&event.payload);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(entries.len(), 0);
    assert_eq!(
        event
            .payload
            .get(constants::field::NETWORK_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        event
            .payload
            .get(NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        event
            .payload
            .get(NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        event
            .payload
            .get(NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(deleted_event_id))
    );
    Ok(())
}

fn read_model(rows: Vec<ActivityNetworkFlowObservation>) -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: rows.len() as u64,
        active_rows: rows.len() as u64,
        tombstone_rows: 0,
        exportable_rows: rows.len() as u64,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        latest_event_id: rows.first().map(|row| row.event_id.clone()),
        latest_observed_at: rows.first().map(|row| row.observed_at.clone()),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
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
    destination_domain: Option<TestString>,
    process_name: Option<TestString>,
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

fn network_activity_event() -> ActivityEvent {
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

fn network_retention_deleted_event(deleted_event_id: &TestStr) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(deleted_event_id.to_string()),
    );
    fields.insert(
        constants::field::DELETED_AT.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        },
        kind: ActivityEventKind::NetworkRetentionDeleted,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Retention,
            subject_id: deleted_event_id.to_string(),
            display_name: None,
        },
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: deleted_event_id.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
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
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn temp_path(suffix: &TestText) -> TestPathBuf {
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_ref());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &TestPathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
