use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameSessionSummary,
};

#[path = "app_game_windows_foreground_apply.rs"]
mod app_game_windows_foreground_apply;
#[path = "app_game_windows_foreground_classification.rs"]
mod app_game_windows_foreground_classification;
#[path = "app_game_windows_foreground_row.rs"]
mod app_game_windows_foreground_row;

use app_game_windows_foreground_apply::apply_foreground_row;
use app_game_windows_foreground_row::row_from_record;

pub struct WindowsForegroundWindowRecord {
    pub foreground_evidence_id: String,
    pub observed_at: String,
    pub process_identity: Option<String>,
    pub process_id: u64,
    pub process_name: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub window_ref: Option<String>,
    pub window_title_ref: Option<String>,
    pub title_capture_state: String,
    pub foreground_started_at: Option<String>,
    pub foreground_ended_at: Option<String>,
    pub foreground_duration_ms: u64,
    pub foreground_state: String,
    pub classification_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_foreground_rows_from_records(
    records: &[WindowsForegroundWindowRecord],
) -> Vec<AppGameForegroundEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn apply_foreground_rows_to_runtime_summaries(
    summaries: &mut [AppGameSessionSummary],
    rows: &[AppGameForegroundEvidenceRow],
) {
    for row in rows {
        if let Some(summary) = summaries
            .iter_mut()
            .find(|summary| summary.primary_process_identity == row.process_identity)
        {
            apply_foreground_row(summary, row);
        }
    }
}
