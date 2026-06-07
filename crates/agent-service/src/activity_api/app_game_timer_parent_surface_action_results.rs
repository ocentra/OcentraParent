use ocentra_parent_agent_protocol::AppGameServiceReadModel;

pub(crate) struct TimerParentSurfaceControlActionResults {
    pub(crate) reference_ids: Vec<String>,
    pub(crate) statuses: Vec<String>,
    pub(crate) capability_states: Vec<String>,
    pub(crate) enforcement_statuses: Vec<String>,
}

pub(crate) fn timer_parent_surface_control_action_results(
    model: &AppGameServiceReadModel,
) -> TimerParentSurfaceControlActionResults {
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
