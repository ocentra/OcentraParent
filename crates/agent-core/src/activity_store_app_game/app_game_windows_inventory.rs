use std::collections::HashSet;

use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::AppGameInventoryEvidenceRow;

#[path = "app_game_windows_inventory_identity.rs"]
mod app_game_windows_inventory_identity;
#[path = "app_game_windows_inventory_row.rs"]
mod app_game_windows_inventory_row;
#[path = "app_game_windows_inventory_state.rs"]
mod app_game_windows_inventory_state;

use app_game_windows_inventory_identity::strong_identity_seen;
use app_game_windows_inventory_row::row_from_record;

pub struct WindowsInstalledAppInventoryRecord {
    pub observed_at: String,
    pub source_kind: String,
    pub source_ref: String,
    pub custody_state: String,
    pub display_label: String,
    pub identity_id: Option<String>,
    pub package_id: Option<String>,
    pub bundle_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub desktop_entry_id: Option<String>,
    pub executable_path_ref: Option<String>,
    pub launcher_ref: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub store_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub inventory_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_installed_inventory_rows_from_records(
    records: &[WindowsInstalledAppInventoryRecord],
) -> Vec<AppGameInventoryEvidenceRow> {
    let mut strong_identities = HashSet::new();
    let mut rows = Vec::new();
    for record in records {
        if strong_identity_seen(record, &mut strong_identities) {
            continue;
        }
        rows.push(row_from_record(record));
    }
    rows
}
