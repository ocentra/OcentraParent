use super::*;

pub(super) fn app_game_timer_parent_surface_summary_details_impl(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    let mut details = summary_status_details(read_model);
    details.extend(summary_control_action_details(read_model));
    details.extend(summary_child_handoff_details(read_model));
    details.extend(summary_runtime_claim_details(read_model, product_claim));
    details.extend(app_game_timer_parent_surface_summary_claim_details(
        product_claim,
    ));
    details
}

fn summary_status_details(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Status",
            app_game_timer_parent_surface_load_state(read_model),
        ),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Custody", read_model.custody_label.as_str()),
        app_game_detail("Capability", read_model.capability_status.as_str()),
        app_game_detail("Rows returned", read_model.returned.to_string()),
        app_game_detail(
            "Read model rows",
            read_model.ready_for_parent_surface_count.to_string(),
        ),
        app_game_detail(
            "Manual review",
            read_model.runtime_manual_required_count.to_string(),
        ),
    ]
}

fn summary_control_action_details(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Control action results",
            read_model.control_action_result_count.to_string(),
        ),
        app_game_detail(
            "Control action result refs",
            app_game_join_strings(&read_model.control_action_result_reference_ids),
        ),
        app_game_detail(
            "Control action result statuses",
            app_game_join_strings(&read_model.control_action_result_statuses),
        ),
        app_game_detail(
            "Control action capabilities",
            app_game_join_strings(&read_model.control_action_result_capability_states),
        ),
        app_game_detail(
            "Control action enforcement statuses",
            app_game_join_strings(&read_model.control_action_result_enforcement_statuses),
        ),
    ]
}

fn summary_child_handoff_details(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Child-facing reason refs",
            app_game_join_strings(&read_model.child_facing_reason_reference_ids),
        ),
        app_game_detail(
            "Child-facing status refs",
            app_game_join_strings(&read_model.child_facing_status_reference_ids),
        ),
        app_game_detail(
            "Child UX handoff ready",
            read_model.child_ux_handoff_ready_count.to_string(),
        ),
        app_game_detail(
            "Child UX handoff blocked",
            read_model.child_ux_handoff_blocked_count.to_string(),
        ),
        app_game_detail(
            "Child UX handoff refs",
            app_game_join_strings(&read_model.child_ux_handoff_reference_ids),
        ),
    ]
}

fn summary_runtime_claim_details(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Timer runtime",
            app_game_claimed_value(read_model.timer_runtime_claimed),
        ),
        app_game_detail(
            "Scheduler persistence",
            app_game_claimed_value(read_model.scheduler_persistence_claimed),
        ),
        app_game_detail(
            "Durable scheduler storage",
            app_game_claimed_value(read_model.durable_scheduler_storage_claimed),
        ),
        app_game_detail(
            "Audit runtime",
            app_game_claimed_value(read_model.audit_runtime_claimed),
        ),
        app_game_detail(
            "Rollback runtime",
            app_game_claimed_value(read_model.rollback_runtime_claimed),
        ),
        app_game_detail("Adapter dispatch", "not-claimed"),
        app_game_detail("Child delivery", "not-claimed"),
        app_game_detail("Platform state", "not-claimed"),
        app_game_detail("Product claim", product_claim),
    ]
}
