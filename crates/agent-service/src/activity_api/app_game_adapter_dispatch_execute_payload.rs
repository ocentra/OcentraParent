use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::app_game_dispatch_evidence::validate_app_game_dispatch_evidence;
use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};
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
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    if command.target.platform != constants::enforcement::PLATFORM_WINDOWS {
        return dispatch_execute_rejected(
            command,
            DispatchReason(constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY),
        );
    }
    match validate_app_game_dispatch_evidence(&command.payload, paths.store_path.clone()).await {
        Ok(()) => {
            let mut enforcement_command = command;
            enforcement_command.command =
                ocentra_parent_agent_protocol::transport::AgentCommandName::AgentEnforcementExecute;
            build_enforcement_audit_report_with_paths(enforcement_command, paths).await
        }
        Err(reason) => dispatch_execute_rejected(command, DispatchReason(reason.as_protocol_str())),
    }
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
