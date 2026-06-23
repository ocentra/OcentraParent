use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_core::network_runtime::{
    evaluate_network_runtime, network_ai_analysis_requested_event, network_evidence_recorded_event,
    network_observed_event, network_policy_evaluation_requested_event, NetworkAdapterState,
    NetworkAiHandoffState, NetworkCapturePermissionState, NetworkObservationIntent,
    NetworkParserState, NetworkPolicyHandoffState, NetworkRuntimeActionState, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainObservedSignal, ChildRuntimeDomain,
};

#[test]
fn runtime_policy_flow_aligns_with_child_domain_policy_chain() {
    let decision =
        evaluate_network_runtime(runtime_input(NetworkObservationIntent::FlowRequiresPolicy));
    let observed_event = network_observed_event(decision.observation_intent);
    let evidence_recorded_event = network_evidence_recorded_event(&observed_event);
    let ai_analysis_requested_event = network_ai_analysis_requested_event(&evidence_recorded_event);
    let policy_evaluation_requested_event =
        network_policy_evaluation_requested_event(&evidence_recorded_event);

    assert_eq!(
        decision.observation_intent,
        NetworkObservationIntent::FlowRequiresPolicy
    );
    assert_eq!(
        decision.runtime_action_state,
        NetworkRuntimeActionState::CaptureAndRecord
    );
    assert_eq!(
        decision.ai_handoff_state,
        NetworkAiHandoffState::NotRequired
    );
    assert_eq!(
        decision.policy_handoff_state,
        NetworkPolicyHandoffState::Publish
    );
    assert_eq!(observed_event.domain, ChildRuntimeDomain::Network);
    assert_eq!(
        observed_event.observed_state,
        ChildDomainObservedSignal::RequiresPolicy.into_observed_state()
    );
    assert!(ai_analysis_requested_event.is_none());
    assert_eq!(
        policy_evaluation_requested_event
            .expect_value("policy request")
            .evidence_refs,
        vec![evidence_recorded_event.evidence_ref]
    );
}

#[test]
fn runtime_unknown_route_aligns_with_ai_chain() {
    let decision = evaluate_network_runtime(runtime_input(
        NetworkObservationIntent::UnknownRouteRequiresAi,
    ));
    let observed_event = network_observed_event(decision.observation_intent);
    let evidence_recorded_event = network_evidence_recorded_event(&observed_event);
    let ai_analysis_requested_event = network_ai_analysis_requested_event(&evidence_recorded_event);
    let policy_evaluation_requested_event =
        network_policy_evaluation_requested_event(&evidence_recorded_event);

    assert_eq!(
        decision.observation_intent,
        NetworkObservationIntent::UnknownRouteRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        NetworkRuntimeActionState::CaptureAndRecord
    );
    assert_eq!(decision.ai_handoff_state, NetworkAiHandoffState::Required);
    assert_eq!(
        decision.policy_handoff_state,
        NetworkPolicyHandoffState::DoNotPublish
    );
    assert_eq!(
        observed_event.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        ai_analysis_requested_event
            .expect_value("network ai request")
            .evidence_refs,
        vec![evidence_recorded_event.evidence_ref]
    );
    assert!(policy_evaluation_requested_event.is_none());
}

#[test]
fn degraded_runtime_downgrades_to_observe_only_chain() {
    let decision = evaluate_network_runtime(NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Drifted,
        observation_intent: NetworkObservationIntent::FlowRequiresPolicy,
    });
    let observed_event = network_observed_event(decision.observation_intent);
    let evidence_recorded_event = network_evidence_recorded_event(&observed_event);
    let ai_analysis_requested_event = network_ai_analysis_requested_event(&evidence_recorded_event);
    let policy_evaluation_requested_event =
        network_policy_evaluation_requested_event(&evidence_recorded_event);

    assert_eq!(
        decision.observation_intent,
        NetworkObservationIntent::TelemetryObservationOnly
    );
    assert_eq!(
        decision.runtime_action_state,
        NetworkRuntimeActionState::ManualRequired
    );
    assert_eq!(
        decision.ai_handoff_state,
        NetworkAiHandoffState::NotRequired
    );
    assert_eq!(
        decision.policy_handoff_state,
        NetworkPolicyHandoffState::DoNotPublish
    );
    assert_eq!(
        observed_event.observed_state,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
    );
    assert!(ai_analysis_requested_event.is_none());
    assert!(policy_evaluation_requested_event.is_none());
}

#[test]
fn runtime_decision_projects_the_protocol_owned_event_chain() {
    let input = runtime_input(NetworkObservationIntent::UnknownRouteRequiresAi);
    let decision = evaluate_network_runtime(input);
    let observed_event = network_observed_event(decision.observation_intent);
    let evidence_recorded_event = network_evidence_recorded_event(&observed_event);
    let ai_analysis_requested_event = network_ai_analysis_requested_event(&evidence_recorded_event);
    let policy_evaluation_requested_event =
        network_policy_evaluation_requested_event(&evidence_recorded_event);

    assert_eq!(decision, evaluate_network_runtime(input));
    assert_eq!(
        observed_event,
        network_observed_event(NetworkObservationIntent::UnknownRouteRequiresAi)
    );
    assert_eq!(
        ai_analysis_requested_event
            .expect_value("unknown route still requests ai")
            .evidence_refs,
        vec![evidence_recorded_event.evidence_ref]
    );
    assert!(policy_evaluation_requested_event.is_none());
}

fn runtime_input(observation_intent: NetworkObservationIntent) -> NetworkRuntimeInput {
    NetworkRuntimeInput {
        adapter_state: NetworkAdapterState::Available,
        capture_permission_state: NetworkCapturePermissionState::Granted,
        parser_state: NetworkParserState::Valid,
        observation_intent,
    }
}
