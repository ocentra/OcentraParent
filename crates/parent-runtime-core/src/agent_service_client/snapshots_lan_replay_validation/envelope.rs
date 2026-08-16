use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeerRole,
};
use ocentra_parent_agent_protocol::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};
use ocentra_schema::parent_ui_bridge::{ParentRoutePeerId, ParentRoutePeerRole};

use super::{canonical_text, parse_rfc3339_timestamp, LAN_REPLAY_CONTEXT};
use crate::agent_service_client::payload_fields::serialized_enum_label;
use crate::agent_service_client::transport::parent_route_peer_role;

pub(in crate::agent_service_client) struct LanReplayEnvelopeIdentity {
    pub(in crate::agent_service_client) source_peer_id: ParentRoutePeerId,
    pub(in crate::agent_service_client) source_role: ParentRoutePeerRole,
    pub(in crate::agent_service_client) target_peer_id: ParentRoutePeerId,
    pub(in crate::agent_service_client) target_role: ParentRoutePeerRole,
    pub(in crate::agent_service_client) severity: String,
}

pub(in crate::agent_service_client) fn validate_replay_envelope(
    response_event: &AgentEventEnvelope,
    command: &AgentCommandName,
    command_message_id: &str,
) -> Result<LanReplayEnvelopeIdentity, String> {
    validate_response_kind(response_event, command)?;
    validate_response_ids(response_event, command_message_id)?;
    validate_response_route(response_event)?;
    validate_response_acceptance(response_event)?;

    Ok(LanReplayEnvelopeIdentity {
        source_peer_id: ParentRoutePeerId::parse(response_event.source.peer_id.clone())
            .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} rejected invalid envelope source peer"))?,
        source_role: parent_route_peer_role(&response_event.source.role),
        target_peer_id: ParentRoutePeerId::parse(response_event.target.peer_id.clone())
            .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} rejected invalid envelope target peer"))?,
        target_role: parent_route_peer_role(&response_event.target.role),
        severity: serialized_enum_label(&response_event.severity),
    })
}

fn validate_response_kind(
    response_event: &AgentEventEnvelope,
    command: &AgentCommandName,
) -> Result<(), String> {
    if command != &AgentCommandName::AgentLanRuntimeEventChainStreamGet {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected response for an unexpected command"
        ));
    }
    if response_event.schema_version != AGENT_PROTOCOL_SCHEMA_VERSION {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected unsupported envelope schemaVersion {}",
            response_event.schema_version
        ));
    }
    if response_event.event != AgentEventName::AgentLanRuntimeEventChainStreamReported {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected unexpected envelope event {}",
            serialized_enum_label(&response_event.event)
        ));
    }
    Ok(())
}

fn validate_response_ids(
    response_event: &AgentEventEnvelope,
    command_message_id: &str,
) -> Result<(), String> {
    validate_runtime_stream_event_id(&response_event.event_id)?;
    let expected_correlation_id = canonical_text(command_message_id, "command.messageId")?;
    let correlation_id = canonical_text(&response_event.correlation_id, "envelope.correlationId")?;
    if correlation_id != expected_correlation_id {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected envelope correlationId that does not match the command messageId"
        ));
    }
    parse_rfc3339_timestamp(&response_event.sent_at, "envelope.sentAt")?;
    Ok(())
}

fn validate_runtime_stream_event_id(event_id: &str) -> Result<(), String> {
    let prefix = constants::lan_pairing::EVENT_RUNTIME_EVENT_CHAIN_STREAM_REPORTED;
    let suffix = event_id
        .strip_prefix(prefix)
        .and_then(|value| value.strip_prefix('-'));
    if suffix
        .is_none_or(|value| value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected noncanonical runtime stream envelope eventId"
        ));
    }
    Ok(())
}

fn validate_response_route(response_event: &AgentEventEnvelope) -> Result<(), String> {
    let source_peer_id = canonical_text(&response_event.source.peer_id, "envelope.source.peerId")?;
    if source_peer_id != constants::peer::LOCAL_DEV_AGENT
        || response_event.source.role != AgentPeerRole::AgentService
    {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected unexpected envelope source peer or role"
        ));
    }
    let target_peer_id = canonical_text(&response_event.target.peer_id, "envelope.target.peerId")?;
    if target_peer_id != constants::peer::PORTAL_DEV
        || response_event.target.role != AgentPeerRole::Portal
    {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected unexpected envelope target peer or role"
        ));
    }
    Ok(())
}

fn validate_response_acceptance(response_event: &AgentEventEnvelope) -> Result<(), String> {
    if response_event.severity != LogLevel::Info || response_event.snapshot.is_some() {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected unexpected envelope severity or snapshot"
        ));
    }
    Ok(())
}
