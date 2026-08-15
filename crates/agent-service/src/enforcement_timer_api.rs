use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

#[path = "enforcement_timer_api/app_game_session_rejection.rs"]
mod app_game_session_rejection;
#[path = "enforcement_timer_api/command.rs"]
mod command;
#[path = "enforcement_timer_api/rejection.rs"]
mod rejection;
#[path = "enforcement_timer_api/state_error.rs"]
mod state_error;
#[path = "enforcement_timer_api/validation.rs"]
mod validation;

use crate::{
    enforcement_api::EnforcementJournalPaths, event_builder::build_event, fields::fields_from_pairs,
};

pub async fn build_enforcement_timer_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_enforcement_timer_report_with_paths(command, EnforcementJournalPaths::from_environment())
        .await
}

pub(crate) async fn build_enforcement_timer_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    match command::execute_timer_command(command, paths).await {
        Ok(payload) => build_event(
            constants::event_id::ENFORCEMENT_TIMER_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentEnforcementTimerReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(error) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    rejection::timer_command_rejection_reason(&error)
                        .0
                        .to_string(),
                ),
            )]),
            None,
        ),
    }
}
