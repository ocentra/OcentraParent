use ocentra_parent_agent_protocol::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn lan_observation_records_presence_evidence_and_requests_policy() {
    let observed = ocentra_lan_core::default_lan_observed_event();
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence)
        .expect("LAN policy request is expected");

    assert_eq!(observed.event_type, ChildRuntimeDomain::Lan.observed_event_type());
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Lan.evidence_recorded_event_type()
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
        ChildRuntimeDomain::Lan.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn lan_unknown_peer_requests_ai_before_policy() {
    let observed = ocentra_lan_core::lan_observed_event(
        ocentra_lan_core::LanObservationIntent::UnknownPeerRequiresAi,
    );
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence)
        .expect("unknown LAN peer requires AI boundary");
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence);

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
        ChildRuntimeDomain::Lan.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn lan_discovery_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_lan_core::lan_observed_event(
        ocentra_lan_core::LanObservationIntent::DiscoveryObservationOnly,
    );
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence);

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
