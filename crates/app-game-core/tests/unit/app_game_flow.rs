use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn app_game_observation_records_evidence_and_requests_policy_not_ai() {
    let observed = ocentra_app_game_core::default_app_game_observed_event();
    let evidence = ocentra_app_game_core::app_game_evidence_recorded_event(&observed);
    let ai = ocentra_app_game_core::app_game_ai_analysis_requested_event(&evidence);
    let policy = ocentra_app_game_core::app_game_policy_evaluation_requested_event(&evidence);
    let Some(policy) = policy else {
        return;
    };

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::AppGame.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::AppGame.evidence_recorded_event_type()
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
    assert_eq!(
        policy.event_type,
        ChildRuntimeDomain::AppGame.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn app_game_ambiguous_usage_requests_ai_before_policy() {
    let observed = ocentra_app_game_core::app_game_observed_event(
        ocentra_app_game_core::AppGameObservationIntent::AmbiguousUsageRequiresAi,
    );
    let evidence = ocentra_app_game_core::app_game_evidence_recorded_event(&observed);
    let ai = ocentra_app_game_core::app_game_ai_analysis_requested_event(&evidence);
    assert!(
        ai.is_some(),
        "ambiguous app game usage requires AI boundary"
    );
    let Some(ai) = ai else {
        return;
    };
    let policy = ocentra_app_game_core::app_game_policy_evaluation_requested_event(&evidence);

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
        ChildRuntimeDomain::AppGame.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn app_game_inventory_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_app_game_core::app_game_observed_event(
        ocentra_app_game_core::AppGameObservationIntent::InventoryObservationOnly,
    );
    let evidence = ocentra_app_game_core::app_game_evidence_recorded_event(&observed);
    let ai = ocentra_app_game_core::app_game_ai_analysis_requested_event(&evidence);
    let policy = ocentra_app_game_core::app_game_policy_evaluation_requested_event(&evidence);

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
