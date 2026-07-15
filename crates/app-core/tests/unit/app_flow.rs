use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn app_foreground_observation_records_evidence_and_requests_policy_not_ai() {
    let observed = ocentra_app_core::default_app_observed_event();
    let evidence = ocentra_app_core::app_evidence_recorded_event(&observed);
    let ai = ocentra_app_core::app_ai_analysis_requested_event(&evidence);
    let policy = ocentra_app_core::app_policy_evaluation_requested_event(&evidence);
    let policy = policy.expect_value("foreground app should request policy");

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::App.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::App.evidence_recorded_event_type()
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
        ChildRuntimeDomain::App.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn app_unknown_usage_requests_ai_before_policy() {
    let observed = ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::UnknownAppRequiresAi,
    );
    let evidence = ocentra_app_core::app_evidence_recorded_event(&observed);
    let ai = ocentra_app_core::app_ai_analysis_requested_event(&evidence);
    let ai = ai.expect_value("unknown app should request AI");
    let policy = ocentra_app_core::app_policy_evaluation_requested_event(&evidence);

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
        ChildRuntimeDomain::App.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn app_inventory_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::InventoryObservationOnly,
    );
    let evidence = ocentra_app_core::app_evidence_recorded_event(&observed);
    let ai = ocentra_app_core::app_ai_analysis_requested_event(&evidence);
    let policy = ocentra_app_core::app_policy_evaluation_requested_event(&evidence);

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
