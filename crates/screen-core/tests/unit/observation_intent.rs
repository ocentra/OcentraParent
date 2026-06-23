use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};
use ocentra_screen_core::{
    screen_ai_analysis_requested_event, screen_evidence_recorded_event, screen_observed_event,
    screen_policy_evaluation_requested_event, ScreenObservationIntent,
};

#[test]
fn ambiguous_content_requests_ai_and_policy_evidence() -> Result<(), String> {
    let observed = screen_observed_event(ScreenObservationIntent::AmbiguousContentRequiresAi);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::Screen);
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
    assert_eq!(screen_policy_evaluation_requested_event(&evidence), None);
    let ai_request = screen_ai_analysis_requested_event(&evidence)
        .ok_or_else(|| String::from("screen ai request"))?;
    assert_eq!(ai_request.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn known_policy_state_skips_ai() -> Result<(), String> {
    let observed = screen_observed_event(ScreenObservationIntent::KnownPolicyStateRequiresPolicy);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    let policy_request = screen_policy_evaluation_requested_event(&evidence)
        .ok_or_else(|| String::from("screen policy request"))?;
    assert_eq!(policy_request.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn idle_observation_stays_evidence_only() {
    let observed = screen_observed_event(ScreenObservationIntent::IdleObservationOnly);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    assert_eq!(screen_policy_evaluation_requested_event(&evidence), None);
}
