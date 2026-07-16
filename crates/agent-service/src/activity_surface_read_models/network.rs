use ocentra_parent_agent_protocol::activity_surface::{
    ActivityNetworkReadModel, ActivityReadModelState, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_read_model_states::{
    offline_network_read_model, request_targets_remote_device, unavailable_network_read_model,
};

use super::network_row;

pub(crate) fn network_read_model(
    request: ActivitySurfaceRequest,
    model: Option<ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel>,
) -> ActivityNetworkReadModel {
    if request_targets_remote_device(&request) {
        return offline_network_read_model(request);
    }

    match model {
        Some(model) if model.returned > 0 => {
            let rows = model
                .rows
                .into_iter()
                .map(|row| network_row(&request, row))
                .collect();
            ActivityNetworkReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at: model.generated_at,
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        Some(model) => ActivityNetworkReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Empty,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
            rows: Vec::new(),
        },
        None => unavailable_network_read_model(request),
    }
}
