use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentChildDeviceId, ParentRouteContext, ParentRoutePeerRole,
};
use serde_json::Value;
use tungstenite::{
    client::{client as websocket_client, IntoClientRequest},
    http::{header::ORIGIN, HeaderValue},
    Error as WebSocketError, Message, WebSocket,
};

mod timeouts;
use self::timeouts::agent_command_timeout_for;

use super::parent_route_event_snapshot;
use super::payload_fields::{resolve_command_origin, serialized_enum_label};
use super::types::AgentServiceCommandResult;

pub(super) fn send_agent_command(
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
    route: AgentRoute,
) -> Result<AgentServiceCommandResult, String> {
    let command_origin = resolve_command_origin(&payload);
    let timeout = agent_command_timeout_for(&command);
    let agent_addr = agent_addr();
    let url = agent_ws_url_for_addr(&agent_addr);
    let mut request = url.as_str().into_client_request().map_err(|error| {
        format!("agent-service WebSocket request build failed at {url}: {error}")
    })?;
    request
        .headers_mut()
        .insert(ORIGIN, header_value(&command_origin)?);
    let stream = connect_agent_stream(&agent_addr, &url, timeout)?;
    let (mut socket, _) = websocket_client(request, stream)
        .map_err(|error| format!("agent-service WebSocket handshake failed at {url}: {error}"))?;
    let ready_event = read_agent_event(&mut socket, "connection-ready", timeout)?;
    if ready_event.event != AgentEventName::AgentConnectionReady {
        return Err(format!(
            "agent-service expected connection ready event, received {}",
            serialized_enum_label(&ready_event.event)
        ));
    }

    let command_envelope = lan_command_envelope(command, payload, context, route);
    let body = serde_json::to_string(&command_envelope)
        .map_err(|error| format!("agent-service command serialization failed: {error}"))?;
    socket
        .send(Message::Text(body))
        .map_err(|error| map_websocket_error("command-send", &error, timeout))?;

    let response_event = read_agent_event(&mut socket, "command-response", timeout)?;

    Ok(AgentServiceCommandResult {
        events: vec![
            parent_route_event_snapshot(&ready_event),
            parent_route_event_snapshot(&response_event),
        ],
        response_event,
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

fn connect_agent_stream(
    agent_addr: &str,
    url: &str,
    timeout: Duration,
) -> Result<TcpStream, String> {
    let socket_addrs = agent_addr
        .to_socket_addrs()
        .map_err(|error| format!("agent-service address {agent_addr} did not resolve: {error}"))?;
    let socket_addrs: Vec<_> = socket_addrs.collect();
    if socket_addrs.is_empty() {
        return Err(format!(
            "agent-service address {agent_addr} did not resolve to any socket addresses"
        ));
    }

    let mut last_error = None;
    for socket_addr in socket_addrs {
        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(stream) => {
                stream.set_read_timeout(Some(timeout)).map_err(|error| {
                    format!("agent-service WebSocket read timeout setup failed at {url}: {error}")
                })?;
                stream.set_write_timeout(Some(timeout)).map_err(|error| {
                    format!("agent-service WebSocket write timeout setup failed at {url}: {error}")
                })?;
                return Ok(stream);
            }
            Err(error) => last_error = Some(error),
        }
    }

    let detail = last_error
        .map(|error| {
            if is_io_timeout(&error) {
                format!("connect timed out after {}ms", timeout.as_millis())
            } else {
                error.to_string()
            }
        })
        .unwrap_or_else(|| "no socket address was attempted".to_string());
    Err(format!(
        "agent-service WebSocket connect failed at {url}: {detail}"
    ))
}

fn read_agent_event<S: Read + Write>(
    socket: &mut WebSocket<S>,
    phase: &str,
    timeout: Duration,
) -> Result<AgentEventEnvelope, String> {
    loop {
        let message = socket
            .read()
            .map_err(|error| map_websocket_error(phase, &error, timeout))?;
        match message {
            Message::Text(text) => {
                return serde_json::from_str::<AgentEventEnvelope>(&text)
                    .map_err(|error| format!("agent-service event parse failed: {error}"));
            }
            Message::Ping(bytes) => {
                socket
                    .send(Message::Pong(bytes))
                    .map_err(|error| format!("agent-service WebSocket pong failed: {error}"))?;
            }
            Message::Binary(_) | Message::Pong(_) | Message::Frame(_) => {}
            Message::Close(frame) => {
                return Err(format!(
                    "agent-service WebSocket closed before response: {}",
                    frame
                        .and_then(|value| value.reason.to_string().into())
                        .unwrap_or_else(|| "no close reason".to_string())
                ));
            }
        }
    }
}

fn map_websocket_error(phase: &str, error: &WebSocketError, timeout: Duration) -> String {
    if is_websocket_timeout(error) {
        return format!(
            "agent-service WebSocket {phase} timed out after {}ms",
            timeout.as_millis()
        );
    }
    format!("agent-service WebSocket {phase} failed: {error}")
}

fn is_websocket_timeout(error: &WebSocketError) -> bool {
    matches!(error, WebSocketError::Io(error) if is_io_timeout(error))
}

fn is_io_timeout(error: &std::io::Error) -> bool {
    matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock)
}

fn lan_command_envelope(
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
    route: AgentRoute,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: command_message_id(&command),
        sent_at: String::new(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: lan_target_child_device_id(context),
            platform: host_platform().to_string(),
            route,
        },
        command,
        payload,
    }
}

pub(super) fn parse_agent_command_name(command_name: &str) -> Result<AgentCommandName, String> {
    serde_json::from_value::<AgentCommandName>(Value::String(command_name.to_string())).map_err(
        |error| {
            format!("parent Rust facade rejected unsupported agent command {command_name}: {error}")
        },
    )
}

fn header_value(value: &str) -> Result<HeaderValue, String> {
    HeaderValue::from_str(value)
        .map_err(|error| format!("agent-service origin header is invalid for {value}: {error}"))
}

pub(super) fn parent_route_peer_role(role: &AgentPeerRole) -> ParentRoutePeerRole {
    match role {
        AgentPeerRole::Portal => ParentRoutePeerRole::Portal,
        AgentPeerRole::AgentService => ParentRoutePeerRole::AgentService,
        AgentPeerRole::CloudRelay => ParentRoutePeerRole::CloudRelay,
    }
}

fn lan_target_child_device_id(context: Option<&ParentRouteContext>) -> String {
    context
        .and_then(|value| value.selected_child_device_id.as_ref())
        .map(ParentChildDeviceId::as_str)
        .map(str::to_string)
        .or_else(|| {
            std::env::var(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV)
                .ok()
                .filter(|value| !value.is_empty())
        })
        .unwrap_or_else(|| constants::lan_pairing::CHILD_DEVICE_ID.to_string())
}

fn host_platform() -> &'static str {
    match std::env::consts::OS {
        "windows" => constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        "linux" => constants::local_ai_runtime::PLATFORM_OS_LINUX,
        "macos" => constants::local_ai_runtime::PLATFORM_OS_MACOS,
        "android" => constants::enforcement::PLATFORM_ANDROID,
        "ios" => constants::enforcement::PLATFORM_IOS,
        _ => constants::lan_pairing::PLATFORM_UNKNOWN,
    }
}

fn command_message_id(command: &AgentCommandName) -> String {
    let command_name = serialized_enum_label(command);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|value| value.as_millis())
        .unwrap_or_default();
    format!("parent-ui-bridge-{command_name}-{millis}")
}

pub(super) fn rejection_message(event: &AgentEventEnvelope) -> String {
    format!(
        "agent-service command rejected by {}",
        serialized_enum_label(&event.event)
    )
}
