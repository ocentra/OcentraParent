use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use super::{
    publish_network_runtime_chain_for_observation, NetworkAiAuditState, NetworkEvidenceGrade,
    NetworkEvidenceScope, NetworkInterventionState, NetworkObservation, NetworkRiskBudgetState,
    NetworkRuntimeEventPayload, NetworkRuntimePhase,
};

#[tokio::test]
async fn network_runtime_chain_publishes_full_metadata_only_flow() {
    let report = publish_network_runtime_chain_for_observation(
        complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        NetworkRuntimePhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
    assert!(!report.manual_required());
    assert_eq!(payloads[0].phase, NetworkRuntimePhase::FlowObserved);
    assert_eq!(payloads[3].ai_audit_state, NetworkAiAuditState::Requested);
    assert_eq!(payloads[4].ai_audit_state, NetworkAiAuditState::Completed);
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && payload.evidence_grade == NetworkEvidenceGrade::DomainAndProcessMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ObserveOnly
            && payload.intervention_state == NetworkInterventionState::DryRunOnly
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.page_content_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn ip_only_or_unknown_process_flow_requires_manual_review() {
    let report = publish_network_runtime_chain_for_observation(
        ip_only_unknown_process_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert!(report.manual_required());
    assert!(payloads.iter().all(|payload| {
        payload.evidence_grade == NetworkEvidenceGrade::IpOrProcessPartialMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ManualReviewRequired
            && payload.intervention_state == NetworkInterventionState::ManualRequired
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn degraded_adapter_flow_stays_unavailable_without_adapter_action() {
    let report = publish_network_runtime_chain_for_observation(
        NetworkObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES_DEGRADED);
    let payloads = decode_payloads(&report);

    assert!(!report.manual_required());
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::AdapterUnavailable
            && payload.evidence_grade == NetworkEvidenceGrade::AdapterUnavailable
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

fn decode_payloads(report: &super::NetworkRuntimeReport) -> Vec<NetworkRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::EventEnvelope<NetworkRuntimeEventPayload> = event
                .decode()
                .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES);
            envelope.payload
        })
        .collect()
}

fn complete_domain_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    }
}

fn ip_only_unknown_process_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: None,
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: None,
        process_name: None,
        associated_pid_count: 0,
    }
}
