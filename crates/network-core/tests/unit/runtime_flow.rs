use ocentra_eventing::DomainEvent;
use ocentra_network_core::{
    evaluate_network_runtime, network_ai_analysis_requested_event,
    network_evidence_recorded_event, network_observed_event,
    network_policy_evaluation_requested_event, network_runtime_decision_recorded_event,
    NetworkAdapterState, NetworkAggregateId, NetworkCapturePermissionState, NetworkParserState,
    NetworkPolicyHandoffState, NetworkRuntimeActionState, NetworkRuntimeDecisionId,
    NetworkObservationIntent, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::{ChildDomainObservedSignal, ChildRuntimeDomain};

#[test]
fn unknown_route_requests_ai_and_policy_evidence() {
    let observed = network_observed_event(NetworkObservationIntent::UnknownRouteRequiresAi);
    let evidence = network_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::Network);
    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        network_ai_analysis_requested_event(&evidence)
            .expect("network ai request")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
    assert_eq!(network_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn runtime_requires_valid_adapter_permission_and_parser_before_policy_handoff() {
    let decision = evaluate_network_runtime(NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Valid,
        observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
    });

    assert_eq!(
        decision.runtime_action_state,
        NetworkRuntimeActionState::CaptureAndRecord
    );
    assert_eq!(decision.policy_handoff_state, NetworkPolicyHandoffState::Publish);
}

#[test]
fn drifted_parser_records_manual_required_without_policy_handoff() {
    let decision = evaluate_network_runtime(NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Drifted,
        observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
    });

    assert_eq!(
        decision.runtime_action_state,
        NetworkRuntimeActionState::ManualRequired
    );
    assert_eq!(
        decision.policy_handoff_state,
        NetworkPolicyHandoffState::DoNotPublish
    );
}

#[test]
fn runtime_decision_event_has_aggregate_and_idempotency_contract() {
    let event = network_runtime_decision_recorded_event(
        NetworkAggregateId::parse("network-aggregate-child-device").expect("network aggregate"),
        NetworkRuntimeDecisionId::parse("network-decision-001").expect("network decision"),
        NetworkRuntimeInput {
            adapter_state: NetworkAdapterState::Available,
            capture_permission_state: NetworkCapturePermissionState::Granted,
            parser_state: NetworkParserState::Valid,
            observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
        },
    );

    assert_eq!(event.aggregate_key().expect("aggregate").as_str(), event.aggregate_id.as_str());
    assert!(
        event
            .idempotency_key()
            .expect("idempotency")
            .as_str()
            .ends_with(event.decision_id.as_str())
    );
}
