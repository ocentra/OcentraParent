use ocentra_app_game_core::runtime_decision::{
    app_game_runtime_decision_recorded_event, app_game_runtime_observed_event,
    evaluate_app_game_runtime, AppGameAggregateId, AppGameAiHandoffState, AppGameCapabilityState,
    AppGameClassificationState, AppGameForegroundState, AppGamePolicyHandoffState,
    AppGameRuntimeActionState, AppGameRuntimeDecisionId, AppGameRuntimeInput,
};
use ocentra_app_game_core::AppGameObservationIntent;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_parent_agent_protocol::child_domain_runtime::ChildRuntimeDomain;

#[test]
fn foreground_known_game_publishes_policy_without_ai() {
    let decision = evaluate_app_game_runtime(AppGameRuntimeInput {
        capability_state: AppGameCapabilityState::Supported,
        foreground_state: AppGameForegroundState::Foreground,
        classification_state: AppGameClassificationState::KnownGame,
    });

    assert_eq!(
        decision.observation_intent,
        AppGameObservationIntent::ForegroundUsageRequiresPolicy
    );
    assert_eq!(
        decision.runtime_action_state,
        AppGameRuntimeActionState::RecordForegroundSession
    );
    assert_eq!(
        decision.ai_handoff_state,
        AppGameAiHandoffState::NotRequired
    );
    assert_eq!(
        decision.policy_handoff_state,
        AppGamePolicyHandoffState::Publish
    );
}

#[test]
fn foreground_unknown_game_routes_to_ai_boundary() {
    let input = AppGameRuntimeInput {
        capability_state: AppGameCapabilityState::Supported,
        foreground_state: AppGameForegroundState::Foreground,
        classification_state: AppGameClassificationState::UnknownGame,
    };
    let decision = evaluate_app_game_runtime(input);
    let observed = app_game_runtime_observed_event(input);

    assert_eq!(
        decision.observation_intent,
        AppGameObservationIntent::AmbiguousUsageRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        AppGameRuntimeActionState::RecordForegroundSession
    );
    assert_eq!(decision.ai_handoff_state, AppGameAiHandoffState::Required);
    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::AppGame.observed_event_type()
    );
}

#[test]
fn missing_capability_forces_manual_review_without_handoffs() {
    let decision = evaluate_app_game_runtime(AppGameRuntimeInput {
        capability_state: AppGameCapabilityState::Missing,
        foreground_state: AppGameForegroundState::Foreground,
        classification_state: AppGameClassificationState::KnownGame,
    });

    assert_eq!(
        decision.observation_intent,
        AppGameObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        decision.runtime_action_state,
        AppGameRuntimeActionState::ManualRequired
    );
    assert_eq!(
        decision.ai_handoff_state,
        AppGameAiHandoffState::NotRequired
    );
    assert_eq!(
        decision.policy_handoff_state,
        AppGamePolicyHandoffState::DoNotPublish
    );
}

#[test]
fn background_inventory_state_records_decision_event_with_typed_contract() {
    let input = AppGameRuntimeInput {
        capability_state: AppGameCapabilityState::Supported,
        foreground_state: AppGameForegroundState::Background,
        classification_state: AppGameClassificationState::InventoryOnly,
    };
    let aggregate_id_result = AppGameAggregateId::parse("app-game.aggregate.child-device-1");
    assert!(
        aggregate_id_result.is_ok(),
        "aggregate id parses: {aggregate_id_result:?}"
    );
    let Ok(aggregate_id) = aggregate_id_result else {
        return;
    };
    let decision_id_result = AppGameRuntimeDecisionId::parse("app-game.runtime-decision-1");
    assert!(
        decision_id_result.is_ok(),
        "decision id parses: {decision_id_result:?}"
    );
    let Ok(decision_id) = decision_id_result else {
        return;
    };

    let recorded = app_game_runtime_decision_recorded_event(aggregate_id, decision_id, input);

    assert_eq!(
        recorded.decision.observation_intent,
        AppGameObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        recorded.decision.runtime_action_state,
        AppGameRuntimeActionState::RecordInventory
    );
    let contract_result = recorded.contract();
    assert!(
        contract_result.is_ok(),
        "app-game runtime contract parses: {contract_result:?}"
    );
    let Ok(contract) = contract_result else {
        return;
    };
    assert_eq!(
        contract.event_type.as_str(),
        "app-game.runtime.decision-recorded"
    );
}
