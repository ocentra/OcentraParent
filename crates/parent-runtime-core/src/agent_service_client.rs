use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_schema::parent_ui_bridge::ParentRouteContext;
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
use self::transport::parent_route_peer_role;
use self::types::{
    AgentCommandText, AgentServiceCommandResult, AgentServiceResult,
    BrowserActivityReadModelAgentServiceSnapshot, BrowserEvidenceReadModelAgentServiceSnapshot,
    BrowserInterventionReadModelAgentServiceSnapshot,
    BrowserInventoryReadModelAgentServiceSnapshot, BrowserManagedStatusAgentServiceSnapshot,
};

#[path = "agent_service_client/app_game_loaders.rs"]
pub(crate) mod app_game_loaders;
mod command_result_projection;
#[path = "agent_service_client/health.rs"]
pub(crate) mod health;
#[path = "agent_service_client/health_validation.rs"]
mod health_validation;
pub(crate) mod loaders;
mod payload_fields;
#[path = "agent_service_client/read_model_loaders.rs"]
pub(crate) mod read_model_loaders;
pub(crate) mod snapshots_app_game;
pub(crate) mod snapshots_browser;
pub(crate) mod snapshots_common;
pub(crate) mod snapshots_lan;
mod snapshots_lan_replay;
mod snapshots_lan_replay_validation;
pub(crate) mod snapshots_network;
pub(crate) mod snapshots_policy;
pub(crate) mod snapshots_social;
pub(crate) mod snapshots_tracking;
pub(crate) mod social_loaders;
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

pub(crate) fn load_network_flow_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<types::NetworkFlowAgentServiceSnapshot> {
    loaders::load_network_flow_read_model_snapshot(context)
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
