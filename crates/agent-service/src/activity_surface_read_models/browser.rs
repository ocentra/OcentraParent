use ocentra_parent_agent_protocol::activity_surface::{
    ActivityBrowserReadModel, ActivityReadModelState, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_read_model_states::{
    offline_browser_read_model, request_targets_remote_device, unavailable_browser_read_model,
};

use super::browser_row;

pub(crate) fn browser_read_model(
    request: ActivitySurfaceRequest,
    model: Option<BrowserEvidenceReadModel>,
) -> ActivityBrowserReadModel {
    if request_targets_remote_device(&request) {
        return offline_browser_read_model(request);
    }

    match model {
        Some(model) if model.returned > 0 => ActivityBrowserReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Ready,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_READY.to_string(),
            rows: model.rows.into_iter().map(browser_row).collect(),
        },
        Some(model) => ActivityBrowserReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Empty,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
            rows: Vec::new(),
        },
        None => unavailable_browser_read_model(request),
    }
}
