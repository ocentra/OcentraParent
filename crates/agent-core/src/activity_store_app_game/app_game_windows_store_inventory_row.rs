use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_CATEGORY_GAME,
    APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE, APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION,
};

use super::app_game_windows_store_inventory_state::{
    capability_status_for_record, catalog_ready_state_for_record, category_candidates_for_record,
    classification_state_for_record,
};
use super::WindowsStorePackageInventoryRecord;

pub(super) fn row_from_record(
    record: &WindowsStorePackageInventoryRecord,
) -> AppGameInventoryEvidenceRow {
    let product_kind = product_kind_for_record(record);
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: record.source_ref.clone(),
        observed_at: record.observed_at.clone(),
        source_kind: APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE.to_string(),
        source_ref: record.source_ref.clone(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE.to_string(),
        product_kind: product_kind.clone(),
        display_label: record.display_label.clone(),
        identity_id: None,
        package_id: record.package_id.clone(),
        bundle_id: record.bundle_id.clone(),
        app_user_model_id: record.app_user_model_id.clone(),
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: record.store_id.clone(),
        catalog_ref: record.catalog_ref.clone(),
        inventory_state: record.inventory_state.clone(),
        classification_state: classification_state_for_record(record, &product_kind),
        catalog_ready_state: catalog_ready_state_for_record(record),
        capability_status: capability_status_for_record(record),
        confidence: record.confidence,
        category_candidates: category_candidates_for_record(record, &product_kind),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: record.evidence.clone(),
    }
}

fn product_kind_for_record(record: &WindowsStorePackageInventoryRecord) -> String {
    if record.category_kind.as_deref() == Some(APP_GAME_INVENTORY_CATEGORY_GAME) {
        APP_GAME_PRODUCT_NATIVE_GAME.to_string()
    } else {
        APP_GAME_PRODUCT_NATIVE_APP.to_string()
    }
}
