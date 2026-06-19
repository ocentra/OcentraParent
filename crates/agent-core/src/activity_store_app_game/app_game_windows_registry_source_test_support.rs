use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::{
    constants, journal::ActivityJournalLine, APP_GAME_TEST_DISPLAY_LABEL,
    APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE, APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE,
    APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX, APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER,
    APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION, APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
    APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE, APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE,
    APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
};

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};

pub(super) fn registry_export() -> String {
    let mut export = registry_header();
    export.push(constants::delimiter::NEWLINE);
    export.push_str(&registry_key(constants::value::EMPTY));
    export.push(constants::delimiter::NEWLINE);
    push_registry_value(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
        APP_GAME_TEST_DISPLAY_LABEL,
    );
    push_registry_value(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
        constants::activity_store::TEST_APP_GAME_PROCESS_PATH,
    );
    export
}

pub(super) fn registry_export_with_two_apps() -> String {
    let mut export = registry_export();
    export.push(constants::delimiter::NEWLINE);
    export.push_str(&registry_key(constants::activity_store::TEST_STORE_SUFFIX));
    export.push(constants::delimiter::NEWLINE);
    push_registry_value(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
        constants::activity_store::TEST_APP_GAME_PROCESS_NAME,
    );
    export
}

pub(super) fn hidden_system_component_export() -> String {
    let mut export = registry_header();
    export.push(constants::delimiter::NEWLINE);
    export.push_str(&registry_key(constants::activity_store::TEST_STORE_SUFFIX));
    export.push(constants::delimiter::NEWLINE);
    push_registry_value(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
        APP_GAME_TEST_DISPLAY_LABEL,
    );
    push_registry_dword_enabled(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE,
    );
    export
}

pub(super) fn registry_export_path(root: &Path) -> PathBuf {
    let mut path = root.to_path_buf();
    path.push(constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_STORE_SUFFIX);
    path.set_extension(APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION);
    path
}

pub(super) fn write_registry_export(path: &Path, export: String) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    }
    fs::write(path, export).expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
}

pub(super) fn append_and_replay(
    events: &[ocentra_parent_agent_protocol::ActivityEvent],
) -> (ActivityStore, Vec<ActivityJournalLine>) {
    let path = temp_journal_path();
    cleanup_journal_files(&path);
    let key = test_key();
    let mut journal =
        ActivityJournal::open(path.clone(), key.clone()).expect(constants::error::JOURNAL_OPENS);
    let mut lines = Vec::new();
    for event in events {
        lines.push(
            journal
                .append(event)
                .expect(constants::error::JOURNAL_APPENDS),
        );
    }
    let reader = ActivityJournal::open(path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    cleanup_journal_files(&path);

    assert_eq!(status.events_ingested, events.len() as u64);
    assert_eq!(status.events_stored, events.len() as u64);
    (store, lines)
}

pub(super) fn temp_registry_root(suffix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(temp_name(suffix));
    path
}

pub(super) fn cleanup_registry_root(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

fn registry_header() -> String {
    String::from(APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER)
}

fn registry_key(suffix: &str) -> String {
    let mut key = String::new();
    key.push(constants::delimiter::OPEN_BRACKET);
    key.push_str(APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE);
    key.push(constants::delimiter::BACKSLASH);
    key.push_str(APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH);
    key.push(constants::delimiter::BACKSLASH);
    if suffix.is_empty() {
        key.push_str(constants::activity_store::TEST_APP_GAME_SESSION_ID);
    } else {
        key.push_str(suffix);
    }
    key.push(constants::delimiter::CLOSE_BRACKET);
    key
}

fn push_registry_value(export: &mut String, name: &str, value: &str) {
    export.push(constants::delimiter::QUOTE);
    export.push_str(name);
    export.push(constants::delimiter::QUOTE);
    export.push(constants::delimiter::EQUALS);
    export.push(constants::delimiter::QUOTE);
    export.push_str(value);
    export.push(constants::delimiter::QUOTE);
    export.push(constants::delimiter::NEWLINE);
}

fn push_registry_dword_enabled(export: &mut String, name: &str) {
    export.push(constants::delimiter::QUOTE);
    export.push_str(name);
    export.push(constants::delimiter::QUOTE);
    export.push(constants::delimiter::EQUALS);
    export.push_str(APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX);
    export.push_str(APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE);
    export.push(constants::delimiter::NEWLINE);
}

fn temp_journal_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(temp_name(constants::activity_store::TEST_JOURNAL_SUFFIX));
    path.set_extension(constants::journal::FILE_EXTENSION);
    path
}

fn temp_name(suffix: &str) -> String {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_JOURNAL_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    name
}

fn cleanup_journal_files(path: &Path) {
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
