use crate::screen_family_ai_hub_routing::{
    ScreenChildLocalAnalysisAttemptState, ScreenEvidenceCustodyState,
    ScreenFamilyAiHubCapabilityState, ScreenFamilyAiHubDegradedState,
    ScreenFamilyAiHubRouteRequest, ScreenFamilyAiHubTransferMode,
};

pub(super) fn screen_family_ai_hub_can_serve(request: &ScreenFamilyAiHubRouteRequest) -> bool {
    [
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
    ]
    .into_iter()
    .all(|value| value)
}

pub(super) fn screen_family_ai_hub_degraded_states_for(
    request: &ScreenFamilyAiHubRouteRequest,
) -> Vec<ScreenFamilyAiHubDegradedState> {
    screen_family_ai_hub_first_degraded_state(request).map_or_else(
        || request.capability.degraded_states.clone(),
        |state| vec![state],
    )
}

fn screen_family_ai_hub_first_degraded_state(
    request: &ScreenFamilyAiHubRouteRequest,
) -> Option<ScreenFamilyAiHubDegradedState> {
    (request.source_child_local_attempt.execution_state
        == ScreenChildLocalAnalysisAttemptState::Selected)
        .then_some(ScreenFamilyAiHubDegradedState::ChildLocalAlreadySelected)
        .or_else(|| {
            (!request.parent_approved_family_hub)
                .then_some(ScreenFamilyAiHubDegradedState::ParentDisabled)
        })
        .or_else(|| {
            (!request
                .capability
                .supported_tasks
                .contains(&request.requested_task))
            .then_some(ScreenFamilyAiHubDegradedState::UnsupportedTask)
        })
        .or_else(|| {
            (!matches!(
                request.source_custody_state,
                ScreenEvidenceCustodyState::ChildDeviceTempQueue
                    | ScreenEvidenceCustodyState::ChildDeviceJournal
            ))
            .then_some(ScreenFamilyAiHubDegradedState::CustodyUnsafe)
        })
        .or_else(|| {
            request
                .capability
                .degraded_states
                .is_empty()
                .then_some(ScreenFamilyAiHubDegradedState::ManualRequired)
        })
}
