use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{
    activity_api::{activity_store_error_event, load_browser_evidence_read_model},
    browser_runtime_stream_payload::{
        browser_runtime_event_chain_stream_payload,
        stream_browser_runtime_event_chain_for_read_model,
    },
    event_builder::build_event,
};

pub async fn build_browser_runtime_event_chain_stream_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_browser_evidence_read_model().await {
        Some(read_model) => {
            let stream = stream_browser_runtime_event_chain_for_read_model(&read_model).await;
            build_event(
                constants::event_id::BROWSER_RUNTIME_EVENT_CHAIN_STREAM_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentBrowserRuntimeEventChainStreamReported,
                LogLevel::Info,
                browser_runtime_event_chain_stream_payload(&stream),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::BROWSER_RUNTIME_EVENT_CHAIN_STREAM_REPORTED,
            AgentEventName::AgentBrowserRuntimeEventChainStreamReported,
        ),
    }
}
