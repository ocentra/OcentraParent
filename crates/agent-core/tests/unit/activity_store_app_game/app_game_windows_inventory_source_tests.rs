use ocentra_eventing::expect_value::ExpectValue;
use std::fmt::Display;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    test_text::TestText,
};

use super::{
    app_game_journal_sqlite_ingest::read_model::app_game_journal_sqlite_read_model,
    app_game_windows_inventory::windows_installed_inventory_rows_from_records,
    app_game_windows_inventory_source::{
        live_windows_inventory_journal_events_from_roots,
        live_windows_inventory_journal_events_with_limit,
        live_windows_inventory_records_from_roots,
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
fn live_inventory_shortcut_source_builds_inventory_rows_without_use_claims() {
    let root = temp_inventory_root(APP_GAME_TEST_LIVE_INVENTORY_SUFFIX);
    cleanup_inventory_root(&root);
    let shortcut = shortcut_path(&root, APP_GAME_TEST_SHORTCUT_FILE_NAME);
    write_shortcut(shortcut.as_ref());

    let records = live_windows_inventory_records_from_roots(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        constants::activity_store::DEFAULT_RECENT_LIMIT as usize,
    );
    let rows = windows_installed_inventory_rows_from_records(&records);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].display_label, APP_GAME_TEST_DISPLAY_LABEL);
    assert_eq!(rows[0].source_kind, APP_GAME_INVENTORY_SOURCE_SHORTCUT);
    assert_eq!(rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert!(rows[0]
        .source_ref
        .starts_with(APP_GAME_INVENTORY_ENTRY_ID_PREFIX));
    let shortcut_path = shortcut.as_ref().to_string_lossy();
    assert_ne!(rows[0].source_ref.as_str(), shortcut_path.as_ref());
    assert!(rows[0]
        .desktop_entry_id
        .as_ref()
        .is_some_and(|value| value.starts_with(APP_GAME_DESKTOP_ENTRY_ID_PREFIX)));
    assert_eq!(rows[0].executable_path_ref, None);
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_NOT_CLAIMED);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
    assert_eq!(rows[0].running_duration_ms, 0);
    assert_eq!(rows[0].foreground_duration_ms, 0);

    cleanup_inventory_root(&root);
}

#[test]
fn live_inventory_source_respects_limit_before_journal_projection() {
    let root = temp_inventory_root(APP_GAME_TEST_SHORTCUT_FILE_NAME);
    cleanup_inventory_root(&root);
    write_shortcut(shortcut_path(&root, APP_GAME_TEST_SHORTCUT_FILE_NAME).as_ref());
    write_shortcut(shortcut_path(&root, APP_GAME_TEST_SECOND_SHORTCUT_FILE_NAME).as_ref());

    let events = live_windows_inventory_journal_events_from_roots(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        std::slice::from_ref(&root.0),
        1,
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(events.len(), 1);
    cleanup_inventory_root(&root);
}

#[test]
fn live_inventory_journal_event_replays_into_sqlite_read_model() {
    let root = temp_inventory_root(APP_GAME_TEST_SECOND_SHORTCUT_FILE_NAME);
    cleanup_inventory_root(&root);
    write_shortcut(shortcut_path(&root, APP_GAME_TEST_SHORTCUT_FILE_NAME).as_ref());
    let events = live_windows_inventory_journal_events_from_roots(
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
        model.inventory_rows[0].runtime_state,
        APP_GAME_RUNTIME_NOT_CLAIMED
    );
    assert_eq!(
        model.inventory_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );

    cleanup_inventory_root(&root);
}

#[test]
fn live_inventory_default_source_is_optional_on_unsupported_platforms() {
    let events = live_windows_inventory_journal_events_with_limit(
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

fn shortcut_path(root: impl AsRef<Path>, file_name: impl Display) -> TestPath {
    let mut path = root.as_ref().to_path_buf();
    path.push(file_name.to_string());
    TestPath(path)
}

fn write_shortcut(path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect_value(constants::error::ACTIVITY_CAPTURE_RECORDS);
    }
    fs::write(path, constants::value::EMPTY)
        .expect_value(constants::error::ACTIVITY_CAPTURE_RECORDS);
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

fn temp_inventory_root(suffix: impl Display) -> TestPath {
    let mut path = std::env::temp_dir();
    path.push(temp_name(suffix).to_string());
    TestPath(path)
}

fn temp_journal_path() -> TestPath {
    let mut path = std::env::temp_dir();
    path.push(temp_name(constants::activity_store::TEST_JOURNAL_SUFFIX));
    path.set_extension(constants::journal::FILE_EXTENSION);
    TestPath(path)
}

fn temp_name(suffix: impl Display) -> TestText {
    let suffix = suffix.to_string();
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&unique_temp_token().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_str());
    TestText::from_display(name)
}

fn unique_temp_token() -> TestText {
    let value = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect_value(constants::error::ACTIVITY_CAPTURE_RECORDS)
        .as_nanos()
        .to_string();
    TestText::from_display(value)
}

fn cleanup_inventory_root(path: impl AsRef<Path>) {
    let _ = fs::remove_dir_all(path.as_ref());
}

fn cleanup_journal_files(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let _ = fs::remove_file(path);
    for index in 1..=3 {
        let mut rotated_path = path.to_path_buf();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = fs::remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([11; JOURNAL_KEY_BYTES])
}
