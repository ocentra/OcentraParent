use ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeActionIntentHandoffResponse;
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct BrowserRuntimeActionIntentChildStatusResponse {
    pub(crate) accepted_row_count: usize,
    pub(crate) child_command_ref: Option<String>,
    pub(crate) child_accepted_event_ref: Option<String>,
    pub(crate) parent_read_model_ref: Option<String>,
    pub(crate) dispatch_attempt_count: u8,
    pub(crate) adapter_execution_count: u8,
    pub(crate) child_intervention_execution_count: u8,
    pub(crate) enforcement_execution_count: u8,
}

pub(crate) async fn action_intent_child_status_from_handoff(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    if !handoff_is_child_status_candidate(handoff) {
        return Some(BrowserRuntimeActionIntentChildStatusResponse::default());
    }

    // This handoff contains a policy preview and action-intent identity, but
    // no trusted parent profile, device, or observation context. Keep child
    // delivery unavailable/manual-required until that typed authority arrives;
    // synthesizing parent-child events here would falsely claim acceptance.
    None
}

fn handoff_is_child_status_candidate(handoff: &BrowserRuntimeActionIntentHandoffResponse) -> bool {
    handoff.candidate_count > 0
        && handoff.dispatch_attempt_count == 0
        && handoff.adapter_execution_count == 0
        && handoff.browser_mutation_count == 0
        && handoff.child_intervention_execution_count == 0
        && handoff.enforcement_execution_count == 0
}
