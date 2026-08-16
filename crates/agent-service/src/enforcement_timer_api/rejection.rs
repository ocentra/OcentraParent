use ocentra_parent_agent_protocol::constants;

use crate::enforcement_timer_payload::EnforcementTimerPayloadError;
use crate::enforcement_timer_report::TimerReportError;

use super::command::EnforcementTimerCommandError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EnforcementTimerRejectionReason(pub(super) &'static str);

pub(super) fn timer_command_rejection_reason(
    error: &EnforcementTimerCommandError,
) -> EnforcementTimerRejectionReason {
    let value = match error {
        EnforcementTimerCommandError::CommandPayloadInvalid => {
            constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID
        }
        EnforcementTimerCommandError::ActiveTimerStateMismatch => {
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_MISMATCH
        }
        EnforcementTimerCommandError::ParentActionRequired => {
            constants::enforcement::REJECTION_PARENT_ACTION_REQUIRED
        }
        EnforcementTimerCommandError::ActiveTimerStateRequired => {
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED
        }
        EnforcementTimerCommandError::Payload(
            EnforcementTimerPayloadError::ParentActionRequired,
        ) => constants::enforcement::REJECTION_PARENT_ACTION_REQUIRED,
        EnforcementTimerCommandError::Payload(EnforcementTimerPayloadError::ProcessIdRequired) => {
            constants::enforcement::REJECTION_PROCESS_ID_REQUIRED
        }
        EnforcementTimerCommandError::Report(TimerReportError::Serialize) => {
            constants::error::AGENT_EVENT_SERIALIZES
        }
        EnforcementTimerCommandError::Report(TimerReportError::Store) => {
            constants::value::ACTIVITY_CAPTURE_STORE_ERROR
        }
        EnforcementTimerCommandError::AppTimeLimitTarget(rejection) => rejection.as_protocol_str(),
        EnforcementTimerCommandError::AppGameSessionEvidence(rejection) => {
            super::app_game_session_rejection::rejection_reason(*rejection).0
        }
    };
    EnforcementTimerRejectionReason(value)
}
