use ocentra_parent_agent_core::{
    enforcement_boundary::EnforcementBoundaryOutcome,
    enforcement_timer_state::active_timer_state_from_outcome,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;

use super::{recovery_store_error, EnforcementRetryRecoveryError};
use crate::enforcement_api::enforcement_command_execution::EnforcementJournalPaths;

pub(super) async fn recovered_active_state(
    paths: &EnforcementJournalPaths,
    outcome: &EnforcementBoundaryOutcome,
) -> Result<Option<EnforcementActiveTimerState>, EnforcementRetryRecoveryError> {
    let state =
        crate::enforcement_timer_state_file::read_active_timer_state(&paths.timer_state_path)
            .await
            .map_err(recovery_store_error)?;
    if active_timer_state_from_outcome(outcome, constants::value::EMPTY).is_none() {
        return Ok(None);
    }
    match state {
        Some(state)
            if state.action == outcome.action
                && state.result == outcome.result
                && state.audit_event == outcome.audit_event
                && outcome.timer_event.as_ref() == Some(&state.timer_event) =>
        {
            Ok(Some(state))
        }
        Some(_) | None => Err(EnforcementRetryRecoveryError::ReconciliationRequired),
    }
}
