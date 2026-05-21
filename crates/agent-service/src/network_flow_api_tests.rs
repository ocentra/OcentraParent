use ocentra_parent_agent_core::NetworkStoreRow;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityEvidenceKind, ActivityEvidenceRef, ActivityNetworkCustodyState,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityObserver,
    ActivityProcessAttributionStatus, LogFieldValue, LogFields,
};

use super::network_flow_api::{network_flow_payload, network_flow_report_from_rows};

fn sample_row() -> NetworkStoreRow {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string()),
    );
    fields.insert(
        constants::field::ADAPTER_ID.to_string(),
        LogFieldValue::String(constants::activity_capture::NETWORK_ADAPTER_ID.to_string()),
    );
    fields.insert(
        constants::field::NETWORK_PROTOCOL.to_string(),
        LogFieldValue::String(constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()),
    );
    fields.insert(
        constants::field::TCP_STATE.to_string(),
        LogFieldValue::String(constants::activity_capture::TCP_STATE_ESTABLISHED.to_string()),
    );
    fields.insert(
        constants::field::DESTINATION_DOMAIN.to_string(),
        LogFieldValue::String(constants::test_network::TEST_DOMAIN.to_string()),
    );
    fields.insert(
        constants::field::DESTINATION_IP.to_string(),
        LogFieldValue::String(constants::test_network::LOOPBACK_IP.to_string()),
    );
    fields.insert(
        constants::field::DESTINATION_PORT.to_string(),
        LogFieldValue::Number(443.0),
    );
    fields.insert(
        constants::field::LOCAL_IP.to_string(),
        LogFieldValue::String(constants::test_network::LOOPBACK_IP.to_string()),
    );
    fields.insert(
        constants::field::LOCAL_PORT.to_string(),
        LogFieldValue::Number(4242.0),
    );
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(f64::from(std::process::id())),
    );
    fields.insert(
        constants::field::PROCESS_NAME.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
    );
    fields.insert(
        constants::field::DOMAIN_ATTRIBUTION_STATUS.to_string(),
        LogFieldValue::String(
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string(),
        ),
    );
    fields.insert(
        constants::field::PROCESS_ATTRIBUTION_STATUS.to_string(),
        LogFieldValue::String(
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string(),
        ),
    );

    NetworkStoreRow {
        event_id: constants::test_network::NETWORK_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: constants::test_network::NETWORK_EVIDENCE_ID.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: Some(constants::test_network::NETWORK_EVIDENCE_DIGEST.to_string()),
            uri: None,
        }],
    }
}

fn sample_report() -> super::network_flow_api::NetworkFlowReport {
    network_flow_report_from_rows(vec![sample_row()])
}

#[test]
fn network_flow_report_builds_digest_from_rows() {
    let report = sample_report();

    assert_eq!(
        report.read_model.custody,
        ActivityNetworkCustodyState::ChildDeviceQueryStore
    );
    assert_eq!(report.read_model.returned, 1);
    assert_eq!(
        report.read_model.capability_status,
        ActivityCaptureCapabilityStatus::Available
    );
    assert_eq!(
        report.read_model.rows[0].observer,
        ActivityObserver::WindowsNetwork
    );
    assert_eq!(
        report.read_model.rows[0].protocol,
        Some(ActivityNetworkProtocol::Tcp)
    );
    assert_eq!(
        report.read_model.rows[0].tcp_state,
        Some(ActivityNetworkTcpState::Established)
    );
    assert_eq!(
        report.read_model.rows[0].domain_attribution_status,
        ActivityDomainAttributionStatus::DomainObserved
    );
    assert_eq!(
        report.read_model.rows[0].process_attribution_status,
        ActivityProcessAttributionStatus::ProcessAttributed
    );
    assert_eq!(
        report.digest.top_processes[0].label,
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME
    );
}

#[test]
fn network_flow_payload_contains_digest_json() {
    let report = sample_report();
    let payload = network_flow_payload(&report);

    assert!(matches!(
        payload.get(constants::field::ACTIVITY_DIGEST),
        Some(LogFieldValue::String(_))
    ));
}
