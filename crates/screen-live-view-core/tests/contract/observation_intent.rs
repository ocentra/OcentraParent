use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};
use ocentra_screen_live_view_core::{
    screen_live_view_ai_analysis_requested_event, screen_live_view_evidence_recorded_event,
    screen_live_view_observed_event, screen_live_view_policy_evaluation_requested_event,
    ScreenLiveViewObservationIntent,
};

#[test]
fn session_requires_policy_without_ai_analysis() {
    let observed =
        screen_live_view_observed_event(ScreenLiveViewObservationIntent::SessionRequiresPolicy);
    let evidence = screen_live_view_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::ScreenLiveView);
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
    assert_eq!(
        screen_live_view_ai_analysis_requested_event(&evidence),
        None
    );
    assert!(matches!(
        screen_live_view_policy_evaluation_requested_event(&evidence).as_ref(),
        Some(policy) if policy.evidence_refs == vec![evidence.evidence_ref.clone()]
    ));
}

#[test]
fn unauthorized_session_still_routes_to_policy_only() {
    let observed = screen_live_view_observed_event(
        ScreenLiveViewObservationIntent::UnauthorizedSessionRequiresPolicy,
    );
    let evidence = screen_live_view_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert_eq!(
        screen_live_view_ai_analysis_requested_event(&evidence),
        None
    );
    assert!(matches!(
        screen_live_view_policy_evaluation_requested_event(&evidence).as_ref(),
        Some(policy) if policy.evidence_refs == vec![evidence.evidence_ref.clone()]
    ));
}

#[test]
fn session_health_observation_stays_evidence_only() {
    let observed = screen_live_view_observed_event(
        ScreenLiveViewObservationIntent::SessionHealthObservationOnly,
    );
    let evidence = screen_live_view_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert_eq!(
        screen_live_view_ai_analysis_requested_event(&evidence),
        None
    );
    assert_eq!(
        screen_live_view_policy_evaluation_requested_event(&evidence),
        None
    );
}
