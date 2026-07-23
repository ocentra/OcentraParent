use std::string::String as TestString;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NetworkEvidenceGrade, NetworkFlowObservedEvent,
    NetworkPolicyDecisionAction, NetworkPolicyDecisionCompletedEvent, NetworkRuntimePhase,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    deliver_network_runtime_for_read_model_for_test,
    network_runtime_event_chain_stream_payload_for_test,
    stream_network_runtime_event_chain_for_read_model_for_test,
};
use serde_json::Value;

#[tokio::test]
async fn service_network_read_model_delivers_local_runtime_chain() -> Result<(), serde_json::Error>
{
    let model = read_model(vec![full_metadata_row()]);
    let report = deliver_network_runtime_for_read_model_for_test(&model).await;
    let stream_report = stream_network_runtime_event_chain_for_read_model_for_test(&model).await;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload_for_test(
        &stream_report,
    ));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.dead_letters, 0);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert!(report.publish_reports > 0);
    assert_eq!(
        report.publish_reports,
        NetworkRuntimePhase::ordered_chain().len() - 4
    );
    assert_eq!(report.publish_reports, report.stored_events);
    let event_types = event_types(&entries);
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
async fn service_network_read_model_keeps_partial_metadata_manual_required(
) -> Result<(), serde_json::Error> {
    let model = read_model(vec![partial_metadata_row()]);
    let report = deliver_network_runtime_for_read_model_for_test(&model).await;
    let stream_report = stream_network_runtime_event_chain_for_read_model_for_test(&model).await;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload_for_test(
        &stream_report,
    ));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(
        report.publish_reports,
        NetworkRuntimePhase::ordered_chain().len() - 4
    );
    let event_types = event_types(&entries);
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
    let flow_event: NetworkFlowObservedEvent =
        serde_json::from_value(entries[0][constants::field::PAYLOAD].clone())?;
    assert_eq!(flow_event.evidence_grade, NetworkEvidenceGrade::C);
    Ok(())
}

#[tokio::test]
async fn empty_service_network_read_model_does_not_invent_runtime_events() {
    let report = deliver_network_runtime_for_read_model_for_test(&read_model(Vec::new())).await;

    assert_eq!(report.observed_rows, 0);
    assert_eq!(report.delivered_rows, 0);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.publish_reports, 0);
    assert_eq!(report.stored_events, 0);
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

fn stream_entries(payload: &ocentra_parent_agent_protocol::logging::LogFields) -> Vec<Value> {
    match payload.get(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM) {
        Some(ocentra_parent_agent_protocol::logging::LogFieldValue::String(text)) => {
            serde_json::from_str(text).unwrap_or_default()
        }
        _ => Vec::new(),
    }
}

fn event_types(entries: &[Value]) -> Vec<&str> {
    entries
        .iter()
        .map(|entry| {
            entry[constants::field::EVENT_TYPE]
                .as_str()
                .unwrap_or_default()
        })
        .collect()
}
