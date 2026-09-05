use super::*;
use crate::parent_ui_bridge::common::events::activity::{
    app_use_read_model_response_event, games_read_model_response_event,
};
use ocentra_parent_agent_protocol::transport::AgentCommandName;

pub(super) fn result_network_flow_row_event_id(
    result: &super::ParentUiActionResult,
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
        command: None,
        payload: json!({}),
        context: None,
    }
}

pub(super) fn projected_route_snapshot_json(
    route: ParentRouteId,
    responses: Vec<(AgentCommandName, AgentEventEnvelope)>,
    label: TestContext,
) -> serde_json::Value {
    let responses = responses
        .into_iter()
        .map(|(command, event)| tests_support::projection_response(command, event))
        .collect();
    require_ok(
        serde_json::to_value(tests_support::projected_route_snapshot(route, responses)),
        label.0,
    )
}

pub(super) fn projected_action_result(
    action: &ParentUiAction,
    responses: Vec<(AgentCommandName, AgentEventEnvelope)>,
) -> ParentUiActionResult {
    let responses = responses
        .into_iter()
        .map(|(command, event)| tests_support::projection_response(command, event))
        .collect();
    tests_support::projected_action_result(action, responses)
}

pub(super) fn assert_owner_unavailable_action(result: &ParentUiActionResult) {
    assert!(!result.accepted);
    assert_eq!(
        result.connection_state,
        ocentra_schema::parent_ui_bridge::ParentBridgeConnectionState::Error
    );
    assert_eq!(
        result.message,
        "parent-local bridge Account owner repository is unavailable"
    );
    assert!(result.events.is_empty());
}

pub(super) fn assert_owner_unavailable_connected_action(result: &ParentUiActionResult) {
    assert!(!result.accepted);
    assert_eq!(
        result.connection_state,
        ocentra_schema::parent_ui_bridge::ParentBridgeConnectionState::Connected
    );
    assert_eq!(
        result.message,
        "parent-local bridge Account owner repository is unavailable"
    );
    assert!(result.events.is_empty());
}

pub(super) fn app_game_route_projection(
    responses: Vec<AgentEventEnvelope>,
) -> Vec<(AgentCommandName, AgentEventEnvelope)> {
    let commands = [
        AgentCommandName::AgentActivityAppUseReadModelGet,
        AgentCommandName::AgentActivityGamesReadModelGet,
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet,
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
        AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
        AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
        AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
        AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
    ];
    assert_eq!(responses.len(), commands.len());
    commands.into_iter().zip(responses).collect()
}

pub(super) fn activity_route_projection() -> Vec<(AgentCommandName, AgentEventEnvelope)> {
    vec![
        (
            AgentCommandName::AgentNetworkFlowReadModelGet,
            network_flow_response_event(),
        ),
        (
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
            network_runtime_event_chain_response_event(),
        ),
        (
            AgentCommandName::AgentPolicyPreviewReadModelGet,
            policy_preview_response_event(),
        ),
        (
            AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_response_event(),
        ),
        (
            AgentCommandName::AgentActivityScreenReadModelGet,
            screen_read_model_response_event(),
        ),
        (
            AgentCommandName::AgentActivityAppUseReadModelGet,
            app_use_read_model_response_event(),
        ),
        (
            AgentCommandName::AgentActivityBrowserReadModelGet,
            browser_activity_read_model_response_event(),
        ),
        (
            AgentCommandName::AgentActivityGamesReadModelGet,
            games_read_model_response_event(),
        ),
    ]
}

pub(super) fn activity_route_projection_after_network_refresh(
) -> Vec<(AgentCommandName, AgentEventEnvelope)> {
    activity_route_projection().into_iter().skip(1).collect()
}

pub(super) fn policy_preview_attention_card(fields: &[(&str, &str)]) -> serde_json::Value {
    let mut response = policy_preview_response_event();
    for (field, value) in fields {
        response.payload.insert(
            (*field).to_string(),
            LogFieldValue::String((*value).to_string()),
        );
    }
    let snapshot = projected_route_snapshot_json(
        ParentRouteId::Approvals,
        vec![(AgentCommandName::AgentPolicyPreviewReadModelGet, response)],
        TestContext("policy preview attention route serializes"),
    );
    snapshot["liveActivity"]["policyPreviewPanel"]["cards"][0].clone()
}
