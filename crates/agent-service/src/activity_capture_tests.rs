use std::fs::{read, remove_file, write};

use ocentra_parent_agent_core::{ActivityJournal, ActivityStore};
use ocentra_parent_agent_protocol::{
    constants, ActivityEventKind, ActivityObserver, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_RUNTIME_RUNNING, APP_GAME_WINDOW_REF_PREFIX,
    APP_GAME_WINDOW_TITLE_REF_PREFIX,
};

use crate::activity_capture::{record_activity_capture_to_paths, ActivityCaptureError};

mod freshness;
mod inventory;

#[test]
fn record_process_snapshot_writes_encrypted_journal_and_sqlite_rows() {
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);

    let status = record_activity_capture_to_paths(&journal_path, &key_path, &store_path, 1, 1)
        .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let journal_bytes = read(&journal_path).expect(constants::error::JOURNAL_READS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let summary = store
        .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let app_game = store
        .app_game_service_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    cleanup_paths(&journal_path, &key_path, &store_path);

    assert_capture_event_count(status.events_ingested);
    assert_capture_event_count(status.events_stored);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_PROCESS_SUBJECT_NAME));
    assert!(matches!(
        summary.most_recent_kind,
        Some(ActivityEventKind::WindowFocused) | Some(ActivityEventKind::ProcessObserved)
    ));
    assert!(matches!(
        summary.most_recent_observer,
        Some(ActivityObserver::WindowsWindow) | Some(ActivityObserver::WindowsProcess)
    ));
    assert_app_game_capture_read_model(&app_game);
}

#[test]
fn record_process_snapshot_reuses_journal_key_for_replay() {
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);

    record_activity_capture_to_paths(&journal_path, &key_path, &store_path, 1, 1)
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
    assert_optional_foreground_event_count(lines.len() as u64);
    let process_event = journal
        .decrypt_line(&lines[0])
        .expect(constants::error::JOURNAL_DECRYPTS);
    let window_event = journal
        .decrypt_line(&lines[1])
        .expect(constants::error::JOURNAL_DECRYPTS);
    let network_event = journal
        .decrypt_line(&lines[2])
        .expect(constants::error::JOURNAL_DECRYPTS);

    cleanup_paths(&journal_path, &key_path, &store_path);

    assert_eq!(process_event.kind, ActivityEventKind::ProcessObserved);
    assert_eq!(
        process_event.source.observer,
        ActivityObserver::WindowsProcess
    );
    assert_eq!(window_event.kind, ActivityEventKind::WindowFocused);
    assert_eq!(
        window_event.source.observer,
        ActivityObserver::WindowsWindow
    );
    assert_eq!(network_event.kind, ActivityEventKind::DomainObserved);
    assert_eq!(
        network_event.source.observer,
        ActivityObserver::WindowsNetwork
    );
}

#[test]
fn record_process_snapshot_rejects_invalid_journal_key() {
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);
    write(&key_path, []).expect(constants::error::JOURNAL_APPENDS);

    let error = record_activity_capture_to_paths(&journal_path, &key_path, &store_path, 1, 1)
        .expect_err(constants::error::ACTIVITY_CAPTURE_REJECTS_INVALID_KEY);

    cleanup_paths(&journal_path, &key_path, &store_path);

    assert_eq!(error, ActivityCaptureError::InvalidKeyLength);
    assert_eq!(
        error.reason(),
        constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH
    );
}

fn temp_path(suffix: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(
    journal_path: &std::path::PathBuf,
    key_path: &std::path::PathBuf,
    store_path: &std::path::PathBuf,
) {
    let _ = remove_file(journal_path);
    let _ = remove_file(key_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    for index in 1..=3 {
        let mut rotated_path = journal_path.clone();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn assert_capture_event_count(event_count: u64) {
    let min_count = expected_capture_event_base_count();
    let max_count = expected_capture_event_base_count()
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1;
    assert!(event_count >= min_count && event_count <= max_count);
}

fn assert_optional_foreground_event_count(event_count: u64) {
    let base_count = expected_capture_event_base_count();
    let max_count = base_count
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1;
    assert!(event_count >= base_count && event_count <= max_count);
}

#[cfg(windows)]
fn expected_capture_event_base_count() -> u64 {
    4
}

#[cfg(not(windows))]
fn expected_capture_event_base_count() -> u64 {
    3
}

#[cfg(windows)]
fn assert_app_game_capture_read_model(
    model: &ocentra_parent_agent_protocol::AppGameServiceReadModel,
) {
    assert_eq!(model.running_now_returned, 1);
    assert_eq!(
        model.running_now_rows[0].runtime_state,
        APP_GAME_RUNTIME_RUNNING
    );
    assert_eq!(
        model.running_now_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        model.running_now_rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert!(model.foreground_now_returned <= 1);
    if let Some(row) = model.foreground_now_rows.first() {
        assert_eq!(row.runtime_state, APP_GAME_RUNTIME_RUNNING);
        assert_eq!(row.foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
        assert_eq!(
            row.content_knowledge_state,
            APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
        );
        if let Some(window_ref) = &row.window_ref {
            assert!(window_ref.starts_with(APP_GAME_WINDOW_REF_PREFIX));
        }
        if let Some(title_ref) = &row.window_title_ref {
            assert!(title_ref.starts_with(APP_GAME_WINDOW_TITLE_REF_PREFIX));
        }
    }
}

#[cfg(not(windows))]
fn assert_app_game_capture_read_model(
    model: &ocentra_parent_agent_protocol::AppGameServiceReadModel,
) {
    assert_eq!(model.running_now_returned, 0);
}
