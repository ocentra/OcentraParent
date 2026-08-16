use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow;

#[path = "app_game_windows_launcher_classification.rs"]
mod app_game_windows_launcher_classification;
#[path = "app_game_windows_launcher_row.rs"]
mod app_game_windows_launcher_row;

use app_game_windows_launcher_row::row_from_record;

pub struct WindowsLauncherEvidenceRecord {
    pub launcher_evidence_id: String,
    pub observed_at: String,
    pub launcher_kind: String,
    pub launcher_ref: String,
    pub launcher_inventory_entry_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_process_identity: Option<String>,
    pub launcher_process_id: Option<u64>,
    pub launcher_process_name: Option<String>,
    pub child_process_identity: Option<String>,
    pub child_inventory_entry_id: Option<String>,
    pub child_game_evidence_claim_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub runtime_state: String,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub capability_status: String,
    pub game_proof_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_launcher_rows_from_records(
    records: &[WindowsLauncherEvidenceRecord],
) -> Vec<AppGameLauncherEvidenceRow> {
    records.iter().map(row_from_record).collect()
}
