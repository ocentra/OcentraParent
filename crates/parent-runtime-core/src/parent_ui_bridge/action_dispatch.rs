use super::*;
use ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot;

pub(super) fn dispatch_parent_ui_action_impl(action: &ParentUiAction) -> ParentUiActionResult {
    let action_owned = matches!(
        action.action,
        ParentUiActionKind::RefreshRoute
            | ParentUiActionKind::Reconnect
            | ParentUiActionKind::AgentCommandRequested
            | ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested
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
    let mut accepted = action_owned && !matches!(lan_route_query, LanRouteQuery::Unavailable(_));
    let mut message = dispatch_parent_ui_action_message(action, &lan_route_query);
    let mut events = lan_route_query.events().to_vec();
    let mut network_flow_snapshot: Option<NetworkFlowAgentServiceSnapshot> = None;
    let mut snapshot_overlay = ParentRouteSnapshotOverlay::default();
    dispatch_parent_ui_action_network_flow_refresh(
        action,
        &mut accepted,
        &mut message,
        &mut events,
        &mut network_flow_snapshot,
    );
    dispatch_parent_ui_action_agent_command(
        action,
        action_owned,
        &mut accepted,
        &mut message,
        &mut events,
        &mut network_flow_snapshot,
    );
    dispatch_parent_ui_action_rust_owned_command(
        action,
        action_owned,
        &mut accepted,
        &mut message,
        &mut events,
        &mut snapshot_overlay,
    );
    let snapshot = build_parent_route_snapshot(
        action.route.clone(),
        &lan_route_query,
        network_flow_snapshot.as_ref(),
        Some(&snapshot_overlay),
    );

    ParentUiActionResult {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        accepted,
        connection_state,
        message,
        snapshot: Some(snapshot),
        events,
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

fn dispatch_parent_ui_action_network_flow_refresh(
    action: &ParentUiAction,
    accepted: &mut bool,
    message: &mut String,
    events: &mut Vec<ParentRouteEventSnapshot>,
    network_flow_snapshot: &mut Option<NetworkFlowAgentServiceSnapshot>,
) {
    if !matches!(
        action.action,
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested
    ) {
        return;
    }

    match load_network_flow_read_model_snapshot(None) {
        Ok(snapshot) => {
            *events = snapshot.events.clone();
            *network_flow_snapshot = Some(snapshot);
        }
        Err(error) => {
            *accepted = false;
            *message = error;
            events.clear();
        }
    }
}

fn dispatch_parent_ui_action_agent_command(
    action: &ParentUiAction,
    action_owned: bool,
    accepted: &mut bool,
    message: &mut String,
    events: &mut Vec<ParentRouteEventSnapshot>,
    network_flow_snapshot: &mut Option<NetworkFlowAgentServiceSnapshot>,
) {
    if !matches!(action.action, ParentUiActionKind::AgentCommandRequested)
        || lan_route::is_lan_surface_route(&action.route)
    {
        return;
    }

    let generic_command_result = action
        .command
        .as_deref()
        .ok_or_else(|| {
            "parent Rust facade rejected agent command request without a command name".to_string()
        })
        .and_then(|command_name| dispatch_agent_command(command_name, &action.payload, None));
    match generic_command_result {
        Ok(result) => {
            *accepted = action_owned && !result.is_rejected();
            if let Some(rejection_message) = result.rejection_message() {
                *message = rejection_message;
            }
            if result.response_event.event == AgentEventName::AgentNetworkFlowReadModelReported {
                match network_flow_snapshot_from_parts(&result.response_event, &result.events) {
                    Ok(snapshot) => *network_flow_snapshot = Some(snapshot),
                    Err(error) => {
                        *accepted = false;
                        *message = error;
                    }
                }
            }
            *events = result.events;
        }
        Err(error) => {
            *accepted = false;
            *message = error;
            events.clear();
        }
    }
}

fn dispatch_parent_ui_action_rust_owned_command(
    action: &ParentUiAction,
    action_owned: bool,
    accepted: &mut bool,
    message: &mut String,
    events: &mut Vec<ParentRouteEventSnapshot>,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) {
    let Some(command) = rust_owned_command_for_action(&action.action) else {
        return;
    };

    match dispatch_known_agent_command(command, &action.payload, None) {
        Ok(result) => {
            *accepted = action_owned && !result.is_rejected();
            if let Some(rejection_message) = result.rejection_message() {
                *message = rejection_message;
            }
            if let Err(error) =
                apply_snapshot_overlay_for_action(&action.action, &result, snapshot_overlay)
            {
                *accepted = false;
                *message = error;
            }
            *events = result.events;
        }
        Err(error) => {
            *accepted = false;
            *message = error;
            events.clear();
        }
    }
}
