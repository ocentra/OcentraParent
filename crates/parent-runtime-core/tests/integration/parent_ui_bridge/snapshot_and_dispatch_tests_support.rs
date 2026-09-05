use super::*;
use crate::parent_ui_bridge::common::events::activity::{
    app_use_read_model_response_event, games_read_model_response_event,
};

pub(super) fn lan_status_projection(
    read_model: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) -> Vec<ParentAgentServiceProjectionResponse> {
    vec![projection_response(
        AgentCommandName::AgentLanPairingStatusGet,
        lan_event(AgentEventName::AgentLanPairingStatusReported, &read_model),
    )]
}

pub(super) fn proof_panels_projection() -> Vec<ParentAgentServiceProjectionResponse> {
    vec![
        projection_response(
            AgentCommandName::AgentNetworkFlowReadModelGet,
            network_flow_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
            network_runtime_event_chain_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentPolicyPreviewReadModelGet,
            policy_preview_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_response_event(),
        ),
    ]
}

pub(super) fn activity_route_projection() -> Vec<ParentAgentServiceProjectionResponse> {
    vec![
        projection_response(
            AgentCommandName::AgentNetworkFlowReadModelGet,
            network_flow_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
            network_runtime_event_chain_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentPolicyPreviewReadModelGet,
            policy_preview_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityScreenReadModelGet,
            screen_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityAppUseReadModelGet,
            app_use_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityBrowserReadModelGet,
            browser_activity_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityGamesReadModelGet,
            games_read_model_response_event(),
        ),
    ]
}

pub(super) fn activity_route_projection_after_network_refresh(
) -> Vec<ParentAgentServiceProjectionResponse> {
    activity_route_projection().into_iter().skip(1).collect()
}

pub(super) fn projected_route_snapshot_json(
    route: ParentRouteId,
    responses: Vec<ParentAgentServiceProjectionResponse>,
    context: TestContext,
) -> Value {
    serialize_json(&projected_route_snapshot(route, responses), context)
}

pub(super) fn projected_subscription_event_json(
    route: ParentRouteId,
    responses: Vec<ParentAgentServiceProjectionResponse>,
    context: TestContext,
) -> Value {
    serialize_json(&projected_subscription_event(route, responses), context)
}

pub(super) fn assert_owner_unavailable_action(result: &super::super::ParentUiActionResult) {
    assert!(!result.accepted);
    assert_eq!(result.connection_state, ParentBridgeConnectionState::Error);
    assert_eq!(
        result.message,
        "parent-local bridge Account owner repository is unavailable"
    );
    assert!(result.events.is_empty());
    assert_eq!(
        result
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.data_source.clone()),
        Some(ParentRouteDataSource::Unavailable)
    );
}

pub(super) fn result_network_flow_row_event_id(
    result: &super::super::ParentUiActionResult,
) -> Option<CommandText> {
    result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.network_flow_read_model.as_ref())
        .and_then(|read_model| read_model.rows.first())
        .map(|row| CommandText(row.event_id.to_string()))
}

pub(super) fn empty_action(action: ParentUiActionKind, route: ParentRouteId) -> ParentUiAction {
    ParentUiAction {
        action,
        route,
        context: None,
        command: None,
        payload: json!({}),
    }
}
