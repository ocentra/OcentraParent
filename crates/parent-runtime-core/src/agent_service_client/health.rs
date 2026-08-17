use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::parent_service_health::{
    ParentAgentServiceAuthenticationState, ParentAgentServiceHealth, ParentAgentServiceHealthState,
};

use super::{transport, types};

pub(crate) fn health_check_for_address(agent_addr: &str) -> ParentAgentServiceHealth {
    let result = match transport::send_agent_command_to_address(
        agent_addr,
        AgentCommandName::AgentHealthCheck,
        LogFields::new(),
        None,
        AgentRoute::Localhost,
    ) {
        Ok(result) => result,
        Err(_) => return ParentAgentServiceHealth::unavailable(),
    };
    let response = result.response_event;
    if !health_response_has_expected_identity(&result, &response)
        || !health_response_has_expected_payload(&response)
    {
        return ParentAgentServiceHealth::unavailable();
    }

    let service_version = response
        .snapshot
        .as_ref()
        .map(|snapshot| snapshot.agent.service_version.clone())
        .filter(|value| !value.is_empty());
    let Some(service_version) = service_version else {
        return ParentAgentServiceHealth::unavailable();
    };

    ParentAgentServiceHealth {
        state: ParentAgentServiceHealthState::Ready,
        route: Some(AgentRoute::Localhost),
        protocol_schema_version: Some(response.schema_version),
        service_version: Some(service_version),
        transport: Some(constants::value::TRANSPORT_WEBSOCKET.to_string()),
        authentication_state: ParentAgentServiceAuthenticationState::Unauthenticated,
    }
}

pub(crate) fn health_check_timeout_ms() -> u64 {
    transport::agent_health_check_timeout_ms()
}

fn health_response_has_expected_identity(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> bool {
    result.command == AgentCommandName::AgentHealthCheck
        && response.schema_version == AGENT_PROTOCOL_SCHEMA_VERSION
        && response.correlation_id == result.command_message_id
        && response.source.peer_id == constants::peer::LOCAL_DEV_AGENT
        && response.source.role == AgentPeerRole::AgentService
        && response.target.peer_id == constants::peer::PORTAL_DEV
        && response.target.role == AgentPeerRole::Portal
        && response.event == AgentEventName::AgentHealthReported
}

fn health_response_has_expected_payload(response: &AgentEventEnvelope) -> bool {
    matches!(
        response.payload.get(constants::field::ONLINE),
        Some(LogFieldValue::Boolean(true))
    ) && matches!(
        response.payload.get(constants::field::TRANSPORT),
        Some(LogFieldValue::String(value))
            if value == constants::value::TRANSPORT_WEBSOCKET
    ) && matches!(
        response.payload.get(constants::field::COMMAND_TARGET_ROUTE),
        Some(LogFieldValue::String(value))
            if value == constants::value::DEVICE_RUNTIME_ROUTE_LOCALHOST
    ) && matches!(
        response.payload.get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(LogFieldValue::String(value))
            if value == constants::value::LAN_AUTH_UNAUTHENTICATED
    )
}
