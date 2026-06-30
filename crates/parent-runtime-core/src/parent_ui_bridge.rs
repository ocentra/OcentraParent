mod action_dispatch;
mod lan_route;
mod presentation;
pub(crate) mod route_metadata;
mod route_snapshot;

use std::collections::HashSet;

use ocentra_parent_agent_protocol::{
    constants,
    transport::{AgentCommandName, AgentEventName},
};
use ocentra_schema::parent_ui_bridge::{
    ParentPortalParentAccessState, ParentRouteContext, ParentRouteId, ParentRouteSnapshot,
    ParentSubscriptionEvent, ParentUiAction, ParentUiActionKind, ParentUiActionResult,
};
use serde::Serialize;
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
const LAN_DISCOVERY_REPORTED_EVENT: &str = "agent.lan-pairing.browser-discovery.reported";

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

fn rust_owned_command_for_action(action: &ParentUiActionKind) -> Option<AgentCommandName> {
    match action {
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            Some(AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm)
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            Some(AgentCommandName::AgentActivityTrackingRetentionSettingsWrite)
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            Some(AgentCommandName::AgentScreenSettingsGet)
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            Some(AgentCommandName::AgentScreenSettingsReplace)
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            Some(AgentCommandName::AgentActivityAppGameAdapterDispatchExecute)
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            Some(AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest)
        }
        _ => None,
    }
}

fn apply_snapshot_overlay_for_action(
    action: &ParentUiActionKind,
    result: &AgentServiceCommandResult,
    snapshot_overlay: &mut ParentRouteSnapshotOverlay,
) -> Result<(), String> {
    if result.is_rejected() {
        return Ok(());
    }

    match action {
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            expect_agent_event(
                &result.response_event.event,
                &AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported,
            )?;
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            expect_agent_event(
                &result.response_event.event,
                &AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
            )?;
            snapshot_overlay.activity_tracking_retention_settings_write_result =
                Some(response_json_payload_field(
                    &result.response_event,
                    constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
                )?);
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            expect_agent_event(
                &result.response_event.event,
                &AgentEventName::AgentScreenSettingsReported,
            )?;
            snapshot_overlay.screen_settings_service_response = Some(response_json_payload_field(
                &result.response_event,
                constants::field::SCREEN_SETTINGS_RESPONSE,
            )?);
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            if !matches!(
                result.response_event.event,
                AgentEventName::AgentScreenSettingsReplaceAccepted
                    | AgentEventName::AgentScreenSettingsReplaceRejected
            ) {
                return Err(format!(
                    "agent-service expected screen settings replace response event, received {}",
                    serialized_label(&result.response_event.event)
                ));
            }
            snapshot_overlay.screen_settings_service_response = Some(response_json_payload_field(
                &result.response_event,
                constants::field::SCREEN_SETTINGS_RESPONSE,
            )?);
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            expect_agent_event(
                &result.response_event.event,
                &AgentEventName::AgentActivityAppGameAdapterDispatchExecuted,
            )?;
            snapshot_overlay.app_game_adapter_dispatch_executed_result =
                Some(response_json_payload_field(
                    &result.response_event,
                    constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT,
                )?);
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            expect_agent_event(
                &result.response_event.event,
                &AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
            )?;
        }
        _ => {}
    }

    Ok(())
}

fn expect_agent_event(actual: &AgentEventName, expected: &AgentEventName) -> Result<(), String> {
    if actual == expected {
        return Ok(());
    }
    Err(format!(
        "agent-service expected {}, received {}",
        serialized_label(&expected),
        serialized_label(actual)
    ))
}

fn serialized_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn route_requires_network_flow_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ProofPanels)
}

fn route_requires_network_runtime_event_chain_stream(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ProofPanels)
}

fn route_requires_policy_preview_read_model(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::PolicyNetwork
            | ParentRouteId::RuleManagement
            | ParentRouteId::Schedules
            | ParentRouteId::Approvals
            | ParentRouteId::Enforcement
            | ParentRouteId::ProofPanels
    )
}

fn route_requires_screen_summary_read_model(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::ScreenAnalysis)
}

fn route_requires_tracking_read_model(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::PolicyTracking | ParentRouteId::ProofPanels
    )
}

fn route_requires_app_game_session_read_models(route: &ParentRouteId) -> bool {
    matches!(route, ParentRouteId::AppGameSessions)
}
