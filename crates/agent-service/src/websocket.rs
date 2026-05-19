use axum::extract::ws::{Message, WebSocket};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentLogSnapshot, AgentPeer, AgentPeerRole, LogFieldValue, LogFields, LogLevel,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{fields::fields_from_pairs, snapshot::build_dev_log_snapshot, time::timestamp_now};

pub async fn handle_socket(mut socket: WebSocket) {
    let ready_event = build_event(
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
    );

    if send_event(&mut socket, ready_event).await.is_err() {
        return;
    }

    while let Some(result) = socket.recv().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        match message {
            Message::Text(text) => {
                let event = handle_command_text(text.as_str());
                if send_event(&mut socket, event).await.is_err() {
                    break;
                }
            }
            Message::Ping(bytes) => {
                if socket.send(Message::Pong(bytes)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

fn handle_command_text(text: &str) -> AgentEventEnvelope {
    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => handle_command(command),
        Err(error) => build_event(
            constants::event_id::COMMAND_REJECTED,
            constants::event_id::UNKNOWN_COMMAND,
            portal_peer(),
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(error.to_string()),
            )]),
            None,
        ),
    }
}

fn handle_command(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command {
        AgentCommandName::AgentHealthCheck => build_health_report(command),
        AgentCommandName::AgentLogSnapshotGet => build_log_snapshot_report(command),
        AgentCommandName::AgentDevEcho => build_event(
            constants::event_id::DEV_ECHOED,
            &command.message_id,
            command.source,
            AgentEventName::AgentDevEchoed,
            LogLevel::Info,
            command.payload,
            None,
        ),
        AgentCommandName::AgentWatchStatusGet => build_watcher_status_report(command),
    }
}

fn build_health_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::HEALTH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::ONLINE, LogFieldValue::Boolean(true)),
            (
                constants::field::TRANSPORT,
                LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
            ),
        ]),
        Some(build_dev_log_snapshot()),
    )
}

fn build_log_snapshot_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLogSnapshotReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ENTRIES,
            LogFieldValue::Number(1.0),
        )]),
        Some(build_dev_log_snapshot()),
    )
}

fn build_watcher_status_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::WATCH_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentWatchStatusReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::AVAILABLE, LogFieldValue::Boolean(false)),
            (
                constants::field::NOTE,
                LogFieldValue::String(constants::value::WATCHER_STATUS_ONLY.to_string()),
            ),
        ]),
        None,
    )
}

async fn send_event(socket: &mut WebSocket, event: AgentEventEnvelope) -> Result<(), axum::Error> {
    let text = serde_json::to_string(&event).expect(constants::error::AGENT_EVENT_SERIALIZES);
    socket.send(Message::Text(text.into())).await
}

fn build_event(
    event_id_suffix: &str,
    correlation_id: &str,
    target: AgentPeer,
    event: AgentEventName,
    severity: LogLevel,
    payload: LogFields,
    snapshot: Option<AgentLogSnapshot>,
) -> AgentEventEnvelope {
    let mut event_id = String::from(event_id_suffix);
    event_id.push('-');
    event_id.push_str(&std::process::id().to_string());

    AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id,
        correlation_id: correlation_id.to_string(),
        sent_at: timestamp_now(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target,
        event,
        severity,
        payload,
        snapshot,
    }
}

fn portal_peer() -> AgentPeer {
    AgentPeer {
        peer_id: constants::peer::PORTAL_DEV.to_string(),
        role: AgentPeerRole::Portal,
    }
}

#[cfg(test)]
mod tests {
    use super::{build_event, constants, fields_from_pairs, portal_peer};
    use ocentra_parent_agent_protocol::{AgentEventName, LogFieldValue, LogLevel};

    #[test]
    fn build_event_targets_portal_peer_without_inline_literals() {
        let event = build_event(
            constants::event_id::HEALTH_REPORTED,
            constants::event_id::HEALTH_REPORTED,
            portal_peer(),
            AgentEventName::AgentHealthReported,
            LogLevel::Info,
            fields_from_pairs(vec![(
                constants::field::ONLINE,
                LogFieldValue::Boolean(true),
            )]),
            None,
        );

        assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
        assert!(event.payload.contains_key(constants::field::ONLINE));
    }
}
