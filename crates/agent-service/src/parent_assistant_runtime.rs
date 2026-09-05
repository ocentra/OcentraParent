use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswerState;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

#[path = "parent_assistant_runtime/core.rs"]
mod core;

use crate::activity_surface_store::local_store_snapshot;
use crate::event_builder::build_event;
use crate::local_ai_provider_scheduler::local_ai_provider_scheduler;
use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;
use crate::parent_assistant_activity_snapshot::ParentAssistantActivitySnapshot;
use crate::parent_assistant_answer_payload::parent_assistant_answer_payload;
use crate::parent_assistant_api::thread_store;
use crate::parent_assistant_report_history::activity_report_history_from_command;

pub async fn build_parent_assistant_answer_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let config = tokio::task::spawn_blocking(LocalAiRuntimeConfigSnapshot::from_environment)
        .await
        .unwrap_or_else(|_| LocalAiRuntimeConfigSnapshot::unconfigured());
    let snapshot = local_store_snapshot().await.map(parent_assistant_snapshot);
    let stored_report_history = activity_report_history_from_command(&command).await;
    let request = core::request_from_command(&command, &config, snapshot, stored_report_history);
    let answer = core::generate_parent_assistant_answer_with_scheduler(
        &command,
        request,
        &config,
        local_ai_provider_scheduler(),
    )
    .await;
    thread_store::record_message_for_thread(thread_store::ParentAssistantThreadId(
        answer.thread_id.clone(),
    ));
    let severity = if answer.answer_state == ParentAssistantAnswerState::Answered {
        LogLevel::Info
    } else {
        LogLevel::Warn
    };

    build_event(
        constants::event_id::PARENT_ASSISTANT_ANSWER_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantAnswerReported,
        severity,
        parent_assistant_answer_payload(&answer),
        None,
    )
}

fn parent_assistant_snapshot(
    snapshot: crate::activity_surface_store::ActivitySurfaceStoreSnapshot,
) -> ParentAssistantActivitySnapshot {
    ParentAssistantActivitySnapshot {
        device_id: snapshot.device_id.0,
        recent_returned: snapshot.recent_returned,
        last_event_id: snapshot.last_event_id,
        last_observed_at: snapshot.last_observed_at,
        browser_returned: snapshot.browser_returned,
        network_returned: snapshot.network_returned,
        games_returned: snapshot.games_returned,
        screen_returned: snapshot.screen_returned,
    }
}
