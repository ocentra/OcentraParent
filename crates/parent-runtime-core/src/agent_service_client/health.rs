use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventEnvelope, AgentRoute};

use crate::parent_service_health::{
    response_timestamp_is_fresh, ParentAgentServiceAuthenticationState, ParentAgentServiceHealth,
    ParentAgentServiceHealthReason, ParentAgentServiceHealthState, ParentAgentServiceHealthTrace,
};

use super::health_validation::health_response_mismatch_reason;
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
    let response = &result.response_event;
    let trace = health_trace(&result, response);

    if let Some(reason) = health_response_mismatch_reason(&result, response) {
        return ParentAgentServiceHealth::degraded(reason, trace);
    }
    if result.request_sent_at.trim().is_empty() || response.sent_at.trim().is_empty() {
        return ParentAgentServiceHealth::degraded(
            ParentAgentServiceHealthReason::ResponseTimestampMissing,
            trace,
        );
    }
    if let Err(reason) = response_timestamp_is_fresh(&result.request_sent_at, &response.sent_at) {
        return ParentAgentServiceHealth::degraded(reason, trace);
    }

    let service_version = response
        .snapshot
        .as_ref()
        .map(|snapshot| snapshot.agent.service_version.clone())
        .filter(|value| !value.is_empty());
    let Some(service_version) = service_version else {
        return ParentAgentServiceHealth::degraded(
            ParentAgentServiceHealthReason::ServiceVersionMissing,
            trace,
        );
    };

    ParentAgentServiceHealth {
        state: ParentAgentServiceHealthState::Ready,
        route: Some(AgentRoute::Localhost),
        protocol_schema_version: Some(response.schema_version),
        service_version: Some(service_version),
        transport: Some(constants::value::TRANSPORT_WEBSOCKET.to_string()),
        authentication_state: ParentAgentServiceAuthenticationState::Unauthenticated,
        reason: ParentAgentServiceHealthReason::Ready,
        trace,
    }
}

pub(crate) fn health_check_timeout_ms() -> u64 {
    transport::agent_health_check_timeout_ms()
}

fn health_trace(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> ParentAgentServiceHealthTrace {
    ParentAgentServiceHealthTrace {
        request_id: non_empty(result.command_message_id.clone()),
        correlation_id: non_empty(response.correlation_id.clone()),
        response_event_id: non_empty(response.event_id.clone()),
        request_sent_at: non_empty(result.request_sent_at.clone()),
        response_sent_at: non_empty(response.sent_at.clone()),
    }
}

fn non_empty(value: String) -> Option<String> {
    (!value.trim().is_empty()).then_some(value)
}
