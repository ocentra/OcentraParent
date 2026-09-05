mod generic_command;
mod network_flow;
mod policy_authoring;
mod rust_owned_command;
mod state;

use self::generic_command::dispatch_parent_ui_action_agent_command;
use self::network_flow::dispatch_parent_ui_action_network_flow_refresh;
use self::policy_authoring::dispatch_parent_ui_action_policy_authoring;
use self::rust_owned_command::dispatch_parent_ui_action_rust_owned_command;
use self::state::ActionDispatchState;
use super::*;

pub(super) fn dispatch_parent_ui_action_impl(
    action: &ParentUiAction,
    service_health: Option<&ParentAgentServiceHealth>,
) -> ParentUiActionResult {
    if let Some(health) = service_health.filter(|health| !health.is_ready()) {
        let lan_route_query = LanRouteQuery::Unavailable(health.redacted_detail());
        let message = health.redacted_detail();
        let snapshot = build_parent_route_snapshot(
            action.route.clone(),
            &lan_route_query,
            None,
            None,
            Some(health),
        );
        return ParentUiActionResult {
            schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
            accepted: false,
            connection_state: ParentBridgeConnectionState::Error,
            message,
            snapshot: Some(snapshot),
            events: Vec::new(),
        };
    }
    let action_owned = matches!(
        action.action,
        ParentUiActionKind::RefreshRoute
            | ParentUiActionKind::Reconnect
            | ParentUiActionKind::AgentCommandRequested
            | ParentUiActionKind::PolicyPreviewAuthoringDraftStaged
            | ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled
            | ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested
            | ParentUiActionKind::PolicyRequestParentResolutionRequested
            | ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested
            | ParentUiActionKind::NetworkFlowReadModelRefreshRequested
            | ParentUiActionKind::TrackingRetentionSettingsWriteRequested
            | ParentUiActionKind::ScreenSettingsGetRequested
            | ParentUiActionKind::ScreenSettingsReplaceRequested
            | ParentUiActionKind::AppGameAdapterDispatchExecuteRequested
            | ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested
    );
    let lan_route_query = lan_route_query_for_action(action);
    let mut state = ActionDispatchState::new(
        action_owned && !matches!(lan_route_query, LanRouteQuery::Unavailable(_)),
        dispatch_parent_ui_action_message(action, &lan_route_query),
        lan_route_query.events().to_vec(),
    );
    if reject_unavailable_lan_route_query(&lan_route_query, &mut state) {
    } else if matches!(
        action.action,
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested
    ) && !lan_route::is_lan_command_route(&action.route)
    {
        state.reject("LAN discovery scan is available only on LAN-owned routes");
    } else {
        let policy_authoring_handled =
            dispatch_parent_ui_action_policy_authoring(action, &lan_route_query, &mut state);
        if !policy_authoring_handled {
            dispatch_parent_ui_action_network_flow_refresh(action, &mut state);
            dispatch_parent_ui_action_agent_command(action, action_owned, &mut state);
            dispatch_parent_ui_action_rust_owned_command(action, action_owned, &mut state);
        }
    }
    let snapshot = build_parent_route_snapshot(
        action.route.clone(),
        &lan_route_query,
        state.network_flow_snapshot.as_ref(),
        Some(&state.snapshot_overlay),
        service_health,
    );
    ParentUiActionResult {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        accepted: state.accepted,
        connection_state: snapshot.connection_state.clone(),
        message: state.message,
        snapshot: Some(snapshot),
        events: state.events,
    }
}

fn reject_unavailable_lan_route_query(
    lan_route_query: &LanRouteQuery,
    state: &mut ActionDispatchState,
) -> bool {
    match lan_route_query {
        LanRouteQuery::Unavailable(error) => {
            state.reject(error.clone());
            true
        }
        _ => false,
    }
}

fn dispatch_parent_ui_action_message(
    action: &ParentUiAction,
    lan_route_query: &LanRouteQuery,
) -> String {
    match lan_route_query {
        LanRouteQuery::Unavailable(error) => error.clone(),
        _ if matches!(action.action, ParentUiActionKind::AgentCommandRequested)
            && !lan_route::is_lan_command_route(&action.route) =>
        {
            "parent Rust facade forwarded generic agent command request".to_string()
        }
        _ => action_result_message(action),
    }
}
