use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore, network_capture::NetworkObservation,
    network_capture_event::network_observation_event,
};
use ocentra_parent_agent_protocol::activity::{ActivityEvent, ActivityEvidenceKind};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_service::test_support::{
    network_flow_read_model_payload_with_runtime_delivery_for_test,
    prove_network_product_path_for_read_model_for_test,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[test]
fn captured_network_metadata_drives_product_path_payload_without_content_or_enforcement_claims(
) -> TestResult {
    let event = product_path_network_event();
    let evidence_id = event.evidence[0].evidence_id.clone();
    let store = ActivityStore::open_in_memory().map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    store
        .ingest_events(std::slice::from_ref(&event))
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;
    let product_path = prove_network_product_path_for_read_model_for_test(&read_model);
    let payload = network_flow_read_model_payload_with_runtime_delivery_for_test(
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
    assert_product_path_payload_refs(&payload, &evidence_id)?;

    Ok(())
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
    product_path: &ocentra_parent_agent_service::test_support::NetworkProductPathServiceProofReportForTest,
    evidence_id: &TestStr,
) {
    let analyzer_alert_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX,
        evidence_id,
    );
    let ai_detection_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX,
        evidence_id,
    );
    let policy_decision_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_POLICY_DECISION_REF_PREFIX,
        evidence_id,
    );
    let action_result_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_ACTION_RESULT_REF_PREFIX,
        evidence_id,
    );
    let retention_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_RETENTION_REF_PREFIX,
        evidence_id,
    );
    let deletion_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_DELETION_REF_PREFIX,
        evidence_id,
    );
    let export_ref = product_path_ref(
        constants::network_flow::PRODUCT_PATH_EXPORT_REF_PREFIX,
        evidence_id,
    );

    assert_eq!(product_path.observed_rows, 1);
    assert_eq!(product_path.proved_rows, 1);
    assert_eq!(product_path.enforcement_command_events, 0);
    assert_eq!(product_path.adapter_action_executed_count, 0);
    assert_eq!(product_path.ai_advisory_rows, 1);
    assert_eq!(product_path.weak_or_unavailable_blocked_rows, 1);
    assert_eq!(&product_path.analyzer_alert_refs, &vec![analyzer_alert_ref]);
    assert_eq!(&product_path.ai_detection_refs, &vec![ai_detection_ref]);
    assert_eq!(
        product_path.risk_budget_refs,
        vec![constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_string()]
    );
    assert_eq!(
        &product_path.policy_decision_refs,
        &vec![policy_decision_ref]
    );
    assert_eq!(&product_path.action_result_refs, &vec![action_result_ref]);
    assert_eq!(&product_path.retention_refs, &vec![retention_ref]);
    assert_eq!(&product_path.deletion_refs, &vec![deletion_ref]);
    assert_eq!(&product_path.export_refs, &vec![export_ref]);
}

fn assert_product_path_payload_refs(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    evidence_id: &TestStr,
) -> TestResult {
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
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_ANALYZER_ALERT_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_AI_DETECTION_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_eq!(
        payload.get(constants::field::NETWORK_PRODUCT_PATH_RISK_BUDGET_REFS),
        Some(&LogFieldValue::String(
            constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_string()
        ))
    );
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_POLICY_DECISION_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_POLICY_DECISION_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_ACTION_RESULT_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_ACTION_RESULT_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_RETENTION_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_RETENTION_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_DELETION_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_DELETION_REF_PREFIX,
            evidence_id,
        ),
    )?;
    assert_payload_ref_equals(
        payload,
        constants::field::NETWORK_PRODUCT_PATH_EXPORT_REFS,
        &product_path_ref(
            constants::network_flow::PRODUCT_PATH_EXPORT_REF_PREFIX,
            evidence_id,
        ),
    )?;

    Ok(())
}

fn assert_payload_ref_equals(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &TestStr,
    expected: &TestStr,
) -> TestResult {
    assert_eq!(
        payload.get(field),
        Some(&LogFieldValue::String(expected.to_string()))
    );

    Ok(())
}

fn product_path_ref(prefix: &TestStr, evidence_id: &TestStr) -> TestString {
    format!("{prefix}{evidence_id}")
}
