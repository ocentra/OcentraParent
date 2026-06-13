use ocentra_parent_agent_protocol::{
    child_domain_ai_request_id, child_domain_child_device_id, child_domain_child_profile_id,
    child_domain_evidence_ref, child_domain_fact_ref_from_ai_request_id,
    child_domain_policy_request_id, ChildDomainEventType,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainRefSuffix, ChildRuntimeDomain,
};

#[test]
fn child_domain_policy_preserves_evidence_refs_for_notification_handoff() {
    let request = ChildDomainPolicyEvaluationRequestedEvent {
        event_type: ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type(),
        domain: ChildRuntimeDomain::Browser,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        policy_request_id: child_domain_policy_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultPolicyRequest,
        ),
        evidence_refs: vec![child_domain_evidence_ref(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultEvidence,
        )],
        source_fact_ref: child_domain_fact_ref_from_ai_request_id(&child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        )),
    };

    let violation = ocentra_child_policy_core::evaluate_child_domain_policy(&request);

    assert_eq!(
        violation.event_type,
        ChildDomainEventType::policy_violation_detected()
    );
    assert_eq!(violation.domain, request.domain);
    assert_eq!(violation.evidence_refs, request.evidence_refs);
}
