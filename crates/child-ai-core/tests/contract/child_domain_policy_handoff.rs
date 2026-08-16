use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_request_id, child_domain_analysis_purpose, child_domain_child_device_id,
    child_domain_child_profile_id, child_domain_evidence_ref, child_domain_observed_at,
    child_domain_policy_evaluation_requested_from_ai_result_event_if_required,
    child_domain_policy_request_id_from_fact_ref, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainAnalysisPurposeKind, ChildDomainEventType, ChildDomainPolicyEvaluationRequirement,
    ChildDomainRefSuffix, ChildRuntimeDomain,
};

#[test]
fn child_domain_ai_completion_hands_off_to_policy_without_authority(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ChildDomainAiAnalysisRequestedEvent {
        event_type: ChildRuntimeDomain::Browser.ai_analysis_requested_event_type(),
        domain: ChildRuntimeDomain::Browser,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        ai_request_id: child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        ),
        evidence_refs: vec![child_domain_evidence_ref(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultEvidence,
        )],
        source_observed_at: child_domain_observed_at(),
        allowed_analysis_purpose: child_domain_analysis_purpose(
            ChildDomainAnalysisPurposeKind::Classification,
        ),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    };

    let completed =
        ocentra_child_ai_core::child_domain_analysis::complete_child_domain_ai_analysis(&request);
    let handoff =
        child_domain_policy_evaluation_requested_from_ai_result_event_if_required(&completed)
            .ok_or_else(|| std::io::Error::other("policy handoff is required"))?;

    assert_eq!(
        completed.event_type,
        ChildDomainEventType::ai_analysis_completed()
    );
    assert_eq!(completed.domain, request.domain);
    assert_eq!(completed.evidence_refs, request.evidence_refs);
    assert_eq!(completed.source_observed_at, request.source_observed_at);
    assert_eq!(
        completed.result_fact_ref.as_str(),
        request.ai_request_id.as_str()
    );
    assert_eq!(
        handoff.event_type,
        ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type()
    );
    assert_eq!(handoff.domain, request.domain);
    assert_eq!(handoff.evidence_refs, request.evidence_refs);
    assert_eq!(handoff.source_observed_at, request.source_observed_at);
    assert_eq!(
        handoff.source_fact_ref.as_str(),
        request.ai_request_id.as_str()
    );
    assert_eq!(
        handoff.policy_request_id,
        child_domain_policy_request_id_from_fact_ref(
            ChildRuntimeDomain::Browser,
            &completed.result_fact_ref
        )
    );
    Ok(())
}

#[test]
fn child_domain_ai_completion_without_policy_requirement_does_not_handoff_to_policy(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ChildDomainAiAnalysisRequestedEvent {
        event_type: ChildRuntimeDomain::Browser.ai_analysis_requested_event_type(),
        domain: ChildRuntimeDomain::Browser,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        ai_request_id: child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        ),
        evidence_refs: vec![child_domain_evidence_ref(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultEvidence,
        )],
        source_observed_at: child_domain_observed_at(),
        allowed_analysis_purpose: child_domain_analysis_purpose(
            ChildDomainAnalysisPurposeKind::Classification,
        ),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::NotRequired,
    };

    let completed =
        ocentra_child_ai_core::child_domain_analysis::complete_child_domain_ai_analysis(&request);
    let handoff =
        child_domain_policy_evaluation_requested_from_ai_result_event_if_required(&completed);

    assert_eq!(
        completed.event_type,
        ChildDomainEventType::ai_analysis_completed()
    );
    assert_eq!(completed.source_observed_at, request.source_observed_at);
    assert!(handoff.is_none());
    Ok(())
}
