use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{
    command_response_event_id_prefix, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentPeerRole,
};

use crate::parent_service_health::{
    response_timestamp_is_current, response_timestamp_is_fresh, ParentAgentServiceHealthReason,
};

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
    if let Err(reason) = response_timestamp_is_current(&response.sent_at) {
        return Err(format!(
            "agent-service connection-ready timestamp rejected: {reason:?}"
        ));
    }
    Ok(())
}

pub(super) fn command_response_mismatch_reason(
    command: &AgentCommandName,
    command_message_id: &str,
    request_nonce: &str,
    request_sent_at: &str,
    response: &AgentEventEnvelope,
) -> Option<ParentAgentServiceHealthReason> {
    [
        (
            response.schema_version != ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
            ParentAgentServiceHealthReason::ResponseSchemaMismatch,
        ),
        (
            !command.response_event_is_expected(&response.event),
            ParentAgentServiceHealthReason::ResponseIdentityMismatch,
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
            !response_event_id_is_bound(command, command_message_id, request_nonce, response),
            ParentAgentServiceHealthReason::ResponseEventIdMismatch,
        ),
        (
            response_timestamp_is_fresh(request_sent_at, &response.sent_at).is_err(),
            response_timestamp_reason(request_sent_at, &response.sent_at),
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
}

fn response_event_id_is_bound(
    command: &AgentCommandName,
    command_message_id: &str,
    request_nonce: &str,
    response: &AgentEventEnvelope,
) -> bool {
    let request_nonce_digest = super::request_nonce_digest(request_nonce);
    let expected_prefix = command_response_event_id_prefix(
        command,
        command_message_id,
        &request_nonce_digest,
        &response.event,
    );
    response
        .event_id
        .strip_prefix(&expected_prefix)
        .is_some_and(|suffix| suffix.starts_with('-') && suffix.len() > 1)
}

fn response_timestamp_reason(
    request_sent_at: &str,
    response_sent_at: &str,
) -> ParentAgentServiceHealthReason {
    response_timestamp_is_fresh(request_sent_at, response_sent_at)
        .err()
        .unwrap_or(ParentAgentServiceHealthReason::ResponseTimestampStale)
}

fn response_has_expected_nonce(request_nonce: &str, response: &AgentEventEnvelope) -> bool {
    let expected_digest = super::request_nonce_digest(request_nonce);
    matches!(
        response.payload.get(constants::field::REQUEST_NONCE_DIGEST),
        Some(LogFieldValue::String(value)) if value == &expected_digest
    )
}
