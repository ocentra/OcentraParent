use crate::screen_family_ai_hub_routing::{
    ScreenChildLocalAnalysisAttempt, ScreenChildLocalAnalysisAttemptState,
    ScreenEvidenceCustodyState, ScreenFamilyAiHubCapability, ScreenFamilyAiHubCapabilityState,
    ScreenFamilyAiHubDegradedState, ScreenFamilyAiHubExecutionState, ScreenFamilyAiHubRequestedTask,
    ScreenFamilyAiHubRoute, ScreenFamilyAiHubRouteRequest, ScreenFamilyAiHubTransferMode,
};

pub(crate) fn screen_family_ai_hub_capability_is_consistent(
    value: &ScreenFamilyAiHubCapability,
) -> bool {
    if value.custody_state != ScreenEvidenceCustodyState::LiveLanChildAgent {
        return false;
    }
    if value.capability_state == ScreenFamilyAiHubCapabilityState::Available {
        return screen_family_ai_hub_available_capability_is_consistent(value);
    }
    screen_family_ai_hub_degraded_capability_is_consistent(value)
}

pub(crate) fn screen_child_local_attempt_is_consistent(
    value: &ScreenChildLocalAnalysisAttempt,
) -> bool {
    if value.execution_state == ScreenChildLocalAnalysisAttemptState::Selected {
        return screen_family_ai_hub_booleans_are_true(&[
            value.model_runtime_ref.is_some(),
            value.degraded_states.is_empty(),
        ]);
    }
    screen_family_ai_hub_booleans_are_true(&[
        value.model_runtime_ref.is_none(),
        !value.degraded_states.is_empty(),
    ])
}

pub(crate) fn screen_family_ai_hub_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    if !screen_family_ai_hub_route_flags_are_consistent(value) {
        return false;
    }
    if value.execution_state == ScreenFamilyAiHubExecutionState::Selected {
        return screen_family_ai_hub_selected_route_is_consistent(value);
    }
    screen_family_ai_hub_unselected_route_is_consistent(value)
}

pub(crate) fn plan_screen_family_ai_hub_route(
    request: &ScreenFamilyAiHubRouteRequest,
) -> ScreenFamilyAiHubRoute {
    let selected = screen_family_ai_hub_can_serve(request);
    let degraded_states = if selected {
        Vec::new()
    } else {
        screen_family_ai_hub_degraded_states_for(request)
    };

    ScreenFamilyAiHubRoute {
        schema_version: crate::screen_family_ai_hub_routing::SCREEN_FAMILY_AI_HUB_ROUTE_SCHEMA_VERSION,
        route_id: request.route_id.clone(),
        queue_job_id: request.queue_job_id.clone(),
        routed_at: request.routed_at.clone(),
        requested_task: request.requested_task.clone(),
        source_child_local_attempt: request.source_child_local_attempt.clone(),
        capability: request.capability.clone(),
        execution_state: if selected {
            ScreenFamilyAiHubExecutionState::Selected
        } else {
            screen_family_ai_hub_execution_state_for(&request.capability)
        },
        selected_runtime_ref: if selected {
            request.capability.model_runtime_ref.clone()
        } else {
            None
        },
        transfer_mode: if selected {
            request.transfer_mode.clone()
        } else {
            ScreenFamilyAiHubTransferMode::NoTransfer
        },
        source_custody_state: request.source_custody_state.clone(),
        destination_custody_state: if selected {
            ScreenEvidenceCustodyState::LiveLanChildAgent
        } else {
            ScreenEvidenceCustodyState::Unavailable
        },
        degraded_states,
        audit_evidence_ids: request.audit_evidence_ids.clone(),
        parent_approved_family_hub: request.parent_approved_family_hub,
        local_provider_attempted: true,
        child_safety_local_fallback_preserved: true,
        summary_first: true,
        redacted_or_cropped_input_required: true,
        raw_full_screenshot_transfer_allowed: false,
        raw_image_retention_allowed: false,
        remote_provider_selected: false,
        remote_api_fallback_allowed: false,
        ocentra_hosted_processing_allowed: false,
        remote_default_for_blocking: false,
    }
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

fn screen_family_ai_hub_can_serve(request: &ScreenFamilyAiHubRouteRequest) -> bool {
    screen_family_ai_hub_booleans_are_true(&[
        request.parent_approved_family_hub,
        request.source_child_local_attempt.execution_state
            != ScreenChildLocalAnalysisAttemptState::Selected,
        request.capability.capability_state == ScreenFamilyAiHubCapabilityState::Available,
        request
            .capability
            .supported_tasks
            .contains(&request.requested_task),
        request.transfer_mode != ScreenFamilyAiHubTransferMode::NoTransfer,
        matches!(
            request.source_custody_state,
            ScreenEvidenceCustodyState::ChildDeviceTempQueue
                | ScreenEvidenceCustodyState::ChildDeviceJournal
        ),
    ])
}

fn screen_family_ai_hub_degraded_states_for(
    request: &ScreenFamilyAiHubRouteRequest,
) -> Vec<ScreenFamilyAiHubDegradedState> {
    if let Some(state) = screen_family_ai_hub_first_degraded_state(request) {
        return vec![state];
    }
    request.capability.degraded_states.clone()
}

fn screen_family_ai_hub_first_degraded_state(
    request: &ScreenFamilyAiHubRouteRequest,
) -> Option<ScreenFamilyAiHubDegradedState> {
    if request.source_child_local_attempt.execution_state == ScreenChildLocalAnalysisAttemptState::Selected {
        return Some(ScreenFamilyAiHubDegradedState::ChildLocalAlreadySelected);
    }
    if !request.parent_approved_family_hub {
        return Some(ScreenFamilyAiHubDegradedState::ParentDisabled);
    }
    if !request
        .capability
        .supported_tasks
        .contains(&request.requested_task)
    {
        return Some(ScreenFamilyAiHubDegradedState::UnsupportedTask);
    }
    if !matches!(
        request.source_custody_state,
        ScreenEvidenceCustodyState::ChildDeviceTempQueue
            | ScreenEvidenceCustodyState::ChildDeviceJournal
    ) {
        return Some(ScreenFamilyAiHubDegradedState::CustodyUnsafe);
    }
    if request.capability.degraded_states.is_empty() {
        return Some(ScreenFamilyAiHubDegradedState::ManualRequired);
    }
    None
}

fn screen_family_ai_hub_execution_state_for(
    capability: &ScreenFamilyAiHubCapability,
) -> ScreenFamilyAiHubExecutionState {
    if capability.capability_state == ScreenFamilyAiHubCapabilityState::HubUnavailable {
        ScreenFamilyAiHubExecutionState::Unavailable
    } else {
        ScreenFamilyAiHubExecutionState::ManualRequired
    }
}

fn screen_family_ai_hub_booleans_are_true(values: &[bool]) -> bool {
    values.iter().copied().all(|value| value)
}
