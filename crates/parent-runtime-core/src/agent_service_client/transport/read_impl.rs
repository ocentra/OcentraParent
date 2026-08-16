use std::io::{ErrorKind, Read, Write};
use std::time::Duration;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use tungstenite::{Error as WebSocketError, Message, WebSocket};

pub(super) fn read_agent_event<S: Read + Write>(
    socket: &mut WebSocket<S>,
    phase: &str,
    timeout: Duration,
) -> Result<AgentEventEnvelope, String> {
    loop {
        let message = read_websocket_message(socket, phase, timeout)?;
        if let Some(event) = handle_agent_event_message(socket, message)? {
            return Ok(event);
        }
    }
}

fn handle_agent_event_message<S: Read + Write>(
    socket: &mut WebSocket<S>,
    message: Message,
) -> Result<Option<AgentEventEnvelope>, String> {
    match message {
        Message::Text(text) => serde_json::from_str::<AgentEventEnvelope>(&text)
            .map(Some)
            .map_err(|error| format!("agent-service event parse failed: {error}")),
        Message::Ping(bytes) => {
            send_websocket_pong(socket, bytes)?;
            Ok(None)
        }
        Message::Binary(_) | Message::Pong(_) | Message::Frame(_) => Ok(None),
        Message::Close(frame) => Err(websocket_close_message(frame)),
    }
}

fn read_websocket_message<S: Read + Write>(
    socket: &mut WebSocket<S>,
    phase: &str,
    timeout: Duration,
) -> Result<Message, String> {
    socket
        .read()
        .map_err(|error| map_websocket_error(phase, &error, timeout))
}

fn send_websocket_pong<S: Read + Write>(
    socket: &mut WebSocket<S>,
    bytes: Vec<u8>,
) -> Result<(), String> {
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
