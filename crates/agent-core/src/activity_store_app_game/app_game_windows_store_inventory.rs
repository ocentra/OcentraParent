use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow,
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_STALE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_STALE, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_CATEGORY_GAME, APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE,
    APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE, APP_GAME_INVENTORY_STATE_ADAPTER_ERROR,
    APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED, APP_GAME_INVENTORY_STATE_STALE,
    APP_GAME_INVENTORY_STATE_UNAVAILABLE, APP_GAME_PRODUCT_NATIVE_APP,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
};

#[path = "app_game_windows_store_inventory_row.rs"]
mod app_game_windows_store_inventory_row;
#[path = "app_game_windows_store_inventory_state.rs"]
mod app_game_windows_store_inventory_state;

use app_game_windows_store_inventory_row::row_from_record;

pub struct WindowsStorePackageInventoryRecord {
    pub observed_at: String,
    pub source_ref: String,
    pub display_label: String,
    pub package_id: Option<String>,
    pub bundle_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub store_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub category_kind: Option<String>,
    pub inventory_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub struct WindowsRuntimePackageIdentity {
    pub package_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub executable_path_ref: Option<String>,
    pub display_label: Option<String>,
}

pub fn windows_store_inventory_rows_from_records(
    records: &[WindowsStorePackageInventoryRecord],
) -> Vec<AppGameInventoryEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn store_package_matches_runtime_identity(
    record: &WindowsStorePackageInventoryRecord,
    runtime_identity: &WindowsRuntimePackageIdentity,
) -> bool {
    record.package_id.is_some() && record.package_id == runtime_identity.package_id
        || record.app_user_model_id.is_some()
            && record.app_user_model_id == runtime_identity.app_user_model_id
}

pub fn store_app_user_model_policy_target(
    record: &WindowsStorePackageInventoryRecord,
) -> Option<String> {
    record.app_user_model_id.clone()
}
