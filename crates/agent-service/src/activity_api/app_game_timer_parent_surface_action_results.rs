use ocentra_parent_agent_protocol::AppGameServiceReadModel;

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
