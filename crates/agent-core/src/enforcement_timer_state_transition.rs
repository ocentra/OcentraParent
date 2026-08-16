use ocentra_parent_agent_protocol::enforcement::{
    EnforcementActiveTimerState, EnforcementResultStatus, EnforcementTimerEventKind,
};

use crate::enforcement_boundary::EnforcementBoundaryOutcome;

use super::enforcement_timer_state_audit::transition_audit_event;
use super::enforcement_timer_state_event::transition_timer_event;
use super::enforcement_timer_state_result::{transition_result, TransitionResultOverride};
use super::EnforcementTimerTransitionIds;

pub(super) fn transition_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
    status: EnforcementResultStatus,
    parent_override: Option<ocentra_parent_agent_protocol::enforcement::ParentActionReference>,
) -> EnforcementBoundaryOutcome {
    transition_outcome_with_result(
        state,
        ids,
        timer_event_kind,
        status,
        TransitionResultOverride {
            adapter_result_code:
                ocentra_parent_agent_protocol::enforcement::EnforcementAdapterResultCode::NoOp,
            rollback_state:
                ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState::NotRequired,
            unavailable_reason: None,
            failed_reason: None,
            rollback_token: None,
            parent_override,
        },
    )
}

pub(super) fn transition_outcome_with_result(
    state: &EnforcementActiveTimerState,
    ids: impl Borrow<EnforcementTimerTransitionIds>,
    timer_event_kind: EnforcementTimerEventKind,
    status: EnforcementResultStatus,
    result_override: impl Borrow<TransitionResultOverride>,
) -> EnforcementBoundaryOutcome {
    let ids = ids.borrow();
    let result_override = result_override.borrow();
    let parent_override = result_override.parent_override.clone();
    let action = transition_action(&state.action, parent_override.clone());
    let result = transition_result(&action, ids, status, result_override);
    let timer_event = transition_timer_event(
        &action,
        &state.timer_event,
        ids,
        timer_event_kind,
        super::enforcement_timer_state_result::unavailable_reason_from_transition(result_override),
    );
    let audit_event = transition_audit_event(
        &action,
        &result,
        ids,
        &state.audit_event.policy_version,
        parent_override,
    );

    EnforcementBoundaryOutcome {
        action,
        result,
        audit_event,
        timer_event: Some(timer_event),
        adapter_request: None,
    }
}

fn transition_action(
    action: &ocentra_parent_agent_protocol::enforcement::EnforcementAction,
    parent_override: Option<ocentra_parent_agent_protocol::enforcement::ParentActionReference>,
) -> ocentra_parent_agent_protocol::enforcement::EnforcementAction {
    let mut transition = action.clone();
    if parent_override.is_some() {
        transition.parent_approval = parent_override;
    }
    transition
}
use std::borrow::Borrow;
