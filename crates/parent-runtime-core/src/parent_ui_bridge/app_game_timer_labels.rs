use super::*;

const APP_GAME_TIMER_TARGET_LABELS: &[(&str, &str)] =
    &[("native-app", "Native app"), ("native-game", "Native game")];
const APP_GAME_TIMER_SURFACE_STATE_LABELS: &[(&str, &str)] = &[
    ("ready-for-parent-surface", "Ready for parent surface"),
    ("blocked-by-source-freshness", "Blocked by source freshness"),
    (
        "blocked-by-compiler-decision",
        "Blocked by compiler decision",
    ),
    ("runtime-manual-required", "Runtime manual required"),
];

pub(super) fn app_game_timer_parent_surface_load_state(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> String {
    if read_model.returned == 0 {
        "unavailable".to_string()
    } else if read_model.ready_for_parent_surface_count == read_model.returned {
        "ready".to_string()
    } else {
        "warn".to_string()
    }
}

pub(super) fn app_game_timer_parent_surface_product_claim(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> String {
    let has_active_state = read_model.timer_runtime_claimed
        || read_model.scheduler_persistence_claimed
        || read_model.durable_scheduler_storage_claimed;
    let has_control_action_results = read_model.control_action_result_count > 0;

    match (has_active_state, has_control_action_results) {
        (true, true) => "Active timer state-store and control action-result rows are visible; live scheduling automation, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.".to_string(),
        (true, false) => "Active timer state-store is visible; live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.".to_string(),
        (false, true) => "Control action-result rows are visible from app/game SQLite replay; live scheduling automation, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.".to_string(),
        (false, false) => "Parent-surface rendering only; active timer state-store is shown only when reported by the service. Live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.".to_string(),
    }
}

pub(super) fn app_game_timer_target_label(target: &str) -> String {
    APP_GAME_TIMER_TARGET_LABELS
        .iter()
        .find(|(raw, _)| *raw == target)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| target.to_string())
}

pub(super) fn app_game_timer_surface_state_label(state: &str) -> String {
    APP_GAME_TIMER_SURFACE_STATE_LABELS
        .iter()
        .find(|(raw, _)| *raw == state)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| state.to_string())
}

pub(super) fn app_game_timer_parent_preference_setup_payload(
    record: &ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord,
    requested_at: &str,
) -> Value {
    let request = AppGameTimerParentPreferenceSetupRequest {
        request_id: format!(
            "app-game-parent-preference-setup-request::{}::{}",
            record.parent_preference_setup_reference_id, requested_at
        ),
        requested_at: requested_at.to_string(),
        parent_surface_intent_reference_id: record
            .source_parent_surface_intent_reference_id
            .clone(),
        parent_preference_setup_reference_id: record.parent_preference_setup_reference_id.clone(),
        request_reference_ids: record.parent_preference_setup_request_reference_ids.clone(),
    };
    json!({
        "ActivityAppGameTimerParentPreferenceSetupRequest":
            serde_json::to_string(&request).unwrap_or_else(|_| "{}".to_string())
    })
}
