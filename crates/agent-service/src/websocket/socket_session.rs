use axum::extract::ws::{Message, WebSocket};
use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{AgentEventEnvelope, AgentEventName},
};
use std::{future::Future, pin::Pin};

use crate::{
    browser_policy_runtime::BrowserPolicyRuntime,
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::LanPairingRuntime,
    screen_settings_runtime::ScreenSettingsRuntime,
    snapshot::build_dev_log_snapshot,
};

use super::{command_entry::handle_command_text, WebsocketCommandOrigin, WebsocketCommandText};

enum SocketLoopControl {
    Continue,
    Break,
}

pub(super) fn handle_socket(
    mut socket: WebSocket,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(async move {
        if send_event(&mut socket, ready_event()).await.is_err() {
            return;
        }

        loop {
            let Some(message) = receive_message(&mut socket).await else {
                break;
            };

            if matches!(
                handle_socket_message(
                    &mut socket,
                    message,
                    &lan_pairing,
                    &browser_policy,
                    &screen_settings,
                    &origin,
                )
                .await,
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
    lan_pairing: &'a LanPairingRuntime,
    browser_policy: &'a BrowserPolicyRuntime,
    screen_settings: &'a ScreenSettingsRuntime,
    origin: &'a WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = SocketLoopControl> + Send + 'a>> {
    Box::pin(async move {
        match message {
            Message::Text(text) => {
                let event = handle_command_text(
                    WebsocketCommandText(text.to_string()),
                    lan_pairing.clone(),
                    browser_policy.clone(),
                    screen_settings.clone(),
                    origin.clone(),
                )
                .await;
                send_event(socket, event)
                    .await
                    .map_or(SocketLoopControl::Break, |_| SocketLoopControl::Continue)
            }
            Message::Ping(bytes) => send_socket_message(socket, Message::Pong(bytes)).await,
            Message::Close(_) => SocketLoopControl::Break,
            _ => SocketLoopControl::Continue,
        }
    })
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
