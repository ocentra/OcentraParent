use crate::screen_family_ai_hub_routing::{
    ScreenChildLocalAnalysisAttempt, ScreenChildLocalAnalysisAttemptState,
    ScreenEvidenceCustodyState, ScreenFamilyAiHubCapability, ScreenFamilyAiHubCapabilityState,
    ScreenFamilyAiHubExecutionState, ScreenFamilyAiHubRoute, ScreenFamilyAiHubTransferMode,
};

pub(super) fn screen_family_ai_hub_capability_is_consistent(
    value: &ScreenFamilyAiHubCapability,
) -> bool {
    value.custody_state == ScreenEvidenceCustodyState::LiveLanChildAgent
        && (value.capability_state != ScreenFamilyAiHubCapabilityState::Available
            || screen_family_ai_hub_available_capability_is_consistent(value))
        && (value.capability_state == ScreenFamilyAiHubCapabilityState::Available
            || screen_family_ai_hub_degraded_capability_is_consistent(value))
}

pub(super) fn screen_child_local_attempt_is_consistent(
    value: &ScreenChildLocalAnalysisAttempt,
) -> bool {
    (value.execution_state != ScreenChildLocalAnalysisAttemptState::Selected
        || screen_family_ai_hub_booleans_are_true(&[
            value.model_runtime_ref.is_some(),
            value.degraded_states.is_empty(),
        ]))
        && (value.execution_state == ScreenChildLocalAnalysisAttemptState::Selected
            || screen_family_ai_hub_booleans_are_true(&[
                value.model_runtime_ref.is_none(),
                !value.degraded_states.is_empty(),
            ]))
}

pub(super) fn screen_family_ai_hub_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    screen_family_ai_hub_route_flags_are_consistent(value)
        && (value.execution_state != ScreenFamilyAiHubExecutionState::Selected
            || screen_family_ai_hub_selected_route_is_consistent(value))
        && (value.execution_state == ScreenFamilyAiHubExecutionState::Selected
            || screen_family_ai_hub_unselected_route_is_consistent(value))
}

fn screen_family_ai_hub_available_capability_is_consistent(
    value: &ScreenFamilyAiHubCapability,
) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        value.model_runtime_ref.is_some(),
        value.household_route_ref.is_some(),
        value.degraded_states.is_empty(),
        value.unavailable_reason.is_none(),
    ])
}

fn screen_family_ai_hub_degraded_capability_is_consistent(
    value: &ScreenFamilyAiHubCapability,
) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        value.model_runtime_ref.is_none(),
        !value.degraded_states.is_empty(),
        value.unavailable_reason.is_some(),
    ])
}

fn screen_family_ai_hub_route_flags_are_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        value.local_provider_attempted,
        value.child_safety_local_fallback_preserved,
        value.summary_first,
        value.redacted_or_cropped_input_required,
        !value.raw_full_screenshot_transfer_allowed,
        !value.raw_image_retention_allowed,
        !value.remote_provider_selected,
        !value.remote_api_fallback_allowed,
        !value.ocentra_hosted_processing_allowed,
        !value.remote_default_for_blocking,
    ])
}

fn screen_family_ai_hub_selected_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        value.parent_approved_family_hub,
        value.source_child_local_attempt.execution_state
            != ScreenChildLocalAnalysisAttemptState::Selected,
        value.capability.capability_state == ScreenFamilyAiHubCapabilityState::Available,
        value
            .capability
            .supported_tasks
            .contains(&value.requested_task),
        value.selected_runtime_ref.is_some(),
        value.transfer_mode != ScreenFamilyAiHubTransferMode::NoTransfer,
        value.destination_custody_state == ScreenEvidenceCustodyState::LiveLanChildAgent,
        value.degraded_states.is_empty(),
    ])
}

fn screen_family_ai_hub_unselected_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        value.selected_runtime_ref.is_none(),
        value.transfer_mode == ScreenFamilyAiHubTransferMode::NoTransfer,
        !value.degraded_states.is_empty(),
    ])
}

fn screen_family_ai_hub_booleans_are_true(values: &[bool]) -> bool {
    values.iter().copied().all(|value| value)
}
