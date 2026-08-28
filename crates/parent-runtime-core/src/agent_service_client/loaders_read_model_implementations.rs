use super::*;

pub(in crate::agent_service_client) fn load_lan_runtime_event_chain_replay_events(
) -> AgentServiceResult<LanRuntimeReplaySnapshot> {
    send_agent_command(
        AgentCommandName::AgentLanRuntimeEventChainStreamGet,
        LogFields::new(),
        None,
        AgentRoute::LocalNetwork,
    )
    .and_then(lan_runtime_replay_events_from_result)
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_network_runtime_event_chain_stream_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<NetworkRuntimeEventChainAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(network_runtime_event_chain_snapshot_from_result)
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_policy_preview_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<PolicyPreviewAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentPolicyPreviewReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(policy_preview_snapshot_from_result)
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_tracking_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<TrackingReadModelAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentActivityTrackingReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(tracking_read_model_snapshot_from_result)
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_app_game_notification_readiness_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGameNotificationReadinessAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(|result| app_game_notification_readiness_snapshot_from_result(&result))
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_app_game_policy_readiness_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGamePolicyReadinessAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(|result| app_game_policy_readiness_snapshot_from_result(&result))
    .map_err(AgentServiceError::from_display)
}

pub(in crate::agent_service_client) fn load_app_game_platform_proof_status_read_model_snapshot(
    _context: Option<&ParentRouteContext>,
) -> AgentServiceResult<AppGamePlatformProofStatusAgentServiceSnapshot> {
    send_agent_command(
        AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    )
    .and_then(|result| app_game_platform_proof_status_snapshot_from_result(&result))
    .map_err(AgentServiceError::from_display)
}
