mod generic_command;
mod network_flow;
mod rust_owned_command;
mod state;

use self::generic_command::dispatch_parent_ui_action_agent_command;
use self::network_flow::dispatch_parent_ui_action_network_flow_refresh;
use self::rust_owned_command::dispatch_parent_ui_action_rust_owned_command;
use self::state::ActionDispatchState;
use super::*;

pub(super) fn dispatch_parent_ui_action_impl(action: &ParentUiAction) -> ParentUiActionResult {
    let action_owned = matches!(
        action.action,
        ParentUiActionKind::RefreshRoute
            | ParentUiActionKind::Reconnect
            | ParentUiActionKind::AgentCommandRequested
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
    let connection_state = connection_state_for_route(&action.route, &lan_route_query);
    let mut state = ActionDispatchState::new(
        action_owned && !matches!(lan_route_query, LanRouteQuery::Unavailable(_)),
        dispatch_parent_ui_action_message(action, &lan_route_query),
        lan_route_query.events().to_vec(),
    );
    dispatch_parent_ui_action_network_flow_refresh(action, &mut state);
    dispatch_parent_ui_action_agent_command(action, action_owned, &mut state);
    dispatch_parent_ui_action_rust_owned_command(action, action_owned, &mut state);
    let snapshot = build_parent_route_snapshot(
        action.route.clone(),
        &lan_route_query,
        state.network_flow_snapshot.as_ref(),
        Some(&state.snapshot_overlay),
    );

    ParentUiActionResult {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        accepted: state.accepted,
        connection_state,
        message: state.message,
        snapshot: Some(snapshot),
        events: state.events,
    }
}

fn dispatch_parent_ui_action_message(
    action: &ParentUiAction,
    lan_route_query: &LanRouteQuery,
) -> String {
    match lan_route_query {
        LanRouteQuery::Unavailable(error) if lan_route::is_lan_surface_route(&action.route) => {
            error.clone()
        }
        _ => action_result_message(action),
    }
}
