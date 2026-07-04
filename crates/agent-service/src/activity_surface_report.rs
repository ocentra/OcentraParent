#[path = "activity_surface_report/logic.rs"]
mod logic;

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReportDocument, ActivityReportRequest, ActivityReportSourceState,
};

use crate::activity_surface_store::ActivitySurfaceStoreSnapshot;

pub(crate) fn report_document(
    request: ActivityReportRequest,
    snapshot: Option<ActivitySurfaceStoreSnapshot>,
    family_sources: Vec<ActivityReportSourceState>,
) -> ActivityReportDocument {
    logic::report_document(request, snapshot, family_sources)
}
