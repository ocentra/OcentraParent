use super::degradation::{
    screen_family_ai_hub_can_serve, screen_family_ai_hub_degraded_states_for,
};
use crate::screen_family_ai_hub_routing::{
    ScreenEvidenceCustodyState, ScreenFamilyAiHubCapability, ScreenFamilyAiHubCapabilityState,
    ScreenFamilyAiHubExecutionState, ScreenFamilyAiHubRoute, ScreenFamilyAiHubRouteRequest,
    ScreenFamilyAiHubTransferMode,
};

pub(super) fn plan_screen_family_ai_hub_route(
    request: &ScreenFamilyAiHubRouteRequest,
) -> ScreenFamilyAiHubRoute {
    let selected = screen_family_ai_hub_can_serve(request);
    let degraded_states = if selected {
        Vec::new()
    } else {
        screen_family_ai_hub_degraded_states_for(request)
    };

    ScreenFamilyAiHubRoute {
        schema_version:
            crate::screen_family_ai_hub_routing::SCREEN_FAMILY_AI_HUB_ROUTE_SCHEMA_VERSION,
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

fn screen_family_ai_hub_execution_state_for(
    capability: &ScreenFamilyAiHubCapability,
) -> ScreenFamilyAiHubExecutionState {
    if capability.capability_state == ScreenFamilyAiHubCapabilityState::HubUnavailable {
        ScreenFamilyAiHubExecutionState::Unavailable
    } else {
        ScreenFamilyAiHubExecutionState::ManualRequired
    }
}
