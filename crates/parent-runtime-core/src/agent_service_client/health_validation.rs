use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{
    command_response_event_id_prefix, AgentCommandName, AgentEventEnvelope, AgentEventName,
};

use crate::parent_service_health::ParentAgentServiceHealthReason;

use super::{transport, types};

pub(super) fn health_response_mismatch_reason(
    result: &types::AgentServiceCommandResult,
    response: &AgentEventEnvelope,
) -> Option<ParentAgentServiceHealthReason> {
    if let Some(reason) = transport::command_response_validation_reason(
        &result.command,
        &result.command_message_id,
        &result.request_nonce,
        &result.request_sent_at,
        response,
    ) {
        return Some(reason);
    }
    [
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
    let expected_prefix = command_response_event_id_prefix(
        &result.command,
        &result.command_message_id,
        &transport::request_nonce_digest(&result.request_nonce),
        &response.event,
    );
    response
        .event_id
        .strip_prefix(&expected_prefix)
        .is_some_and(|suffix| suffix.starts_with('-') && suffix.len() > 1)
}
