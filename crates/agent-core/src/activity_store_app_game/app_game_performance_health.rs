use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CAPABILITY_STATUS_STALE, APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
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
        returned: model.inventory_returned.saturating_add(model.running_now_returned)
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

#[cfg(test)]
mod tests {
    use super::*;
    use ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED;

    fn model(status: &str) -> AppGameServiceReadModel {
        AppGameServiceReadModel {
            schema_version: 1,
            generated_at: "2026-08-28T00:00:00Z".into(),
            limit: 10,
            custody_label: "local-sqlite".into(),
            replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.into(),
            capability_status: status.into(),
            inventory_returned: 0,
            running_now_returned: 0,
            foreground_now_returned: 0,
            launcher_returned: 0,
            daily_rollup_returned: 0,
            evidence_claim_returned: 0,
            identity_returned: 0,
            approval_authority_returned: 0,
            approval_action_result_returned: 0,
            platform_authority_matrix_returned: 0,
            ai_classifier_result_returned: 0,
            inventory_rows: Vec::new(),
            running_now_rows: Vec::new(),
            foreground_now_rows: Vec::new(),
            launcher_rows: Vec::new(),
            daily_rollups: Vec::new(),
            evidence_claim_rows: Vec::new(),
            identity_rows: Vec::new(),
            approval_authority_rows: Vec::new(),
            approval_action_result_rows: Vec::new(),
            platform_authority_matrices: Vec::new(),
            ai_classifier_result_rows: Vec::new(),
        }
    }

    #[test]
    fn health_matrix_is_fail_closed_and_preserves_persisted_bounds() {
        for (status, expected) in [
            (APP_GAME_CAPABILITY_STATUS_AVAILABLE, AppGameHealthStatus::Healthy),
            (APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, AppGameHealthStatus::Degraded),
            (APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, AppGameHealthStatus::Degraded),
            (APP_GAME_CAPABILITY_STATUS_STALE, AppGameHealthStatus::Degraded),
            (APP_GAME_CAPABILITY_STATUS_DEGRADED, AppGameHealthStatus::Degraded),
            (APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, AppGameHealthStatus::Unavailable),
            (APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM, AppGameHealthStatus::Unavailable),
            (APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, AppGameHealthStatus::ManualRequired),
            (APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, AppGameHealthStatus::NotClaimed),
            ("unknown", AppGameHealthStatus::Unavailable),
        ] {
            assert_eq!(app_game_performance_health(&model(status)).status, expected);
        }
    }

    #[test]
    fn health_rejects_missing_custody_zero_limit_and_count_mismatch() {
        let mut missing = model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
        missing.custody_label.clear();
        assert_eq!(app_game_performance_health(&missing).status, AppGameHealthStatus::Unavailable);
        let mut invalid = model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
        invalid.limit = 0;
        assert_eq!(app_game_performance_health(&invalid).status, AppGameHealthStatus::Degraded);
        invalid.limit = 10;
        invalid.inventory_returned = 1;
        assert_eq!(app_game_performance_health(&invalid).status, AppGameHealthStatus::Degraded);
        for field in 0..5 {
            let mut over = model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
            match field {
                0 => over.inventory_returned = 11,
                1 => over.running_now_returned = 11,
                2 => over.foreground_now_returned = 11,
                3 => over.launcher_returned = 11,
                _ => over.daily_rollup_returned = 11,
            }
            assert_eq!(app_game_performance_health(&over).status, AppGameHealthStatus::Degraded);
        }
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
    [model.inventory_returned, model.running_now_returned, model.foreground_now_returned,
        model.launcher_returned, model.daily_rollup_returned]
        .iter()
        .any(|count| *count > model.limit)
}
