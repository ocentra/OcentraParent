use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityReadModelState, ActivityReportDocument,
    ActivityReportFrequency, ActivityReportRequest, ActivityReportSection,
    ActivityReportSectionKind, ActivityReportSourceState, ActivitySavedReportMetadata,
    ActivitySavedReportState, ActivitySurfaceRequest, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{activity_surface_store::ActivitySurfaceStoreSnapshot, time::timestamp_now};

pub(crate) fn report_document(
    request: ActivityReportRequest,
    snapshot: Option<ActivitySurfaceStoreSnapshot>,
) -> ActivityReportDocument {
    match snapshot {
        Some(snapshot) => report_document_from_snapshot(request, snapshot),
        None => unavailable_report_document(request),
    }
}

pub(crate) fn saved_report_document(mut report: ActivityReportDocument) -> ActivityReportDocument {
    report.saved_metadata = Some(ActivitySavedReportMetadata {
        report_id: report.report_id.clone(),
        file_name: report_file_name(report.frequency).to_string(),
        saved_state: ActivitySavedReportState::StorageUnavailable,
        saved_at: None,
        storage_reason: Some(constants::activity_surface::SUMMARY_STORAGE_UNAVAILABLE.to_string()),
    });
    report
}

pub(crate) fn history_list(request: ActivitySurfaceRequest) -> ActivityHistoricalReportList {
    ActivityHistoricalReportList {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::ScaffoldOnly,
        reports: Vec::new(),
    }
}

fn report_document_from_snapshot(
    request: ActivityReportRequest,
    snapshot: ActivitySurfaceStoreSnapshot,
) -> ActivityReportDocument {
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report_id(request.frequency).to_string(),
        frequency: request.frequency,
        scope: request.scope,
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
        generated_at: timestamp_now(),
        saved_metadata: None,
        source_states: vec![ActivityReportSourceState {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            state: ActivityReadModelState::Ready,
            reason: None,
            last_updated_at: snapshot.last_event_id,
        }],
        sections: vec![
            report_section(ActivityReportSectionKind::Summary, snapshot.recent_returned),
            report_section(ActivityReportSectionKind::Screen, snapshot.screen_returned),
            report_section(ActivityReportSectionKind::AppUse, snapshot.recent_returned),
            report_section(
                ActivityReportSectionKind::Browser,
                snapshot.browser_returned,
            ),
            report_section(ActivityReportSectionKind::Games, snapshot.games_returned),
            report_section(
                ActivityReportSectionKind::Network,
                snapshot.network_returned,
            ),
        ],
    }
}

fn unavailable_report_document(request: ActivityReportRequest) -> ActivityReportDocument {
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report_id(request.frequency).to_string(),
        frequency: request.frequency,
        scope: request.scope,
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
        generated_at: timestamp_now(),
        saved_metadata: None,
        source_states: vec![ActivityReportSourceState {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            state: ActivityReadModelState::Unavailable,
            reason: Some(constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string()),
            last_updated_at: None,
        }],
        sections: vec![
            unavailable_section(ActivityReportSectionKind::Summary),
            unavailable_section(ActivityReportSectionKind::Screen),
            unavailable_section(ActivityReportSectionKind::AppUse),
            unavailable_section(ActivityReportSectionKind::Browser),
            unavailable_section(ActivityReportSectionKind::Games),
            unavailable_section(ActivityReportSectionKind::Network),
        ],
    }
}

fn report_section(kind: ActivityReportSectionKind, item_count: u64) -> ActivityReportSection {
    ActivityReportSection {
        section_kind: kind,
        title: section_title(kind).to_string(),
        state: if item_count > 0 {
            ActivityReadModelState::Ready
        } else {
            ActivityReadModelState::Empty
        },
        summary: report_section_summary(item_count).to_string(),
        item_count,
        evidence: Vec::new(),
    }
}

fn unavailable_section(kind: ActivityReportSectionKind) -> ActivityReportSection {
    ActivityReportSection {
        section_kind: kind,
        title: section_title(kind).to_string(),
        state: ActivityReadModelState::Unavailable,
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        item_count: 0,
        evidence: Vec::new(),
    }
}

fn report_section_summary(item_count: u64) -> &'static str {
    if item_count > 0 {
        constants::activity_surface::SUMMARY_READY
    } else {
        constants::activity_surface::SUMMARY_EMPTY
    }
}

fn section_title(kind: ActivityReportSectionKind) -> &'static str {
    match kind {
        ActivityReportSectionKind::Summary => constants::activity_surface::SECTION_SUMMARY,
        ActivityReportSectionKind::Screen => constants::activity_surface::SECTION_SCREEN,
        ActivityReportSectionKind::AppUse => constants::activity_surface::SECTION_APP_USE,
        ActivityReportSectionKind::Browser => constants::activity_surface::SECTION_BROWSER,
        ActivityReportSectionKind::Games => constants::activity_surface::SECTION_GAMES,
        ActivityReportSectionKind::Network => constants::activity_surface::SECTION_NETWORK,
    }
}

fn report_id(frequency: ActivityReportFrequency) -> &'static str {
    match frequency {
        ActivityReportFrequency::Daily => constants::activity_surface::REPORT_ID_DAILY,
        ActivityReportFrequency::Weekly => constants::activity_surface::REPORT_ID_WEEKLY,
        ActivityReportFrequency::Monthly => constants::activity_surface::REPORT_ID_MONTHLY,
    }
}

fn report_file_name(frequency: ActivityReportFrequency) -> &'static str {
    match frequency {
        ActivityReportFrequency::Daily => constants::activity_surface::REPORT_FILE_DAILY,
        ActivityReportFrequency::Weekly => constants::activity_surface::REPORT_FILE_WEEKLY,
        ActivityReportFrequency::Monthly => constants::activity_surface::REPORT_FILE_MONTHLY,
    }
}
