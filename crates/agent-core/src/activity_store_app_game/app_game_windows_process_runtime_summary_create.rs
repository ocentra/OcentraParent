use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, AppGameSessionSummary, APP_GAME_SCHEMA_VERSION,
    APP_GAME_SESSION_END_REASON_PROCESS_EXIT, APP_GAME_SESSION_ID_PREFIX,
};

pub(crate) fn upsert_runtime_summary(
    summaries: &mut Vec<AppGameSessionSummary>,
    row: &AppGameRuntimeEvidenceRow,
) {
    match summaries
        .iter_mut()
        .find(|summary| summary.primary_process_identity == row.process_identity)
    {
        Some(summary) => {
            super::app_game_windows_process_runtime_summary_update::update_runtime_summary(
                summary, row,
            )
        }
        None => summaries.push(summary_from_runtime_row(row)),
    }
}

fn summary_from_runtime_row(row: &AppGameRuntimeEvidenceRow) -> AppGameSessionSummary {
    AppGameSessionSummary {
        schema_version: APP_GAME_SCHEMA_VERSION,
        session_id: session_id(&row.process_identity),
        primary_process_identity: row.process_identity.clone(),
        display_name: row.process_name.clone(),
        classification_state: row.classification_state.clone(),
        catalog_ready_state: row.catalog_ready_state.clone(),
        inventory_entry_id: row.inventory_entry_id.clone(),
        launcher_ref: row.launcher_ref.clone(),
        catalog_ref: row.catalog_ref.clone(),
        started_at: row
            .started_at
            .clone()
            .unwrap_or_else(|| row.observed_at.clone()),
        last_observed_at: row.observed_at.clone(),
        ended_at: row.exited_at.clone(),
        end_reason: row
            .exited_at
            .as_ref()
            .map(|_| APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string()),
        running_duration_ms: row.running_duration_ms,
        foreground_duration_ms: 0,
        background_duration_ms: row.running_duration_ms,
        last_foreground_at: None,
        last_background_at: if row.running_duration_ms > 0 {
            Some(row.observed_at.clone())
        } else {
            None
        },
        observation_gap_ms: 0,
        observation_count: 1,
        evidence_count: row.evidence.len() as u64,
        evidence: row.evidence.clone(),
        ai_digest_ref: None,
        confidence: row.confidence,
    }
}

fn session_id(process_identity: &str) -> String {
    let mut session_id = String::from(APP_GAME_SESSION_ID_PREFIX);
    session_id.push_str(process_identity);
    session_id
}
