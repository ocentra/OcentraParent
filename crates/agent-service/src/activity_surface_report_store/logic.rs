use std::{
    env,
    fs::{create_dir_all, write},
    io::{Error as IoError, Result as IoResult},
    path::PathBuf,
};

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportList, ActivityReadModelState, ActivityReportCustodyLabel,
    ActivityReportDocument, ActivityReportSourceLabel, ActivitySavedReportMetadata,
    ActivitySavedReportState, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_report_file_name::{report_file_name, ReportFileName};
use crate::time::timestamp_now;

use super::{history::load_saved_reports, ReportStorageDir};

#[derive(Clone, Debug, PartialEq, Eq)]
struct SavedMetadataArgs {
    report_id: String,
    file_name: String,
    saved_state: ActivitySavedReportState,
    saved_at: Option<String>,
    storage_reason: Option<String>,
}

pub(crate) fn draft_metadata_for_report(
    report: &ActivityReportDocument,
) -> ActivitySavedReportMetadata {
    saved_metadata(SavedMetadataArgs {
        report_id: report.report_id.clone(),
        file_name: report_file_name(report).0,
        saved_state: ActivitySavedReportState::Draft,
        saved_at: None,
        storage_reason: Some(constants::activity_surface::SUMMARY_STORAGE_DRAFT.to_string()),
    })
}

pub(crate) fn save_report_document_to_dir(
    mut report: ActivityReportDocument,
    directory: ReportStorageDir,
) -> ActivityReportDocument {
    let file_name = report_file_name(&report);
    let saved_file_name = file_name.0.clone();
    let saved_at: String = timestamp_now();
    report.saved_metadata = Some(saved_metadata(SavedMetadataArgs {
        report_id: report.report_id.clone(),
        file_name: saved_file_name.clone(),
        saved_state: ActivitySavedReportState::Saved,
        saved_at: Some(saved_at),
        storage_reason: Some(constants::activity_surface::SUMMARY_STORAGE_SAVED.to_string()),
    }));
    match write_report_to_dir(directory, file_name, &report) {
        Ok(()) => report,
        Err(_) => {
            report.saved_metadata = Some(saved_metadata(SavedMetadataArgs {
                report_id: report.report_id.clone(),
                file_name: saved_file_name,
                saved_state: ActivitySavedReportState::StorageUnavailable,
                saved_at: None,
                storage_reason: Some(
                    constants::activity_surface::SUMMARY_STORAGE_UNAVAILABLE.to_string(),
                ),
            }));
            report
        }
    }
}

pub(crate) fn history_list_from_dir(
    request: ActivitySurfaceRequest,
    directory: ReportStorageDir,
) -> ActivityHistoricalReportList {
    match load_saved_reports(&request, directory) {
        Ok(result) => ActivityHistoricalReportList {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: if result.reports.is_empty() {
                ActivityReadModelState::Empty
            } else {
                ActivityReadModelState::Ready
            },
            storage_state: if result.skipped_reports == 0 {
                ActivitySavedReportState::Saved
            } else {
                ActivitySavedReportState::Degraded
            },
            storage_reason: if result.skipped_reports == 0 {
                None
            } else {
                Some(constants::activity_surface::SUMMARY_STORAGE_DEGRADED.to_string())
            },
            reports: result.reports,
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
    directory: ReportStorageDir,
    file_name: ReportFileName,
    report: &ActivityReportDocument,
) -> IoResult<()> {
    let directory = directory.0;
    create_dir_all(&directory)?;
    let mut path = PathBuf::from(&directory);
    path.push(file_name.0);
    let body =
        serde_json::to_string_pretty(report).map_err(|error| IoError::other(error.to_string()))?;
    write(path, body)?;
    Ok(())
}

fn saved_metadata(args: SavedMetadataArgs) -> ActivitySavedReportMetadata {
    ActivitySavedReportMetadata {
        report_id: args.report_id,
        file_name: args.file_name,
        saved_state: args.saved_state,
        saved_at: args.saved_at,
        storage_reason: args.storage_reason,
        custody_label: ActivityReportCustodyLabel::ParentDeviceLocalReportJson,
        source_label: ActivityReportSourceLabel::SavedReportJson,
        raw_child_evidence_included: false,
    }
}

pub(crate) fn activity_report_storage_dir() -> ReportStorageDir {
    let directory = env::var(constants::env_var::DEV_LOG_DIR)
        .unwrap_or_else(|_| constants::dev_log::DEFAULT_DIR.to_owned());
    let mut path = PathBuf::from(directory);
    path.push(constants::activity_surface::REPORT_STORAGE_DIR);
    ReportStorageDir(path)
}
