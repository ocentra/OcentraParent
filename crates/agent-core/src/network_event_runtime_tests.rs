use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use crate::network_event_runtime::{
    request_network_runtime_review_for_observation, NetworkRuntimeReviewReport,
    NetworkRuntimeReviewResponse,
};

use super::{
    publish_network_runtime_chain_for_observation, NetworkAiAuditState, NetworkEvidenceGrade,
    NetworkEvidenceScope, NetworkInterventionState, NetworkObservation, NetworkRiskBudgetState,
    NetworkRuntimeEventPayload, NetworkRuntimePhase, NetworkRuntimeReport,
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
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        1
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        1
    );
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
async fn network_runtime_chain_carries_exact_refs_without_direct_enforcement_shortcut() {
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let report =
        publish_network_runtime_chain_for_observation(complete_domain_observation(), observed_at)
            .await
            .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_ai_request_refs(&payloads, observed_at);
    assert_policy_evaluation_refs(&payloads, observed_at);
    assert_enforcement_command_refs(&payloads, observed_at);
    assert_audit_and_portal_refs(&payloads, observed_at);
}

#[tokio::test]
async fn manual_required_network_evidence_does_not_publish_enforcement_command() {
    let report = publish_network_runtime_chain_for_observation(
        ip_only_unknown_process_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert!(report.manual_required());
    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        0
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        0
    );
    assert!(payloads.iter().all(|payload| {
        payload.evidence_grade == NetworkEvidenceGrade::IpOrProcessPartialMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ManualReviewRequired
            && payload.intervention_state == NetworkInterventionState::ManualRequired
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.adapter_action_executed
    }));

    let audit_entry = payload_for_phase(&payloads, NetworkRuntimePhase::AuditEntryCommitted);
    assert_eq!(
        audit_entry.previous_phase_ref,
        Some(expected_phase_ref(
            ActivityCaptureCapabilityStatus::Available,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            NetworkRuntimePhase::PolicyDecisionCompleted
        ))
    );
    assert_eq!(audit_entry.adapter_capability_ref, None);
    assert_eq!(audit_entry.enforcement_command_ref, None);
    assert_eq!(audit_entry.enforcement_result_ref, None);
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
    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        0
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        0
    );
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::AdapterUnavailable
            && payload.evidence_grade == NetworkEvidenceGrade::AdapterUnavailable
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn network_runtime_review_request_resolves_associated_response() {
    let report: NetworkRuntimeReviewReport = request_network_runtime_review_for_observation(
        ip_only_unknown_process_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES);
    let response: &NetworkRuntimeReviewResponse = &report.request_report.response;

    assert_eq!(
        report.request_report.publish_report.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED
    );
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        response.evidence_grade,
        NetworkEvidenceGrade::IpOrProcessPartialMetadata
    );
    assert_eq!(
        response.risk_budget_state,
        NetworkRiskBudgetState::ManualReviewRequired
    );
    assert_eq!(
        response.intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert!(response.review_required);
    assert!(!response.claim_boundary.adapter_action_executed);
    assert_eq!(report.stored_events.len(), 1);
    assert_eq!(
        report.stored_events[0].contract.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED
    );
    assert!(report.dead_letters.is_empty());
}

fn decode_payloads(report: &NetworkRuntimeReport) -> Vec<NetworkRuntimeEventPayload> {
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

fn count_event_type(report: &NetworkRuntimeReport, event_type: &str) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type)
        .count()
}

fn assert_ai_request_refs(payloads: &[NetworkRuntimeEventPayload], observed_at: &str) {
    let ai_request = payload_for_phase(payloads, NetworkRuntimePhase::AiAnalysisRequested);
    assert_eq!(
        ai_request.previous_phase_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::ActivityClassified
        ))
    );
    assert_eq!(
        ai_request.evidence_ref,
        expected_available_ref(observed_at, NetworkRuntimePhase::FlowObserved)
    );
    assert_eq!(
        ai_request.ai_request_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AiAnalysisRequested
        ))
    );
    assert_eq!(ai_request.ai_analysis_ref, None);
    assert_eq!(ai_request.policy_decision_ref, None);
    assert_eq!(ai_request.enforcement_command_ref, None);
}

fn assert_policy_evaluation_refs(payloads: &[NetworkRuntimeEventPayload], observed_at: &str) {
    let policy_evaluation =
        payload_for_phase(payloads, NetworkRuntimePhase::PolicyEvaluationRequested);
    assert_eq!(
        policy_evaluation.previous_phase_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AiAnalysisCompleted
        ))
    );
    assert_eq!(
        policy_evaluation.ai_request_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AiAnalysisRequested
        ))
    );
    assert_eq!(
        policy_evaluation.ai_analysis_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AiAnalysisCompleted
        ))
    );
    assert_eq!(
        policy_evaluation.policy_evaluation_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::PolicyEvaluationRequested
        ))
    );
    assert_eq!(policy_evaluation.policy_decision_ref, None);
    assert_eq!(policy_evaluation.enforcement_command_ref, None);
}

fn assert_enforcement_command_refs(payloads: &[NetworkRuntimeEventPayload], observed_at: &str) {
    let enforcement_command =
        payload_for_phase(payloads, NetworkRuntimePhase::EnforcementCommandIssued);
    let mut adapter_ref =
        expected_correlation_ref(ActivityCaptureCapabilityStatus::Available, observed_at);
    adapter_ref.push(constants::delimiter::HYPHEN);
    adapter_ref.push_str(constants::network_flow::TARGET_ENFORCEMENT_DRY_RUN);

    assert_eq!(
        enforcement_command.previous_phase_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::PolicyDecisionCompleted
        ))
    );
    assert_eq!(
        enforcement_command.policy_decision_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::PolicyDecisionCompleted
        ))
    );
    assert_eq!(
        enforcement_command.adapter_capability_ref,
        Some(adapter_ref)
    );
    assert_eq!(
        enforcement_command.enforcement_command_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::EnforcementCommandIssued
        ))
    );
    assert_eq!(enforcement_command.enforcement_result_ref, None);
    assert!(!enforcement_command.claim_boundary.adapter_action_executed);
}

fn assert_audit_and_portal_refs(payloads: &[NetworkRuntimeEventPayload], observed_at: &str) {
    let audit_entry = payload_for_phase(payloads, NetworkRuntimePhase::AuditEntryCommitted);
    assert_eq!(
        audit_entry.previous_phase_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::EnforcementResultObserved
        ))
    );
    assert_eq!(
        audit_entry.enforcement_result_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::EnforcementResultObserved
        ))
    );
    assert_eq!(
        audit_entry.audit_entry_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AuditEntryCommitted
        ))
    );
    let portal_update = payload_for_phase(payloads, NetworkRuntimePhase::PortalReadModelUpdated);
    assert_eq!(
        portal_update.previous_phase_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AuditEntryCommitted
        ))
    );
    assert_eq!(
        portal_update.audit_entry_ref,
        Some(expected_available_ref(
            observed_at,
            NetworkRuntimePhase::AuditEntryCommitted
        ))
    );
}

fn payload_for_phase(
    payloads: &[NetworkRuntimeEventPayload],
    phase: NetworkRuntimePhase,
) -> NetworkRuntimeEventPayload {
    payloads
        .iter()
        .find(|payload| payload.phase == phase)
        .cloned()
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES)
}

fn expected_phase_ref(
    status: ActivityCaptureCapabilityStatus,
    observed_at: &str,
    phase: NetworkRuntimePhase,
) -> String {
    let mut value = expected_correlation_ref(status, observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(phase.event_type());
    value
}

fn expected_available_ref(observed_at: &str, phase: NetworkRuntimePhase) -> String {
    expected_phase_ref(
        ActivityCaptureCapabilityStatus::Available,
        observed_at,
        phase,
    )
}

fn expected_correlation_ref(status: ActivityCaptureCapabilityStatus, observed_at: &str) -> String {
    let mut value = String::from(constants::network_flow::CORRELATION_NETWORK_RUNTIME_PREFIX);
    value.push_str(status.as_protocol_str());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(observed_at);
    value
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
