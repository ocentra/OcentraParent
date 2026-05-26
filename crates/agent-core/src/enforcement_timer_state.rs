use ocentra_parent_agent_protocol::{
    constants::enforcement as enforcement_constants, EnforcementAction,
    EnforcementActiveTimerState, EnforcementAdapterResultCode, EnforcementAuditEvent,
    EnforcementAuditEventKind, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementTimerEvent, EnforcementTimerEventKind,
    ParentActionReference,
};

use crate::enforcement_boundary::EnforcementBoundaryOutcome;

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
    let timer_event = outcome.timer_event.clone()?;
    active_timer_event(&timer_event, &outcome.result).then(|| EnforcementActiveTimerState {
        schema_version: outcome.action.schema_version.clone(),
        state_id: active_timer_state_id(&outcome.action.action_id),
        action: outcome.action.clone(),
        result: outcome.result.clone(),
        audit_event: outcome.audit_event.clone(),
        timer_event,
        stored_at: stored_at.to_string(),
    })
}

pub fn restart_recovered_timer_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
) -> EnforcementBoundaryOutcome {
    transition_outcome(
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
    transition_outcome(
        state,
        ids,
        EnforcementTimerEventKind::Cancelled,
        EnforcementResultStatus::Superseded,
        Some(parent_override),
    )
}

fn active_timer_event(timer_event: &EnforcementTimerEvent, result: &EnforcementResult) -> bool {
    matches!(
        timer_event.timer_event_kind,
        EnforcementTimerEventKind::Created
            | EnforcementTimerEventKind::Extended
            | EnforcementTimerEventKind::RestartRecovered
    ) && matches!(
        result.status,
        EnforcementResultStatus::WouldEnforce
            | EnforcementResultStatus::ActuallyEnforced
            | EnforcementResultStatus::NoOp
    )
}

fn transition_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
    status: EnforcementResultStatus,
    parent_override: Option<ParentActionReference>,
) -> EnforcementBoundaryOutcome {
    let action = transition_action(&state.action, parent_override.clone());
    let result = transition_result(&action, &ids, status);
    let timer_event = transition_timer_event(&action, &state.timer_event, &ids, timer_event_kind);
    let audit_event = transition_audit_event(
        &action,
        &result,
        &ids,
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
    action: &EnforcementAction,
    parent_override: Option<ParentActionReference>,
) -> EnforcementAction {
    let mut transition = action.clone();
    if parent_override.is_some() {
        transition.parent_approval = parent_override;
    }
    transition
}

fn transition_result(
    action: &EnforcementAction,
    ids: &EnforcementTimerTransitionIds,
    status: EnforcementResultStatus,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: action.schema_version.clone(),
        result_id: ids.result_id.clone(),
        action_id: action.action_id.clone(),
        status,
        adapter_result_code: EnforcementAdapterResultCode::NoOp,
        started_at: ids.observed_at.clone(),
        completed_at: Some(ids.observed_at.clone()),
        rollback_token: action.rollback_token.clone(),
        rollback_state: EnforcementRollbackState::NotRequired,
        unavailable_reason: None,
        unavailable_status: None,
        failed_reason: None,
        next_check_at: next_check_at(action, status),
        capability: action.capability.clone(),
    }
}

fn transition_timer_event(
    action: &EnforcementAction,
    previous_timer: &EnforcementTimerEvent,
    ids: &EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
) -> EnforcementTimerEvent {
    EnforcementTimerEvent {
        schema_version: action.schema_version.clone(),
        timer_event_id: ids.timer_event_id.clone(),
        timer_event_kind,
        action_id: action.action_id.clone(),
        policy_decision_id: action.policy_decision_id.clone(),
        evidence_references: action.evidence_references.clone(),
        scheduled_at: previous_timer.scheduled_at.clone(),
        effective_at: timer_effective_at(action, timer_event_kind),
        rollback_token: action.rollback_token.clone(),
        recovered_after_restart: timer_event_kind == EnforcementTimerEventKind::RestartRecovered,
        unavailable_reason: None,
    }
}

fn transition_audit_event(
    action: &EnforcementAction,
    result: &EnforcementResult,
    ids: &EnforcementTimerTransitionIds,
    previous_policy_version: &str,
    parent_override: Option<ParentActionReference>,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: action.schema_version.clone(),
        audit_event_id: ids.audit_event_id.clone(),
        audit_event_kind: audit_kind(result.status),
        action: action.clone(),
        result: result.clone(),
        capability: result.capability.clone(),
        unavailable_status: None,
        policy_version: parent_override
            .as_ref()
            .map(|reference| reference.policy_version.clone())
            .unwrap_or_else(|| previous_policy_version.to_string()),
        evidence_references: action.evidence_references.clone(),
        actor: parent_override
            .as_ref()
            .map(|reference| reference.actor.clone()),
        parent_override,
        journal_sequence: None,
        observed_at: ids.observed_at.clone(),
    }
}

fn active_timer_state_id(action_id: &str) -> String {
    let mut value = String::from(enforcement_constants::TIMER_STATE_ID_PREFIX);
    value.push_str(action_id);
    value
}

fn timer_effective_at(
    action: &EnforcementAction,
    timer_event_kind: EnforcementTimerEventKind,
) -> Option<String> {
    match timer_event_kind {
        EnforcementTimerEventKind::RestartRecovered => action.expires_at.clone(),
        EnforcementTimerEventKind::Cancelled => None,
        _ => None,
    }
}

fn next_check_at(action: &EnforcementAction, status: EnforcementResultStatus) -> Option<String> {
    match status {
        EnforcementResultStatus::NoOp => action.expires_at.clone(),
        _ => None,
    }
}

fn audit_kind(status: EnforcementResultStatus) -> EnforcementAuditEventKind {
    match status {
        EnforcementResultStatus::Superseded => EnforcementAuditEventKind::Cancelled,
        _ => EnforcementAuditEventKind::Attempted,
    }
}
