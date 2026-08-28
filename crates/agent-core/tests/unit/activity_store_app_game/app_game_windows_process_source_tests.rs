use ocentra_eventing::expect_value::ExpectValue;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::activity::{ActivityEvent, ActivityEventKind};
use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    process_capture::process_snapshot_events_from_system,
};

use super::{
    app_game_journal_sqlite_ingest::read_model::app_game_journal_sqlite_read_model,
    app_game_windows_process_runtime::windows_process_runtime_rows_from_records,
    app_game_windows_process_source::{
        live_windows_process_snapshot_journal_event_for_pid,
        live_windows_process_snapshot_record_for_pid, live_windows_process_snapshot_records,
    },
};

#[derive(Clone)]
struct TestPath(PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[test]
fn live_process_snapshot_reads_current_process_without_foreground_claim() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let current_pid = std::process::id();
    let record = live_windows_process_snapshot_record_for_pid(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        current_pid,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let rows = windows_process_runtime_rows_from_records(&[record]);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].process_id, u64::from(current_pid));
    assert_eq!(
        rows[0].observed_at,
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    );
    assert_eq!(
        rows[0].observation_mode,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert_eq!(rows[0].catalog_ready_state, APP_GAME_CATALOG_UNAVAILABLE);
    assert_eq!(
        rows[0].capability_status,
        APP_GAME_CAPABILITY_STATUS_AVAILABLE
    );
    assert_eq!(rows[0].inventory_entry_id, None);
    assert_eq!(rows[0].launcher_ref, None);
    assert_eq!(rows[0].catalog_ref, None);
    assert_eq!(rows[0].evidence.len(), 0);
}

#[test]
fn live_process_snapshot_uses_opaque_path_refs_when_executable_is_visible() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let current_pid = std::process::id();
    let record = live_windows_process_snapshot_record_for_pid(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        current_pid,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let Ok(current_exe) = std::env::current_exe() else {
        return;
    };
    let Some(path_ref) = record.executable_path_ref else {
        return;
    };

    assert!(path_ref.starts_with(APP_GAME_EXECUTABLE_PATH_REF_PREFIX));
    assert_ne!(path_ref.as_str(), current_exe.to_string_lossy());
}

#[test]
fn live_process_snapshot_collection_contains_current_process_once() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let current_pid = u64::from(std::process::id());
    let records =
        live_windows_process_snapshot_records(constants::activity_store::TEST_FIRST_OBSERVED_AT);
    let current_process_count = records
        .iter()
        .filter(|record| record.process_id == current_pid)
        .count();

    assert_eq!(current_process_count, 1);
}

#[test]
fn live_process_snapshot_targeted_missing_pid_returns_no_record() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let record = live_windows_process_snapshot_record_for_pid(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        u32::MAX,
    );

    assert!(record.is_none());
}

#[test]
fn shared_process_snapshot_preserves_generation_across_runtime_captures() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let system = super::app_game_windows_process_source::live_windows_process_snapshot_system();
    let current_pid = u64::from(std::process::id());
    let generic_events = process_snapshot_events_from_system(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        usize::MAX,
        &system,
    );
    let app_game_events =
        super::live_windows_process_and_launcher_snapshot_journal_events_from_system(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            usize::MAX,
            &system,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    let generic_event = generic_events
        .iter()
        .find(|event| {
            event.kind == ActivityEventKind::ProcessObserved && has_process_id(event, current_pid)
        })
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let app_game_event = app_game_events
        .iter()
        .find(|event| {
            event.source.source_id == APP_GAME_JOURNAL_SOURCE_ID
                && event.kind == ActivityEventKind::ProcessObserved
                && has_process_id(event, current_pid)
        })
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        generic_event.subject.subject_id,
        app_game_event.subject.subject_id
    );
    assert_ne!(
        generic_event.subject.subject_id,
        format!(
            "{}{}",
            constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX,
            current_pid
        )
    );
}

#[test]
fn live_process_snapshot_journal_event_replays_into_sqlite_read_model() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let current_pid = std::process::id();
    let event = live_windows_process_snapshot_journal_event_for_pid(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        current_pid,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let (store, lines) = append_and_replay(&[event]);
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(lines.len(), 1);
    assert_eq!(model.running_now_returned, 1);
    assert_eq!(model.foreground_now_returned, 0);
    assert_eq!(model.running_now_rows[0].process_id, u64::from(current_pid));
    assert_eq!(
        model.running_now_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        model.running_now_rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
}

#[test]
fn live_process_snapshot_journal_event_ids_change_per_observation() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let current_pid = std::process::id();
    let first_event = live_windows_process_snapshot_journal_event_for_pid(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        current_pid,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let second_event = live_windows_process_snapshot_journal_event_for_pid(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        current_pid,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_ne!(first_event.event_id, second_event.event_id);
    assert_eq!(
        first_event.subject.subject_id,
        second_event.subject.subject_id
    );
}

fn append_and_replay(events: &[ActivityEvent]) -> (ActivityStore, Vec<ActivityJournalLine>) {
    let path = temp_journal_path();
    cleanup_journal_files(&path);
    let key = test_key();
    let mut journal = ActivityJournal::open(path.0.clone(), key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let mut lines = Vec::new();
    for event in events {
        lines.push(
            journal
                .append(event)
                .expect_value(constants::error::JOURNAL_APPENDS),
        );
    }
    let reader =
        ActivityJournal::open(path.0.clone(), key).expect_value(constants::error::JOURNAL_OPENS);
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .ingest_journal(&reader)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    cleanup_journal_files(&path);

    assert_eq!(status.events_ingested, events.len() as u64);
    assert_eq!(status.events_stored, events.len() as u64);
    (store, lines)
}

fn temp_journal_path() -> TestPath {
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::journal::TEST_LIVE_PROCESS_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(APP_GAME_TEST_RUNTIME_EVIDENCE_ID);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::journal::FILE_EXTENSION);
    TestPath(path)
}

fn cleanup_journal_files(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let _ = remove_file(path);
    for index in 1..=3 {
        let mut rotated_path = path.to_path_buf();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([9; JOURNAL_KEY_BYTES])
}

fn has_process_id(event: &ActivityEvent, process_id: u64) -> bool {
    matches!(
        event.fields.get(constants::field::PID),
        Some(LogFieldValue::Number(value)) if *value as u64 == process_id
    )
}
