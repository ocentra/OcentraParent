use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadResponse;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

#[path = "parent_assistant_api/action_results.rs"]
mod action_results;
#[path = "parent_assistant_api/api_boundary.rs"]
pub(crate) mod api_boundary;
#[path = "parent_assistant_api/payload_fields.rs"]
mod payload_fields;
#[path = "parent_assistant_api/provider_status.rs"]
mod provider_status;
#[path = "parent_assistant_api/thread_store.rs"]
pub(crate) mod thread_store;

use crate::event_builder::build_event;
use crate::fields::fields_from_pairs;
use crate::parent_assistant_payload::{
    parent_assistant_action_confirm_payload, parent_assistant_action_preview_payload,
    parent_assistant_provider_status_payload, parent_assistant_run_cancel_payload,
    parent_assistant_thread_payload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParentAssistantText(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantTextRef<'a>(&'a str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantPayloadFieldName(&'static str);

impl ParentAssistantTextRef<'_> {
    fn into_text(self) -> ParentAssistantText {
        ParentAssistantText(self.0.to_string())
    }
}

pub fn build_parent_assistant_scaffold_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command {
        AgentCommandName::AgentParentAssistantThreadList
        | AgentCommandName::AgentParentAssistantThreadCreate
        | AgentCommandName::AgentParentAssistantThreadOpen
        | AgentCommandName::AgentParentAssistantThreadArchive => build_thread_event(command),
        AgentCommandName::AgentParentAssistantProviderStatusGet => {
            build_provider_status_event(command)
        }
        AgentCommandName::AgentParentAssistantRunCancel => build_run_cancel_event(command),
        AgentCommandName::AgentParentAssistantActionPreview => build_action_preview_event(command),
        AgentCommandName::AgentParentAssistantActionConfirm => build_action_confirm_event(command),
        _ => build_scaffold_fallback_event(command),
    }
}

fn build_thread_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let response = thread_response_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_THREAD_UPDATED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantThreadUpdated,
        LogLevel::Info,
        parent_assistant_thread_payload(&response),
        None,
    )
}

fn build_provider_status_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let status = provider_status::provider_status_for_command(&command);
    let severity = if status.provider_state == ParentAssistantProviderState::Configured {
        LogLevel::Info
    } else {
        LogLevel::Warn
    };
    build_event(
        constants::event_id::PARENT_ASSISTANT_PROVIDER_DEGRADED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantProviderDegraded,
        severity,
        parent_assistant_provider_status_payload(&status),
        None,
    )
}

fn build_run_cancel_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let result = action_results::run_cancel_result_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantErrorReported,
        LogLevel::Warn,
        parent_assistant_run_cancel_payload(&result),
        None,
    )
}

fn build_action_confirm_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let result = action_results::action_confirm_result_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_ACTION_CONFIRMED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantActionConfirmed,
        LogLevel::Warn,
        parent_assistant_action_confirm_payload(&result),
        None,
    )
}

fn build_action_preview_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let result = action_results::action_preview_result_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_ACTION_PREVIEWED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantActionPreviewed,
        LogLevel::Info,
        parent_assistant_action_preview_payload(&result),
        None,
    )
}

fn build_scaffold_fallback_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantErrorReported,
        LogLevel::Warn,
        fields_from_pairs(vec![
            (
                ParentAssistantPayloadFieldName(constants::field::SCHEMA_VERSION).0,
                payload_fields::string_field_value(ParentAssistantTextRef(
                    policy::CONTRACT_SCHEMA_VERSION_V0_6,
                )),
            ),
            (
                ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_BACKEND_STATE).0,
                payload_fields::string_field_value(ParentAssistantTextRef(
                    constants::parent_assistant::BACKEND_STATE_SCAFFOLD_ONLY,
                )),
            ),
            (
                ParentAssistantPayloadFieldName(constants::field::REASON).0,
                payload_fields::string_field_value(ParentAssistantTextRef(
                    constants::parent_assistant::BACKEND_NOT_CONNECTED,
                )),
            ),
        ]),
        None,
    )
}

fn thread_response_for_command(command: &AgentCommandEnvelope) -> ParentAssistantThreadResponse {
    thread_store::thread_response_for_command(command)
}
