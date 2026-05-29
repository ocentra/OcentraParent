use std::{
    env,
    fs::{remove_dir_all, remove_file, write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, ActivityReadModelState, ActivityReportDocument, ActivityReportFrequency,
    ActivityReportSection, ActivityReportSectionKind, ActivityReportSourceReachabilityState,
    ActivityReportSourceState, ActivitySavedReportState, ActivitySurfaceRequest,
    ActivitySurfaceScope, ActivitySurfaceScopeKind, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::activity_surface_report_store::{history_list_from_dir, save_report_document_to_dir};

#[test]
fn activity_report_store_keeps_family_and_device_reports_separate() {
    let report_dir = TempReportDir::new();
    let family_saved = save_report_document_to_dir(
        report(family_scope(), family_source_states()),
        report_dir.path(),
    );
    let device_saved = save_report_document_to_dir(
        report(device_scope(), device_source_states()),
        report_dir.path(),
    );

    let family_metadata = family_saved.saved_metadata.as_ref().unwrap();
    let device_metadata = device_saved.saved_metadata.as_ref().unwrap();

    assert_eq!(family_metadata.saved_state, ActivitySavedReportState::Saved);
    assert_eq!(device_metadata.saved_state, ActivitySavedReportState::Saved);
    assert_ne!(family_metadata.file_name, device_metadata.file_name);
    assert!(family_metadata
        .file_name
        .contains(constants::activity_surface::SCOPE_FAMILY));
    assert!(device_metadata
        .file_name
        .contains(constants::activity_surface::SCOPE_DEVICE));

    let family_history = history_list_from_dir(surface_request(family_scope()), report_dir.path());
    let device_history = history_list_from_dir(surface_request(device_scope()), report_dir.path());

    assert_eq!(family_history.state, ActivityReadModelState::Ready);
    assert_eq!(device_history.state, ActivityReadModelState::Ready);
    assert_eq!(
        family_history.storage_state,
        ActivitySavedReportState::Saved
    );
    assert_eq!(family_history.storage_reason, None);
    assert_eq!(family_history.reports.len(), 1);
    assert_eq!(device_history.reports.len(), 1);
    assert_eq!(
        family_history.reports[0].parsed_report.scope.scope_kind,
        ActivitySurfaceScopeKind::Family
    );
    assert_eq!(
        device_history.reports[0].parsed_report.scope.scope_kind,
        ActivitySurfaceScopeKind::Device
    );

    assert_family_source_states(&family_history.reports[0].parsed_report.source_states);
}

#[test]
fn activity_report_history_returns_storage_unavailable_for_unreadable_storage_path() {
    let report_dir = TempReportDir::new();
    write(
        report_dir.path(),
        constants::activity_surface::SUMMARY_STORE_UNAVAILABLE,
    )
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    let history = history_list_from_dir(surface_request(family_scope()), report_dir.path());

    assert_eq!(history.state, ActivityReadModelState::Unavailable);
    assert_eq!(
        history.storage_state,
        ActivitySavedReportState::StorageUnavailable
    );
    assert_eq!(
        history.storage_reason,
        Some(constants::activity_surface::SUMMARY_STORAGE_UNAVAILABLE.to_string())
    );
    assert_eq!(history.reports.len(), 0);
}

fn assert_family_source_states(source_states: &[ActivityReportSourceState]) {
    assert_eq!(source_states.len(), 3);
    assert_eq!(
        source_states[0].reachability_state,
        ActivityReportSourceReachabilityState::Reachable
    );
    assert_eq!(source_states[0].state, ActivityReadModelState::Ready);
    assert_eq!(
        source_states[1].reachability_state,
        ActivityReportSourceReachabilityState::Offline
    );
    assert_eq!(source_states[1].state, ActivityReadModelState::Offline);
    assert_eq!(
        source_states[2].reachability_state,
        ActivityReportSourceReachabilityState::Error
    );
    assert_eq!(source_states[2].state, ActivityReadModelState::Unavailable);
}

fn report(
    scope: ActivitySurfaceScope,
    source_states: Vec<ActivityReportSourceState>,
) -> ActivityReportDocument {
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: constants::activity_surface::REPORT_ID_DAILY.to_string(),
        frequency: ActivityReportFrequency::Daily,
        scope,
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        saved_metadata: None,
        source_states,
        sections: vec![section()],
    }
}

fn section() -> ActivityReportSection {
    ActivityReportSection {
        section_kind: ActivityReportSectionKind::Summary,
        title: constants::activity_surface::SECTION_SUMMARY.to_string(),
        state: ActivityReadModelState::Ready,
        summary: constants::activity_surface::SUMMARY_READY.to_string(),
        item_count: 1,
        evidence: Vec::new(),
    }
}

fn surface_request(scope: ActivitySurfaceScope) -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope,
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn family_scope() -> ActivitySurfaceScope {
    ActivitySurfaceScope {
        scope_kind: ActivitySurfaceScopeKind::Family,
        family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
        device_id: None,
    }
}

fn device_scope() -> ActivitySurfaceScope {
    ActivitySurfaceScope {
        scope_kind: ActivitySurfaceScopeKind::Device,
        family_id: None,
        device_id: Some(constants::activity_surface::DEFAULT_DEVICE_ID.to_string()),
    }
}

fn family_source_states() -> Vec<ActivityReportSourceState> {
    vec![
        source_record(
            constants::activity_surface::DEFAULT_DEVICE_ID,
            ActivityReportSourceReachabilityState::Reachable,
            ActivityReadModelState::Ready,
            constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE,
        ),
        source_record(
            constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID,
            ActivityReportSourceReachabilityState::Offline,
            ActivityReadModelState::Offline,
            constants::activity_surface::SUMMARY_FAMILY_SOURCE_UNREACHABLE,
        ),
        source_record(
            constants::activity_surface::FAMILY_SOURCE_ERROR_ID,
            ActivityReportSourceReachabilityState::Error,
            ActivityReadModelState::Unavailable,
            constants::activity_surface::SUMMARY_FAMILY_SOURCE_ERROR,
        ),
    ]
}

fn device_source_states() -> Vec<ActivityReportSourceState> {
    vec![source_record(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        ActivityReportSourceReachabilityState::Reachable,
        ActivityReadModelState::Ready,
        constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE,
    )]
}

fn source_record(
    device_id: &str,
    reachability_state: ActivityReportSourceReachabilityState,
    state: ActivityReadModelState,
    reason: &str,
) -> ActivityReportSourceState {
    ActivityReportSourceState {
        device_id: device_id.to_string(),
        reachability_state,
        state,
        reason: Some(reason.to_string()),
        last_updated_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    }
}

struct TempReportDir {
    path: PathBuf,
}

impl TempReportDir {
    fn new() -> Self {
        let mut path = env::temp_dir();
        let mut directory_name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        directory_name.push_str(constants::activity_surface::REPORT_STORAGE_DIR);
        directory_name.push(constants::delimiter::HYPHEN);
        directory_name.push_str(&nanos_now().to_string());
        path.push(directory_name);
        Self { path }
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Drop for TempReportDir {
    fn drop(&mut self) {
        let _ = remove_dir_all(&self.path);
        let _ = remove_file(&self.path);
    }
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}
