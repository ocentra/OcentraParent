use ocentra_parent_agent_protocol::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn network_observation_records_flow_evidence_and_requests_policy() {
    let observed = ocentra_network_core::default_network_observed_event();
    let evidence = ocentra_network_core::network_evidence_recorded_event(&observed);
    let ai = ocentra_network_core::network_ai_analysis_requested_event(&evidence);
    let policy = ocentra_network_core::network_policy_evaluation_requested_event(&evidence)
        .expect("network policy request is expected");

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Network.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Network.evidence_recorded_event_type()
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
        ChildRuntimeDomain::Network.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn network_unknown_route_requests_ai_before_policy() {
    let observed = ocentra_network_core::network_observed_event(
        ocentra_network_core::NetworkObservationIntent::UnknownRouteRequiresAi,
    );
    let evidence = ocentra_network_core::network_evidence_recorded_event(&observed);
    let ai = ocentra_network_core::network_ai_analysis_requested_event(&evidence)
        .expect("unknown network route requires AI boundary");
    let policy = ocentra_network_core::network_policy_evaluation_requested_event(&evidence);

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
        ChildRuntimeDomain::Network.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn network_telemetry_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_network_core::network_observed_event(
        ocentra_network_core::NetworkObservationIntent::TelemetryObservationOnly,
    );
    let evidence = ocentra_network_core::network_evidence_recorded_event(&observed);
    let ai = ocentra_network_core::network_ai_analysis_requested_event(&evidence);
    let policy = ocentra_network_core::network_policy_evaluation_requested_event(&evidence);

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
