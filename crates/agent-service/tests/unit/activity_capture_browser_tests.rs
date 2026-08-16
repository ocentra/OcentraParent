use std::{
    fs::read,
    fs::remove_file,
    path::{Path, PathBuf},
};

use crate::test_text::TestText;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeTargetObservation,
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_service::test_support::record_activity_events_to_paths_for_test;

type TestResult = Result<(), TestText>;

#[test]
fn record_browser_events_replays_appended_journal_lines_into_sqlite_read_model() -> TestResult {
    let paths = browser_capture_paths();
    cleanup_paths(&paths);
    let event = browser_event()?;

    let status = record_activity_events_to_paths_for_test(
        &paths.journal_path,
        &paths.key_path,
        &paths.store_path,
        &[event.clone(), event.clone()],
    )
    .map_err(|error| TestText::from_display(format!("{error:?}")))?;
    let journal_bytes =
        read(&paths.journal_path).map_err(|error| TestText::from_display(format!("{error:?}")))?;
    let read_model = browser_read_model_from_store(&paths.store_path)?;
    let restarted = browser_read_model_from_store(&paths.store_path)?;
    cleanup_paths(&paths);

    assert_browser_capture_status(&status, &event);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
    assert_eq!(
        journal_bytes.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "duplicate event IDs must not create duplicate encrypted journal lines"
    );
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
    Ok(())
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

fn browser_read_model_from_store(
    store_path: impl AsRef<Path>,
) -> Result<BrowserEvidenceReadModel, TestText> {
    let store = ActivityStore::open(store_path.as_ref())
        .map_err(|error| TestText::from_display(format!("{error:?}")))?;
    store
        .browser_evidence_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .map_err(|error| TestText::from_display(format!("{error:?}")))
}

fn browser_event() -> Result<ActivityEvent, TestText> {
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
    .map_err(|error| TestText::from_display(format!("{error:?}")))
}

struct BrowserCapturePaths {
    journal_path: PathBuf,
    key_path: PathBuf,
    store_path: PathBuf,
}

fn browser_capture_paths() -> BrowserCapturePaths {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    BrowserCapturePaths {
        journal_path: build_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: build_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: build_path(
            constants::activity_store::TEST_CAPTURE_BROWSER_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
    }
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
