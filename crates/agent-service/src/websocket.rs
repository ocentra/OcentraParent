use axum::extract::ws::{Message, WebSocket};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    LogFieldValue, LogLevel,
};

use crate::{
    activity_api::{
        build_activity_ingest_status_report, build_activity_recent_summary_report,
        build_browser_evidence_recent_report, build_network_flow_read_model_report,
    },
    browser_runtime::build_browser_managed_status_report,
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    local_ai_chat_generation::build_local_ai_chat_generation_report,
    local_ai_runtime_status::build_local_ai_runtime_status_report,
    policy_preview_api::build_policy_preview_read_model_report,
    snapshot::build_dev_log_snapshot,
};

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
                let event = handle_command_text(text.as_str()).await;
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

async fn handle_command_text(text: &str) -> AgentEventEnvelope {
    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => handle_command(command).await,
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

async fn handle_command(command: AgentCommandEnvelope) -> AgentEventEnvelope {
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
        AgentCommandName::AgentActivityIngestStatusGet => {
            build_activity_ingest_status_report(command).await
        }
        AgentCommandName::AgentActivityRecentSummaryGet => {
            build_activity_recent_summary_report(command).await
        }
        AgentCommandName::AgentBrowserEvidenceRecentGet => {
            build_browser_evidence_recent_report(command).await
        }
        AgentCommandName::AgentBrowserManagedBridgePoll => {
            build_browser_managed_status_report(command).await
        }
        AgentCommandName::AgentNetworkFlowReadModelGet => {
            build_network_flow_read_model_report(command).await
        }
        AgentCommandName::AgentLocalAiRuntimeStatusGet => {
            build_local_ai_runtime_status_report(command).await
        }
        AgentCommandName::AgentLocalAiChatGenerate => {
            build_local_ai_chat_generation_report(command).await
        }
        AgentCommandName::AgentPolicyPreviewReadModelGet => {
            build_policy_preview_read_model_report(command).await
        }
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
