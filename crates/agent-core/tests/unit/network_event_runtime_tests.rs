use std::fmt::{Debug, Display};

use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAuditState, NetworkEvidenceScope, NetworkInterventionState, NetworkRiskBudgetState,
    NetworkRuntimeEvidenceGrade, NetworkRuntimePhase,
};

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_capture::NetworkObservation;
use ocentra_parent_agent_core::network_event_runtime::review::{
    request_network_runtime_review_for_observation, NetworkRuntimeReviewReport,
    NetworkRuntimeReviewResponse,
};
use ocentra_parent_agent_core::network_event_runtime::{
    publish_network_runtime_chain_for_observation, NetworkRuntimeEventPayload, NetworkRuntimeReport,
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

fn some<T>(value: Option<T>, context: impl Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}

#[tokio::test]
async fn network_runtime_chain_publishes_full_metadata_only_flow() -> TestResult {
    let report = ok(
        publish_network_runtime_chain_for_observation(
            complete_domain_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(
        report.stored_events.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert!(report.dead_letters.is_empty());
    assert_eq!(
        report.handled_phases,
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>()
    );
    assert!(!report.manual_required());
    assert_eq!(payloads[0].phase, NetworkRuntimePhase::FlowObserved);
    assert_eq!(payloads[3].ai_audit_state, NetworkAiAuditState::Requested);
    assert_eq!(payloads[4].ai_audit_state, NetworkAiAuditState::Completed);
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
        payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && payload.evidence_grade == NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata
            && payload.evidence_grade_contract
                == ocentra_parent_agent_protocol::NetworkEvidenceGrade::B
            && payload.risk_budget_state == NetworkRiskBudgetState::ObserveOnly
            && payload.intervention_state == NetworkInterventionState::DryRunOnly
            && payload.policy_action
                == ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Observe
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.page_content_available
            && !payload.claim_boundary.adapter_action_executed
    }));

    Ok(())
}

#[tokio::test]
async fn network_runtime_chain_carries_exact_refs_without_direct_enforcement_shortcut() -> TestResult
{
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let report = ok(
        publish_network_runtime_chain_for_observation(complete_domain_observation(), observed_at)
            .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

    assert_ai_request_refs(&payloads, observed_at)?;
    assert_policy_evaluation_refs(&payloads, observed_at)?;
    assert!(payloads.iter().all(|payload| {
        !matches!(
            payload.phase,
            NetworkRuntimePhase::EnforcementCommandIssued
                | NetworkRuntimePhase::EnforcementResultObserved
        )
    }));
    assert_audit_and_portal_refs(&payloads, observed_at)?;

    Ok(())
}

#[tokio::test]
async fn manual_required_network_evidence_does_not_publish_enforcement_command() -> TestResult {
    let report = ok(
        publish_network_runtime_chain_for_observation(
            ip_only_unknown_process_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

    assert!(report.manual_required());
    assert_eq!(
        report.handled_phases,
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>()
    );
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
        payload.evidence_grade == NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ManualReviewRequired
            && payload.intervention_state == NetworkInterventionState::ManualRequired
            && payload.evidence_grade_contract
                == ocentra_parent_agent_protocol::NetworkEvidenceGrade::C
            && payload.policy_action
                == ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::AskParent
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.adapter_action_executed
    }));

    let audit_entry = payload_for_phase(&payloads, NetworkRuntimePhase::AuditEntryCommitted)?;
    assert_eq!(
        audit_entry.previous_phase_ref,
        Some(
            expected_phase_ref(
                &ActivityCaptureCapabilityStatus::Available,
                constants::activity_store::TEST_FIRST_OBSERVED_AT,
                NetworkRuntimePhase::PolicyDecisionCompleted
            )
            .to_string()
        )
    );
    assert_eq!(audit_entry.adapter_capability_ref, None);
    assert_eq!(audit_entry.enforcement_command_ref, None);
    assert_eq!(audit_entry.enforcement_result_ref, None);

    Ok(())
}

#[tokio::test]
async fn degraded_adapter_flow_stays_unavailable_without_adapter_action() -> TestResult {
    let report = ok(
        publish_network_runtime_chain_for_observation(
            NetworkObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES_DEGRADED,
    )?;
    let payloads = decode_payloads(&report)?;

    assert!(!report.manual_required());
    assert_eq!(
        report.handled_phases,
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 4
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
            && payload.evidence_grade == NetworkRuntimeEvidenceGrade::AdapterUnavailable
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && payload.evidence_grade_contract
                == ocentra_parent_agent_protocol::NetworkEvidenceGrade::D
            && payload.policy_action
                == ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::ManualReview
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.adapter_action_executed
    }));

    Ok(())
}

#[tokio::test]
async fn network_runtime_review_request_resolves_associated_response() -> TestResult {
    let report: NetworkRuntimeReviewReport = ok(
        request_network_runtime_review_for_observation(
            ip_only_unknown_process_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES,
    )?;
    let response: &NetworkRuntimeReviewResponse = &report.request_report.response;

    assert_eq!(
        report.request_report.publish_report.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED
    );
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        response.evidence_grade,
        NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata
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

    Ok(())
}

fn decode_payloads(
    report: &NetworkRuntimeReport,
) -> Result<Vec<NetworkRuntimeEventPayload>, TestText> {
    report
        .stored_events
        .iter()
        .map(|event| {
            ok(
                event.decode::<NetworkRuntimeEventPayload>(),
                constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES,
            )
            .map(|envelope| envelope.payload)
        })
        .collect()
}

fn count_event_type(report: &NetworkRuntimeReport, event_type: impl Display) -> usize {
    let event_type = TestText::from_display(event_type);
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type.0.as_str())
        .count()
}

fn assert_ai_request_refs(
    payloads: &[NetworkRuntimeEventPayload],
    observed_at: impl Display,
) -> TestResult {
    let observed_at = TestText::from_display(observed_at);
    let ai_request = payload_for_phase(payloads, NetworkRuntimePhase::AiAnalysisRequested)?;
    assert_eq!(
        ai_request.previous_phase_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::ActivityClassified
            )
            .to_string()
        )
    );
    assert_eq!(
        ai_request.evidence_ref,
        expected_available_ref(observed_at.0.as_str(), NetworkRuntimePhase::FlowObserved)
            .to_string()
    );
    assert_eq!(
        ai_request.ai_request_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AiAnalysisRequested
            )
            .to_string()
        )
    );
    assert_eq!(ai_request.ai_analysis_ref, None);
    assert_eq!(ai_request.policy_decision_ref, None);
    assert_eq!(ai_request.enforcement_command_ref, None);

    Ok(())
}

fn assert_policy_evaluation_refs(
    payloads: &[NetworkRuntimeEventPayload],
    observed_at: impl Display,
) -> TestResult {
    let observed_at = TestText::from_display(observed_at);
    let policy_evaluation =
        payload_for_phase(payloads, NetworkRuntimePhase::PolicyEvaluationRequested)?;
    assert_eq!(
        policy_evaluation.previous_phase_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AiAnalysisCompleted
            )
            .to_string()
        )
    );
    assert_eq!(
        policy_evaluation.ai_request_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AiAnalysisRequested
            )
            .to_string()
        )
    );
    assert_eq!(
        policy_evaluation.ai_analysis_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AiAnalysisCompleted
            )
            .to_string()
        )
    );
    assert_eq!(
        policy_evaluation.policy_evaluation_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::PolicyEvaluationRequested
            )
            .to_string()
        )
    );
    assert_eq!(policy_evaluation.policy_decision_ref, None);
    assert_eq!(policy_evaluation.enforcement_command_ref, None);

    Ok(())
}

fn assert_audit_and_portal_refs(
    payloads: &[NetworkRuntimeEventPayload],
    observed_at: impl Display,
) -> TestResult {
    let observed_at = TestText::from_display(observed_at);
    let audit_entry = payload_for_phase(payloads, NetworkRuntimePhase::AuditEntryCommitted)?;
    assert_eq!(
        audit_entry.previous_phase_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::PolicyDecisionCompleted
            )
            .to_string()
        )
    );
    assert_eq!(audit_entry.enforcement_result_ref, None);
    assert_eq!(
        audit_entry.audit_entry_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AuditEntryCommitted
            )
            .to_string()
        )
    );
    let portal_update = payload_for_phase(payloads, NetworkRuntimePhase::PortalReadModelUpdated)?;
    assert_eq!(
        portal_update.previous_phase_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AuditEntryCommitted
            )
            .to_string()
        )
    );
    assert_eq!(
        portal_update.audit_entry_ref,
        Some(
            expected_available_ref(
                observed_at.0.as_str(),
                NetworkRuntimePhase::AuditEntryCommitted
            )
            .to_string()
        )
    );

    Ok(())
}

fn payload_for_phase(
    payloads: &[NetworkRuntimeEventPayload],
    phase: NetworkRuntimePhase,
) -> Result<NetworkRuntimeEventPayload, TestText> {
    some(
        payloads
            .iter()
            .find(|payload| payload.phase == phase)
            .cloned(),
        constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES,
    )
}

fn expected_phase_ref(
    status: &ActivityCaptureCapabilityStatus,
    observed_at: impl Display,
    phase: NetworkRuntimePhase,
) -> TestText {
    let mut value = expected_correlation_ref(status, observed_at).to_string();
    value.push(constants::delimiter::HYPHEN);
    value.push_str(phase.event_type());
    TestText::from_display(value)
}

fn expected_available_ref(observed_at: impl Display, phase: NetworkRuntimePhase) -> TestText {
    expected_phase_ref(
        &ActivityCaptureCapabilityStatus::Available,
        observed_at,
        phase,
    )
}

fn expected_correlation_ref(
    status: &ActivityCaptureCapabilityStatus,
    observed_at: impl Display,
) -> TestText {
    let observed_at = TestText::from_display(observed_at);
    let mut value = String::from(constants::network_flow::CORRELATION_NETWORK_RUNTIME_PREFIX);
    value.push_str(status.as_protocol_str());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(observed_at.0.as_str());
    TestText::from_display(value)
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
