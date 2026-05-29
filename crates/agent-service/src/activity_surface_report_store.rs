use std::{
    env,
    fs::{create_dir_all, read_dir, read_to_string, write},
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityHistoricalReportListItem,
    ActivityReadModelState, ActivityReportDocument, ActivitySavedReportMetadata,
    ActivitySavedReportState, ActivitySurfaceRequest, ActivitySurfaceScope,
    ActivitySurfaceScopeKind, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::activity_surface_report_file_name::report_file_name;
use crate::time::timestamp_now;

pub(crate) fn save_report_document(report: ActivityReportDocument) -> ActivityReportDocument {
    save_report_document_to_dir(report, activity_report_storage_dir())
}

pub(crate) fn draft_metadata_for_report(
    report: &ActivityReportDocument,
) -> ActivitySavedReportMetadata {
    saved_metadata(
        &report.report_id,
        &report_file_name(report),
        ActivitySavedReportState::Draft,
        None,
        Some(constants::activity_surface::SUMMARY_STORAGE_DRAFT.to_string()),
    )
}

pub(crate) fn save_report_document_to_dir(
    mut report: ActivityReportDocument,
    directory: PathBuf,
) -> ActivityReportDocument {
    let file_name = report_file_name(&report);
    let saved_at = timestamp_now();
    report.saved_metadata = Some(saved_metadata(
        &report.report_id,
        &file_name,
        ActivitySavedReportState::Saved,
        Some(saved_at),
        Some(constants::activity_surface::SUMMARY_STORAGE_SAVED.to_string()),
    ));
    match write_report_to_dir(&directory, &file_name, &report) {
        Ok(()) => report,
        Err(_) => {
            report.saved_metadata = Some(saved_metadata(
                &report.report_id,
                &file_name,
                ActivitySavedReportState::StorageUnavailable,
                None,
                Some(constants::activity_surface::SUMMARY_STORAGE_UNAVAILABLE.to_string()),
            ));
            report
        }
    }
}

pub(crate) fn history_list(request: ActivitySurfaceRequest) -> ActivityHistoricalReportList {
    history_list_from_dir(request, activity_report_storage_dir())
}

pub(crate) fn history_list_from_dir(
    request: ActivitySurfaceRequest,
    directory: PathBuf,
) -> ActivityHistoricalReportList {
    match load_saved_reports(&request, &directory) {
        Ok(reports) => ActivityHistoricalReportList {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: if reports.is_empty() {
                ActivityReadModelState::Empty
            } else {
                ActivityReadModelState::Ready
            },
            storage_state: ActivitySavedReportState::Saved,
            storage_reason: None,
            reports,
        },
        Err(_) => ActivityHistoricalReportList {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Unavailable,
            storage_state: ActivitySavedReportState::StorageUnavailable,
            storage_reason: Some(
                constants::activity_surface::SUMMARY_STORAGE_UNAVAILABLE.to_string(),
            ),
            reports: Vec::new(),
        },
    }
}

fn write_report_to_dir(
    directory: &Path,
    file_name: &str,
    report: &ActivityReportDocument,
) -> Result<(), ()> {
    create_dir_all(directory).map_err(|_| ())?;
    let mut path = PathBuf::from(directory);
    path.push(file_name);
    let body = serde_json::to_string_pretty(report).map_err(|_| ())?;
    write(path, body).map_err(|_| ())?;
    Ok(())
}

fn load_saved_reports(
    request: &ActivitySurfaceRequest,
    directory: &Path,
) -> Result<Vec<ActivityHistoricalReportListItem>, ()> {
    if !directory.exists() {
        return Ok(Vec::new());
    }

    let entries = read_dir(directory).map_err(|_| ())?;
    let mut reports = Vec::new();
    for entry in entries {
        let path = entry.map_err(|_| ())?.path();
        if !path_is_report_json(&path) {
            continue;
        }
        let Ok(body) = read_to_string(&path) else {
            continue;
        };
        let Ok(report) = serde_json::from_str::<ActivityReportDocument>(&body) else {
            continue;
        };
        if scope_matches(&request.scope, &report.scope) {
            reports.push(history_item_from_report(path, report));
        }
    }

    reports.sort_by(|left, right| right.report_date.cmp(&left.report_date));
    Ok(reports)
}

fn history_item_from_report(
    path: PathBuf,
    report: ActivityReportDocument,
) -> ActivityHistoricalReportListItem {
    let metadata = report.saved_metadata.clone();
    let file_name = metadata
        .as_ref()
        .map(|value| value.file_name.clone())
        .or_else(|| {
            path.file_name()
                .map(|value| value.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| report_file_name(&report));
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
        summary: history_summary(&report),
        saved_state,
        saved_at,
        parsed_report: report,
    }
}

fn history_summary(report: &ActivityReportDocument) -> String {
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
        .unwrap_or_else(|| constants::activity_surface::SUMMARY_HISTORY_EMPTY.to_string())
}

fn saved_metadata(
    report_id: &str,
    file_name: &str,
    saved_state: ActivitySavedReportState,
    saved_at: Option<String>,
    storage_reason: Option<String>,
) -> ActivitySavedReportMetadata {
    ActivitySavedReportMetadata {
        report_id: report_id.to_string(),
        file_name: file_name.to_string(),
        saved_state,
        saved_at,
        storage_reason,
    }
}

fn activity_report_storage_dir() -> PathBuf {
    let directory = env::var(constants::env_var::DEV_LOG_DIR)
        .unwrap_or_else(|_| constants::dev_log::DEFAULT_DIR.to_owned());
    let mut path = PathBuf::from(directory);
    path.push(constants::activity_surface::REPORT_STORAGE_DIR);
    path
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
