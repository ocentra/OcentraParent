use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_protocol::activity::ActivityEvidenceKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkEndpoint;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowCounters;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS;
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;

use super::{
    activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery,
    network_product_path_bridge::NetworkProductPathServiceProofReport,
    network_runtime_delivery::NetworkRuntimeServiceDeliveryReport,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[test]
fn network_flow_payload_contains_contract_shaped_digest_json() -> TestResult {
    let read_model = read_model();

    let payload = network_flow_read_model_payload_with_runtime_delivery(&read_model, None, None);
    let digest_json = payload
        .get(constants::field::ACTIVITY_DIGEST)
        .and_then(|value| match value {
            LogFieldValue::String(text) => Some(text),
            _ => None,
        })
        .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    let digest: ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowDigest =
        serde_json::from_str(digest_json).map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })?;

    assert_eq!(
        digest.top_destinations[0].label,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(digest.unusual_indicators.len(), 0);
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY),
        Some(&LogFieldValue::String(
            NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT.to_string()
        ))
    );

    Ok(())
}

#[test]
fn network_flow_payload_reports_tombstone_refs_without_active_rows() {
    let read_model = ActivityNetworkFlowReadModel {
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
        ..read_model()
    };

    let payload = network_flow_read_model_payload_with_runtime_delivery(&read_model, None, None);

    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_NETWORK_EVENT_ID.to_string()
        ))
    );
}

#[test]
fn network_flow_payload_includes_runtime_delivery_counts_when_supplied() {
    let read_model = read_model();
    let delivery = NetworkRuntimeServiceDeliveryReport {
        observed_rows: 1,
        delivered_rows: 1,
        failed_rows: 0,
        publish_reports: 11,
        stored_events: 11,
        dead_letters: 0,
        manual_required_rows: 0,
        enforcement_command_events: 1,
    };

    let payload =
        network_flow_read_model_payload_with_runtime_delivery(&read_model, Some(&delivery), None);

    assert_eq!(
        payload.get(constants::field::NETWORK_RUNTIME_OBSERVED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_RUNTIME_DELIVERED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS),
        Some(&LogFieldValue::Number(1.0))
    );
}

#[test]
fn network_flow_payload_includes_product_path_counts_and_refs_when_supplied() {
    let read_model = read_model();
    let product_path = product_path_report();

    let payload = network_flow_read_model_payload_with_runtime_delivery(
        &read_model,
        None,
        Some(&product_path),
    );

    assert_product_path_payload(&payload);
}

fn product_path_report() -> NetworkProductPathServiceProofReport {
    NetworkProductPathServiceProofReport {
        observed_rows: 1,
        proved_rows: 1,
        skipped_rows: 0,
        failed_rows: 0,
        manual_required_rows: 1,
        unavailable_rows: 0,
        policy_decision_count: 1,
        action_result_count: 1,
        retention_record_count: 1,
        delete_record_count: 1,
        export_record_count: 1,
        portal_read_model_count: 1,
        enforcement_command_events: 0,
        adapter_action_executed_count: 0,
        ai_advisory_rows: 1,
        weak_or_unavailable_blocked_rows: 1,
        analyzer_alert_refs: vec![row_ref(
            constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX,
        )],
        ai_detection_refs: vec![row_ref(
            constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX,
        )],
        risk_budget_refs: vec![constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_string()],
        policy_decision_refs: vec![constants::network_flow::TEST_POLICY_DECISION_REF.to_string()],
        action_result_refs: vec![constants::network_flow::TEST_ENFORCEMENT_RESULT_REF.to_string()],
        retention_refs: vec![
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF.to_string(),
        ],
        deletion_refs: vec![
            constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF.to_string(),
        ],
        export_refs: vec![
            constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF.to_string(),
        ],
        portal_read_model_refs: vec![
            constants::network_flow::TEST_PORTAL_READ_MODEL_REF.to_string()
        ],
    }
}

fn assert_product_path_payload(payload: &std::collections::BTreeMap<String, LogFieldValue>) {
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_OBSERVED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_PROVED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_ENFORCEMENT_COMMAND_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_WEAK_OR_UNAVAILABLE_BLOCKED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_ANALYZER_ALERT_REFS),
        Some(&LogFieldValue::String(row_ref(
            constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX
        )))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_AI_DETECTION_REFS),
        Some(&LogFieldValue::String(row_ref(
            constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX
        )))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_RISK_BUDGET_REFS),
        Some(&LogFieldValue::String(
            constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_POLICY_DECISION_REFS),
        Some(&LogFieldValue::String(
            constants::network_flow::TEST_POLICY_DECISION_REF.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_PORTAL_READ_MODEL_REFS),
        Some(&LogFieldValue::String(
            constants::network_flow::TEST_PORTAL_READ_MODEL_REF.to_string()
        ))
    );
}

fn read_model() -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        active_rows: 1,
        tombstone_rows: 0,
        exportable_rows: 1,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        latest_event_id: Some(constants::activity_store::TEST_NETWORK_EVENT_ID.to_string()),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
        rows: vec![observation()],
    }
}

fn observation() -> ActivityNetworkFlowObservation {
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
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        domain_attribution_status:
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string(),
        process_attribution_status:
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string(),
        process_id: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
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

fn test_network_evidence_id() -> String {
    let mut evidence_id = String::from(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(constants::activity_store::TEST_NETWORK_EVENT_ID);
    evidence_id
}

fn row_ref(prefix: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(&test_network_evidence_id());
    value
}
