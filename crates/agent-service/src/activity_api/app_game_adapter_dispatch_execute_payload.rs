use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::enforcement_api::EnforcementJournalPaths;
use crate::{event_builder::build_event, fields::fields_from_pairs};

#[derive(Clone, Debug, PartialEq, Eq)]
struct DispatchText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DispatchReason(&'static str);

pub async fn build_activity_app_game_adapter_dispatch_execute_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_adapter_dispatch_execute_report_with_paths(
        command,
        EnforcementJournalPaths::from_environment(),
    )
    .await
}

pub(crate) async fn build_activity_app_game_adapter_dispatch_execute_report_with_paths(
    command: AgentCommandEnvelope,
    _paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    if command.target.platform != constants::enforcement::PLATFORM_WINDOWS {
        return dispatch_execute_rejected(
            command,
            DispatchReason(constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY),
        );
    }
    dispatch_execute_rejected(
        command,
        DispatchReason(constants::enforcement::REJECTION_APP_GAME_SESSION_EVIDENCE_REQUIRED),
    )
}

fn dispatch_execute_rejected(
    command: AgentCommandEnvelope,
    reason: DispatchReason,
) -> AgentEventEnvelope {
    dispatch_execute_rejected_from_parts(
        &DispatchText(command.message_id.to_string()),
        command.source,
        reason,
    )
}

fn dispatch_execute_rejected_from_parts(
    correlation_id: &DispatchText,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    reason: DispatchReason,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &correlation_id.0,
        target,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(reason.0.to_string()),
        )]),
        None,
    )
}
