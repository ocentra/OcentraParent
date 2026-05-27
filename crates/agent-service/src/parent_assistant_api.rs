use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, LogFieldValue, LogLevel,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub fn build_parent_assistant_scaffold_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let event = parent_assistant_event_for_command(&command.command);
    build_event(
        parent_assistant_event_id_for_command(&command.command),
        &command.message_id,
        command.source,
        event,
        LogLevel::Warn,
        fields_from_pairs(vec![
            (
                constants::field::SCHEMA_VERSION,
                LogFieldValue::String(policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string()),
            ),
            (
                constants::field::PARENT_ASSISTANT_BACKEND_STATE,
                LogFieldValue::String(
                    constants::parent_assistant::BACKEND_STATE_SCAFFOLD_ONLY.to_string(),
                ),
            ),
            (
                constants::field::REASON,
                LogFieldValue::String(
                    constants::parent_assistant::BACKEND_NOT_CONNECTED.to_string(),
                ),
            ),
        ]),
        None,
    )
}

fn parent_assistant_event_for_command(command: &AgentCommandName) -> AgentEventName {
    match command {
        AgentCommandName::AgentParentAssistantThreadList
        | AgentCommandName::AgentParentAssistantThreadCreate
        | AgentCommandName::AgentParentAssistantThreadOpen
        | AgentCommandName::AgentParentAssistantThreadArchive => {
            AgentEventName::AgentParentAssistantThreadUpdated
        }
        AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart => {
            AgentEventName::AgentParentAssistantMessageAccepted
        }
        AgentCommandName::AgentParentAssistantActionPreview => {
            AgentEventName::AgentParentAssistantActionPreviewed
        }
        AgentCommandName::AgentParentAssistantActionConfirm => {
            AgentEventName::AgentParentAssistantActionConfirmed
        }
        AgentCommandName::AgentParentAssistantProviderStatusGet => {
            AgentEventName::AgentParentAssistantProviderDegraded
        }
        AgentCommandName::AgentParentAssistantRunCancel => {
            AgentEventName::AgentParentAssistantErrorReported
        }
        _ => AgentEventName::AgentParentAssistantErrorReported,
    }
}

fn parent_assistant_event_id_for_command(command: &AgentCommandName) -> &'static str {
    match command {
        AgentCommandName::AgentParentAssistantThreadList
        | AgentCommandName::AgentParentAssistantThreadCreate
        | AgentCommandName::AgentParentAssistantThreadOpen
        | AgentCommandName::AgentParentAssistantThreadArchive => {
            constants::event_id::PARENT_ASSISTANT_THREAD_UPDATED
        }
        AgentCommandName::AgentParentAssistantMessageSend
        | AgentCommandName::AgentParentAssistantQuickActionStart => {
            constants::event_id::PARENT_ASSISTANT_MESSAGE_ACCEPTED
        }
        AgentCommandName::AgentParentAssistantActionPreview => {
            constants::event_id::PARENT_ASSISTANT_ACTION_PREVIEWED
        }
        AgentCommandName::AgentParentAssistantActionConfirm => {
            constants::event_id::PARENT_ASSISTANT_ACTION_CONFIRMED
        }
        AgentCommandName::AgentParentAssistantProviderStatusGet => {
            constants::event_id::PARENT_ASSISTANT_PROVIDER_DEGRADED
        }
        AgentCommandName::AgentParentAssistantRunCancel => {
            constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED
        }
        _ => constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED,
    }
}
