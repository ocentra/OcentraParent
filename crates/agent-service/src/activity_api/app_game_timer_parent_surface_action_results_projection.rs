use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
};
use ocentra_parent_agent_protocol::constants;

use super::{
    child_ux_handoff_reference_ids, child_ux_local_handoff_artifact_records,
    child_ux_parent_preference_setup_records, child_ux_parent_surface_intent_records,
    child_ux_reference_ids, unique_action_result_values, ActionResultValueInput, ChildUxPrefix,
    TimerParentSurfaceControlActionResults,
};

struct ActionResultProjection {
    reference_ids: Vec<String>,
    statuses: Vec<String>,
    capability_states: Vec<String>,
    enforcement_statuses: Vec<String>,
    child_reason_reference_ids: Vec<String>,
    child_status_reference_ids: Vec<String>,
    adapter_dispatch_claimed: bool,
    platform_enforcement_claimed: bool,
}

struct ChildUxProjection {
    handoff_ready_count: u64,
    handoff_blocked_count: u64,
    handoff_reference_ids: Vec<String>,
    local_handoff_artifact_reference_ids: Vec<String>,
    local_handoff_artifact_records:
        Vec<ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxLocalArtifactRecord>,
    parent_surface_intent_reference_ids: Vec<String>,
    parent_surface_intent_records: Vec<
        ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord,
    >,
    parent_preference_setup_reference_ids: Vec<String>,
    parent_preference_setup_records: Vec<
        ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord,
    >,
}

pub(super) fn build_timer_parent_surface_control_action_results(
    model: &AppGameServiceReadModel,
) -> TimerParentSurfaceControlActionResults {
    let action_results = action_result_projection(model);
    let child_ux = child_ux_projection(model);

    TimerParentSurfaceControlActionResults {
        reference_ids: action_results.reference_ids,
        statuses: action_results.statuses,
        capability_states: action_results.capability_states,
        enforcement_statuses: action_results.enforcement_statuses,
        child_reason_reference_ids: action_results.child_reason_reference_ids,
        child_status_reference_ids: action_results.child_status_reference_ids,
        child_ux_handoff_ready_count: child_ux.handoff_ready_count,
        child_ux_handoff_blocked_count: child_ux.handoff_blocked_count,
        child_ux_handoff_reference_ids: child_ux.handoff_reference_ids,
        child_ux_local_handoff_artifact_reference_ids: child_ux
            .local_handoff_artifact_reference_ids,
        child_ux_local_handoff_artifact_records: child_ux.local_handoff_artifact_records,
        child_ux_parent_surface_intent_reference_ids: child_ux.parent_surface_intent_reference_ids,
        child_ux_parent_surface_intent_records: child_ux.parent_surface_intent_records,
        child_ux_parent_preference_setup_reference_ids: child_ux
            .parent_preference_setup_reference_ids,
        child_ux_parent_preference_setup_records: child_ux.parent_preference_setup_records,
        adapter_dispatch_claimed: action_results.adapter_dispatch_claimed,
        platform_enforcement_claimed: action_results.platform_enforcement_claimed,
    }
}

fn action_result_projection(model: &AppGameServiceReadModel) -> ActionResultProjection {
    ActionResultProjection {
        reference_ids: model
            .approval_action_result_rows
            .iter()
            .map(|row| row.result_id.clone())
            .collect(),
        statuses: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.result_status.clone())
                .collect(),
        ))
        .0,
        capability_states: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.capability_state.clone())
                .collect(),
        ))
        .0,
        enforcement_statuses: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .filter_map(|row| row.enforcement_result.as_ref())
                .map(|result| result.status.clone())
                .collect(),
        ))
        .0,
        child_reason_reference_ids: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_reason_references.iter().cloned())
                .collect(),
        ))
        .0,
        child_status_reference_ids: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_status_references.iter().cloned())
                .collect(),
        ))
        .0,
        adapter_dispatch_claimed: model
            .approval_action_result_rows
            .iter()
            .any(|row| row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED),
        platform_enforcement_claimed: model.approval_action_result_rows.iter().any(|row| {
            row.enforcement_result.as_ref().is_some_and(|result| {
                result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
            })
        }),
    }
}

fn child_ux_projection(model: &AppGameServiceReadModel) -> ChildUxProjection {
    let handoff_reference_ids = child_ux_handoff_reference_ids(model);
    let handoff_ready_count = handoff_reference_ids.0.len() as u64;
    let handoff_blocked_count =
        model.approval_action_result_rows.len() as u64 - handoff_ready_count;
    let local_handoff_artifact_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX),
        &handoff_reference_ids,
    )
    .0;
    let local_handoff_artifact_records = child_ux_local_handoff_artifact_records(model);
    let parent_surface_intent_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX),
        &handoff_reference_ids,
    )
    .0;
    let parent_surface_intent_records =
        child_ux_parent_surface_intent_records(&local_handoff_artifact_records);
    let parent_preference_setup_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX),
        &handoff_reference_ids,
    )
    .0;
    let parent_preference_setup_records =
        child_ux_parent_preference_setup_records(&parent_surface_intent_records);

    ChildUxProjection {
        handoff_ready_count,
        handoff_blocked_count,
        handoff_reference_ids: handoff_reference_ids.0,
        local_handoff_artifact_reference_ids,
        local_handoff_artifact_records,
        parent_surface_intent_reference_ids,
        parent_surface_intent_records,
        parent_preference_setup_reference_ids,
        parent_preference_setup_records,
    }
}
