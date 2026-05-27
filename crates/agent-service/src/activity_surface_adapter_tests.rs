use std::{
    fs::remove_file,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivityReadModelState,
    ActivityReportFrequency, ActivityReportRequest, ActivitySource, ActivitySubject,
    ActivitySubjectKind, ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
    LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{
    activity_surface_read_models::{app_use_read_model, browser_read_model, network_read_model},
    activity_surface_report::{report_document, saved_report_document},
    activity_surface_store::{
        load_browser_model_from_path, load_recent_summary_from_path, local_store_snapshot_from_path,
    },
};

#[tokio::test]
async fn activity_surface_report_uses_real_activity_store_snapshot() {
    let store_path = temp_store_path();
    cleanup_store(&store_path);
    write_process_event(&store_path);

    let snapshot = local_store_snapshot_from_path(store_path.clone())
        .await
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let report = report_document(report_request(), Some(snapshot));
    let saved = saved_report_document(report.clone());

    cleanup_store(&store_path);

    assert_eq!(report.source_states[0].state, ActivityReadModelState::Ready);
    assert_eq!(report.sections.len(), 6);
    assert_eq!(report.sections[0].state, ActivityReadModelState::Ready);
    assert_eq!(report.sections[2].state, ActivityReadModelState::Ready);
    assert_eq!(report.sections[3].state, ActivityReadModelState::Empty);
    assert_eq!(
        saved
            .saved_metadata
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .saved_state,
        ocentra_parent_agent_protocol::ActivitySavedReportState::StorageUnavailable
    );
}

#[tokio::test]
async fn activity_tab_read_models_map_ready_empty_and_unavailable_states() {
    let store_path = temp_store_path();
    cleanup_store(&store_path);
    write_process_event(&store_path);

    let recent = load_recent_summary_from_path(store_path.clone()).await;
    let browser = load_browser_model_from_path(store_path.clone()).await;
    let app_use = app_use_read_model(surface_request(), recent);
    let browser_model = browser_read_model(surface_request(), browser);
    let network_model = network_read_model(surface_request(), None);

    cleanup_store(&store_path);

    assert_eq!(app_use.state, ActivityReadModelState::Ready);
    assert_eq!(app_use.rows[0].launch_count, 1);
    assert_eq!(browser_model.state, ActivityReadModelState::Empty);
    assert_eq!(browser_model.rows.len(), 0);
    assert_eq!(network_model.state, ActivityReadModelState::Unavailable);
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

fn surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: surface_scope(),
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

fn temp_store_path() -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::activity_store::TEST_STORE_SUFFIX);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos()
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
