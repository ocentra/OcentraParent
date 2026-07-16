use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_EXECUTABLE_PATH_REF_PREFIX,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_ENTRY_ID_PREFIX,
    APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_TEST_DISPLAY_LABEL,
};
use ocentra_parent_agent_protocol::constants;

use super::{
    app_game_journal_sqlite_ingest::read_model::app_game_journal_sqlite_read_model,
    app_game_windows_inventory::windows_installed_inventory_rows_from_records,
    app_game_windows_registry_source::{
        live_windows_registry_inventory_journal_events_from_roots,
        live_windows_registry_inventory_journal_events_with_limit,
        live_windows_registry_inventory_records_from_roots,
    },
    app_game_windows_registry_source_support::{
        append_and_replay, cleanup_registry_root, hidden_system_component_export, registry_export,
        registry_export_path, registry_export_with_two_apps, temp_registry_root,
        write_registry_export,
    },
};

#[test]
fn registry_inventory_source_builds_inventory_rows_without_raw_paths_or_use_claims() {
    let root = temp_registry_root(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    cleanup_registry_root(&root);
    write_registry_export(registry_export_path(root.clone()), registry_export());

    let records = live_windows_registry_inventory_records_from_roots(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        constants::activity_store::DEFAULT_RECENT_LIMIT as usize,
    );
    let rows = windows_installed_inventory_rows_from_records(&records);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].display_label, APP_GAME_TEST_DISPLAY_LABEL);
    assert_eq!(
        rows[0].source_kind,
        APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD
    );
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert_eq!(rows[0].inventory_state, APP_GAME_INVENTORY_STATE_INSTALLED);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_APP
    );
    assert!(rows[0]
        .source_ref
        .starts_with(APP_GAME_INVENTORY_ENTRY_ID_PREFIX));
    assert!(rows[0]
        .executable_path_ref
        .as_ref()
        .is_some_and(|value| value.starts_with(APP_GAME_EXECUTABLE_PATH_REF_PREFIX)));
    assert!(!rows[0].executable_path_ref.as_ref().is_some_and(|value| {
        value.contains(constants::activity_store::TEST_APP_GAME_PROCESS_PATH)
    }));
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_NOT_CLAIMED);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
    assert_eq!(rows[0].running_duration_ms, 0);
    assert_eq!(rows[0].foreground_duration_ms, 0);

    cleanup_registry_root(&root);
}

#[test]
fn registry_inventory_source_respects_limit_before_journal_projection() {
    let root = temp_registry_root(constants::activity_store::TEST_CAPTURE_REPLAY_STORE_SUFFIX);
    cleanup_registry_root(&root);
    write_registry_export(
        registry_export_path(root.clone()),
        registry_export_with_two_apps(),
    );

    let events = live_windows_registry_inventory_journal_events_from_roots(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        1,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(events.len(), 1);
    cleanup_registry_root(&root);
}

#[test]
fn registry_inventory_journal_event_replays_into_sqlite_read_model() {
    let root = temp_registry_root(constants::activity_store::TEST_CAPTURE_APP_GAME_STORE_SUFFIX);
    cleanup_registry_root(&root);
    write_registry_export(registry_export_path(root.clone()), registry_export());
    let events = live_windows_registry_inventory_journal_events_from_roots(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        constants::activity_store::DEFAULT_RECENT_LIMIT as usize,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let (store, lines) = append_and_replay(&events);
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(lines.len(), 1);
    assert_eq!(model.inventory_returned, 1);
    assert_eq!(model.running_now_returned, 0);
    assert_eq!(model.foreground_now_returned, 0);
    assert_eq!(
        model.inventory_rows[0].display_label,
        APP_GAME_TEST_DISPLAY_LABEL
    );
    assert_eq!(
        model.inventory_rows[0].source_kind,
        APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD
    );
    assert_eq!(
        model.inventory_rows[0].runtime_state,
        APP_GAME_RUNTIME_NOT_CLAIMED
    );

    cleanup_registry_root(&root);
}

#[test]
fn registry_inventory_default_source_is_optional_on_unsupported_platforms() {
    let events = live_windows_registry_inventory_journal_events_with_limit(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::DEFAULT_RECENT_LIMIT as usize,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    for event in events {
        assert_eq!(event.evidence.len(), 0);
    }
}

#[test]
fn registry_inventory_source_skips_hidden_system_components() {
    let root = temp_registry_root(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_registry_root(&root);
    write_registry_export(
        registry_export_path(root.clone()),
        hidden_system_component_export(),
    );

    let records = live_windows_registry_inventory_records_from_roots(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        constants::activity_store::DEFAULT_RECENT_LIMIT as usize,
    );

    assert_eq!(records.len(), 0);
    cleanup_registry_root(&root);
}
