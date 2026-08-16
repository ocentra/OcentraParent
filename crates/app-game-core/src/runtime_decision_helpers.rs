use super::{
    AppGameAiHandoffState, AppGameClassificationState, AppGameForegroundState,
    AppGamePolicyHandoffState, AppGameRuntimeActionState, AppGameRuntimeDecision,
    AppGameRuntimeInput,
};
use crate::AppGameObservationIntent;

pub(super) fn missing_capability_decision() -> AppGameRuntimeDecision {
    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppGameRuntimeActionState::ManualRequired,
        ai_handoff_state: AppGameAiHandoffState::NotRequired,
        policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
    }
}

pub(super) fn foreground_known_game_decision() -> AppGameRuntimeDecision {
    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::ForegroundUsageRequiresPolicy,
        runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
        ai_handoff_state: AppGameAiHandoffState::NotRequired,
        policy_handoff_state: AppGamePolicyHandoffState::Publish,
    }
}

pub(super) fn foreground_unknown_game_decision() -> AppGameRuntimeDecision {
    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::AmbiguousUsageRequiresAi,
        runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
        ai_handoff_state: AppGameAiHandoffState::Required,
        policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
    }
}

pub(super) fn foreground_inventory_only_decision() -> AppGameRuntimeDecision {
    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
        ai_handoff_state: AppGameAiHandoffState::NotRequired,
        policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
    }
}

pub(super) fn inventory_background_decision() -> AppGameRuntimeDecision {
    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppGameRuntimeActionState::RecordInventory,
        ai_handoff_state: AppGameAiHandoffState::NotRequired,
        policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
    }
}

pub(super) fn evaluate_app_game_runtime(input: AppGameRuntimeInput) -> AppGameRuntimeDecision {
    if input.capability_state == super::AppGameCapabilityState::Missing {
        return missing_capability_decision();
    }

    if input.foreground_state != AppGameForegroundState::Foreground {
        return inventory_background_decision();
    }

    match input.classification_state {
        AppGameClassificationState::KnownGame => foreground_known_game_decision(),
        AppGameClassificationState::UnknownGame => foreground_unknown_game_decision(),
        AppGameClassificationState::InventoryOnly => foreground_inventory_only_decision(),
    }
}
