use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};
use ocentra_parent_agent_protocol::app_game_boundary_read_model::{
    AppGameHealthStatus, AppGamePerformanceHealthReadModel,
};

pub fn app_game_performance_health(
    model: &AppGameServiceReadModel,
) -> AppGamePerformanceHealthReadModel {
    let status = app_game_health_status(model);
    AppGamePerformanceHealthReadModel {
        status,
        limit: model.limit,
        returned: model
            .inventory_returned
            .saturating_add(model.running_now_returned)
            .saturating_add(model.foreground_now_returned)
            .saturating_add(model.launcher_returned)
            .saturating_add(model.daily_rollup_returned),
        inventory_returned: model.inventory_returned,
        running_now_returned: model.running_now_returned,
        foreground_now_returned: model.foreground_now_returned,
        launcher_returned: model.launcher_returned,
        daily_rollup_returned: model.daily_rollup_returned,
        custody_label: model.custody_label.clone(),
        replay_state: model.replay_state.clone(),
    }
}

fn app_game_health_status(model: &AppGameServiceReadModel) -> AppGameHealthStatus {
    if model.replay_state.trim().is_empty() || model.custody_label.trim().is_empty() {
        return AppGameHealthStatus::Unavailable;
    }
    if model.limit == 0 || !counts_match_rows(model) || counts_exceed_limit(model) {
        return AppGameHealthStatus::Degraded;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR
        || model.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
        || model.capability_status == APP_GAME_CAPABILITY_STATUS_STALE
    {
        return AppGameHealthStatus::Degraded;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_DEGRADED {
        return AppGameHealthStatus::Degraded;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_UNAVAILABLE {
        return AppGameHealthStatus::Unavailable;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM {
        return AppGameHealthStatus::Unavailable;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED {
        return AppGameHealthStatus::ManualRequired;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED {
        return AppGameHealthStatus::NotClaimed;
    }
    if model.capability_status == APP_GAME_CAPABILITY_STATUS_AVAILABLE {
        return AppGameHealthStatus::Healthy;
    }
    AppGameHealthStatus::Unavailable
}

fn counts_match_rows(model: &AppGameServiceReadModel) -> bool {
    model.inventory_returned == model.inventory_rows.len() as u64
        && model.running_now_returned == model.running_now_rows.len() as u64
        && model.foreground_now_returned == model.foreground_now_rows.len() as u64
        && model.launcher_returned == model.launcher_rows.len() as u64
        && model.daily_rollup_returned == model.daily_rollups.len() as u64
}

fn counts_exceed_limit(model: &AppGameServiceReadModel) -> bool {
    [
        model.inventory_returned,
        model.running_now_returned,
        model.foreground_now_returned,
        model.launcher_returned,
        model.daily_rollup_returned,
    ]
    .iter()
    .any(|count| *count > model.limit)
}
