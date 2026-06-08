use ocentra_parent_agent_core::{network_observation_event, ActivityStore, NetworkObservation};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityEvent, ActivityEvidenceKind,
    ActivityNetworkProtocol, ActivityNetworkTcpState, LogFieldValue,
};

use crate::{
    activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery,
    network_product_path_bridge::prove_network_product_path_for_read_model,
};

#[test]
fn captured_network_metadata_drives_product_path_payload_without_content_or_enforcement_claims() {
    let event = product_path_network_event();
    let evidence_id = event.evidence[0].evidence_id.clone();
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let product_path = prove_network_product_path_for_read_model(&read_model);
    let payload = network_flow_read_model_payload_with_runtime_delivery(
        &read_model,
        None,
        Some(&product_path),
    );

    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.rows[0].evidence.len(), 1);
    assert_eq!(
        read_model.rows[0].evidence[0].kind,
        ActivityEvidenceKind::LocalDbRow
    );
    assert!(evidence_id.starts_with(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX));
    assert_product_path_report_refs(&product_path, &evidence_id);
    assert_product_path_payload_refs(&payload, &evidence_id);
}

fn product_path_network_event() -> ActivityEvent {
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

fn assert_product_path_report_refs(
    product_path: &crate::network_product_path_bridge::NetworkProductPathServiceProofReport,
    evidence_id: &str,
) {
    assert_eq!(product_path.observed_rows, 1);
    assert_eq!(product_path.proved_rows, 1);
    assert_eq!(product_path.enforcement_command_events, 0);
    assert_eq!(product_path.adapter_action_executed_count, 0);
    assert_eq!(product_path.ai_advisory_rows, 1);
    assert_eq!(product_path.weak_or_unavailable_blocked_rows, 1);
    assert!(product_path.policy_decision_refs[0].contains(evidence_id));
    assert!(product_path.action_result_refs[0].contains(evidence_id));
    assert!(product_path.retention_refs[0].contains(evidence_id));
    assert!(product_path.deletion_refs[0].contains(evidence_id));
    assert!(product_path.export_refs[0].contains(evidence_id));
}

fn assert_product_path_payload_refs(
    payload: &ocentra_parent_agent_protocol::LogFields,
    evidence_id: &str,
) {
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_PROVED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_ENFORCEMENT_COMMAND_EVENTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_ADAPTER_ACTION_EXECUTED),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_payload_ref_contains(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_POLICY_DECISION_REFS,
        evidence_id,
    );
    assert_payload_ref_contains(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_ACTION_RESULT_REFS,
        evidence_id,
    );
    assert_payload_ref_contains(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_RETENTION_REFS,
        evidence_id,
    );
}

fn assert_payload_ref_contains(
    payload: &ocentra_parent_agent_protocol::LogFields,
    field: &str,
    expected: &str,
) {
    let value = payload
        .get(field)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let LogFieldValue::String(text) = value else {
        std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES);
    };
    assert!(text.contains(expected));
}
