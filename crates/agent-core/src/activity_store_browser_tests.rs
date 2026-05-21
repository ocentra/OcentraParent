use std::fs::{read, remove_file};

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily, LogFieldValue, LogFields,
};

use super::{
    browser_tab_observation_event, ActivityJournal, ActivityStore, BrowserBridgeTargetObservation,
    JournalKey, JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_latest_browser_evidence_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let summary = store
        .browser_recent_summary()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.returned, 1);
    assert_eq!(summary.latest_event_id, Some(event.event_id));
    assert_eq!(
        summary.browser_family,
        Some(constants::browser::FAMILY_EDGE.to_string())
    );
    assert_eq!(
        summary.active_state,
        Some(constants::browser::ACTIVE_STATE_UNKNOWN.to_string())
    );
    assert_eq!(
        summary.url,
        Some(constants::activity_store::TEST_BROWSER_URL.to_string())
    );
    assert_eq!(
        summary.domain,
        Some(constants::activity_store::TEST_BROWSER_DOMAIN.to_string())
    );
}

#[test]
fn activity_store_replays_browser_evidence_from_encrypted_journal() {
    let journal_path = temp_path(
        constants::activity_store::TEST_BROWSER_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_BROWSER_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect(constants::error::JOURNAL_OPENS);
    let event = browser_event();
    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let journal_bytes = read(&journal_path).expect(constants::error::JOURNAL_READS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let summary = store
        .browser_recent_summary()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_eq!(
        summary.browser_evidence_id,
        string_field(&event.fields, constants::field::BROWSER_EVIDENCE_ID)
    );
    assert_eq!(
        summary.url,
        Some(constants::activity_store::TEST_BROWSER_URL.to_string())
    );
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
}

#[test]
fn activity_store_reports_empty_browser_evidence_without_inventing_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let summary = store
        .browser_recent_summary()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.returned, 0);
    assert_eq!(summary.latest_event_id, None);
    assert_eq!(summary.url, None);
    assert_eq!(summary.capability_status, None);
}

fn browser_event() -> ActivityEvent {
    browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: 4242,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: None,
            active_state: BrowserActiveTabState::Unknown,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
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

fn cleanup_paths(journal_path: &std::path::PathBuf, store_path: &std::path::PathBuf) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([9; JOURNAL_KEY_BYTES])
}
