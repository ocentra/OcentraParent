use ocentra_app_game_core::{
    app_game_ai_analysis_requested_event, app_game_evidence_recorded_event,
    app_game_observed_event, app_game_policy_evaluation_requested_event, AppGameObservationIntent,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn foreground_usage_requires_policy_without_ai_analysis() {
    let observed = app_game_observed_event(AppGameObservationIntent::ForegroundUsageRequiresPolicy);
    let evidence = app_game_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::AppGame);
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
    assert_eq!(app_game_ai_analysis_requested_event(&evidence), None);
    let policy = app_game_policy_evaluation_requested_event(&evidence);
    let Some(policy) = policy else {
        return;
    };
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn ambiguous_usage_requests_ai_and_policy_evidence() {
    let observed = app_game_observed_event(AppGameObservationIntent::AmbiguousUsageRequiresAi);
    let evidence = app_game_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    let ai = app_game_ai_analysis_requested_event(&evidence);
    let Some(ai) = ai else {
        return;
    };
    assert_eq!(app_game_policy_evaluation_requested_event(&evidence), None);
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn inventory_observation_stays_evidence_only() {
    let observed = app_game_observed_event(AppGameObservationIntent::InventoryObservationOnly);
    let evidence = app_game_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert_eq!(app_game_ai_analysis_requested_event(&evidence), None);
    assert_eq!(app_game_policy_evaluation_requested_event(&evidence), None);
}
