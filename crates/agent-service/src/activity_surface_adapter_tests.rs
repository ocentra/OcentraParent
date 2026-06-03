use std::{
    fs::{remove_dir_all, remove_file, write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_core::{
    browser_tab_observation_event, ActivityStore, BrowserBridgeTargetObservation,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivityReadModelState,
    ActivityRecentSummary, ActivityReportFrequency, ActivityReportRequest, ActivitySource,
    ActivitySubject, ActivitySubjectKind, ActivitySurfaceRequest, ActivitySurfaceScope,
    ActivitySurfaceScopeKind, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily, LogFieldValue, LogFields, ACTIVITY_QUERY_SCHEMA_VERSION,
    ACTIVITY_SCHEMA_VERSION, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{
    activity_surface_read_models::{app_use_read_model, browser_read_model, network_read_model},
    activity_surface_report::report_document,
    activity_surface_report_store::{history_list_from_dir, save_report_document_to_dir},
    activity_surface_store::{
        load_browser_model_from_path, load_recent_summary_from_path, local_store_snapshot_from_path,
    },
};

mod activity_surface_report_command_tests;

static TEMP_PATH_COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::test]
async fn activity_surface_report_uses_real_activity_store_snapshot() {
    let store_path = temp_store_path();
    cleanup_store(&store_path);
    write_process_event(&store_path);

    let snapshot = local_store_snapshot_from_path(store_path.clone())
        .await
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let report = report_document(report_request(), Some(snapshot), Vec::new());
    let report_dir = temp_report_dir();
    cleanup_report_dir(&report_dir);
    let saved = save_report_document_to_dir(report.clone(), report_dir.clone());
    let history = history_list_from_dir(surface_request(), report_dir.clone());

    cleanup_store(&store_path);
    cleanup_report_dir(&report_dir);

    assert_eq!(report.source_states[0].state, ActivityReadModelState::Ready);
    let draft_metadata = report
        .saved_metadata
        .clone()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        draft_metadata.saved_state,
        ocentra_parent_agent_protocol::ActivitySavedReportState::Draft
    );
    assert_eq!(draft_metadata.saved_at, None);
    assert_eq!(report.sections.len(), 6);
    assert_eq!(report.sections[0].state, ActivityReadModelState::Ready);
    assert_eq!(report.sections[2].state, ActivityReadModelState::Ready);
    assert_eq!(report.sections[3].state, ActivityReadModelState::Empty);
    let metadata = saved
        .saved_metadata
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        metadata.saved_state,
        ocentra_parent_agent_protocol::ActivitySavedReportState::Saved
    );
    assert_eq!(
        metadata.storage_reason,
        Some(constants::activity_surface::SUMMARY_STORAGE_SAVED.to_string())
    );
    assert_eq!(history.state, ActivityReadModelState::Ready);
    assert_eq!(
        history.storage_state,
        ocentra_parent_agent_protocol::ActivitySavedReportState::Saved
    );
    assert_eq!(history.reports[0].source_state_summary.ready_sources, 1);
    assert_eq!(history.reports[0].parsed_report.report_id, report.report_id);
}

#[tokio::test]
async fn activity_tab_read_models_map_service_backed_ready_and_unavailable_states() {
    let store_path = temp_store_path();
    cleanup_store(&store_path);
    write_process_event(&store_path);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let browser_event = browser_tab_observation_event(
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
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
    store
        .ingest_events(&[browser_event])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let recent = load_recent_summary_from_path(store_path.clone()).await;
    let browser = load_browser_model_from_path(store_path.clone()).await;
    let app_use = app_use_read_model(surface_request(), recent);
    let browser_model = browser_read_model(surface_request(), browser);
    let network_model = network_read_model(surface_request(), None);

    cleanup_store(&store_path);

    assert_eq!(app_use.state, ActivityReadModelState::Ready);
    assert_eq!(app_use.rows[0].launch_count, 2);
    assert_eq!(browser_model.state, ActivityReadModelState::Ready);
    assert_eq!(
        browser_model.rows[0].domain_label,
        constants::activity_store::TEST_BROWSER_DOMAIN
    );
    assert_eq!(network_model.state, ActivityReadModelState::Unavailable);
}

#[tokio::test]
async fn activity_surface_device_scope_reports_offline_for_nonlocal_device() {
    let recent = Some(ActivityRecentSummary {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        first_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        last_observed_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        last_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        most_recent_kind: Some(ActivityEventKind::ProcessObserved),
        most_recent_observer: Some(ActivityObserver::WindowsProcess),
        most_recent_subject_kind: Some(ActivitySubjectKind::Process),
        most_recent_subject_id: Some(
            constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
        ),
        most_recent_subject_name: Some(
            constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
        ),
    });
    let app_use = app_use_read_model(remote_device_surface_request(), recent);
    let snapshot = crate::activity_surface_store::ActivitySurfaceStoreSnapshot {
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        recent_returned: 1,
        last_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        last_observed_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };
    let report = report_document(remote_device_report_request(), Some(snapshot), Vec::new());

    assert_eq!(app_use.state, ActivityReadModelState::Offline);
    assert_eq!(
        app_use.summary,
        constants::activity_surface::SUMMARY_DEVICE_OFFLINE
    );
    assert_eq!(app_use.rows.len(), 0);
    assert_eq!(
        report.source_states[0].state,
        ActivityReadModelState::Offline
    );
    assert_eq!(report.sections[0].state, ActivityReadModelState::Offline);
}

#[tokio::test]
async fn activity_report_history_skips_rejected_json_without_losing_saved_reports() {
    let store_path = temp_store_path();
    cleanup_store(&store_path);
    write_process_event(&store_path);

    let snapshot = local_store_snapshot_from_path(store_path.clone())
        .await
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let report = report_document(report_request(), Some(snapshot), Vec::new());
    let report_dir = temp_report_dir();
    cleanup_report_dir(&report_dir);
    let saved = save_report_document_to_dir(report.clone(), report_dir.clone());
    write_invalid_report_file(&report_dir);
    let history = history_list_from_dir(surface_request(), report_dir.clone());

    cleanup_store(&store_path);
    cleanup_report_dir(&report_dir);

    assert_eq!(
        saved
            .saved_metadata
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .file_name,
        history.reports[0].file_name
    );
    assert_eq!(history.state, ActivityReadModelState::Ready);
    assert_eq!(
        history.storage_state,
        ocentra_parent_agent_protocol::ActivitySavedReportState::Degraded
    );
    assert_eq!(
        history.storage_reason,
        Some(constants::activity_surface::SUMMARY_STORAGE_DEGRADED.to_string())
    );
    assert_eq!(history.reports.len(), 1);
    assert_eq!(history.reports[0].parsed_report.report_id, report.report_id);
}

fn write_process_event(store_path: &PathBuf) {
    let store = ActivityStore::open(store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[process_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
}

fn process_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(4242.0),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::event_id::HEALTH_REPORTED.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: ActivitySource {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
            display_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn report_request() -> ActivityReportRequest {
    ActivityReportRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        frequency: ActivityReportFrequency::Daily,
        scope: surface_scope(),
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn remote_device_report_request() -> ActivityReportRequest {
    ActivityReportRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        frequency: ActivityReportFrequency::Daily,
        scope: remote_device_scope(),
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: surface_scope(),
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn remote_device_surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: remote_device_scope(),
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn surface_scope() -> ActivitySurfaceScope {
    ActivitySurfaceScope {
        scope_kind: ActivitySurfaceScopeKind::Family,
        family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
        device_id: None,
    }
}

fn remote_device_scope() -> ActivitySurfaceScope {
    ActivitySurfaceScope {
        scope_kind: ActivitySurfaceScopeKind::Device,
        family_id: None,
        device_id: Some(constants::activity_store::TEST_REMOTE_DEVICE_ID.to_string()),
    }
}

fn temp_store_path() -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&temp_path_suffix());
    name.push_str(constants::activity_store::TEST_STORE_SUFFIX);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn temp_report_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&temp_path_suffix());
    name.push_str(constants::activity_surface::REPORT_STORAGE_DIR);
    path.push(name);
    path
}

fn temp_path_suffix() -> String {
    let mut suffix = String::new();
    suffix.push_str(&std::process::id().to_string());
    suffix.push(constants::delimiter::HYPHEN);
    suffix.push_str(
        &TEMP_PATH_COUNTER
            .fetch_add(1, Ordering::Relaxed)
            .to_string(),
    );
    suffix.push(constants::delimiter::HYPHEN);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos();
    suffix.push_str(&nanos.to_string());
    suffix.push(constants::delimiter::HYPHEN);
    suffix
}

fn cleanup_store(store_path: &PathBuf) {
    let _ = remove_file(store_path);
    let mut wal_path = store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}

fn cleanup_report_dir(path: &PathBuf) {
    let _ = remove_dir_all(path);
}

fn write_invalid_report_file(report_dir: &Path) {
    let mut path = report_dir.to_path_buf();
    path.push(constants::activity_surface::REPORT_ID_FALLBACK);
    path.set_extension(constants::activity_surface::REPORT_FILE_EXTENSION);
    write(path, constants::activity_surface::SUMMARY_STORE_UNAVAILABLE)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
}
