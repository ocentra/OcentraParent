use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{AppGameRuntimeEvidenceRow, AppGameSessionSummary};

#[path = "app_game_windows_process_runtime_row.rs"]
mod app_game_windows_process_runtime_row;
#[path = "app_game_windows_process_runtime_state.rs"]
mod app_game_windows_process_runtime_state;
#[path = "app_game_windows_process_runtime_summary.rs"]
mod app_game_windows_process_runtime_summary;

use app_game_windows_process_runtime_row::row_from_record;
use app_game_windows_process_runtime_summary::upsert_runtime_summary;

pub struct WindowsProcessRuntimeRecord {
    pub runtime_evidence_id: String,
    pub observed_at: String,
    pub process_identity: Option<String>,
    pub process_id: u64,
    pub parent_process_id: Option<u64>,
    pub process_name: String,
    pub executable_path_ref: Option<String>,
    pub publisher_signature_ref: Option<String>,
    pub file_hash_ref: Option<String>,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub started_at: Option<String>,
    pub exited_at: Option<String>,
    pub running_duration_ms: u64,
    pub observation_mode: String,
    pub classification_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_process_runtime_rows_from_records(
    records: &[WindowsProcessRuntimeRecord],
) -> Vec<AppGameRuntimeEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn runtime_session_summaries_from_rows(
    rows: &[AppGameRuntimeEvidenceRow],
) -> Vec<AppGameSessionSummary> {
    let mut summaries = Vec::new();
    for row in rows {
        upsert_runtime_summary(&mut summaries, row);
    }
    summaries
}
