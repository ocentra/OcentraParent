use std::fs::{read, remove_file};

use ocentra_parent_agent_core::{
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_capture::freshness::{
    record_activity_capture_freshness_to_paths, ActivityCaptureFreshnessRequest,
    ActivityCaptureFreshnessStatus,
};

#[test]
fn recurring_capture_refreshes_app_game_runtime_and_optional_foreground_rows_without_content_claim()
{
    let journal_path = temp_path(
        constants::activity_store::TEST_CAPTURE_FRESHNESS_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = temp_path(
        constants::activity_store::TEST_CAPTURE_FRESHNESS_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_CAPTURE_FRESHNESS_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);

    let freshness = record_activity_capture_freshness_to_paths(ActivityCaptureFreshnessRequest {
        journal_path: &journal_path,
        key_path: &key_path,
        store_path: &store_path,
        process_limit: 1,
        network_limit: 1,
        first_observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT,
        next_observed_ats: &[constants::activity_store::TEST_SECOND_OBSERVED_AT],
        generated_at: constants::activity_store::TEST_THIRD_OBSERVED_AT,
    })
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let key_bytes = read(&key_path).expect(constants::error::JOURNAL_READS);
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    let journal = ActivityJournal::open(journal_path.clone(), JournalKey::from_bytes(key))
        .expect(constants::error::JOURNAL_OPENS);
    let lines = journal.lines().expect(constants::error::JOURNAL_READS);

    cleanup_paths(&journal_path, &key_path, &store_path);

    assert_optional_foreground_event_count(lines.len() as u64);
    assert_recurring_app_game_freshness(&freshness);
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
}

#[cfg(windows)]
fn expected_capture_event_base_count() -> u64 {
    4
}

#[cfg(not(windows))]
fn expected_capture_event_base_count() -> u64 {
    3
}

fn assert_optional_foreground_event_count(event_count: u64) {
    let min_count = expected_capture_event_base_count() * 2;
    let max_count = (expected_capture_event_base_count()
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1)
        * 2;
    assert!(event_count >= min_count && event_count <= max_count);
}

#[cfg(windows)]
fn assert_optional_latest_ingest_count(event_count: u64) {
    let min_count = expected_capture_event_base_count();
    let max_count = expected_capture_event_base_count()
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1;
    assert!(event_count >= min_count && event_count <= max_count);
}

#[cfg(windows)]
fn assert_recurring_app_game_freshness(freshness: &ActivityCaptureFreshnessStatus) {
    assert_eq!(freshness.capture_runs, 2);
    assert_optional_latest_ingest_count(freshness.latest_ingest.events_ingested);
    assert_eq!(
        freshness.app_game_generated_at,
        constants::activity_store::TEST_THIRD_OBSERVED_AT
    );
    assert_eq!(freshness.app_game_running_now_returned, 2);
    assert!(freshness.app_game_foreground_now_returned <= 2);
    assert_eq!(
        freshness.app_game_last_observed_at.as_deref(),
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT)
    );
}

#[cfg(not(windows))]
fn assert_recurring_app_game_freshness(freshness: &ActivityCaptureFreshnessStatus) {
    assert_eq!(freshness.capture_runs, 2);
    assert_eq!(
        freshness.latest_ingest.events_ingested,
        expected_capture_event_base_count()
    );
    assert_eq!(
        freshness.app_game_generated_at,
        constants::activity_store::TEST_THIRD_OBSERVED_AT
    );
    assert_eq!(freshness.app_game_running_now_returned, 0);
    assert_eq!(freshness.app_game_foreground_now_returned, 0);
    assert_eq!(freshness.app_game_last_observed_at, None);
}
