use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeerRole,
};

use crate::parent_service_health::ParentAgentServiceHealthReason;

use super::{transport, types};

pub(super) fn health_response_mismatch_reason(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> Option<ParentAgentServiceHealthReason> {
    [
        (
            response.schema_version != ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
            ParentAgentServiceHealthReason::ResponseSchemaMismatch,
        ),
        (
            !health_response_has_expected_identity(result, response),
            ParentAgentServiceHealthReason::ResponseIdentityMismatch,
        ),
        (
            !health_response_has_expected_payload(response),
            ParentAgentServiceHealthReason::ResponsePayloadMismatch,
        ),
        (
            !health_response_has_expected_nonce(result, response),
            ParentAgentServiceHealthReason::ResponseNonceMismatch,
        ),
        (
            !health_response_has_expected_event_id(result, response),
            ParentAgentServiceHealthReason::ResponseEventIdMismatch,
        ),
    ]
    .into_iter()
    .find_map(|(mismatch, reason)| mismatch.then_some(reason))
}

fn health_response_has_expected_identity(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> bool {
    result.command == AgentCommandName::AgentHealthCheck
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

fn health_response_has_expected_nonce(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> bool {
    let expected_digest = transport::request_nonce_digest(&result.request_nonce);
    matches!(
        response.payload.get(constants::field::REQUEST_NONCE_DIGEST),
        Some(LogFieldValue::String(value)) if value == &expected_digest
    )
}

fn health_response_has_expected_event_id(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> bool {
    let expected_prefix = format!(
        "{}-{}-",
        constants::event_id::HEALTH_REPORTED,
        transport::request_nonce_digest(&result.request_nonce)
    );
    response.event_id.starts_with(&expected_prefix)
}
