use ocentra_parent_agent_protocol::{
    constants, AppGameControlActionResult, AppGameServiceReadModel,
    AppGameTimerParentSurfaceChildUxLocalArtifactRecord, APP_GAME_CONTROL_POLICY_KIND_GAME,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

pub(crate) struct TimerParentSurfaceControlActionResults {
    pub(crate) reference_ids: Vec<String>,
    pub(crate) statuses: Vec<String>,
    pub(crate) capability_states: Vec<String>,
    pub(crate) enforcement_statuses: Vec<String>,
    pub(crate) child_reason_reference_ids: Vec<String>,
    pub(crate) child_status_reference_ids: Vec<String>,
    pub(crate) child_ux_handoff_ready_count: u64,
    pub(crate) child_ux_handoff_blocked_count: u64,
    pub(crate) child_ux_handoff_reference_ids: Vec<String>,
    pub(crate) child_ux_local_handoff_artifact_record_count: u64,
    pub(crate) child_ux_local_handoff_artifact_skipped_count: u64,
    pub(crate) child_ux_local_handoff_artifact_reference_ids: Vec<String>,
    pub(crate) child_ux_local_handoff_artifact_records:
        Vec<AppGameTimerParentSurfaceChildUxLocalArtifactRecord>,
}

pub(crate) fn timer_parent_surface_control_action_results(
    model: &AppGameServiceReadModel,
) -> TimerParentSurfaceControlActionResults {
    let child_ux_handoff_reference_ids: Vec<String> = model
        .approval_action_result_rows
        .iter()
        .filter(|row| {
            !row.request.child_reason_references.is_empty()
                && !row.request.child_status_references.is_empty()
        })
        .map(|row| row.result_id.clone())
        .collect();
    let child_ux_handoff_ready_count = child_ux_handoff_reference_ids.len() as u64;
    let child_ux_handoff_blocked_count =
        model.approval_action_result_rows.len() as u64 - child_ux_handoff_ready_count;
    let child_ux_local_handoff_artifact_reference_ids = child_ux_handoff_reference_ids
        .iter()
        .map(|reference_id| {
            let mut artifact_reference_id =
                String::from(constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX);
            artifact_reference_id.push_str(reference_id);
            artifact_reference_id
        })
        .collect::<Vec<_>>();
    let child_ux_local_handoff_artifact_records = model
        .approval_action_result_rows
        .iter()
        .filter(|row| child_ux_local_artifact_row_is_ready(row))
        .map(child_ux_local_artifact_record)
        .collect::<Vec<_>>();

    TimerParentSurfaceControlActionResults {
        reference_ids: model
            .approval_action_result_rows
            .iter()
            .map(|row| row.result_id.clone())
            .collect(),
        statuses: unique_action_result_values(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.result_status.clone()),
        ),
        capability_states: unique_action_result_values(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.capability_state.clone()),
        ),
        enforcement_statuses: unique_action_result_values(
            model
                .approval_action_result_rows
                .iter()
                .filter_map(|row| row.enforcement_result.as_ref())
                .map(|result| result.status.clone()),
        ),
        child_reason_reference_ids: unique_action_result_values(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_reason_references.iter().cloned()),
        ),
        child_status_reference_ids: unique_action_result_values(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_status_references.iter().cloned()),
        ),
        child_ux_handoff_ready_count,
        child_ux_handoff_blocked_count,
        child_ux_handoff_reference_ids,
        child_ux_local_handoff_artifact_record_count: child_ux_local_handoff_artifact_reference_ids
            .len() as u64,
        child_ux_local_handoff_artifact_skipped_count: child_ux_handoff_blocked_count,
        child_ux_local_handoff_artifact_reference_ids,
        child_ux_local_handoff_artifact_records,
    }
}

fn child_ux_local_artifact_row_is_ready(row: &AppGameControlActionResult) -> bool {
    !row.request.child_reason_references.is_empty()
        && !row.request.child_status_references.is_empty()
}

fn child_ux_local_artifact_record(
    row: &AppGameControlActionResult,
) -> AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
    AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        artifact_reference_id: child_ux_local_artifact_reference_id(&row.result_id),
        source_result_id: row.result_id.clone(),
        target_domain: child_ux_local_artifact_target_domain(row),
        child_reason_reference_ids: row.request.child_reason_references.clone(),
        child_status_reference_ids: row.request.child_status_references.clone(),
        child_delivery_claimed: false,
        notification_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}

fn child_ux_local_artifact_reference_id(reference_id: &str) -> String {
    let mut artifact_reference_id =
        String::from(constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX);
    artifact_reference_id.push_str(reference_id);
    artifact_reference_id
}

fn child_ux_local_artifact_target_domain(row: &AppGameControlActionResult) -> String {
    if row.request.policy_kind == APP_GAME_CONTROL_POLICY_KIND_GAME {
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string()
    } else {
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP.to_string()
    }
}

fn unique_action_result_values(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut unique = Vec::new();
    for value in values {
        if !value.is_empty() && !unique.iter().any(|existing| existing == &value) {
            unique.push(value);
        }
    }
    unique
}
