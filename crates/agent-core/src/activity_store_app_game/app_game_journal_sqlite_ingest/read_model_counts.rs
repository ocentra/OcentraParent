use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};

pub(super) fn refresh_returned_counts(model: &mut AppGameServiceReadModel) {
    model.inventory_returned = model.inventory_rows.len() as u64;
    model.running_now_returned = model.running_now_rows.len() as u64;
    model.foreground_now_returned = model.foreground_now_rows.len() as u64;
    model.launcher_returned = model.launcher_rows.len() as u64;
    model.daily_rollup_returned = model.daily_rollups.len() as u64;
    model.evidence_claim_returned = model.evidence_claim_rows.len() as u64;
    model.identity_returned = model.identity_rows.len() as u64;
    model.approval_authority_returned = model.approval_authority_rows.len() as u64;
    model.approval_action_result_returned = model.approval_action_result_rows.len() as u64;
    model.platform_authority_matrix_returned = model.platform_authority_matrices.len() as u64;
    model.ai_classifier_result_returned = model.ai_classifier_result_rows.len() as u64;
    model.capability_status = model_capability_status(model);
}

fn model_capability_status(model: &AppGameServiceReadModel) -> String {
    for status in [
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
        APP_GAME_CAPABILITY_STATUS_DEGRADED,
        APP_GAME_CAPABILITY_STATUS_STALE,
        APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
        APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
        APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    ] {
        if has_capability_status(model, status) {
            return status.to_string();
        }
    }
    if model.inventory_returned
        + model.running_now_returned
        + model.foreground_now_returned
        + model.launcher_returned
        + model.daily_rollup_returned
        > 0
    {
        return APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string();
    }
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string()
}

fn has_capability_status(model: &AppGameServiceReadModel, status: &str) -> bool {
    model
        .inventory_rows
        .iter()
        .any(|row| row.capability_status == status)
        || model
            .running_now_rows
            .iter()
            .any(|row| row.capability_status == status)
        || model
            .foreground_now_rows
            .iter()
            .any(|row| row.capability_status == status)
        || model
            .launcher_rows
            .iter()
            .any(|row| row.capability_status == status)
}
