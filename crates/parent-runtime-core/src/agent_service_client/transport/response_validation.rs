use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeerRole,
};

use crate::parent_service_health::ParentAgentServiceHealthReason;

use super::super::payload_fields::serialized_enum_label;

pub(super) fn validate_connection_ready_event(response: &AgentEventEnvelope) -> Result<(), String> {
    if response.schema_version != ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION {
        return Err("agent-service connection-ready schema mismatch".to_string());
    }
    if response.correlation_id != constants::event_id::CONNECTION_READY
        || response.source.peer_id != constants::peer::LOCAL_DEV_AGENT
        || response.source.role != AgentPeerRole::AgentService
        || response.target.peer_id != constants::peer::PORTAL_DEV
        || response.target.role != AgentPeerRole::Portal
        || response.event != AgentEventName::AgentConnectionReady
        || !response
            .event_id
            .starts_with(constants::event_id::CONNECTION_READY)
    {
        return Err("agent-service connection-ready identity mismatch".to_string());
    }
    if !matches!(
        response.payload.get(constants::field::ONLINE),
        Some(LogFieldValue::Boolean(true))
    ) {
        return Err("agent-service connection-ready payload mismatch".to_string());
    }
    if response.sent_at.trim().is_empty() {
        return Err("agent-service connection-ready timestamp missing".to_string());
    }
    Ok(())
}

pub(super) fn command_response_mismatch_reason(
    command: &AgentCommandName,
    command_message_id: &str,
    request_nonce: &str,
    response: &AgentEventEnvelope,
) -> Option<ParentAgentServiceHealthReason> {
    [
        (
            response.schema_version != ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
            ParentAgentServiceHealthReason::ResponseSchemaMismatch,
        ),
        (
            !command_identity_is_bound(command, command_message_id, request_nonce)
                || !response_identity_is_bound(command_message_id, response),
            ParentAgentServiceHealthReason::ResponseIdentityMismatch,
        ),
        (
            !response_has_expected_nonce(request_nonce, response),
            ParentAgentServiceHealthReason::ResponseNonceMismatch,
        ),
        (
            response.event_id.trim().is_empty(),
            ParentAgentServiceHealthReason::ResponseEventIdMismatch,
        ),
        (
            response.sent_at.trim().is_empty(),
            ParentAgentServiceHealthReason::ResponseTimestampMissing,
        ),
    ]
    .into_iter()
    .find_map(|(mismatch, reason)| mismatch.then_some(reason))
}

fn command_identity_is_bound(
    command: &AgentCommandName,
    command_message_id: &str,
    request_nonce: &str,
) -> bool {
    if request_nonce.trim().is_empty() || command_message_id.trim().is_empty() {
        return false;
    }
    let expected_message_id = format!(
        "parent-ui-bridge-{}-{}",
        serialized_enum_label(command),
        super::request_nonce_digest(request_nonce)
    );
    command_message_id == expected_message_id
}

fn response_identity_is_bound(command_message_id: &str, response: &AgentEventEnvelope) -> bool {
    response.correlation_id == command_message_id
        && response.source.peer_id == constants::peer::LOCAL_DEV_AGENT
        && response.source.role == AgentPeerRole::AgentService
        && response.target.peer_id == constants::peer::PORTAL_DEV
        && response.target.role == AgentPeerRole::Portal
        && response.event != AgentEventName::AgentConnectionReady
}

fn response_has_expected_nonce(request_nonce: &str, response: &AgentEventEnvelope) -> bool {
    let expected_digest = super::request_nonce_digest(request_nonce);
    matches!(
        response.payload.get(constants::field::REQUEST_NONCE_DIGEST),
        Some(LogFieldValue::String(value)) if value == &expected_digest
    )
}
