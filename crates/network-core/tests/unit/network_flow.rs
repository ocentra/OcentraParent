use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_core::network_runtime::{
    default_network_observed_event, evaluate_network_runtime, network_ai_analysis_requested_event,
    network_evidence_recorded_event, network_observed_event,
    network_policy_evaluation_requested_event, NetworkAdapterState, NetworkCapturePermissionState,
    NetworkObservationIntent, NetworkParserState, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn default_network_observation_preserves_policy_first_contract() {
    let observed = default_network_observed_event();
    let evidence = network_evidence_recorded_event(&observed);
    let ai = network_ai_analysis_requested_event(&evidence);
    let policy =
        network_policy_evaluation_requested_event(&evidence).expect_value("network policy request");

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Network.observed_event_type()
    );
    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
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
fn unknown_route_contract_requests_ai_before_policy() {
    let observed = network_observed_event(NetworkObservationIntent::UnknownRouteRequiresAi);
    let evidence = network_evidence_recorded_event(&observed);
    let ai = network_ai_analysis_requested_event(&evidence)
        .expect_value("unknown network route requires AI boundary");
    let policy = network_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
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
        ChildRuntimeDomain::Network.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn telemetry_contract_stays_observe_only() {
    let observed = network_observed_event(NetworkObservationIntent::TelemetryObservationOnly);
    let evidence = network_evidence_recorded_event(&observed);
    let ai = network_ai_analysis_requested_event(&evidence);
    let policy = network_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
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

#[test]
fn runtime_decision_reuses_protocol_owned_observed_and_evidence_chain() {
    let input = NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Valid,
        observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
    };
    let decision = evaluate_network_runtime(input);
    let observed = network_observed_event(decision.observation_intent);
    let expected_observed = network_observed_event(NetworkObservationIntent::FlowRequiresPolicy);

    assert_eq!(observed, expected_observed);
    assert_eq!(
        network_evidence_recorded_event(&observed),
        network_evidence_recorded_event(&expected_observed)
    );
}
