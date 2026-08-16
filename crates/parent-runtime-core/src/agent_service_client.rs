use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModel;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_schema::parent_ui_bridge::ParentRouteContext;
use ocentra_schema::parent_ui_bridge::{
    ParentActivityTrackingReadModelFailureReason, ParentActivityTrackingReadModelResultSnapshot,
    ParentNetworkRuntimeEventChainStreamSnapshot, ParentNetworkRuntimeEventResultSnapshot,
    ParentNetworkRuntimeEventValueSnapshot, ParentPolicyPreviewReadModelSnapshot,
    ParentRouteEventCorrelationId, ParentRouteEventId, ParentRouteEventSnapshot, ParentRoutePeerId,
};
use serde_json::Value;

use self::payload_fields::log_fields_from_json;
use self::payload_fields::{log_field_string, serialized_enum_label};
use self::snapshots_common::{
    list_field, optional_bool_field, optional_string_field, optional_u16_field, optional_u64_field,
    parent_route_event_snapshot, required_string_field, required_u64_field,
    required_u64_field_with_context,
};
use self::snapshots_network::{
    network_flow_read_model_from_payload, network_runtime_event_chain_stream_from_payload,
};
use self::snapshots_policy::policy_preview_read_model_from_payload;
use self::transport::{parent_route_peer_role, rejection_message};
use self::types::{
    AgentCommandText, AgentServiceCommandResult, AgentServiceResult,
    AppGameAdapterDispatchPreflightAgentServiceSnapshot,
    AppGameAdapterDispatchResultAgentServiceSnapshot,
    AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
    AppGameTimerParentSurfaceAgentServiceSnapshot, AppUseReadModelAgentServiceSnapshot,
    BrowserActivityReadModelAgentServiceSnapshot, BrowserEvidenceReadModelAgentServiceSnapshot,
    BrowserInterventionReadModelAgentServiceSnapshot,
    BrowserInventoryReadModelAgentServiceSnapshot, BrowserManagedStatusAgentServiceSnapshot,
    GamesReadModelAgentServiceSnapshot, LanAgentServiceSnapshot, NetworkFlowAgentServiceSnapshot,
    NetworkRuntimeEventChainAgentServiceSnapshot, PolicyPreviewAgentServiceSnapshot,
    ScreenReadModelAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};

mod command_result_projection;
pub(crate) mod loaders;
mod payload_fields;
pub(crate) mod snapshots_app_game;
pub(crate) mod snapshots_browser;
pub(crate) mod snapshots_common;
pub(crate) mod snapshots_lan;
mod snapshots_lan_replay;
mod snapshots_lan_replay_validation;
pub(crate) mod snapshots_network;
pub(crate) mod snapshots_policy;
pub(crate) mod snapshots_tracking;
pub(crate) mod transport;
pub(crate) mod types;

pub(crate) fn load_lan_status_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::LanAgentServiceSnapshot> {
    loaders::load_lan_status_snapshot(context)
}

pub(crate) fn request_lan_browser_discovery_scan(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::LanAgentServiceSnapshot> {
    loaders::request_lan_browser_discovery_scan(context)
}

pub(crate) fn load_lan_runtime_event_chain_replay_events(
) -> AgentServiceResult<types::LanRuntimeReplaySnapshot> {
    loaders::load_lan_runtime_event_chain_replay_events()
}

pub(crate) fn load_network_flow_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::NetworkFlowAgentServiceSnapshot> {
    loaders::load_network_flow_read_model_snapshot(context)
}

pub(crate) fn load_network_runtime_event_chain_stream_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::NetworkRuntimeEventChainAgentServiceSnapshot> {
    loaders::load_network_runtime_event_chain_stream_snapshot(context)
}

pub(crate) fn load_policy_preview_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::PolicyPreviewAgentServiceSnapshot> {
    loaders::load_policy_preview_read_model_snapshot(context)
}

pub(crate) fn load_tracking_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::TrackingReadModelAgentServiceSnapshot> {
    loaders::load_tracking_read_model_snapshot(context)
}

pub(crate) fn load_activity_screen_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::ScreenReadModelAgentServiceSnapshot> {
    loaders::load_activity_screen_read_model_snapshot(context)
}

pub(crate) fn load_activity_app_use_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppUseReadModelAgentServiceSnapshot> {
    loaders::load_activity_app_use_read_model_snapshot(context)
}

pub(crate) fn load_activity_games_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::GamesReadModelAgentServiceSnapshot> {
    loaders::load_activity_games_read_model_snapshot(context)
}

pub(crate) fn load_browser_activity_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<BrowserActivityReadModelAgentServiceSnapshot> {
    loaders::load_browser_activity_read_model_snapshot(context)
}

pub(crate) fn load_browser_managed_status_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<BrowserManagedStatusAgentServiceSnapshot> {
    loaders::load_browser_managed_status_snapshot(context)
}

pub(crate) fn load_browser_inventory_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<BrowserInventoryReadModelAgentServiceSnapshot> {
    loaders::load_browser_inventory_read_model_snapshot(context)
}

pub(crate) fn load_browser_evidence_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<BrowserEvidenceReadModelAgentServiceSnapshot> {
    loaders::load_browser_evidence_read_model_snapshot(context)
}

pub(crate) fn load_browser_intervention_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<BrowserInterventionReadModelAgentServiceSnapshot> {
    loaders::load_browser_intervention_read_model_snapshot(context)
}

pub(crate) fn load_app_game_notification_readiness_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGameNotificationReadinessAgentServiceSnapshot> {
    loaders::load_app_game_notification_readiness_read_model_snapshot(context)
}

pub(crate) fn load_app_game_policy_readiness_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGamePolicyReadinessAgentServiceSnapshot> {
    loaders::load_app_game_policy_readiness_read_model_snapshot(context)
}

pub(crate) fn load_app_game_platform_proof_status_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGamePlatformProofStatusAgentServiceSnapshot> {
    loaders::load_app_game_platform_proof_status_read_model_snapshot(context)
}

pub(crate) fn load_app_game_child_runtime_transport_receipt_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGameChildRuntimeTransportReceiptAgentServiceSnapshot> {
    loaders::load_app_game_child_runtime_transport_receipt_read_model_snapshot(context)
}

pub(crate) fn load_app_game_adapter_dispatch_preflight_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGameAdapterDispatchPreflightAgentServiceSnapshot> {
    loaders::load_app_game_adapter_dispatch_preflight_read_model_snapshot(context)
}

pub(crate) fn load_app_game_adapter_dispatch_result_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGameAdapterDispatchResultAgentServiceSnapshot> {
    loaders::load_app_game_adapter_dispatch_result_read_model_snapshot(context)
}

pub(crate) fn load_app_game_timer_parent_surface_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AppGameTimerParentSurfaceAgentServiceSnapshot> {
    loaders::load_app_game_timer_parent_surface_read_model_snapshot(context)
}

pub(crate) fn dispatch_agent_command(
    command_name: AgentCommandText<'_>,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AgentServiceCommandResult> {
    loaders::dispatch_agent_command(command_name, payload, context)
}

pub(crate) fn dispatch_known_agent_command(
    command: AgentCommandName,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::AgentServiceCommandResult> {
    loaders::dispatch_known_agent_command(command, payload, context)
}

pub(crate) fn dispatch_lan_agent_command(
    command_name: AgentCommandText<'_>,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::LanAgentServiceSnapshot> {
    loaders::dispatch_lan_agent_command(command_name, payload, context)
}
