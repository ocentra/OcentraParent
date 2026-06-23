use ocentra_eventing::expect_value::ExpectValue;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn browser_observation_records_evidence_and_requests_ai_boundary() {
    let observed = ocentra_browser_core::default_browser_observed_event();
    let evidence = ocentra_browser_core::browser_evidence_recorded_event(&observed);
    let ai = ocentra_browser_core::browser_ai_analysis_requested_event(&evidence)
        .expect_value("browser AI request is expected");
    let policy = ocentra_browser_core::browser_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Browser.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Browser.evidence_recorded_event_type()
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(
        ai.event_type,
        ChildRuntimeDomain::Browser.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert_eq!(ai.private_payload_state, PrivatePayloadState::Excluded);
    assert!(policy.is_none());
}

#[test]
fn browser_known_policy_navigation_bypasses_ai_boundary() {
    let observed = ocentra_browser_core::browser_observed_event(
        ocentra_browser_core::BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy,
    );
    let evidence = ocentra_browser_core::browser_evidence_recorded_event(&observed);
    let ai = ocentra_browser_core::browser_ai_analysis_requested_event(&evidence);
    let policy = ocentra_browser_core::browser_policy_evaluation_requested_event(&evidence)
        .expect_value("known browser policy navigation requests policy directly");

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert!(ai.is_none());
    assert_eq!(
        policy.event_type,
        ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn browser_inventory_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_browser_core::browser_observed_event(
        ocentra_browser_core::BrowserObservationIntent::InventoryObservationOnly,
    );
    let evidence = ocentra_browser_core::browser_evidence_recorded_event(&observed);
    let ai = ocentra_browser_core::browser_ai_analysis_requested_event(&evidence);
    let policy = ocentra_browser_core::browser_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::NotRequired
    );
    assert!(ai.is_none());
    assert!(policy.is_none());
}
