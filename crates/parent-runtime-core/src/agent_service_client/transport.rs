use std::time::{Duration, Instant};

use ocentra_family_identity_core::session_lifecycle_custody::parent_local_bridge::IssuedParentLocalBridgeSession;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeerRole, AgentRoute,
};
use ocentra_schema::parent_ui_bridge::{ParentRouteContext, ParentRoutePeerRole};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tungstenite::client::{client as websocket_client, IntoClientRequest};
use tungstenite::http::header::ORIGIN;
use tungstenite::Message;

#[path = "transport/connection.rs"]
mod connection;
#[path = "transport/envelope.rs"]
mod envelope;
#[path = "transport/read.rs"]
mod read;
#[path = "transport/read_impl.rs"]
mod read_impl;
#[path = "transport/response_validation.rs"]
mod response_validation;
mod timeouts;
use self::timeouts::agent_command_timeout_for;
use self::{connection::*, envelope::*, read::read_agent_event, read_impl::map_websocket_error};

pub(super) fn agent_health_check_timeout_ms() -> u64 {
    timeouts::agent_health_check_timeout_ms()
}

pub(super) fn command_response_validation_reason(
    command: &AgentCommandName,
    command_message_id: &str,
    request_nonce: &str,
    request_sent_at: &str,
    response: &AgentEventEnvelope,
) -> Option<crate::parent_service_health::ParentAgentServiceHealthReason> {
    response_validation::command_response_mismatch_reason(
        command,
        command_message_id,
        request_nonce,
        request_sent_at,
        response,
    )
}

use super::parent_route_event_snapshot;
use super::payload_fields::{resolve_command_origin, serialized_enum_label};
use super::types::AgentServiceCommandResult;

pub(super) fn send_agent_command(
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
    route: AgentRoute,
) -> Result<AgentServiceCommandResult, String> {
    let agent_addr = agent_addr();
    send_agent_command_to_address(&agent_addr, command, payload, context, route)
}

pub(super) fn send_agent_command_to_address(
    agent_addr: &str,
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
    route: AgentRoute,
) -> Result<AgentServiceCommandResult, String> {
    let owner_session = crate::parent_local_bridge_runtime::require_authenticated_transport_owner()
        .map_err(|error| error.to_string())?;
    let command_origin = resolve_command_origin();
    let timeout = agent_command_timeout_for(&command);
    let deadline = Instant::now() + timeout;
    let url = agent_ws_url_for_addr(agent_addr);
    let mut request = url.as_str().into_client_request().map_err(|error| {
        format!("agent-service WebSocket request build failed at {url}: {error}")
    })?;
    request
        .headers_mut()
        .insert(ORIGIN, header_value(&command_origin)?);
    let stream = connect_agent_stream(agent_addr, &url, timeout, deadline)?;
    configure_socket_timeouts(&stream, remaining_timeout(deadline)?, &url)?;
    let (mut socket, _) = websocket_client(request, stream)
        .map_err(|error| format!("agent-service WebSocket handshake failed at {url}: {error}"))?;
    configure_socket_timeouts(socket.get_mut(), remaining_timeout(deadline)?, &url)?;
    send_owner_handshake(owner_session, |body| {
        socket
            .send(Message::Text(body))
            .map_err(|error| map_websocket_error("bridge-handshake-send", &error, timeout))
    })?;
    let ready_event = read_agent_event(&mut socket, "connection-ready", timeout, deadline)?;
    if ready_event.event != AgentEventName::AgentConnectionReady {
        return Err(format!(
            "agent-service expected connection ready event, received {}",
            serialized_enum_label(&ready_event.event)
        ));
    }
    response_validation::validate_connection_ready_event(&ready_event)?;
    ensure_phase_deadline(deadline, timeout, "connection-ready")?;

    let (command_envelope, request_nonce) = lan_command_envelope(command, payload, context, route)?;
    let command = command_envelope.command.clone();
    let command_message_id = command_envelope.message_id.clone();
    let request_sent_at = command_envelope.sent_at.clone();
    let body = serde_json::to_string(&command_envelope)
        .map_err(|error| format!("agent-service command serialization failed: {error}"))?;
    configure_socket_timeouts(socket.get_mut(), remaining_timeout(deadline)?, &url)?;
    socket
        .send(Message::Text(body))
        .map_err(|error| map_websocket_error("command-send", &error, timeout))?;

    let response_event = read_agent_event(&mut socket, "command-response", timeout, deadline)?;
    if let Some(reason) = command_response_validation_reason(
        &command,
        &command_message_id,
        &request_nonce,
        &request_sent_at,
        &response_event,
    ) {
        if command != AgentCommandName::AgentHealthCheck {
            return Err(format!(
                "agent-service response rejected for {}: {reason:?}",
                serialized_enum_label(&command)
            ));
        }
    }
    ensure_phase_deadline(deadline, timeout, "command-response")?;

    Ok(AgentServiceCommandResult {
        command,
        command_message_id,
        request_nonce,
        request_sent_at,
        events: vec![
            parent_route_event_snapshot(&ready_event),
            parent_route_event_snapshot(&response_event),
        ],
        response_event,
    })
}

pub(super) fn request_nonce_digest(request_nonce: &str) -> String {
    format!("{:x}", Sha256::digest(request_nonce.as_bytes()))
}

fn send_owner_handshake<F>(
    owner_session: IssuedParentLocalBridgeSession,
    send: F,
) -> Result<(), String>
where
    F: FnOnce(String) -> Result<(), String>,
{
    owner_session.present_first_message(|handshake| {
        let body = serde_json::to_string(&handshake).map_err(|error| {
            format!("agent-service bridge handshake serialization failed: {error}")
        })?;
        send(body)
    })
}

fn ensure_phase_deadline(deadline: Instant, timeout: Duration, phase: &str) -> Result<(), String> {
    remaining_timeout(deadline).map(|_| ()).map_err(|error| {
        format!(
            "agent-service WebSocket {phase} timed out after {}ms: {error}",
            timeout.as_millis(),
        )
    })
}

fn agent_addr() -> String {
    std::env::var(constants::env_var::AGENT_ADDR)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| constants::bind::DEFAULT_AGENT_ADDR.to_string())
}

fn agent_ws_url_for_addr(agent_addr: &str) -> String {
    format!("ws://{agent_addr}{}", constants::endpoint::DEV_WS)
}

pub(super) fn parse_agent_command_name(command_name: &str) -> Result<AgentCommandName, String> {
    serde_json::from_value::<AgentCommandName>(Value::String(command_name.to_string())).map_err(
        |error| {
            format!("parent Rust facade rejected unsupported agent command {command_name}: {error}")
        },
    )
}

pub(super) fn parent_route_peer_role(role: &AgentPeerRole) -> ParentRoutePeerRole {
    match role {
        AgentPeerRole::Portal => ParentRoutePeerRole::Portal,
        AgentPeerRole::AgentService => ParentRoutePeerRole::AgentService,
        AgentPeerRole::CloudRelay => ParentRoutePeerRole::CloudRelay,
    }
}

pub(super) fn rejection_message(event: &AgentEventEnvelope) -> String {
    format!(
        "agent-service command rejected by {}",
        serialized_enum_label(&event.event)
    )
}
