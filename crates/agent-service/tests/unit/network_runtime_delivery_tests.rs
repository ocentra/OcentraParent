use std::string::String as TestString;

use crate::network_runtime_test_support::{
    deliver_network_runtime_for_read_model_for_test, lock_activity_report_env_for_test,
    network_runtime_event_chain_stream_payload_for_test, network_runtime_journal_path_for_test,
    seed_network_runtime_for_test, stream_network_runtime_event_chain_for_read_model_for_test,
    NetworkRuntimeTestError,
};
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NetworkEvidenceGrade, NetworkFlowObservedEvent,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
use serde_json::Value;

#[tokio::test]
async fn service_network_read_model_delivers_local_runtime_chain(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let model = read_model(vec![full_metadata_row()]);
    seed_network_runtime_for_test(&model).await?;
    let before_projection = journal_line_count()?;
    let report = deliver_network_runtime_for_read_model_for_test(&model).await?;
    let stream_report = stream_network_runtime_event_chain_for_read_model_for_test(&model).await?;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload_for_test(
        &stream_report,
    ));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.dead_letters, 0);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(report.publish_reports, 0);
    assert_eq!(report.stored_events, 3);
    assert_eq!(before_projection, journal_line_count()?);
    let event_types = event_types(&entries);
    assert_eq!(
        event_types,
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
        ]
    );
    let flow_event: NetworkFlowObservedEvent =
        serde_json::from_value(entries[0][constants::field::PAYLOAD].clone())?;
    assert_eq!(flow_event.evidence_grade, NetworkEvidenceGrade::B);
    Ok(())
}

#[tokio::test]
async fn service_network_read_model_keeps_partial_metadata_manual_required(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let model = read_model(vec![partial_metadata_row()]);
    seed_network_runtime_for_test(&model).await?;
    let report = deliver_network_runtime_for_read_model_for_test(&model).await?;
    let stream_report = stream_network_runtime_event_chain_for_read_model_for_test(&model).await?;
    let entries = stream_entries(&network_runtime_event_chain_stream_payload_for_test(
        &stream_report,
    ));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(report.publish_reports, 0);
    assert_eq!(report.stored_events, 2);
    let event_types = event_types(&entries);
    assert_eq!(
        event_types,
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
        ]
    );
    let flow_event: NetworkFlowObservedEvent =
        serde_json::from_value(entries[0][constants::field::PAYLOAD].clone())?;
    assert_eq!(flow_event.evidence_grade, NetworkEvidenceGrade::C);
    Ok(())
}

#[tokio::test]
async fn empty_service_network_read_model_does_not_invent_runtime_events(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let report = deliver_network_runtime_for_read_model_for_test(&read_model(Vec::new())).await?;

    assert_eq!(report.observed_rows, 0);
    assert_eq!(report.delivered_rows, 0);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.publish_reports, 0);
    assert_eq!(report.stored_events, 0);

    Ok(())
}

#[tokio::test]
async fn invalid_source_event_id_fails_closed_without_projection_events(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let mut invalid = full_metadata_row();
    invalid.event_id = "invalid event id".to_string();
    let report =
        deliver_network_runtime_for_read_model_for_test(&read_model(vec![invalid])).await?;

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 0);
    assert_eq!(report.failed_rows, 1);
    assert_eq!(report.stored_events, 0);

    Ok(())
}

#[tokio::test]
async fn malformed_persisted_network_rows_fail_closed_without_journal_mutation(
) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let valid_model = read_model(vec![full_metadata_row()]);
    seed_network_runtime_for_test(&valid_model).await?;
    let before = journal_line_count()?;

    let mut malformed_rows = Vec::new();
    let mut unknown_status = full_metadata_row();
    unknown_status.capability_status = "unknown-capability-status".to_string();
    malformed_rows.push(unknown_status);

    let mut invalid_protocol = full_metadata_row();
    invalid_protocol.protocol = Some("unknown-protocol".to_string());
    malformed_rows.push(invalid_protocol);

    let mut invalid_tcp_state = full_metadata_row();
    invalid_tcp_state.tcp_state = Some("unknown-tcp-state".to_string());
    malformed_rows.push(invalid_tcp_state);

    let mut oversized_process_id = full_metadata_row();
    oversized_process_id.process_id = Some(u64::MAX);
    malformed_rows.push(oversized_process_id);

    let mut malformed_observed_at = full_metadata_row();
    malformed_observed_at.observed_at = "not-rfc3339".to_string();
    malformed_rows.push(malformed_observed_at);

    for malformed in malformed_rows {
        let report =
            deliver_network_runtime_for_read_model_for_test(&read_model(vec![malformed])).await?;
        assert_eq!(report.failed_rows, 1);
        assert_eq!(report.stored_events, 0);
        assert_eq!(journal_line_count()?, before);
    }

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
    row_with_event_id(
        "network-flow-event-full-1",
        Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        Some(4242),
        Some(1),
    )
}

fn partial_metadata_row() -> ActivityNetworkFlowObservation {
    row_with_event_id("network-flow-event-partial-1", None, None, None, Some(0))
}

fn row_with_event_id(
    event_id: &str,
    destination_domain: Option<TestString>,
    process_name: Option<TestString>,
    process_id: Option<u64>,
    associated_pid_count: Option<usize>,
) -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: event_id.to_string(),
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
        associated_pid_count,
        counters: ActivityNetworkFlowCounters {
            connection_count: 1,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        },
        evidence: vec![ActivityEvidenceRef {
            evidence_id: event_id.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
}

fn journal_line_count() -> Result<usize, NetworkRuntimeTestError> {
    Ok(
        std::fs::read_to_string(network_runtime_journal_path_for_test()?.as_path())
            .map(|text| text.lines().count())
            .unwrap_or_default(),
    )
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
