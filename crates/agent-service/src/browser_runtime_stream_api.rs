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
use crate::browser_runtime_stream_events::BrowserRuntimeText;
use ocentra_parent_agent_core::parent_child_event_runtime::publish_parent_child_runtime_for_validated_intent;
use ocentra_parent_agent_protocol::child_agent::child_agent_events::ChildCommandKind;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentControllerActionKind, ParentControllerSource,
};
use ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

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
    let report = publish_parent_child_runtime_for_validated_intent(
        parent_child_input_from_handoff(handoff)?,
    )
    .await
    .ok()?;
    let payloads = report
        .stored_events
        .iter()
        .filter_map(|event| {
            event
                .decode::<ParentChildRuntimeEventPayload>()
                .ok()
                .map(|envelope| envelope.payload)
        })
        .collect::<Vec<_>>();
    child_status_response_from_payloads(handoff, &payloads)
}

fn handoff_is_child_status_candidate(handoff: &BrowserRuntimeActionIntentHandoffResponse) -> bool {
    handoff.candidate_count > 0
        && handoff.dispatch_attempt_count == 0
        && handoff.adapter_execution_count == 0
        && handoff.browser_mutation_count == 0
        && handoff.child_intervention_execution_count == 0
        && handoff.enforcement_execution_count == 0
}

fn parent_child_input_from_handoff(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
) -> Option<ParentChildRuntimeInput> {
    Some(ParentChildRuntimeInput {
        parent_intent_ref: handoff.action_intent_id.clone()?,
        parent_profile_ref: constants::parent_controller::TEST_PARENT_PROFILE_REF.to_string(),
        device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        action_kind: ParentControllerActionKind::Review,
        source: ParentControllerSource::PortalTypedIntent,
        child_command_kind: ChildCommandKind::BrowserActionIntentHandoff,
    })
}

fn child_status_response_from_payloads(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
    payloads: &[ParentChildRuntimeEventPayload],
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    let child_command_ref = child_command_ref(payloads)?;
    if !child_command_ref
        .0
        .contains(handoff.action_intent_id.as_deref()?)
    {
        return None;
    }
    Some(BrowserRuntimeActionIntentChildStatusResponse {
        accepted_row_count: 1,
        child_command_ref: Some(child_command_ref.into()),
        child_accepted_event_ref: child_accepted_event_ref(payloads).map(Into::into),
        parent_read_model_ref: parent_read_model_ref(payloads).map(Into::into),
        ..BrowserRuntimeActionIntentChildStatusResponse::default()
    })
}

fn child_command_ref(payloads: &[ParentChildRuntimeEventPayload]) -> Option<BrowserRuntimeText> {
    payloads.iter().find_map(|payload| match payload {
        ParentChildRuntimeEventPayload::ChildCommandReceived(event)
            if event.command_kind == ChildCommandKind::BrowserActionIntentHandoff =>
        {
            Some(BrowserRuntimeText(event.child_command_ref.clone()))
        }
        _ => None,
    })
}

fn child_accepted_event_ref(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Option<BrowserRuntimeText> {
    payloads.iter().find_map(|payload| {
        let ParentChildRuntimeEventPayload::ChildCommandAccepted(event) = payload else {
            return None;
        };
        Some(BrowserRuntimeText(event.command_accepted_event_ref.clone()))
    })
}

fn parent_read_model_ref(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Option<BrowserRuntimeText> {
    payloads.iter().find_map(|payload| {
        let ParentChildRuntimeEventPayload::ParentReadModelProjected(event) = payload else {
            return None;
        };
        event
            .visible_to_portal
            .then(|| BrowserRuntimeText(event.read_model_ref.clone()))
    })
}
