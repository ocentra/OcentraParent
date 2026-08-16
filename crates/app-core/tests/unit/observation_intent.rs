use ocentra_app_core::{
    app_ai_analysis_requested_event, app_evidence_recorded_event, app_observed_event,
    app_policy_evaluation_requested_event, AppObservationIntent,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn foreground_app_requires_policy_without_ai_analysis() {
    let observed = app_observed_event(AppObservationIntent::ForegroundAppRequiresPolicy);
    let evidence = app_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::App);
    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert_eq!(
        observed.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        observed.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(app_ai_analysis_requested_event(&evidence), None);
    let policy = app_policy_evaluation_requested_event(&evidence);
    let policy = policy.expect_value("foreground app policy request");
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn unknown_app_requests_ai_then_policy_evidence() {
    let observed = app_observed_event(AppObservationIntent::UnknownAppRequiresAi);
    let evidence = app_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    let ai = app_ai_analysis_requested_event(&evidence);
    let ai = ai.expect_value("unknown app ai request");
    assert_eq!(app_policy_evaluation_requested_event(&evidence), None);
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn inventory_observation_stays_evidence_only() {
    let observed = app_observed_event(AppObservationIntent::InventoryObservationOnly);
    let evidence = app_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert_eq!(app_ai_analysis_requested_event(&evidence), None);
    assert_eq!(app_policy_evaluation_requested_event(&evidence), None);
}
