use std::{
    fs::{create_dir_all, read, remove_dir_all, remove_file, write},
    path::{Path, PathBuf},
};

use ocentra_parent_agent_core::ActivityJournal;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, AppGameServiceReadModel,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_SOURCE_SHORTCUT,
    APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_TEST_LIVE_INVENTORY_SUFFIX, APP_GAME_TEST_SHORTCUT_FILE_NAME,
    APP_GAME_TEST_STORE_APP_DISPLAY_LABEL, APP_GAME_TEST_STORE_APP_PACKAGE_ID,
    APP_GAME_TEST_STORE_PACKAGE_MANIFEST_USER_MODEL_ID, APP_GAME_TEST_STORE_PACKAGE_MANIFEST_XML,
    APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
};

use crate::activity_capture::capture_events::{
    record_activity_capture_to_paths_at_with_inventory_roots,
    record_activity_capture_to_paths_at_with_store_package_roots,
};

#[test]
fn record_capture_with_inventory_root_writes_inventory_journal_and_sqlite_rows() {
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let inventory_root = temp_inventory_root();
    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&inventory_root);
    create_dir_all(&inventory_root).expect(constants::error::JOURNAL_APPENDS);
    let mut shortcut_path = inventory_root.clone();
    shortcut_path.push(APP_GAME_TEST_SHORTCUT_FILE_NAME);
    write(&shortcut_path, []).expect(constants::error::JOURNAL_APPENDS);

    let status = record_activity_capture_to_paths_at_with_inventory_roots(
        &journal_path,
        &key_path,
        &store_path,
        1,
        1,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&inventory_root),
    )
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let events = decrypted_events(&journal_path, &key_path);
    let app_game = app_game_read_model(&store_path);

    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&inventory_root);

    assert_eq!(status.events_ingested, status.events_stored);
    assert!(events.iter().any(
        |event| event.kind == ActivityEventKind::DeviceIdleStateObserved
            && event.subject.subject_id == APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID
    ));
    assert_eq!(app_game.inventory_returned, 1);
    assert_eq!(
        app_game.inventory_rows[0].source_kind,
        APP_GAME_INVENTORY_SOURCE_SHORTCUT
    );
    assert_eq!(
        app_game.inventory_rows[0].inventory_state,
        APP_GAME_INVENTORY_STATE_INSTALLED
    );
    assert_eq!(
        app_game.inventory_rows[0].runtime_state,
        APP_GAME_RUNTIME_NOT_CLAIMED
    );
    assert_eq!(
        app_game.inventory_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
}

#[test]
fn record_capture_with_store_package_root_writes_inventory_journal_and_sqlite_rows() {
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_package_root = temp_store_package_root();
    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&store_package_root);
    write_store_package_manifest(&store_package_root);

    let status = record_activity_capture_to_paths_at_with_store_package_roots(
        &journal_path,
        &key_path,
        &store_path,
        1,
        1,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&store_package_root),
    )
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let events = decrypted_events(&journal_path, &key_path);
    let app_game = app_game_read_model(&store_path);

    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&store_package_root);

    assert_eq!(status.events_ingested, status.events_stored);
    assert!(events.iter().any(
        |event| event.kind == ActivityEventKind::DeviceIdleStateObserved
            && event.subject.subject_id == APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID
    ));
    assert_eq!(app_game.inventory_returned, 1);
    assert_eq!(
        app_game.inventory_rows[0].display_label,
        APP_GAME_TEST_STORE_APP_DISPLAY_LABEL
    );
    assert_eq!(
        app_game.inventory_rows[0].source_kind,
        APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE
    );
    assert_eq!(
        app_game.inventory_rows[0].package_id.as_deref(),
        Some(APP_GAME_TEST_STORE_APP_PACKAGE_ID)
    );
    assert_eq!(
        app_game.inventory_rows[0].app_user_model_id.as_deref(),
        Some(APP_GAME_TEST_STORE_PACKAGE_MANIFEST_USER_MODEL_ID)
    );
    assert_eq!(
        app_game.inventory_rows[0].runtime_state,
        APP_GAME_RUNTIME_NOT_CLAIMED
    );
    assert_eq!(
        app_game.inventory_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
}

fn temp_path(suffix: &str, extension: &str) -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn temp_inventory_root() -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(APP_GAME_TEST_LIVE_INVENTORY_SUFFIX);

    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn temp_store_package_root() -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::activity_store::TEST_APP_GAME_STORE_PACKAGE_MANIFEST_SUFFIX);

    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn decrypted_events(journal_path: &Path, key_path: &Path) -> Vec<ActivityEvent> {
    let key_bytes = read(key_path).expect(constants::error::JOURNAL_READS);
    let mut key = [0; ocentra_parent_agent_core::JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    let journal = ActivityJournal::open(
        journal_path.to_path_buf(),
        ocentra_parent_agent_core::JournalKey::from_bytes(key),
    )
    .expect(constants::error::JOURNAL_OPENS);
    journal
        .lines()
        .expect(constants::error::JOURNAL_READS)
        .iter()
        .map(|line| journal.decrypt_line(line))
        .collect::<Result<Vec<_>, _>>()
        .expect(constants::error::JOURNAL_DECRYPTS)
}

fn app_game_read_model(store_path: &Path) -> AppGameServiceReadModel {
    let store = ocentra_parent_agent_core::ActivityStore::open(store_path)
        .expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .app_game_service_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES)
}

fn write_store_package_manifest(root: &Path) {
    create_dir_all(root).expect(constants::error::JOURNAL_APPENDS);
    let mut path = root.to_path_buf();
    path.push(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
    write(path, APP_GAME_TEST_STORE_PACKAGE_MANIFEST_XML).expect(constants::error::JOURNAL_APPENDS);
}

fn cleanup_paths(journal_path: &Path, key_path: &Path, store_path: &Path) {
    let _ = remove_file(journal_path);
    let _ = remove_file(key_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn cleanup_inventory_root(path: &Path) {
    let _ = remove_dir_all(path);
}
