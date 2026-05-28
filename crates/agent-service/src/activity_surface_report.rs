use ocentra_parent_agent_protocol::{
    constants, ActivityReadModelState, ActivityReportDocument, ActivityReportFrequency,
    ActivityReportRequest, ActivityReportSection, ActivityReportSectionKind,
    ActivityReportSourceState, ActivitySurfaceScope, ActivitySurfaceScopeKind,
    ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{activity_surface_store::ActivitySurfaceStoreSnapshot, time::timestamp_now};

pub(crate) fn report_document(
    request: ActivityReportRequest,
    snapshot: Option<ActivitySurfaceStoreSnapshot>,
) -> ActivityReportDocument {
    if request_targets_remote_device(&request.scope) {
        return offline_device_report_document(request);
    }

    match snapshot {
        Some(snapshot) => report_document_from_snapshot(request, snapshot),
        None => unavailable_report_document(request),
    }
}

fn report_document_from_snapshot(
    request: ActivityReportRequest,
    snapshot: ActivitySurfaceStoreSnapshot,
) -> ActivityReportDocument {
    let generated_at = timestamp_now();
    let source_state = if snapshot_has_rows(&snapshot) {
        ActivityReadModelState::Ready
    } else {
        ActivityReadModelState::Empty
    };
    let source_states = source_states_for_request(&request.scope, &snapshot, source_state);
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report_id(request.frequency, &generated_at),
        frequency: request.frequency,
        scope: request.scope,
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
        generated_at,
        saved_metadata: None,
        source_states,
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

fn source_states_for_request(
    scope: &ActivitySurfaceScope,
    snapshot: &ActivitySurfaceStoreSnapshot,
    source_state: ActivityReadModelState,
) -> Vec<ActivityReportSourceState> {
    let mut states = vec![ActivityReportSourceState {
        device_id: snapshot.device_id.clone(),
        state: source_state,
        reason: Some(constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE.to_string()),
        last_updated_at: snapshot.last_observed_at.clone(),
    }];

    if scope.scope_kind == ActivitySurfaceScopeKind::Family {
        states.push(ActivityReportSourceState {
            device_id: constants::activity_surface::FAMILY_FANOUT_SOURCE_ID.to_string(),
            state: ActivityReadModelState::Unavailable,
            reason: Some(
                constants::activity_surface::SUMMARY_FAMILY_FANOUT_UNAVAILABLE.to_string(),
            ),
            last_updated_at: None,
        });
    }

    states
}

fn unavailable_report_document(request: ActivityReportRequest) -> ActivityReportDocument {
    let generated_at = timestamp_now();
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report_id(request.frequency, &generated_at),
        frequency: request.frequency,
        scope: request.scope,
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
        generated_at,
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

fn offline_device_report_document(request: ActivityReportRequest) -> ActivityReportDocument {
    let generated_at = timestamp_now();
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report_id(request.frequency, &generated_at),
        frequency: request.frequency,
        scope: request.scope.clone(),
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
        generated_at,
        saved_metadata: None,
        source_states: vec![ActivityReportSourceState {
            device_id: request
                .scope
                .device_id
                .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string()),
            state: ActivityReadModelState::Offline,
            reason: Some(constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string()),
            last_updated_at: None,
        }],
        sections: vec![
            offline_section(ActivityReportSectionKind::Summary),
            offline_section(ActivityReportSectionKind::Screen),
            offline_section(ActivityReportSectionKind::AppUse),
            offline_section(ActivityReportSectionKind::Browser),
            offline_section(ActivityReportSectionKind::Games),
            offline_section(ActivityReportSectionKind::Network),
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

fn offline_section(kind: ActivityReportSectionKind) -> ActivityReportSection {
    ActivityReportSection {
        section_kind: kind,
        title: section_title(kind).to_string(),
        state: ActivityReadModelState::Offline,
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        item_count: 0,
        evidence: Vec::new(),
    }
}

fn snapshot_has_rows(snapshot: &ActivitySurfaceStoreSnapshot) -> bool {
    snapshot.recent_returned > 0
        || snapshot.browser_returned > 0
        || snapshot.network_returned > 0
        || snapshot.games_returned > 0
        || snapshot.screen_returned > 0
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

fn report_id(frequency: ActivityReportFrequency, generated_at: &str) -> String {
    let mut id = String::from(match frequency {
        ActivityReportFrequency::Daily => constants::activity_surface::REPORT_ID_DAILY,
        ActivityReportFrequency::Weekly => constants::activity_surface::REPORT_ID_WEEKLY,
        ActivityReportFrequency::Monthly => constants::activity_surface::REPORT_ID_MONTHLY,
    });
    id.push(constants::delimiter::HYPHEN);
    id.extend(generated_at.chars().filter(char::is_ascii_alphanumeric));
    id
}

fn request_targets_remote_device(scope: &ActivitySurfaceScope) -> bool {
    scope.scope_kind == ActivitySurfaceScopeKind::Device
        && scope.device_id.as_deref() != Some(constants::activity_surface::DEFAULT_DEVICE_ID)
}
