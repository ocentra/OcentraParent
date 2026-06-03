use std::fs::{read, remove_file};

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, BrowserActiveProofSource, BrowserActiveTabState,
    BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
    BrowserQueryVisibilityLabel, BrowserTabEvidence, LogFieldValue, LogFields,
};

use super::{
    browser_tab_observation_event, ActivityJournal, ActivityStore, BrowserBridgeTargetObservation,
    JournalKey, JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_typed_browser_tab_read_model_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = browser_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.latest_event_id, Some(event.event_id.clone()));
    assert_eq!(
        read_model.capability_status,
        Some(BrowserCapabilityStatus::TabListOnly)
    );
    let row = &read_model.rows[0];
    assert_browser_row_matches_event(row, &event);
    assert_eq!(
        row.fresh_until,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        row.stale_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
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
    let read_model = store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_browser_row_matches_event(&read_model.rows[0], &event);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
}

#[test]
fn activity_store_counts_duplicate_browser_journal_replay_without_duplicate_rows() {
    let journal_path = temp_path(
        constants::activity_store::TEST_BROWSER_DUPLICATE_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_BROWSER_DUPLICATE_STORE_SUFFIX,
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
    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_eq!(status.duplicate_events, 1);
    assert_eq!(status.events_stored, 1);
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.latest_event_id, Some(event.event_id));
}

#[test]
fn activity_store_reconstructs_stale_degraded_browser_evidence_after_restart() {
    let journal_path = temp_path(
        constants::activity_store::TEST_BROWSER_RESTART_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_BROWSER_RESTART_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect(constants::error::JOURNAL_OPENS);
    let event = browser_event_with_status(
        BrowserCapabilityStatus::Stale,
        Some(constants::value::BROWSER_BRIDGE_STALE_SESSION),
    );
    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    {
        let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
        let status = store
            .ingest_journal(&reader)
            .expect(constants::error::ACTIVITY_STORE_INGESTS);
        assert_eq!(status.events_ingested, 1);
        assert_eq!(status.events_stored, 1);
    }

    let restarted_store =
        ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let read_model = restarted_store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.latest_event_id, Some(event.event_id));
    let row = &read_model.rows[0];
    assert_eq!(row.capability_status, BrowserCapabilityStatus::Stale);
    assert_eq!(
        row.degraded_reason.as_deref(),
        Some(constants::value::BROWSER_BRIDGE_STALE_SESSION)
    );
    assert_eq!(
        row.fresh_until,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        row.stale_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(row.active_state, BrowserActiveTabState::Unknown);
    assert_eq!(
        row.active_proof_source,
        BrowserActiveProofSource::TargetListOnly
    );
}

#[test]
fn activity_store_reports_empty_browser_evidence_without_inventing_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(read_model.capability_status, None);
}

fn browser_event() -> ActivityEvent {
    browser_event_with_status(BrowserCapabilityStatus::TabListOnly, None)
}

fn browser_event_with_status(
    capability_status: BrowserCapabilityStatus,
    degraded_reason: Option<&str>,
) -> ActivityEvent {
    browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: None,
            active_state: BrowserActiveTabState::Unknown,
            active_proof_source: BrowserActiveProofSource::TargetListOnly,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status,
            degraded_reason: degraded_reason.map(ToOwned::to_owned),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

fn assert_browser_row_matches_event(row: &BrowserTabEvidence, event: &ActivityEvent) {
    assert_eq!(
        row.browser_evidence_id,
        string_field(&event.fields, constants::field::BROWSER_EVIDENCE_ID)
            .expect(constants::error::ACTIVITY_STORE_QUERIES)
    );
    assert_eq!(
        row.source_id,
        constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS
    );
    assert_eq!(
        row.adapter_id,
        constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS
    );
    assert_eq!(row.device_id, constants::peer::LOCAL_DEV_AGENT);
    assert_eq!(row.browser_family, BrowserFamily::Edge);
    assert_eq!(row.browser_channel, BrowserChannel::Stable);
    assert_eq!(
        row.managed_browser_session_id,
        constants::browser::SESSION_ID_DEV
    );
    assert_eq!(row.profile_id, constants::browser::PROFILE_ID_DEV);
    assert_eq!(
        row.process_id,
        constants::activity_store::TEST_BROWSER_PROCESS_ID
    );
    assert_eq!(
        row.tab_id.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_TAB_ID)
    );
    assert_eq!(
        row.target_id.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_TARGET_ID)
    );
    assert_eq!(row.active_state, BrowserActiveTabState::Unknown);
    assert_eq!(
        row.active_proof_source,
        BrowserActiveProofSource::TargetListOnly
    );
    assert_eq!(row.url, constants::activity_store::TEST_BROWSER_URL);
    assert_eq!(row.origin, constants::activity_store::TEST_BROWSER_ORIGIN);
    assert_eq!(row.domain, constants::activity_store::TEST_BROWSER_DOMAIN);
    assert_eq!(
        row.title.as_deref(),
        Some(constants::activity_store::TEST_BROWSER_TITLE)
    );
    assert_eq!(row.capability_status, BrowserCapabilityStatus::TabListOnly);
    assert_eq!(row.degraded_reason, None);
    assert_eq!(row.custody_label, BrowserCustodyLabel::ChildDeviceLocal);
    assert_eq!(row.query_visibility, BrowserQueryVisibilityLabel::LiveLocal);
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
