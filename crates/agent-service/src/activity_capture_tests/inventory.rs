use std::{
    fs::{create_dir_all, read, remove_dir_all, remove_file, write},
    path::PathBuf,
};

use ocentra_parent_agent_core::ActivityJournal;
use ocentra_parent_agent_protocol::{
    constants, ActivityEventKind, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_SOURCE_SHORTCUT, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_TEST_LIVE_INVENTORY_SUFFIX, APP_GAME_TEST_SHORTCUT_FILE_NAME,
};

use crate::activity_capture::capture_events::record_activity_capture_to_paths_at_with_inventory_roots;

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
        &[inventory_root.clone()],
    )
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let key_bytes = read(&key_path).expect(constants::error::JOURNAL_READS);
    let mut key = [0; ocentra_parent_agent_core::JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    let journal = ActivityJournal::open(
        journal_path.clone(),
        ocentra_parent_agent_core::JournalKey::from_bytes(key),
    )
    .expect(constants::error::JOURNAL_OPENS);
    let lines = journal.lines().expect(constants::error::JOURNAL_READS);
    let events = lines
        .iter()
        .map(|line| journal.decrypt_line(line))
        .collect::<Result<Vec<_>, _>>()
        .expect(constants::error::JOURNAL_DECRYPTS);
    let store = ocentra_parent_agent_core::ActivityStore::open(&store_path)
        .expect(constants::error::ACTIVITY_STORE_OPENS);
    let app_game = store
        .app_game_service_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

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

fn cleanup_paths(journal_path: &PathBuf, key_path: &PathBuf, store_path: &PathBuf) {
    let _ = remove_file(journal_path);
    let _ = remove_file(key_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn cleanup_inventory_root(path: &PathBuf) {
    let _ = remove_dir_all(path);
}
