mod action_dispatch;
mod action_result_app_game;
pub mod lan_replay_rejection_episode;
mod lan_route;
mod parent_desktop_distribution;
mod presentation;
pub mod projection;
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
    ParentBridgeConnectionState, ParentPortalParentAccessState, ParentRouteContext, ParentRouteId,
    ParentRouteSnapshot, ParentSubscriptionEvent, ParentUiAction, ParentUiActionKind,
    ParentUiActionResult,
};
use serde_json::Value;

use crate::parent_service_health::ParentAgentServiceHealth;

use crate::agent_service_client::health::{health_check_for_address, health_check_timeout_ms};
use crate::agent_service_client::read_model_loaders::{
    load_lan_runtime_event_chain_replay_events, load_policy_preview_read_model_snapshot,
};
use crate::agent_service_client::snapshots_lan::network_flow_snapshot_from_parts;
use crate::agent_service_client::snapshots_network::response_json_payload_field;
use crate::agent_service_client::types::{
    AgentServiceCommandResult, AppGameAdapterDispatchPreflightAgentServiceSnapshot,
    AppGameAdapterDispatchResultAgentServiceSnapshot,
    AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
    AppGameTimerParentSurfaceAgentServiceSnapshot, AppUseReadModelAgentServiceSnapshot,
    BrowserActivityReadModelAgentServiceSnapshot, BrowserEvidenceReadModelAgentServiceSnapshot,
    BrowserInterventionReadModelAgentServiceSnapshot,
    BrowserInventoryReadModelAgentServiceSnapshot, BrowserManagedStatusAgentServiceSnapshot,
    GamesReadModelAgentServiceSnapshot, LanRuntimeReplaySnapshot, NetworkFlowAgentServiceSnapshot,
    NetworkRuntimeEventChainAgentServiceSnapshot, PolicyPreviewAgentServiceSnapshot,
    ScreenReadModelAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};
use crate::agent_service_client::{
    dispatch_agent_command, dispatch_known_agent_command, load_network_flow_read_model_snapshot,
};

use self::lan_replay_rejection_episode::ParentRouteSubscriptionLoadState;
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
    app_use_read_model_snapshot: Option<&'a AppUseReadModelAgentServiceSnapshot>,
    browser_activity_read_model_snapshot: Option<&'a BrowserActivityReadModelAgentServiceSnapshot>,
    games_read_model_snapshot: Option<&'a GamesReadModelAgentServiceSnapshot>,
    browser_inventory_read_model_snapshot:
        Option<&'a BrowserInventoryReadModelAgentServiceSnapshot>,
    browser_evidence_read_model_snapshot: Option<&'a BrowserEvidenceReadModelAgentServiceSnapshot>,
    browser_managed_status_snapshot: Option<&'a BrowserManagedStatusAgentServiceSnapshot>,
    browser_intervention_read_model_snapshot:
        Option<&'a BrowserInterventionReadModelAgentServiceSnapshot>,
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
    build_parent_route_snapshot(route, &lan_route_query, None, None, None)
}

pub fn load_parent_route_snapshot_with_service_health(
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
    service_health: &ParentAgentServiceHealth,
) -> ParentRouteSnapshot {
    let lan_route_query = if service_health.is_ready() {
        lan_route_query_for_load(&route, context)
    } else {
        LanRouteQuery::Unavailable(service_health.redacted_detail())
    };
    build_parent_route_snapshot(route, &lan_route_query, None, None, Some(service_health))
}

pub fn parent_agent_service_health_for_address(agent_addr: &str) -> ParentAgentServiceHealth {
    health_check_for_address(agent_addr)
}

pub fn parent_agent_service_health_timeout_ms() -> u64 {
    health_check_timeout_ms()
}

pub fn load_parent_subscription_event(
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> ParentSubscriptionEvent {
    ParentRouteSubscriptionLoadState::default().load(route, context)
}

pub fn load_parent_subscription_event_with_service_health(
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
    service_health: &ParentAgentServiceHealth,
) -> ParentSubscriptionEvent {
    ParentRouteSubscriptionLoadState::default().load_with_service_health(
        route,
        context,
        service_health,
    )
}

fn load_parent_subscription_event_with_state(
    state: &mut ParentRouteSubscriptionLoadState,
    route: ParentRouteId,
    context: Option<&ParentRouteContext>,
    service_health: Option<&ParentAgentServiceHealth>,
) -> ParentSubscriptionEvent {
    let lan_route_query = service_health
        .filter(|health| !health.is_ready())
        .map(|health| LanRouteQuery::Unavailable(health.redacted_detail()))
        .unwrap_or_else(|| lan_route_query_for_load(&route, context));
    let replay = if matches!(&lan_route_query, LanRouteQuery::Available(_)) {
        match load_lan_runtime_event_chain_replay_events() {
            Ok(replay) => ParentSubscriptionReplay::Reported(replay),
            Err(_redacted_error) => ParentSubscriptionReplay::Rejected,
        }
    } else {
        ParentSubscriptionReplay::NotRequested
    };
    let snapshot =
        build_parent_route_snapshot(route.clone(), &lan_route_query, None, None, service_health);
    build_parent_subscription_event_from_parts(state, route, &lan_route_query, replay, snapshot)
}

pub(super) enum ParentSubscriptionReplay {
    NotRequested,
    Reported(LanRuntimeReplaySnapshot),
    Rejected,
}

pub(super) fn build_parent_subscription_event_from_parts(
    state: &mut ParentRouteSubscriptionLoadState,
    route: ParentRouteId,
    lan_route_query: &LanRouteQuery,
    replay: ParentSubscriptionReplay,
    snapshot: ParentRouteSnapshot,
) -> ParentSubscriptionEvent {
    let (mut events, replay_rejected) = match replay {
        ParentSubscriptionReplay::Reported(replay)
            if lan_replay_is_bound_to_status(&replay, lan_route_query) =>
        {
            (replay.events, false)
        }
        ParentSubscriptionReplay::Reported(_) | ParentSubscriptionReplay::Rejected => {
            (Vec::new(), true)
        }
        ParentSubscriptionReplay::NotRequested => (Vec::new(), false),
    };
    events.extend_from_slice(lan_route_query.events());
    if replay_rejected {
        events.push(state.replay_rejection_diagnostic());
    } else {
        state.complete_replay_rejection_episode();
    }
    let events = dedupe_route_events_by_event_id(&events);
    ParentSubscriptionEvent {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route,
        snapshot,
        events: (!events.is_empty()).then_some(events),
    }
}

fn lan_replay_is_bound_to_status(
    replay: &LanRuntimeReplaySnapshot,
    lan_route_query: &LanRouteQuery,
) -> bool {
    let Some(status_history) = lan_route_query
        .read_model()
        .map(|read_model| &read_model.discovery_event_history)
    else {
        return false;
    };

    replay.history_state == status_history.state
        && replay.latest_event_id == status_history.latest_event_id
        && replay.latest_observed_at == status_history.latest_observed_at
}

pub fn dispatch_parent_ui_action(action: &ParentUiAction) -> ParentUiActionResult {
    action_dispatch::dispatch_parent_ui_action_impl(action, None)
}

pub fn dispatch_parent_ui_action_with_service_health(
    action: &ParentUiAction,
    service_health: &ParentAgentServiceHealth,
) -> ParentUiActionResult {
    action_dispatch::dispatch_parent_ui_action_impl(action, Some(service_health))
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
    service_health: Option<&ParentAgentServiceHealth>,
) -> ParentRouteSnapshot {
    build_parent_route_snapshot_impl(
        route,
        lan_route_query,
        network_flow_snapshot,
        snapshot_overlay,
        service_health,
    )
}
