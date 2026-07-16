use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameSessionSummary, APP_GAME_FOREGROUND_BACKGROUND,
    APP_GAME_FOREGROUND_FOREGROUND,
};

pub(super) fn apply_foreground_row(
    summary: &mut AppGameSessionSummary,
    row: &AppGameForegroundEvidenceRow,
) {
    let foreground_duration = summary
        .foreground_duration_ms
        .max(row.foreground_duration_ms)
        .min(summary.running_duration_ms);
    summary.foreground_duration_ms = foreground_duration;
    summary.background_duration_ms = summary
        .running_duration_ms
        .saturating_sub(summary.foreground_duration_ms);
    if row.foreground_state == APP_GAME_FOREGROUND_FOREGROUND {
        summary.last_foreground_at = row.foreground_started_at.clone();
    }
    if row.foreground_state == APP_GAME_FOREGROUND_BACKGROUND {
        summary.last_background_at = row.foreground_ended_at.clone();
    }
    summary.last_observed_at = row.observed_at.clone();
    summary.evidence_count += row.evidence.len() as u64;
    summary.evidence.extend(row.evidence.clone());
    if is_stronger_foreground_classification(row, summary) {
        summary.display_name = row.process_name.clone();
        summary.classification_state = row.classification_state.clone();
        summary.catalog_ready_state = row.catalog_ready_state.clone();
        summary.inventory_entry_id = row.inventory_entry_id.clone();
        summary.launcher_ref = row.launcher_ref.clone();
        summary.catalog_ref = row.catalog_ref.clone();
        summary.confidence = row.confidence;
    }
}

fn is_stronger_foreground_classification(
    row: &AppGameForegroundEvidenceRow,
    summary: &AppGameSessionSummary,
) -> bool {
    foreground_classification_rank(&row.classification_state)
        > foreground_classification_rank(&summary.classification_state)
}

fn foreground_classification_rank(classification_state: &str) -> u8 {
    match classification_state {
        ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_APP => 4,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_GAME => 4,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER => 3,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
