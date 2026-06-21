use std::{fs::read, fs::remove_file, path::PathBuf};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeTargetObservation,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityIngestStatus, BrowserActiveProofSource,
    BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserEvidenceReadModel, BrowserFamily, BrowserQueryVisibilityLabel,
};

use crate::activity_capture::record_activity_events_to_paths;

#[test]
fn record_browser_events_replays_appended_journal_lines_into_sqlite_read_model() {
    let paths = browser_capture_paths();
    cleanup_paths(&paths);
    let event = browser_event();

    let status = record_activity_events_to_paths(
        &paths.journal_path,
        &paths.key_path,
        &paths.store_path,
        &[event.clone(), event.clone()],
    )
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    let journal_bytes = read(&paths.journal_path).expect(constants::error::JOURNAL_READS);
    let read_model = browser_read_model_from_store(&paths.store_path);
    let restarted = browser_read_model_from_store(&paths.store_path);
    cleanup_paths(&paths);

    assert_browser_capture_status(&status, &event);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.latest_event_id, Some(event.event_id));
    assert_browser_row_is_journal_replayed(&read_model);
    assert_eq!(
        read_model.rows[0].browser_evidence_id,
        restarted.rows[0].browser_evidence_id
    );
    assert_eq!(
        restarted.rows[0].stale_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
}

fn assert_browser_capture_status(status: &ActivityIngestStatus, event: &ActivityEvent) {
    assert_eq!(status.events_ingested, 1);
    assert_eq!(status.duplicate_events, 1);
    assert_eq!(status.events_stored, 1);
    assert_eq!(
        status.last_event_id.as_deref(),
        Some(event.event_id.as_str())
    );
}

fn assert_browser_row_is_journal_replayed(read_model: &BrowserEvidenceReadModel) {
    let row = &read_model.rows[0];
    assert_eq!(row.url, constants::activity_store::TEST_BROWSER_URL);
    assert_eq!(row.origin, constants::activity_store::TEST_BROWSER_ORIGIN);
    assert_eq!(row.domain, constants::activity_store::TEST_BROWSER_DOMAIN);
    assert_eq!(row.capability_status, BrowserCapabilityStatus::TabListOnly);
    assert_eq!(
        row.degraded_reason.as_deref(),
        Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS)
    );
    assert_eq!(row.active_state, BrowserActiveTabState::Unknown);
    assert_eq!(
        row.active_proof_source,
        BrowserActiveProofSource::TargetListOnly
    );
}

fn browser_read_model_from_store(store_path: &PathBuf) -> BrowserEvidenceReadModel {
    let store = ActivityStore::open(store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES)
}

fn browser_event() -> ActivityEvent {
    browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: Some(constants::activity_store::TEST_BROWSER_WINDOW_ID.to_string()),
            active_state: BrowserActiveTabState::Unknown,
            active_proof_source: BrowserActiveProofSource::TargetListOnly,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

struct BrowserCapturePaths {
    journal_path: PathBuf,
    key_path: PathBuf,
    store_path: PathBuf,
}

fn browser_capture_paths() -> BrowserCapturePaths {
    BrowserCapturePaths {
        journal_path: temp_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: temp_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: temp_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
    }
}

fn temp_path(suffix: &str, extension: &str) -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(paths: &BrowserCapturePaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let mut store_wal_path = paths.store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = paths.store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}
