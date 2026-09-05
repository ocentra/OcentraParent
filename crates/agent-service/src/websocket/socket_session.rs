use axum::extract::ws::{Message, WebSocket};
use ocentra_family_identity_core::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{AgentEventEnvelope, AgentEventName},
};
use std::{future::Future, pin::Pin};

use crate::{
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    snapshot::build_dev_log_snapshot,
};

use super::{
    handle_command_text, socket_handshake::authenticate_connection, WebsocketCommandText,
    WebsocketSocketRuntime,
};

enum SocketLoopControl {
    Continue,
    Break,
}

pub(super) fn handle_socket(
    mut socket: WebSocket,
    runtime: WebsocketSocketRuntime,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(async move {
        let Some(authenticated) = authenticate_connection(&mut socket, &runtime.admission).await
        else {
            return;
        };
        if send_event(&mut socket, ready_event()).await.is_err() {
            return;
        }

        loop {
            let Some(message) = receive_message(&mut socket).await else {
                break;
            };

            if matches!(
                handle_socket_message(&mut socket, message, &runtime, &authenticated,).await,
                SocketLoopControl::Break
            ) {
                break;
            }
        }
    })
}

async fn receive_message(socket: &mut WebSocket) -> Option<Message> {
    match socket.recv().await {
        Some(Ok(message)) => Some(message),
        Some(Err(_)) | None => None,
    }
}

fn handle_socket_message<'a>(
    socket: &'a mut WebSocket,
    message: Message,
    runtime: &'a WebsocketSocketRuntime,
    authenticated: &'a ocentra_family_identity_core::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession,
) -> Pin<Box<dyn Future<Output = SocketLoopControl> + Send + 'a>> {
    Box::pin(async move {
        match message {
            Message::Text(text) => {
                handle_authenticated_command(
                    socket,
                    WebsocketCommandText(text.to_string()),
                    runtime,
                    authenticated,
                )
                .await
            }
            Message::Ping(bytes) => send_socket_message(socket, Message::Pong(bytes)).await,
            Message::Close(_) => SocketLoopControl::Break,
            _ => SocketLoopControl::Continue,
        }
    })
}

async fn handle_authenticated_command(
    socket: &mut WebSocket,
    command: WebsocketCommandText,
    runtime: &WebsocketSocketRuntime,
    authenticated: &AuthenticatedParentLocalBridgeSession,
) -> SocketLoopControl {
    if runtime.admission.revalidate(authenticated).is_err() {
        return SocketLoopControl::Break;
    }
    let event = handle_command_text(command, runtime.command.clone()).await;
    send_event(socket, event)
        .await
        .map_or(SocketLoopControl::Break, |_| SocketLoopControl::Continue)
}

async fn send_socket_message(socket: &mut WebSocket, message: Message) -> SocketLoopControl {
    socket
        .send(message)
        .await
        .map_or(SocketLoopControl::Break, |_| SocketLoopControl::Continue)
}

fn ready_event() -> AgentEventEnvelope {
    build_event(
        constants::event_id::CONNECTION_READY,
        constants::event_id::CONNECTION_READY,
        portal_peer(),
        AgentEventName::AgentConnectionReady,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ONLINE,
            LogFieldValue::Boolean(true),
        )]),
        Some(build_dev_log_snapshot()),
    )
}

async fn send_event(socket: &mut WebSocket, event: AgentEventEnvelope) -> Result<(), axum::Error> {
    let text = serde_json::to_string(&event).map_err(axum::Error::new)?;
    socket.send(Message::Text(text.into())).await
}
