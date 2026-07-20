use std::string::String as TestString;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NetworkRuntimePhase,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::deliver_network_runtime_for_read_model_for_test;

#[tokio::test]
async fn service_network_read_model_delivers_local_runtime_chain() {
    let report =
        deliver_network_runtime_for_read_model_for_test(&read_model(vec![full_metadata_row()]))
            .await;

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.dead_letters, 0);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert!(report.publish_reports > 0);
    assert_eq!(
        report.publish_reports,
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(report.publish_reports, report.stored_events);
}

#[tokio::test]
async fn service_network_read_model_keeps_partial_metadata_manual_required() {
    let report =
        deliver_network_runtime_for_read_model_for_test(&read_model(vec![partial_metadata_row()]))
            .await;

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(
        report.publish_reports,
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
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
