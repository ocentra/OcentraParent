use ocentra_app_core::runtime_decision::{
    evaluate_app_runtime, AppAiHandoffState, AppCapabilityState, AppClassificationState,
    AppForegroundState, AppPolicyHandoffState, AppRuntimeActionState, AppRuntimeDecision,
    AppRuntimeInput,
};
use ocentra_app_core::AppObservationIntent;

#[derive(Clone, Copy)]
struct RuntimeDecisionCase {
    input: AppRuntimeInput,
    expected: AppRuntimeDecision,
}

#[test]
fn runtime_decision_matrix_preserves_handoff_boundaries() {
    for runtime_case in supported_runtime_decision_cases()
        .into_iter()
        .chain(missing_runtime_decision_cases())
    {
        assert_eq!(
            evaluate_app_runtime(runtime_case.input),
            runtime_case.expected
        );
    }
}

#[test]
fn manual_required_decisions_never_publish_ai_or_policy_handoffs() {
    let unsupported_foreground_known_app = AppRuntimeInput {
        capability_state: AppCapabilityState::Missing,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::KnownPolicyApp,
    };

    let decision = evaluate_app_runtime(unsupported_foreground_known_app);

    assert_eq!(
        decision,
        AppRuntimeDecision {
            observation_intent: AppObservationIntent::InventoryObservationOnly,
            runtime_action_state: AppRuntimeActionState::ManualRequired,
            ai_handoff_state: AppAiHandoffState::NotRequired,
            policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
        }
    );
}

fn supported_runtime_decision_cases() -> [RuntimeDecisionCase; 9] {
    [
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Foreground,
                AppClassificationState::KnownPolicyApp,
            ),
            foreground_policy_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Foreground,
                AppClassificationState::UnknownApp,
            ),
            foreground_unknown_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Foreground,
                AppClassificationState::InventoryOnly,
            ),
            foreground_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Background,
                AppClassificationState::KnownPolicyApp,
            ),
            background_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Background,
                AppClassificationState::UnknownApp,
            ),
            background_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Background,
                AppClassificationState::InventoryOnly,
            ),
            background_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Unknown,
                AppClassificationState::KnownPolicyApp,
            ),
            background_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Unknown,
                AppClassificationState::UnknownApp,
            ),
            background_inventory_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Supported,
                AppForegroundState::Unknown,
                AppClassificationState::InventoryOnly,
            ),
            background_inventory_decision(),
        ),
    ]
}

fn missing_runtime_decision_cases() -> [RuntimeDecisionCase; 9] {
    [
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Foreground,
                AppClassificationState::KnownPolicyApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Foreground,
                AppClassificationState::UnknownApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Foreground,
                AppClassificationState::InventoryOnly,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Background,
                AppClassificationState::KnownPolicyApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Background,
                AppClassificationState::UnknownApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Background,
                AppClassificationState::InventoryOnly,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Unknown,
                AppClassificationState::KnownPolicyApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Unknown,
                AppClassificationState::UnknownApp,
            ),
            manual_required_decision(),
        ),
        runtime_case(
            runtime_input(
                AppCapabilityState::Missing,
                AppForegroundState::Unknown,
                AppClassificationState::InventoryOnly,
            ),
            manual_required_decision(),
        ),
    ]
}

fn runtime_case(input: AppRuntimeInput, expected: AppRuntimeDecision) -> RuntimeDecisionCase {
    RuntimeDecisionCase { input, expected }
}

fn runtime_input(
    capability_state: AppCapabilityState,
    foreground_state: AppForegroundState,
    classification_state: AppClassificationState,
) -> AppRuntimeInput {
    AppRuntimeInput {
        capability_state,
        foreground_state,
        classification_state,
    }
}

fn manual_required_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppRuntimeActionState::ManualRequired,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

fn background_inventory_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppRuntimeActionState::RecordInventory,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

fn foreground_inventory_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppRuntimeActionState::RecordInventory,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

fn foreground_policy_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::ForegroundAppRequiresPolicy,
        runtime_action_state: AppRuntimeActionState::RecordForeground,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::Publish,
    }
}

fn foreground_unknown_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::UnknownAppRequiresAi,
        runtime_action_state: AppRuntimeActionState::RecordForeground,
        ai_handoff_state: AppAiHandoffState::Required,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}
