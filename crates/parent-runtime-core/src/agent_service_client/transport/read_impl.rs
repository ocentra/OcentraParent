use std::io::ErrorKind;
use std::net::TcpStream;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use tungstenite::{Error as WebSocketError, Message, WebSocket};

pub(super) fn read_agent_event(
    socket: &mut WebSocket<TcpStream>,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<AgentEventEnvelope, String> {
    loop {
        let message = read_websocket_message(socket, phase, timeout, deadline)?;
        if super::connection::remaining_timeout(deadline).is_err() {
            return Err(format!(
                "agent-service WebSocket {phase} timed out after {}ms",
                timeout.as_millis()
            ));
        }
        if let Some(event) = handle_agent_event_message(socket, message, phase, timeout, deadline)?
        {
            return Ok(event);
        }
    }
}

fn handle_agent_event_message(
    socket: &mut WebSocket<TcpStream>,
    message: Message,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<Option<AgentEventEnvelope>, String> {
    match message {
        Message::Text(text) => serde_json::from_str::<AgentEventEnvelope>(&text)
            .map(Some)
            .map_err(|error| format!("agent-service event parse failed: {error}")),
        Message::Ping(bytes) => {
            send_websocket_pong(socket, bytes, phase, timeout, deadline)?;
            Ok(None)
        }
        Message::Binary(_) | Message::Pong(_) | Message::Frame(_) => Ok(None),
        Message::Close(frame) => Err(websocket_close_message(frame)),
    }
}

fn read_websocket_message(
    socket: &mut WebSocket<TcpStream>,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<Message, String> {
    let remaining = super::connection::remaining_timeout(deadline).map_err(|_| {
        format!(
            "agent-service WebSocket {phase} timed out after {}ms",
            timeout.as_millis()
        )
    })?;
    socket
        .get_mut()
        .set_read_timeout(Some(remaining))
        .map_err(|error| {
            format!("agent-service WebSocket {phase} read timeout setup failed: {error}")
        })?;
    socket
        .read()
        .map_err(|error| map_websocket_error(phase, &error, timeout))
}

fn send_websocket_pong(
    socket: &mut WebSocket<TcpStream>,
    bytes: Vec<u8>,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<(), String> {
    let remaining = super::connection::remaining_timeout(deadline).map_err(|_| {
        format!(
            "agent-service WebSocket {phase} timed out after {}ms",
            timeout.as_millis()
        )
    })?;
    socket
        .get_mut()
        .set_write_timeout(Some(remaining))
        .map_err(|error| {
            format!("agent-service WebSocket {phase} write timeout setup failed: {error}")
        })?;
    socket
        .send(Message::Pong(bytes))
        .map_err(|error| format!("agent-service WebSocket pong failed: {error}"))
}

fn websocket_close_message(frame: Option<tungstenite::protocol::CloseFrame<'_>>) -> String {
    let reason = frame
        .map(|value| value.reason.to_string())
        .unwrap_or_else(|| "no close reason".to_string());
    format!("agent-service WebSocket closed before response: {reason}")
}

pub(super) fn map_websocket_error(
    phase: &str,
    error: &WebSocketError,
    timeout: Duration,
) -> String {
    match is_websocket_timeout(error) {
        true => format!(
            "agent-service WebSocket {phase} timed out after {}ms",
            timeout.as_millis()
        ),
        false => format!("agent-service WebSocket {phase} failed: {error}"),
    }
}

fn is_websocket_timeout(error: &WebSocketError) -> bool {
    matches!(error, WebSocketError::Io(error) if is_io_timeout(error))
}

pub(super) fn is_io_timeout(error: &std::io::Error) -> bool {
    matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock)
}
