use ocentra_parent_agent_protocol::enforcement::{
    AppGameTimerSessionBinding, EnforcementActiveTimerState, EnforcementResultStatus,
    EnforcementTimerEventKind, ParentActionReference,
};

use crate::enforcement_boundary::EnforcementBoundaryOutcome;

#[path = "enforcement_timer_state_audit.rs"]
mod enforcement_timer_state_audit;
#[path = "enforcement_timer_state_event.rs"]
mod enforcement_timer_state_event;
#[path = "enforcement_timer_state_helpers.rs"]
mod enforcement_timer_state_helpers;
#[path = "enforcement_timer_state_result.rs"]
mod enforcement_timer_state_result;
#[path = "enforcement_timer_state_transition.rs"]
mod enforcement_timer_state_transition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementTimerTransitionIds {
    pub result_id: String,
    pub audit_event_id: String,
    pub timer_event_id: String,
    pub observed_at: String,
}

pub fn active_timer_state_from_outcome(
    outcome: &EnforcementBoundaryOutcome,
    stored_at: &str,
) -> Option<EnforcementActiveTimerState> {
    active_timer_state_from_outcome_with_app_game_session(outcome, stored_at, None)
}

pub fn active_timer_state_from_outcome_with_app_game_session(
    outcome: &EnforcementBoundaryOutcome,
    stored_at: &str,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> Option<EnforcementActiveTimerState> {
    let timer_event = outcome.timer_event.clone()?;
    enforcement_timer_state_helpers::active_timer_event(&timer_event, &outcome.result).then(|| {
        EnforcementActiveTimerState {
            schema_version: outcome.action.schema_version.clone(),
            state_id: enforcement_timer_state_helpers::active_timer_state_id(
                &outcome.action.action_id,
            ),
            action: outcome.action.clone(),
            result: outcome.result.clone(),
            audit_event: outcome.audit_event.clone(),
            timer_event,
            stored_at: stored_at.to_string(),
            app_game_session,
        }
    })
}

pub fn restart_recovered_timer_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
) -> EnforcementBoundaryOutcome {
    enforcement_timer_state_transition::transition_outcome(
        state,
        ids,
        EnforcementTimerEventKind::RestartRecovered,
        EnforcementResultStatus::NoOp,
        None,
    )
}

pub fn cancelled_timer_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    parent_override: ParentActionReference,
) -> EnforcementBoundaryOutcome {
    enforcement_timer_state_transition::transition_outcome(
        state,
        ids,
        EnforcementTimerEventKind::Cancelled,
        EnforcementResultStatus::Superseded,
        Some(parent_override),
    )
}

pub fn expired_timer_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    adapter_outcome: crate::enforcement_adapter::EnforcementAdapterOutcome,
) -> EnforcementBoundaryOutcome {
    enforcement_timer_state_transition::transition_outcome_with_result(
        state,
        ids,
        enforcement_timer_state_event::timer_event_kind_for_expiry(adapter_outcome.status),
        adapter_outcome.status,
        enforcement_timer_state_result::TransitionResultOverride {
            adapter_result_code: adapter_outcome.adapter_result_code,
            rollback_state: adapter_outcome.rollback_state,
            unavailable_reason: adapter_outcome.unavailable_reason,
            failed_reason: adapter_outcome.failed_reason,
            rollback_token: adapter_outcome.rollback_token,
            parent_override: None,
        },
    )
}
