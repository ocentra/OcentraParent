use std::{
    fs::{read_dir, read_to_string},
    io::Result as IoResult,
    path::Path,
};

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportListItem, ActivityReadModelState, ActivityReportCustodyLabel,
    ActivityReportDocument, ActivityReportSourceLabel, ActivityReportSourceReachabilityState,
    ActivityReportSourceStateSummary, ActivitySavedReportState, ActivitySurfaceRequest,
    ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_report_file_name::report_file_name;

use super::{ReportPath, ReportStorageDir};

#[derive(Clone, Debug, PartialEq, Eq)]
struct HistorySummaryText(String);

pub(super) struct LoadSavedReportsResult {
    pub(super) reports: Vec<ActivityHistoricalReportListItem>,
    pub(super) skipped_reports: usize,
}

pub(super) fn load_saved_reports(
    request: &ActivitySurfaceRequest,
    directory: ReportStorageDir,
) -> IoResult<LoadSavedReportsResult> {
    let ReportStorageDir(directory) = directory;
    let directory = directory.as_path();
    if !directory.exists() {
        return Ok(LoadSavedReportsResult {
            reports: Vec::new(),
            skipped_reports: 0,
        });
    }

    let entries = read_dir(directory)?;
    let mut reports = Vec::new();
    let mut skipped_reports = 0;
    for entry in entries {
        let path = entry?.path();
        if !path_is_report_json(path.as_path()) {
            continue;
        }
        let Ok(body) = read_to_string(&path) else {
            skipped_reports += 1;
            continue;
        };
        let Ok(report) = serde_json::from_str::<ActivityReportDocument>(&body) else {
            skipped_reports += 1;
            continue;
        };
        if scope_matches(&request.scope, &report.scope) && range_matches(request, &report) {
            reports.push(history_item_from_report(&ReportPath(path), report));
        }
    }

    reports.sort_by(|left, right| right.report_date.cmp(&left.report_date));
    Ok(LoadSavedReportsResult {
        reports,
        skipped_reports,
    })
}

fn history_item_from_report(
    path: &ReportPath,
    report: ActivityReportDocument,
) -> ActivityHistoricalReportListItem {
    let path = path.0.as_path();
    let metadata = report.saved_metadata.clone();
    let file_name = metadata
        .as_ref()
        .map(|value| value.file_name.clone())
        .or_else(|| {
            path.file_name()
                .map(|value| value.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| report_file_name(&report).0);
    let saved_state = metadata
        .as_ref()
        .map(|value| value.saved_state)
        .unwrap_or(ActivitySavedReportState::Draft);
    let saved_at = metadata.as_ref().and_then(|value| value.saved_at.clone());
    let report_date = saved_at
        .clone()
        .unwrap_or_else(|| report.generated_at.clone());

    ActivityHistoricalReportListItem {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: report.report_id.clone(),
        file_name,
        report_date,
        range_start: report.range_start.clone(),
        range_end: report.range_end.clone(),
        summary: history_summary(&report).0,
        saved_state,
        saved_at,
        source_state_summary: source_state_summary(&report),
        parsed_report: report,
        custody_label: ActivityReportCustodyLabel::ParentDeviceLocalHistory,
        source_label: ActivityReportSourceLabel::SavedReportHistory,
        raw_child_evidence_included: false,
    }
}

fn source_state_summary(report: &ActivityReportDocument) -> ActivityReportSourceStateSummary {
    ActivityReportSourceStateSummary {
        total_sources: report.source_states.len() as u64,
        ready_sources: report
            .source_states
            .iter()
            .filter(|source| source.state == ActivityReadModelState::Ready)
            .count() as u64,
        offline_sources: report
            .source_states
            .iter()
            .filter(|source| source.state == ActivityReadModelState::Offline)
            .count() as u64,
        stale_sources: report
            .source_states
            .iter()
            .filter(|source| source.state == ActivityReadModelState::Stale)
            .count() as u64,
        unavailable_sources: report
            .source_states
            .iter()
            .filter(|source| source.state == ActivityReadModelState::Unavailable)
            .count() as u64,
        unreachable_sources: report
            .source_states
            .iter()
            .filter(|source| {
                source.reachability_state == ActivityReportSourceReachabilityState::Unreachable
            })
            .count() as u64,
        error_sources: report
            .source_states
            .iter()
            .filter(|source| {
                source.reachability_state == ActivityReportSourceReachabilityState::Error
            })
            .count() as u64,
    }
}

fn history_summary(report: &ActivityReportDocument) -> HistorySummaryText {
    HistorySummaryText(
        report
            .sections
            .iter()
            .find(|section| section.state == ActivityReadModelState::Ready)
            .map(|section| section.summary.clone())
            .or_else(|| {
                report
                    .sections
                    .first()
                    .map(|section| section.summary.clone())
            })
            .unwrap_or_else(|| constants::activity_surface::SUMMARY_HISTORY_EMPTY.to_string()),
    )
}

fn path_is_report_json(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|value| value == constants::activity_surface::REPORT_FILE_EXTENSION)
        .unwrap_or(false)
}

fn scope_matches(request: &ActivitySurfaceScope, report: &ActivitySurfaceScope) -> bool {
    match request.scope_kind {
        ActivitySurfaceScopeKind::Family => {
            report.scope_kind == ActivitySurfaceScopeKind::Family
                && request.family_id == report.family_id
        }
        ActivitySurfaceScopeKind::Device => {
            report.scope_kind == ActivitySurfaceScopeKind::Device
                && request.device_id == report.device_id
        }
    }
}

fn range_matches(request: &ActivitySurfaceRequest, report: &ActivityReportDocument) -> bool {
    report.range_end.as_str() >= request.range_start.as_str()
        && report.range_start.as_str() <= request.range_end.as_str()
}
