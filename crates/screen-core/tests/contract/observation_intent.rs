use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[derive(Debug)]
struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

#[test]
fn screen_observation_records_evidence_and_requests_ai_boundary() -> Result<(), TestError> {
    let observed = ocentra_screen_core::default_screen_observed_event();
    let evidence = ocentra_screen_core::screen_evidence_recorded_event(&observed);
    let ai = ocentra_screen_core::screen_ai_analysis_requested_event(&evidence)
        .ok_or_else(|| TestError(String::from("screen AI request is expected")))?;
    let policy = ocentra_screen_core::screen_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Screen.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Screen.evidence_recorded_event_type()
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
        ChildRuntimeDomain::Screen.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert_eq!(ai.private_payload_state, PrivatePayloadState::Excluded);
    assert!(policy.is_none());
    Ok(())
}

#[test]
fn screen_known_policy_state_bypasses_ai_boundary() -> Result<(), TestError> {
    let observed = ocentra_screen_core::screen_observed_event(
        ocentra_screen_core::ScreenObservationIntent::KnownPolicyStateRequiresPolicy,
    );
    let evidence = ocentra_screen_core::screen_evidence_recorded_event(&observed);
    let ai = ocentra_screen_core::screen_ai_analysis_requested_event(&evidence);
    let policy = ocentra_screen_core::screen_policy_evaluation_requested_event(&evidence)
        .ok_or_else(|| {
            TestError(String::from(
                "known screen policy state requests policy directly",
            ))
        })?;

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
        ChildRuntimeDomain::Screen.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn screen_idle_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_screen_core::screen_observed_event(
        ocentra_screen_core::ScreenObservationIntent::IdleObservationOnly,
    );
    let evidence = ocentra_screen_core::screen_evidence_recorded_event(&observed);
    let ai = ocentra_screen_core::screen_ai_analysis_requested_event(&evidence);
    let policy = ocentra_screen_core::screen_policy_evaluation_requested_event(&evidence);

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
