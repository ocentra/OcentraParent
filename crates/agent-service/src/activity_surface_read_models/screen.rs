use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityScreenReadModel, ActivityScreenReadModelRow,
    ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenEvidenceRecentSummary;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_read_model_states::{
    empty_screen_read_model, offline_screen_read_model, request_targets_remote_device,
    unavailable_screen_read_model,
};

#[path = "screen_row.rs"]
mod screen_row;

use screen_row::activity_screen_row_from_result as convert_activity_screen_row;

pub(crate) fn screen_read_model(
    request: ActivitySurfaceRequest,
    summary: Option<ScreenEvidenceRecentSummary>,
) -> ActivityScreenReadModel {
    if request_targets_remote_device(&request) {
        return offline_screen_read_model(request);
    }

    match summary {
        Some(summary) if summary.returned > 0 => ActivityScreenReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Ready,
            generated_at: summary.generated_at,
            summary: summary
                .latest_summary
                .unwrap_or_else(|| constants::activity_surface::SUMMARY_READY.to_string()),
            rows: summary
                .results
                .into_iter()
                .map(convert_activity_screen_row)
                .collect(),
        },
        Some(summary) => empty_screen_read_model(
            request,
            crate::activity_surface_read_model_states::GeneratedAtText(summary.generated_at),
        ),
        None => unavailable_screen_read_model(request),
    }
}

pub(crate) fn activity_screen_row_from_result(
    result: ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult,
) -> ActivityScreenReadModelRow {
    convert_activity_screen_row(result)
}
