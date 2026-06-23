use ocentra_browser_core::{
    browser_ai_analysis_requested_event, browser_evidence_recorded_event, browser_observed_event,
    browser_policy_evaluation_requested_event, BrowserObservationIntent,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn ambiguous_navigation_requests_ai_and_policy_evidence() {
    let observed = browser_observed_event(BrowserObservationIntent::AmbiguousNavigationRequiresAi);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::Browser);
    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        observed.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_eq!(
        observed.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(
        browser_ai_analysis_requested_event(&evidence)
            .expect_value("browser ai request")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
    assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn known_policy_navigation_skips_ai() {
    let observed =
        browser_observed_event(BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert_eq!(browser_ai_analysis_requested_event(&evidence), None);
    assert_eq!(
        browser_policy_evaluation_requested_event(&evidence)
            .expect_value("browser policy request")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
}

#[test]
fn inventory_observation_stays_evidence_only() {
    let observed = browser_observed_event(BrowserObservationIntent::InventoryObservationOnly);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert_eq!(browser_ai_analysis_requested_event(&evidence), None);
    assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
}
