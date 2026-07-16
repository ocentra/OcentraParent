use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;

use crate::enforcement_timer_payload::EnforcementTimerCommandPayload;

use super::command::EnforcementTimerCommandError;

pub(super) fn validate_expected_action(
    request: &EnforcementTimerCommandPayload,
    state: &EnforcementActiveTimerState,
) -> Result<(), EnforcementTimerCommandError> {
    match request.expected_action_id.as_ref() {
        Some(action_id) if action_id.0 != state.action.action_id => {
            Err(EnforcementTimerCommandError::ActiveTimerStateMismatch)
        }
        _ => Ok(()),
    }
}
