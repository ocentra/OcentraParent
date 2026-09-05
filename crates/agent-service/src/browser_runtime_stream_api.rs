#[path = "browser_runtime_stream_api/child_status.rs"]
mod child_status;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    activity_api::{
        activity_store_error_event::activity_store_error_event, load_browser_evidence_read_model,
        ActivityEventId,
    },
    browser_runtime_stream_payload::browser_runtime_event_chain_stream_payload,
    browser_runtime_stream_request::request_browser_runtime_service_stream_report,
    event_builder::build_event,
    policy_preview_api::load_policy_preview_read_model,
};

pub async fn build_browser_runtime_event_chain_stream_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_browser_evidence_read_model().await {
        Some(read_model) => {
            let policy_preview = load_policy_preview_read_model().await;
            let stream = request_browser_runtime_service_stream_report(read_model, policy_preview)
                .await
                .unwrap_or_default();
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
            ActivityEventId(constants::event_id::BROWSER_RUNTIME_EVENT_CHAIN_STREAM_REPORTED),
            AgentEventName::AgentBrowserRuntimeEventChainStreamReported,
        ),
    }
}
pub(crate) type BrowserRuntimeActionIntentChildStatusResponse =
    child_status::BrowserRuntimeActionIntentChildStatusResponse;

pub(crate) async fn action_intent_child_status_from_handoff(
    handoff: &ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeActionIntentHandoffResponse,
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    child_status::action_intent_child_status_from_handoff(handoff).await
}
