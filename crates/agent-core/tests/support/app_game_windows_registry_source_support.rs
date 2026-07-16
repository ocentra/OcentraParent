use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::journal::ActivityJournal;
use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
    APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE, APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX,
    APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER, APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION,
    APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE, APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE,
    APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE, APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;

use crate::test_text::TestText;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct TestPath(pub(crate) PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl From<&TestPath> for TestPath {
    fn from(value: &TestPath) -> Self {
        value.clone()
    }
}

pub fn registry_export() -> TestText {
    let mut export = registry_header();
    export.0.push(constants::delimiter::NEWLINE);
    export
        .0
        .push_str(registry_key(constants::value::EMPTY).0.as_str());
    export.0.push(constants::delimiter::NEWLINE);
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

pub fn registry_export_with_two_apps() -> TestText {
    let mut export = registry_export();
    export.0.push(constants::delimiter::NEWLINE);
    export.0.push_str(
        registry_key(constants::activity_store::TEST_STORE_SUFFIX)
            .0
            .as_str(),
    );
    export.0.push(constants::delimiter::NEWLINE);
    push_registry_value(
        &mut export,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
        constants::activity_store::TEST_APP_GAME_PROCESS_NAME,
    );
    export
}

pub fn hidden_system_component_export() -> TestText {
    let mut export = registry_header();
    export.0.push(constants::delimiter::NEWLINE);
    export.0.push_str(
        registry_key(constants::activity_store::TEST_STORE_SUFFIX)
            .0
            .as_str(),
    );
    export.0.push(constants::delimiter::NEWLINE);
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

pub fn registry_export_path(root: impl Into<TestPath>) -> TestPath {
    let mut path = root.into().0;
    path.push(constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_STORE_SUFFIX);
    path.set_extension(APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION);
    TestPath(path)
}

pub fn write_registry_export(path: impl Into<TestPath>, export: impl Display) {
    let export = TestText::from_display(export);
    let path = path.into().0;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect_value(constants::error::ACTIVITY_CAPTURE_RECORDS);
    }
    fs::write(&path, export.0.as_str()).expect_value(constants::error::ACTIVITY_CAPTURE_RECORDS);
}

pub fn append_and_replay(events: &[ActivityEvent]) -> (ActivityStore, Vec<ActivityJournalLine>) {
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

pub fn temp_registry_root(suffix: impl Display) -> TestPath {
    let mut path = std::env::temp_dir();
    path.push(temp_name(suffix));
    TestPath(path)
}

pub fn cleanup_registry_root(path: impl Into<TestPath>) {
    let _ = fs::remove_dir_all(path.into().0);
}

fn registry_header() -> TestText {
    TestText::from_display(APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER)
}

fn registry_key(suffix: impl Display) -> TestText {
    let suffix = TestText::from_display(suffix);
    let mut key = String::new();
    key.push(constants::delimiter::OPEN_BRACKET);
    key.push_str(APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE);
    key.push(constants::delimiter::BACKSLASH);
    key.push_str(APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH);
    key.push(constants::delimiter::BACKSLASH);
    if suffix.is_empty() {
        key.push_str(constants::activity_store::TEST_APP_GAME_SESSION_ID);
    } else {
        key.push_str(suffix.0.as_str());
    }
    key.push(constants::delimiter::CLOSE_BRACKET);
    TestText::from_display(key)
}

fn push_registry_value(export: &mut TestText, name: impl Display, value: impl Display) {
    let name = TestText::from_display(name);
    let value = TestText::from_display(value);
    export.0.push(constants::delimiter::QUOTE);
    export.0.push_str(name.0.as_str());
    export.0.push(constants::delimiter::QUOTE);
    export.0.push(constants::delimiter::EQUALS);
    export.0.push(constants::delimiter::QUOTE);
    export.0.push_str(value.0.as_str());
    export.0.push(constants::delimiter::QUOTE);
    export.0.push(constants::delimiter::NEWLINE);
}

fn push_registry_dword_enabled(export: &mut TestText, name: impl Display) {
    let name = TestText::from_display(name);
    export.0.push(constants::delimiter::QUOTE);
    export.0.push_str(name.0.as_str());
    export.0.push(constants::delimiter::QUOTE);
    export.0.push(constants::delimiter::EQUALS);
    export.0.push_str(APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX);
    export
        .0
        .push_str(APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE);
    export.0.push(constants::delimiter::NEWLINE);
}

fn temp_journal_path() -> TestPath {
    let mut path = std::env::temp_dir();
    path.push(temp_name(constants::activity_store::TEST_JOURNAL_SUFFIX));
    path.set_extension(constants::journal::FILE_EXTENSION);
    TestPath(path)
}

fn temp_name(suffix: impl Display) -> TestText {
    let suffix = TestText::from_display(suffix);
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_JOURNAL_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.0.as_str());
    TestText::from_display(name)
}

fn cleanup_journal_files(path: impl Into<TestPath>) {
    let path = path.into().0;
    let _ = fs::remove_file(&path);
    for index in 1..=3 {
        let mut rotated_path = path.clone();
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
