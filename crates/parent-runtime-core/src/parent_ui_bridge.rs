mod action_dispatch;
mod lan_route;
mod presentation;
pub(crate) mod route_metadata;
#[path = "parent_ui_bridge/route_requirements.rs"]
mod route_requirements;
mod route_snapshot;
#[path = "parent_ui_bridge/snapshot_overlay.rs"]
mod snapshot_overlay;

use std::collections::HashSet;

use ocentra_parent_agent_protocol::{
    constants,
    transport::{AgentCommandName, AgentEventName},
};
use ocentra_schema::parent_ui_bridge::{
    ParentPortalParentAccessState, ParentRouteContext, ParentRouteId, ParentRouteSnapshot,
    ParentSubscriptionEvent, ParentUiAction, ParentUiActionKind, ParentUiActionResult,
};
use serde_json::Value;

use crate::agent_service_client::snapshots_lan::network_flow_snapshot_from_parts;
use crate::agent_service_client::snapshots_network::response_json_payload_field;
use crate::agent_service_client::types::{
    AgentServiceCommandResult, AppGameAdapterDispatchPreflightAgentServiceSnapshot,
    AppGameAdapterDispatchResultAgentServiceSnapshot,
    AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
    AppGameTimerParentSurfaceAgentServiceSnapshot, NetworkFlowAgentServiceSnapshot,
    NetworkRuntimeEventChainAgentServiceSnapshot, PolicyPreviewAgentServiceSnapshot,
    ScreenReadModelAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};
use crate::agent_service_client::{
    dispatch_agent_command, dispatch_known_agent_command, load_activity_screen_read_model_snapshot,
    load_app_game_adapter_dispatch_preflight_read_model_snapshot,
    load_app_game_adapter_dispatch_result_read_model_snapshot,
    load_app_game_child_runtime_transport_receipt_read_model_snapshot,
    load_app_game_notification_readiness_read_model_snapshot,
    load_app_game_platform_proof_status_read_model_snapshot,
    load_app_game_policy_readiness_read_model_snapshot,
    load_app_game_timer_parent_surface_read_model_snapshot, load_network_flow_read_model_snapshot,
    load_network_runtime_event_chain_stream_snapshot, load_policy_preview_read_model_snapshot,
    load_tracking_read_model_snapshot,
};

use self::lan_route::{
    command_enabled_for_route, connection_state_for_route, data_source_for_route,
    is_dev_tools_route, lan_route_query_for_action, lan_route_query_for_load, LanRouteQuery,
};
use self::presentation::{
    action_result_message, browser_route_panels_snapshot, live_activity_snapshot,
    parent_portal_rows_for_route, parent_portal_shell_status, setup_first_run_panel_snapshot,
    summary_for_route,
};
use self::route_metadata::season_label_for_connection;
use self::route_snapshot::build_parent_route_snapshot_impl;

const PARENT_UI_BRIDGE_SCHEMA_VERSION: u16 = 1;
const EMPTY_TIMESTAMP: &str = "";
const HOST_BRIDGE_URL: &str = "host-bridge://tauri-parent";

#[derive(Default)]
struct ParentRouteSnapshotOverlay {
    screen_settings_service_response: Option<Value>,
    app_game_adapter_dispatch_executed_result: Option<Value>,
    activity_tracking_retention_settings_write_result: Option<Value>,
}

struct ParentRouteLiveActivitySnapshotInput<'a> {
    route: &'a ParentRouteId,
    lan_route_query: &'a LanRouteQuery,
    network_flow_snapshot: Option<&'a NetworkFlowAgentServiceSnapshot>,
    network_runtime_event_chain_snapshot: Option<&'a NetworkRuntimeEventChainAgentServiceSnapshot>,
    policy_preview_snapshot: Option<&'a PolicyPreviewAgentServiceSnapshot>,
    parent_access_state: &'a ParentPortalParentAccessState,
    tracking_read_model_snapshot: Option<&'a TrackingReadModelAgentServiceSnapshot>,
    screen_read_model_snapshot: Option<&'a ScreenReadModelAgentServiceSnapshot>,
    app_game_notification_readiness_snapshot:
        Option<&'a AppGameNotificationReadinessAgentServiceSnapshot>,
    app_game_policy_readiness_snapshot: Option<&'a AppGamePolicyReadinessAgentServiceSnapshot>,
    app_game_platform_proof_status_snapshot:
        Option<&'a AppGamePlatformProofStatusAgentServiceSnapshot>,
    app_game_child_runtime_transport_receipt_snapshot:
        Option<&'a AppGameChildRuntimeTransportReceiptAgentServiceSnapshot>,
    app_game_adapter_dispatch_preflight_snapshot:
        Option<&'a AppGameAdapterDispatchPreflightAgentServiceSnapshot>,
    app_game_adapter_dispatch_result_snapshot:
        Option<&'a AppGameAdapterDispatchResultAgentServiceSnapshot>,
    app_game_timer_parent_surface_snapshot:
        Option<&'a AppGameTimerParentSurfaceAgentServiceSnapshot>,
    app_game_adapter_dispatch_execute_result: Option<&'a Value>,
}

pub fn load_parent_route_snapshot(
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> ParentRouteSnapshot {
    let lan_route_query = lan_route_query_for_load(&route, context);
    build_parent_route_snapshot(route, &lan_route_query, None, None)
}

pub fn load_parent_subscription_event(
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> ParentSubscriptionEvent {
    let lan_route_query = lan_route_query_for_load(&route, context);
    let events = dedupe_route_events_by_event_id(lan_route_query.events());
    let snapshot = build_parent_route_snapshot(route.clone(), &lan_route_query, None, None);
    ParentSubscriptionEvent {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route,
        snapshot,
        events: (!events.is_empty()).then_some(events),
    }
}

pub fn dispatch_parent_ui_action(action: &ParentUiAction) -> ParentUiActionResult {
    action_dispatch::dispatch_parent_ui_action_impl(action)
}

fn dedupe_route_events_by_event_id(
    events: &[ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot],
) -> Vec<ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot> {
    let mut seen_event_ids = HashSet::new();
    let mut deduped_events = Vec::with_capacity(events.len());

    for event in events.iter().rev() {
        let Some(event_id) = event.event_id.as_ref().map(|event_id| event_id.as_str()) else {
            deduped_events.push(event.clone());
            continue;
        };
        if seen_event_ids.insert(event_id.to_string()) {
            deduped_events.push(event.clone());
        }
    }

    deduped_events.reverse();
    deduped_events
}

fn build_parent_route_snapshot(
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    snapshot_overlay: Option<&ParentRouteSnapshotOverlay>,
) -> ParentRouteSnapshot {
    build_parent_route_snapshot_impl(
        route,
        lan_route_query,
        network_flow_snapshot,
        snapshot_overlay,
    )
}
