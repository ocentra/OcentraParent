use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_store_inventory::{
    store_app_user_model_policy_target, store_package_matches_runtime_identity,
    windows_store_inventory_rows_from_records, WindowsRuntimePackageIdentity,
    WindowsStorePackageInventoryRecord,
};

#[test]
fn store_app_row_decodes_as_first_class_package_inventory() {
    let rows = windows_store_inventory_rows_from_records(&[store_app_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].source_kind, APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE);
    assert_eq!(
        rows[0].custody_state,
        APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE
    );
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_APP
    );
    assert_eq!(
        rows[0].package_id,
        Some(APP_GAME_TEST_STORE_APP_PACKAGE_ID.to_string())
    );
    assert_eq!(
        rows[0].app_user_model_id,
        Some(APP_GAME_TEST_STORE_APP_USER_MODEL_ID.to_string())
    );
}

#[test]
fn store_game_row_decodes_with_game_category_candidate() {
    let rows = windows_store_inventory_rows_from_records(&[store_game_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_GAME);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(
        rows[0].category_candidates[0].category_kind,
        APP_GAME_INVENTORY_CATEGORY_GAME
    );
    assert_eq!(
        rows[0].catalog_ref,
        Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string())
    );
}

#[test]
fn store_package_and_runtime_process_merge_only_by_deterministic_identity() {
    let record = store_app_record();
    let matching_runtime = WindowsRuntimePackageIdentity {
        package_id: Some(APP_GAME_TEST_STORE_APP_PACKAGE_ID.to_string()),
        app_user_model_id: None,
        executable_path_ref: None,
        display_label: None,
    };
    let display_only_runtime = WindowsRuntimePackageIdentity {
        package_id: None,
        app_user_model_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        display_label: Some(APP_GAME_TEST_STORE_APP_DISPLAY_LABEL.to_string()),
    };

    assert!(store_package_matches_runtime_identity(
        &record,
        &matching_runtime
    ));
    assert!(!store_package_matches_runtime_identity(
        &record,
        &display_only_runtime
    ));
}

#[test]
fn app_user_model_id_target_can_be_handed_to_later_policy_work() {
    let target = store_app_user_model_policy_target(&store_game_record());

    assert_eq!(
        target,
        Some(APP_GAME_TEST_STORE_GAME_USER_MODEL_ID.to_string())
    );
}

#[test]
fn store_package_inventory_never_marks_use() {
    let rows =
        windows_store_inventory_rows_from_records(&[store_app_record(), store_game_record()]);

    for row in rows {
        assert_eq!(row.runtime_state, APP_GAME_RUNTIME_NOT_CLAIMED);
        assert_eq!(row.foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
        assert_eq!(row.running_duration_ms, 0);
        assert_eq!(row.foreground_duration_ms, 0);
    }
}

fn store_app_record() -> WindowsStorePackageInventoryRecord {
    WindowsStorePackageInventoryRecord {
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_ref: APP_GAME_TEST_STORE_APP_SOURCE_REF.to_string(),
        display_label: APP_GAME_TEST_STORE_APP_DISPLAY_LABEL.to_string(),
        package_id: Some(APP_GAME_TEST_STORE_APP_PACKAGE_ID.to_string()),
        bundle_id: Some(APP_GAME_TEST_STORE_APP_BUNDLE_ID.to_string()),
        app_user_model_id: Some(APP_GAME_TEST_STORE_APP_USER_MODEL_ID.to_string()),
        store_id: Some(APP_GAME_TEST_STORE_APP_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_STORE_APP_CATALOG_REF.to_string()),
        category_kind: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: 0.88,
        evidence: Vec::new(),
    }
}

fn store_game_record() -> WindowsStorePackageInventoryRecord {
    WindowsStorePackageInventoryRecord {
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_ref: APP_GAME_TEST_STORE_GAME_SOURCE_REF.to_string(),
        display_label: APP_GAME_TEST_STORE_GAME_DISPLAY_LABEL.to_string(),
        package_id: Some(APP_GAME_TEST_STORE_GAME_PACKAGE_ID.to_string()),
        bundle_id: Some(APP_GAME_TEST_STORE_GAME_BUNDLE_ID.to_string()),
        app_user_model_id: Some(APP_GAME_TEST_STORE_GAME_USER_MODEL_ID.to_string()),
        store_id: Some(APP_GAME_TEST_STORE_GAME_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string()),
        category_kind: Some(APP_GAME_INVENTORY_CATEGORY_GAME.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: 0.91,
        evidence: Vec::new(),
    }
}
