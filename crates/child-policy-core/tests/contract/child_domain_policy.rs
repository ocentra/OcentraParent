use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_request_id, child_domain_child_device_id, child_domain_child_profile_id,
    child_domain_evidence_ref, child_domain_fact_ref_from_ai_request_id, child_domain_observed_at,
    child_domain_policy_request_id, child_domain_policy_violation_id_from_policy_request_id,
    ChildDomainEventType, ChildDomainPolicyEvaluationRequestedEvent, ChildDomainRefSuffix,
    ChildRuntimeDomain,
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
        source_observed_at: child_domain_observed_at(),
        source_fact_ref: child_domain_fact_ref_from_ai_request_id(&child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        )),
    };

    let violation =
        ocentra_child_policy_core::child_domain_policy::evaluate_child_domain_policy(&request)
            .expect_value("valid child-domain policy request");

    assert_eq!(
        violation.event_type,
        ChildDomainEventType::policy_violation_detected()
    );
    assert_eq!(violation.domain, request.domain);
    assert_eq!(
        violation.violation_id,
        child_domain_policy_violation_id_from_policy_request_id(&request.policy_request_id)
    );
    assert_eq!(violation.detected_at, request.source_observed_at);
    assert_eq!(violation.evidence_refs, request.evidence_refs);
}

#[test]
fn child_domain_policy_canonicalizes_duplicate_evidence_refs_before_violation_handoff() {
    let evidence_ref = child_domain_evidence_ref(
        ChildRuntimeDomain::Browser,
        ChildDomainRefSuffix::DefaultEvidence,
    );
    let request = ChildDomainPolicyEvaluationRequestedEvent {
        event_type: ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type(),
        domain: ChildRuntimeDomain::Browser,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        policy_request_id: child_domain_policy_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultPolicyRequest,
        ),
        evidence_refs: vec![evidence_ref.clone(), evidence_ref],
        source_observed_at: child_domain_observed_at(),
        source_fact_ref: child_domain_fact_ref_from_ai_request_id(&child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        )),
    };

    let violation =
        ocentra_child_policy_core::child_domain_policy::evaluate_child_domain_policy(&request)
            .expect_value("valid child-domain policy request");

    assert_eq!(violation.detected_at, request.source_observed_at);
    assert_eq!(violation.evidence_refs.len(), 1);
}

#[test]
fn child_domain_policy_rejects_wrong_event_type_and_missing_evidence() {
    let mut request = ChildDomainPolicyEvaluationRequestedEvent {
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
        source_observed_at: child_domain_observed_at(),
        source_fact_ref: child_domain_fact_ref_from_ai_request_id(&child_domain_ai_request_id(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultAiRequest,
        )),
    };

    request.event_type = ChildDomainEventType::policy_violation_detected();
    let wrong_event =
        ocentra_child_policy_core::child_domain_policy::evaluate_child_domain_policy(&request)
            .expect_err_value("must reject wrong event type");
    assert_eq!(
        wrong_event,
        ocentra_eventing::error::EventingError::InvalidValue {
            field: "child_domain_policy.event_type",
            value: ChildDomainEventType::policy_violation_detected()
                .as_str()
                .to_owned(),
        }
    );

    request.event_type = ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type();
    request.evidence_refs.clear();
    let missing_evidence =
        ocentra_child_policy_core::child_domain_policy::evaluate_child_domain_policy(&request)
            .expect_err_value("must reject empty evidence refs");
    assert_eq!(
        missing_evidence,
        ocentra_eventing::error::EventingError::InvalidValue {
            field: "child_domain_policy.evidence_refs",
            value: String::from("empty"),
        }
    );
}
