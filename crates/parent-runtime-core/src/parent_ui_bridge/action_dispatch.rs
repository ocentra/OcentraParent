mod generic_command;
mod network_flow;
mod rust_owned_command;
mod state;

use self::generic_command::dispatch_parent_ui_action_agent_command;
use self::network_flow::dispatch_parent_ui_action_network_flow_refresh;
use self::rust_owned_command::dispatch_parent_ui_action_rust_owned_command;
use self::state::ActionDispatchState;
use super::policy_preview::authoring;
use super::presentation::parent_access_state_for_lan_read_model;
use super::*;

pub(super) fn dispatch_parent_ui_action_impl(action: &ParentUiAction) -> ParentUiActionResult {
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
    let connection_state = connection_state_for_route(&action.route, &lan_route_query);
    let mut state = ActionDispatchState::new(
        action_owned && !matches!(lan_route_query, LanRouteQuery::Unavailable(_)),
        dispatch_parent_ui_action_message(action, &lan_route_query),
        lan_route_query.events().to_vec(),
    );
    let policy_authoring_handled =
        dispatch_parent_ui_action_policy_authoring(action, &lan_route_query, &mut state);
    if !policy_authoring_handled {
        dispatch_parent_ui_action_network_flow_refresh(action, &mut state);
        dispatch_parent_ui_action_agent_command(action, action_owned, &mut state);
        dispatch_parent_ui_action_rust_owned_command(action, action_owned, &mut state);
    }
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

fn dispatch_parent_ui_action_policy_authoring(
    action: &ParentUiAction,
    lan_route_query: &LanRouteQuery,
    state: &mut ActionDispatchState,
) -> bool {
    let is_policy_authoring_action = matches!(
        action.action,
        ParentUiActionKind::PolicyPreviewAuthoringDraftStaged
            | ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled
            | ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested
    );
    if !is_policy_authoring_action {
        return false;
    }
    if !matches!(action.route, ParentRouteId::PolicyNetwork) {
        state.reject("policy preview authoring action is bound to the policy route");
        return true;
    }
    let Some(read_model) = load_policy_preview_read_model_snapshot(None).ok() else {
        state.reject("policy preview authoring requires a current policy preview");
        return true;
    };
    let Some(preview_id) = read_model.preview_id.as_ref() else {
        state.reject("policy preview authoring requires a preview identifier");
        return true;
    };
    let parent_access_state = parent_access_state_for_lan_read_model(lan_route_query.read_model());
    match action.action {
        ParentUiActionKind::PolicyPreviewAuthoringDraftStaged => {
            match authoring::stage(&action.payload, preview_id, &parent_access_state) {
                Ok(_) => {
                    state.accepted = true;
                    state.message =
                        "parent Rust facade staged a bounded policy preview draft".to_string();
                }
                Err(error) => state.reject(error),
            }
        }
        ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled => {
            match authoring::cancel(&action.payload, preview_id, &parent_access_state) {
                Ok(()) => {
                    state.accepted = true;
                    state.message =
                        "parent Rust facade invalidated the policy preview draft".to_string();
                }
                Err(error) => state.reject(error),
            }
        }
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            match authoring::consume(&action.payload, preview_id, &parent_access_state) {
                Ok(_) => state.reject(
                    "policy preview handle consumed; confirmed-request relay remains deferred",
                ),
                Err(error) => state.reject(error),
            }
        }
        _ => {}
    }
    true
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
