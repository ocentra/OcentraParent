use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use std::fmt::Display;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};

use super::{
    app_game_journal_sqlite_ingest::read_model::app_game_journal_sqlite_read_model,
    app_game_windows_foreground::windows_foreground_rows_from_records,
    app_game_windows_foreground_source::{
        live_windows_foreground_window_journal_event,
        live_windows_foreground_window_journal_event_from_snapshot,
        live_windows_foreground_window_record, live_windows_foreground_window_record_from_snapshot,
        live_windows_foreground_window_record_from_system, LiveWindowsForegroundWindowSnapshot,
    },
};

use crate::process_capture::process_snapshot_events_from_system;

#[derive(Clone)]
struct TestPath(PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[test]
fn live_foreground_snapshot_uses_opaque_window_refs_without_title_content() {
    let record = live_windows_foreground_window_record_from_snapshot(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &foreground_snapshot(),
    );
    let rows = windows_foreground_rows_from_records(&[record]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].observed_at,
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    );
    assert_eq!(
        rows[0].observation_mode,
        APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert_eq!(
        rows[0].capability_status,
        APP_GAME_CAPABILITY_STATUS_AVAILABLE
    );
    assert_eq!(
        rows[0].content_knowledge_state,
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
    assert!(rows[0]
        .window_ref
        .as_ref()
        .is_some_and(|value| value.starts_with(APP_GAME_WINDOW_REF_PREFIX)));
    assert_ne!(
        rows[0].window_ref.as_deref(),
        Some(constants::activity_store::TEST_WINDOW_ID)
    );
    assert!(rows[0]
        .window_title_ref
        .as_ref()
        .is_some_and(|value| value.starts_with(APP_GAME_WINDOW_TITLE_REF_PREFIX)));
    assert_ne!(
        rows[0].window_title_ref.as_deref(),
        Some(constants::activity_store::TEST_WINDOW_TITLE)
    );
}

#[test]
fn live_foreground_source_fails_closed_without_process_generation() {
    let system = sysinfo::System::new();
    let record = live_windows_foreground_window_record_from_system(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &system,
    );

    assert!(record.is_none());
}

#[test]
fn live_foreground_source_joins_shared_process_generation() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let system = super::app_game_windows_process_source::live_windows_process_snapshot_system();
    let Some(record) = live_windows_foreground_window_record_from_system(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &system,
    ) else {
        return;
    };
    let process_id = record.process_id;
    let process_identity = record.process_identity.clone();
    let rows = windows_foreground_rows_from_records(&[record]);
    let process_event = process_snapshot_events_from_system(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        usize::MAX,
        &system,
    )
    .into_iter()
    .find(|event| has_process_id(event, process_id))
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        process_identity.as_deref(),
        Some(process_event.subject.subject_id.as_str())
    );
    assert_eq!(rows[0].process_identity, process_event.subject.subject_id);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
}

#[test]
fn live_foreground_snapshot_omits_empty_title_without_content_capture() {
    let mut snapshot = foreground_snapshot();
    snapshot.window_title = String::new();
    let event = live_windows_foreground_window_journal_event_from_snapshot(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &snapshot,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let (store, _) = append_and_replay(&[event], APP_GAME_TEST_FOREGROUND_PERMISSION_EVIDENCE_ID);
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(model.foreground_now_returned, 1);
    assert_eq!(model.foreground_now_rows[0].window_title_ref, None);
    assert_eq!(
        model.foreground_now_rows[0].title_capture_state,
        APP_GAME_TITLE_CAPTURE_TITLE_OMITTED
    );
    assert_eq!(
        model.foreground_now_rows[0].content_knowledge_state,
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
}

#[test]
fn live_foreground_snapshot_journal_event_replays_into_sqlite_read_model() {
    let event = live_windows_foreground_window_journal_event_from_snapshot(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &foreground_snapshot(),
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let (store, lines) = append_and_replay(&[event], APP_GAME_TEST_FOREGROUND_CLOSED_EVIDENCE_ID);
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(lines.len(), 1);
    assert_eq!(model.running_now_returned, 0);
    assert_eq!(model.foreground_now_returned, 1);
    assert_eq!(
        model.foreground_now_rows[0].foreground_state,
        APP_GAME_FOREGROUND_FOREGROUND
    );
    assert_eq!(
        model.foreground_now_rows[0].runtime_state,
        APP_GAME_RUNTIME_RUNNING
    );
    assert_eq!(
        model.foreground_now_rows[0].title_capture_state,
        APP_GAME_TITLE_CAPTURE_TITLE_REF
    );
}

#[test]
fn live_foreground_adapter_smoke_keeps_unavailable_platform_optional() {
    let record =
        live_windows_foreground_window_record(constants::activity_store::TEST_FIRST_OBSERVED_AT);
    let event = live_windows_foreground_window_journal_event(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    if let Some(record) = record {
        let rows = windows_foreground_rows_from_records(&[record]);
        assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
        assert_eq!(
            rows[0].content_knowledge_state,
            APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
        );
    } else {
        assert!(event.is_none());
    }
}

fn foreground_snapshot() -> LiveWindowsForegroundWindowSnapshot {
    LiveWindowsForegroundWindowSnapshot {
        process_id: u64::from(constants::activity_store::TEST_BROWSER_PROCESS_ID),
        process_name: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        window_id: constants::activity_store::TEST_WINDOW_ID.to_string(),
        window_title: constants::activity_store::TEST_WINDOW_TITLE.to_string(),
    }
}

fn append_and_replay(
    events: &[ActivityEvent],
    suffix: impl Display,
) -> (ActivityStore, Vec<ActivityJournalLine>) {
    let path = temp_journal_path(suffix);
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

fn temp_journal_path(suffix: impl Display) -> TestPath {
    let suffix = suffix.to_string();
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::journal::TEST_LIVE_FOREGROUND_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_str());

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
    JournalKey::from_bytes([10; JOURNAL_KEY_BYTES])
}

fn has_process_id(event: &ActivityEvent, process_id: u64) -> bool {
    matches!(
        event.fields.get(constants::field::PID),
        Some(LogFieldValue::Number(value)) if *value as u64 == process_id
    )
}
