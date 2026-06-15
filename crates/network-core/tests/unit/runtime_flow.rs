use ocentra_network_core::{
    evaluate_network_runtime, network_runtime_ai_analysis_requested_event,
    network_runtime_event_chain, network_runtime_observed_event,
    network_runtime_policy_evaluation_requested_event, NetworkAdapterState, NetworkAiHandoffState,
    NetworkCapturePermissionState, NetworkObservationIntent, NetworkParserState,
    NetworkPolicyHandoffState, NetworkRuntimeActionState, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::{ChildDomainObservedSignal, ChildRuntimeDomain};

#[test]
fn runtime_policy_flow_aligns_with_child_domain_policy_chain() {
    let chain =
        network_runtime_event_chain(runtime_input(NetworkObservationIntent::FlowRequiresPolicy));

    assert_eq!(
        chain.decision.observation_intent,
        NetworkObservationIntent::FlowRequiresPolicy
    );
    assert_eq!(
        chain.decision.runtime_action_state,
        NetworkRuntimeActionState::CaptureAndRecord
    );
    assert_eq!(
        chain.decision.ai_handoff_state,
        NetworkAiHandoffState::NotRequired
    );
    assert_eq!(
        chain.decision.policy_handoff_state,
        NetworkPolicyHandoffState::Publish
    );
    assert_eq!(chain.observed_event.domain, ChildRuntimeDomain::Network);
    assert_eq!(
        chain.observed_event.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert!(chain.ai_analysis_requested_event.is_none());
    assert_eq!(
        chain
            .policy_evaluation_requested_event
            .expect("policy request")
            .evidence_refs,
        vec![chain.evidence_recorded_event.evidence_ref.clone()]
    );
}

#[test]
fn runtime_unknown_route_aligns_with_ai_chain() {
    let chain = network_runtime_event_chain(runtime_input(
        NetworkObservationIntent::UnknownRouteRequiresAi,
    ));

    assert_eq!(
        chain.decision.observation_intent,
        NetworkObservationIntent::UnknownRouteRequiresAi
    );
    assert_eq!(
        chain.decision.runtime_action_state,
        NetworkRuntimeActionState::CaptureAndRecord
    );
    assert_eq!(
        chain.decision.ai_handoff_state,
        NetworkAiHandoffState::Required
    );
    assert_eq!(
        chain.decision.policy_handoff_state,
        NetworkPolicyHandoffState::DoNotPublish
    );
    assert_eq!(
        chain.observed_event.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        chain
            .ai_analysis_requested_event
            .expect("network ai request")
            .evidence_refs,
        vec![chain.evidence_recorded_event.evidence_ref.clone()]
    );
    assert!(chain.policy_evaluation_requested_event.is_none());
}

#[test]
fn degraded_runtime_downgrades_to_observe_only_chain() {
    let chain = network_runtime_event_chain(NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Drifted,
        observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
    });

    assert_eq!(
        chain.decision.observation_intent,
        NetworkObservationIntent::TelemetryObservationOnly
    );
    assert_eq!(
        chain.decision.runtime_action_state,
        NetworkRuntimeActionState::ManualRequired
    );
    assert_eq!(
        chain.decision.ai_handoff_state,
        NetworkAiHandoffState::NotRequired
    );
    assert_eq!(
        chain.decision.policy_handoff_state,
        NetworkPolicyHandoffState::DoNotPublish
    );
    assert_eq!(
        chain.observed_event.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert!(chain.ai_analysis_requested_event.is_none());
    assert!(chain.policy_evaluation_requested_event.is_none());
}

#[test]
fn runtime_wrapper_helpers_match_full_event_chain_projection() {
    let input = runtime_input(NetworkObservationIntent::UnknownRouteRequiresAi);
    let chain = network_runtime_event_chain(input);

    assert_eq!(evaluate_network_runtime(input), chain.decision);
    assert_eq!(network_runtime_observed_event(input), chain.observed_event);
    assert_eq!(
        network_runtime_ai_analysis_requested_event(input),
        chain.ai_analysis_requested_event
    );
    assert_eq!(
        network_runtime_policy_evaluation_requested_event(input),
        chain.policy_evaluation_requested_event
    );
}

fn runtime_input(observation_intent: NetworkObservationIntent) -> NetworkRuntimeInput {
    NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Valid,
        observation_intent,
    }
}
