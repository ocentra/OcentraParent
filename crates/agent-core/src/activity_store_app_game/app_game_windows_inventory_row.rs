use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST, APP_GAME_INVENTORY_SOURCE_UNKNOWN,
    APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
};

use super::app_game_windows_inventory_state::{
    capability_status_for_record, catalog_ready_state_for_record, category_candidates_for_record,
    classification_state_for_record,
};
use super::WindowsInstalledAppInventoryRecord;

pub(super) fn row_from_record(
    record: &WindowsInstalledAppInventoryRecord,
) -> AppGameInventoryEvidenceRow {
    let product_kind = product_kind_for_record(record);
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: record.source_ref.clone(),
        observed_at: record.observed_at.clone(),
        source_kind: record.source_kind.clone(),
        source_ref: record.source_ref.clone(),
        custody_state: record.custody_state.clone(),
        product_kind: product_kind.clone(),
        display_label: record.display_label.clone(),
        identity_id: record.identity_id.clone(),
        package_id: record.package_id.clone(),
        bundle_id: record.bundle_id.clone(),
        app_user_model_id: record.app_user_model_id.clone(),
        desktop_entry_id: record.desktop_entry_id.clone(),
        executable_path_ref: record.executable_path_ref.clone(),
        launcher_ref: record.launcher_ref.clone(),
        launcher_app_id: record.launcher_app_id.clone(),
        launcher_manifest_id: record.launcher_manifest_id.clone(),
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

fn product_kind_for_record(record: &WindowsInstalledAppInventoryRecord) -> String {
    if record.source_kind == APP_GAME_INVENTORY_SOURCE_UNKNOWN {
        APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE.to_string()
    } else if record.source_kind == APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST
        || record.launcher_app_id.is_some()
        || record.launcher_manifest_id.is_some()
    {
        APP_GAME_PRODUCT_NATIVE_GAME.to_string()
    } else if record.launcher_ref.is_some() {
        APP_GAME_PRODUCT_LAUNCHER.to_string()
    } else {
        APP_GAME_PRODUCT_NATIVE_APP.to_string()
    }
}
