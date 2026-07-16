use std::fmt::Display;

use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_inventory::{
    windows_installed_inventory_rows_from_records, WindowsInstalledAppInventoryRecord,
};

#[test]
fn registry_app_record_becomes_native_app_inventory() {
    let rows = windows_installed_inventory_rows_from_records(&[registry_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].source_kind,
        APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD
    );
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_APP
    );
    assert_eq!(
        rows[0].executable_path_ref,
        Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string())
    );
}

#[test]
fn start_menu_app_and_launcher_game_records_are_detected() {
    let rows =
        windows_installed_inventory_rows_from_records(&[shortcut_record(), launcher_record()]);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].source_kind, APP_GAME_INVENTORY_SOURCE_SHORTCUT);
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert_eq!(
        rows[1].source_kind,
        APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST
    );
    assert_eq!(rows[1].product_kind, APP_GAME_PRODUCT_NATIVE_GAME);
    assert_eq!(
        rows[1].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(
        rows[1].category_candidates[0].category_kind,
        APP_GAME_INVENTORY_CATEGORY_GAME
    );
}

#[test]
fn strong_identity_deduplicates_registry_and_shortcut_records() {
    let rows =
        windows_installed_inventory_rows_from_records(&[registry_record(), shortcut_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].source_ref, APP_GAME_TEST_REGISTRY_SOURCE_REF);
}

#[test]
fn matching_display_label_without_identity_does_not_merge() {
    let rows = windows_installed_inventory_rows_from_records(&[
        display_only_unknown_record(APP_GAME_TEST_UNKNOWN_SOURCE_REF),
        display_only_unknown_record(APP_GAME_TEST_SECOND_UNKNOWN_SOURCE_REF),
    ]);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].display_label, rows[1].display_label);
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
}

#[test]
fn installed_inventory_adapter_never_marks_use() {
    let rows =
        windows_installed_inventory_rows_from_records(&[registry_record(), launcher_record()]);

    for row in rows {
        assert_eq!(row.runtime_state, APP_GAME_RUNTIME_NOT_CLAIMED);
        assert_eq!(row.foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
        assert_eq!(row.running_duration_ms, 0);
        assert_eq!(row.foreground_duration_ms, 0);
    }
}

fn registry_record() -> WindowsInstalledAppInventoryRecord {
    WindowsInstalledAppInventoryRecord {
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD.to_string(),
        source_ref: APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: 0.82,
        evidence: Vec::new(),
    }
}

fn shortcut_record() -> WindowsInstalledAppInventoryRecord {
    WindowsInstalledAppInventoryRecord {
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_SHORTCUT.to_string(),
        source_ref: APP_GAME_TEST_SHORTCUT_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: Some(APP_GAME_TEST_APP_USER_MODEL_ID.to_string()),
        desktop_entry_id: Some(APP_GAME_TEST_DESKTOP_ENTRY_ID.to_string()),
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: 0.8,
        evidence: Vec::new(),
    }
}

fn launcher_record() -> WindowsInstalledAppInventoryRecord {
    WindowsInstalledAppInventoryRecord {
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST.to_string(),
        source_ref: APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LAUNCHER_MANIFEST.to_string(),
        display_label: APP_GAME_TEST_GAME_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: Some(APP_GAME_TEST_SECOND_SHORTCUT_SOURCE_REF.to_string()),
        executable_path_ref: Some(APP_GAME_TEST_SECOND_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        launcher_app_id: Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string()),
        launcher_manifest_id: Some(APP_GAME_TEST_LAUNCHER_MANIFEST_ID.to_string()),
        store_id: Some(APP_GAME_TEST_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: 0.96,
        evidence: Vec::new(),
    }
}

fn display_only_unknown_record(source_ref: impl Display) -> WindowsInstalledAppInventoryRecord {
    WindowsInstalledAppInventoryRecord {
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_UNKNOWN.to_string(),
        source_ref: source_ref.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_UNKNOWN.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_DETECTABLE.to_string(),
        confidence: 0.3,
        evidence: Vec::new(),
    }
}
