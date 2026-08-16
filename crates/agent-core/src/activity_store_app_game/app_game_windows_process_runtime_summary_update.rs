use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, AppGameSessionSummary, APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
};

pub(super) fn update_runtime_summary(
    summary: &mut AppGameSessionSummary,
    row: &AppGameRuntimeEvidenceRow,
) {
    summary.last_observed_at = row.observed_at.clone();
    if row.exited_at.is_some() {
        summary.ended_at = row.exited_at.clone();
        summary.end_reason = Some(
            ocentra_parent_agent_protocol::app_game::APP_GAME_SESSION_END_REASON_PROCESS_EXIT
                .to_string(),
        );
    }
    summary.running_duration_ms = summary.running_duration_ms.max(row.running_duration_ms);
    summary.background_duration_ms = summary.running_duration_ms;
    summary.last_background_at = if row.running_duration_ms > 0 {
        Some(row.observed_at.clone())
    } else {
        summary.last_background_at.clone()
    };
    summary.observation_count += 1;
    summary.evidence_count += row.evidence.len() as u64;
    summary.evidence.extend(row.evidence.clone());
    if is_stronger_runtime_classification(row, summary) {
        summary.display_name = row.process_name.clone();
        summary.classification_state = row.classification_state.clone();
        summary.catalog_ready_state = row.catalog_ready_state.clone();
        summary.inventory_entry_id = row.inventory_entry_id.clone();
        summary.launcher_ref = row.launcher_ref.clone();
        summary.catalog_ref = row.catalog_ref.clone();
        summary.confidence = row.confidence;
    }
}

fn is_stronger_runtime_classification(
    row: &AppGameRuntimeEvidenceRow,
    summary: &AppGameSessionSummary,
) -> bool {
    runtime_classification_rank(&row.classification_state)
        > runtime_classification_rank(&summary.classification_state)
}

fn runtime_classification_rank(classification_state: &str) -> u8 {
    match classification_state {
        APP_GAME_CLASSIFICATION_KNOWN_APP => 3,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER => 2,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
