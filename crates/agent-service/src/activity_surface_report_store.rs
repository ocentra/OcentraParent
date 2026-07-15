#[path = "activity_surface_report_store/history.rs"]
mod history;
#[path = "activity_surface_report_store/logic.rs"]
pub(crate) mod logic;

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportList, ActivitySavedReportMetadata, ActivitySurfaceRequest,
};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportStorageDir(pub(crate) PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportPath(pub(crate) PathBuf);

pub(crate) fn draft_metadata_for_report(
    report: &ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument,
) -> ActivitySavedReportMetadata {
    logic::draft_metadata_for_report(report)
}

pub(crate) fn save_report_document(
    report: ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument {
    logic::save_report_document_to_dir(report, logic::activity_report_storage_dir())
}

pub(crate) fn history_list(request: ActivitySurfaceRequest) -> ActivityHistoricalReportList {
    logic::history_list_from_dir(request, logic::activity_report_storage_dir())
}
