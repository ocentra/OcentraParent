use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};
use ocentra_screen_live_view_core::ScreenLiveViewObservationIntent;

#[test]
fn screen_live_view_observation_requests_policy_not_ai_analysis() {
    let observed = ocentra_screen_live_view_core::default_screen_live_view_observed_event();
    let evidence =
        ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(&observed);
    let ai = ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(&evidence);
    let policy = ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
        &evidence,
    );

    assert!(ai.is_none());
    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert!(matches!(
        policy.as_ref(),
        Some(policy)
            if policy.event_type
                == ChildRuntimeDomain::ScreenLiveView.policy_evaluation_requested_event_type()
                && policy.evidence_refs == vec![evidence.evidence_ref]
    ));
}

#[test]
fn unauthorized_screen_live_view_session_still_uses_policy_not_ai() {
    let observed = ocentra_screen_live_view_core::screen_live_view_observed_event(
        ScreenLiveViewObservationIntent::UnauthorizedSessionRequiresPolicy,
    );
    let evidence =
        ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(&observed);
    let ai = ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(&evidence);
    let policy = ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
        &evidence,
    );

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert!(ai.is_none());
    assert!(matches!(
        policy.as_ref(),
        Some(policy) if policy.evidence_refs == vec![evidence.evidence_ref]
    ));
}

#[test]
fn screen_live_view_health_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_screen_live_view_core::screen_live_view_observed_event(
        ScreenLiveViewObservationIntent::SessionHealthObservationOnly,
    );
    let evidence =
        ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(&observed);
    let ai = ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(&evidence);
    let policy = ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
        &evidence,
    );

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
