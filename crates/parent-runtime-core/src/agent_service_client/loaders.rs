use super::snapshots_app_game::{
    app_game_adapter_dispatch_preflight_snapshot_from_result,
    app_game_adapter_dispatch_result_snapshot_from_result,
    app_game_child_runtime_transport_receipt_snapshot_from_result,
    app_game_notification_readiness_snapshot_from_result,
    app_game_platform_proof_status_snapshot_from_result,
    app_game_policy_readiness_snapshot_from_result,
    app_game_timer_parent_surface_snapshot_from_result,
};
use super::snapshots_lan::lan_snapshot_from_result;
use super::snapshots_lan::{
    network_flow_snapshot_from_result, network_runtime_event_chain_snapshot_from_result,
    policy_preview_snapshot_from_result,
};
use super::snapshots_tracking::{
    activity_screen_read_model_snapshot_from_result, tracking_read_model_snapshot_from_result,
};
use super::transport::{parse_agent_command_name, send_agent_command};
use super::types::{
    AgentServiceCommandResult, AppGameAdapterDispatchPreflightAgentServiceSnapshot,
    AppGameAdapterDispatchResultAgentServiceSnapshot,
    AppGameChildRuntimeTransportReceiptAgentServiceSnapshot,
    AppGameNotificationReadinessAgentServiceSnapshot,
    AppGamePlatformProofStatusAgentServiceSnapshot, AppGamePolicyReadinessAgentServiceSnapshot,
    AppGameTimerParentSurfaceAgentServiceSnapshot, LanAgentServiceSnapshot,
    NetworkFlowAgentServiceSnapshot, NetworkRuntimeEventChainAgentServiceSnapshot,
    PolicyPreviewAgentServiceSnapshot, ScreenReadModelAgentServiceSnapshot,
    TrackingReadModelAgentServiceSnapshot,
};
use super::*;

pub(crate) fn load_lan_status_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    // Status snapshots are always read from the local parent-owned agent-service route.
    // UI device selection is route/read-model state, not the transport target identity.
    send_agent_command(
        AgentCommandName::AgentLanPairingStatusGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(lan_snapshot_from_result)
}

pub(crate) fn request_lan_browser_discovery_scan(
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        LogFields::new(),
        context,
        AgentRoute::LocalNetwork,
    )
    .and_then(lan_snapshot_from_result)
}

pub(crate) fn load_network_flow_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<NetworkFlowAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentNetworkFlowReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(network_flow_snapshot_from_result)
}

pub(crate) fn load_network_runtime_event_chain_stream_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<NetworkRuntimeEventChainAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(network_runtime_event_chain_snapshot_from_result)
}

pub(crate) fn load_policy_preview_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<PolicyPreviewAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentPolicyPreviewReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(policy_preview_snapshot_from_result)
}

pub(crate) fn load_tracking_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<TrackingReadModelAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityTrackingReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(tracking_read_model_snapshot_from_result)
}

pub(crate) fn load_activity_screen_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<ScreenReadModelAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityScreenReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(activity_screen_read_model_snapshot_from_result)
}

pub(crate) fn load_app_game_notification_readiness_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGameNotificationReadinessAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_notification_readiness_snapshot_from_result)
}

pub(crate) fn load_app_game_policy_readiness_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGamePolicyReadinessAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_policy_readiness_snapshot_from_result)
}

pub(crate) fn load_app_game_platform_proof_status_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGamePlatformProofStatusAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_platform_proof_status_snapshot_from_result)
}

pub(crate) fn load_app_game_child_runtime_transport_receipt_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_child_runtime_transport_receipt_snapshot_from_result)
}

pub(crate) fn load_app_game_adapter_dispatch_preflight_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGameAdapterDispatchPreflightAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_adapter_dispatch_preflight_snapshot_from_result)
}

pub(crate) fn load_app_game_adapter_dispatch_result_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGameAdapterDispatchResultAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_adapter_dispatch_result_snapshot_from_result)
}

pub(crate) fn load_app_game_timer_parent_surface_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> Result<AppGameTimerParentSurfaceAgentServiceSnapshot, String> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(app_game_timer_parent_surface_snapshot_from_result)
}

pub(crate) fn dispatch_agent_command(
    command_name: &str,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> Result<AgentServiceCommandResult, String> {
    let command = parse_agent_command_name(command_name)?;
    dispatch_known_agent_command(command, payload, context)
}

pub(crate) fn dispatch_known_agent_command(
    command: AgentCommandName,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> Result<AgentServiceCommandResult, String> {
    let payload = log_fields_from_json(payload)?;
    send_agent_command(command, payload, context, AgentRoute::Localhost)
}

pub(crate) fn dispatch_lan_agent_command(
    command_name: &str,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    let command = parse_agent_command_name(command_name)?;
    let payload = log_fields_from_json(payload)?;
    send_agent_command(command, payload, context, AgentRoute::LocalNetwork)
        .and_then(lan_snapshot_from_result)
}
