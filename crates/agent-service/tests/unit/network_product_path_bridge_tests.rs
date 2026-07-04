use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::prove_network_product_path_for_read_model_for_test;
use std::primitive::str as TestStr;
use std::string::String as TestString;

#[test]
fn stored_network_flow_rows_drive_product_path_refs_without_enforcement() {
    let report =
        prove_network_product_path_for_read_model_for_test(&read_model(vec![full_metadata_row()]));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.proved_rows, 1);
    assert_eq!(report.skipped_rows, 0);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.unavailable_rows, 0);
    assert_eq!(report.policy_decision_count, 1);
    assert_eq!(report.action_result_count, 1);
    assert_eq!(report.retention_record_count, 1);
    assert_eq!(report.delete_record_count, 1);
    assert_eq!(report.export_record_count, 1);
    assert_eq!(report.portal_read_model_count, 1);
    assert_eq!(report.enforcement_command_events, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.ai_advisory_rows, 1);
    assert_eq!(report.weak_or_unavailable_blocked_rows, 1);
    assert_eq!(
        report.analyzer_alert_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX
        )]
    );
    assert_eq!(
        report.ai_detection_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX
        )]
    );
    assert_eq!(
        report.risk_budget_refs,
        vec![constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_string()]
    );
    assert_eq!(
        report.policy_decision_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_POLICY_DECISION_REF_PREFIX
        )]
    );
    assert_eq!(
        report.action_result_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_ACTION_RESULT_REF_PREFIX
        )]
    );
    assert_eq!(
        report.retention_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_RETENTION_REF_PREFIX
        )]
    );
    assert_eq!(
        report.deletion_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_DELETION_REF_PREFIX
        )]
    );
    assert_eq!(
        report.export_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_EXPORT_REF_PREFIX
        )]
    );
    assert_eq!(
        report.portal_read_model_refs,
        vec![row_ref(
            constants::network_flow::PRODUCT_PATH_PORTAL_READ_MODEL_REF_PREFIX
        )]
    );
}

#[test]
fn tombstoned_network_flow_rows_do_not_drive_product_path_decisions() {
    let report =
        prove_network_product_path_for_read_model_for_test(&ActivityNetworkFlowReadModel {
            returned: 0,
            active_rows: 0,
            tombstone_rows: 1,
            exportable_rows: 0,
            latest_event_id: Some(
                constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
            ),
            latest_observed_at: Some(
                constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
            ),
            latest_tombstone_event_id: Some(
                constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
            ),
            latest_tombstone_observed_at: Some(
                constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
            ),
            deleted_evidence_reference_ids: vec![
                constants::activity_store::TEST_NETWORK_EVENT_ID.to_string()
            ],
            rows: Vec::new(),
            ..read_model(Vec::new())
        });

    assert_eq!(report.observed_rows, 0);
    assert_eq!(report.proved_rows, 0);
    assert_eq!(report.skipped_rows, 0);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.policy_decision_count, 0);
    assert_eq!(report.action_result_count, 0);
    assert_eq!(report.retention_record_count, 0);
    assert_eq!(report.delete_record_count, 0);
    assert_eq!(report.export_record_count, 0);
    assert_eq!(report.portal_read_model_count, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert!(report.analyzer_alert_refs.is_empty());
    assert!(report.ai_detection_refs.is_empty());
    assert!(report.risk_budget_refs.is_empty());
    assert!(report.policy_decision_refs.is_empty());
    assert!(report.action_result_refs.is_empty());
    assert!(report.retention_refs.is_empty());
    assert!(report.deletion_refs.is_empty());
    assert!(report.export_refs.is_empty());
    assert!(report.portal_read_model_refs.is_empty());
}

#[test]
fn rows_without_domain_target_are_skipped_without_inventing_policy_refs() {
    let report = prove_network_product_path_for_read_model_for_test(&read_model(vec![row(
        None, None, None,
    )]));

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.proved_rows, 0);
    assert_eq!(report.skipped_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.policy_decision_count, 0);
    assert_eq!(report.action_result_count, 0);
    assert_eq!(report.enforcement_command_events, 0);
    assert!(report.analyzer_alert_refs.is_empty());
    assert!(report.ai_detection_refs.is_empty());
    assert!(report.risk_budget_refs.is_empty());
    assert!(report.policy_decision_refs.is_empty());
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
            evidence_id: test_network_evidence_id(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    }
}

fn row_ref(prefix: &TestStr) -> TestString {
    let mut value = TestString::from(prefix);
    value.push_str(&test_network_evidence_id());
    value
}

fn test_network_evidence_id() -> TestString {
    let mut evidence_id = TestString::from(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(constants::activity_store::TEST_NETWORK_EVENT_ID);
    evidence_id
}
