use std::fs::{read, remove_file, write};

use ocentra_parent_agent_core::{ActivityJournal, ActivityStore};
use ocentra_parent_agent_protocol::{constants, ActivityEventKind, ActivityObserver};

use crate::activity_capture::{record_activity_capture_to_paths, ActivityCaptureError};

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

    cleanup_paths(&journal_path, &key_path, &store_path);

    assert_eq!(status.events_ingested, 3);
    assert_eq!(status.events_stored, 3);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_PROCESS_SUBJECT_NAME));
    assert_eq!(
        summary.most_recent_kind,
        Some(ActivityEventKind::WindowFocused)
    );
    assert_eq!(
        summary.most_recent_observer,
        Some(ActivityObserver::WindowsWindow)
    );
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
