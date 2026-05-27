use ocentra_parent_agent_protocol::{
    constants::enforcement as enforcement_constants, EnforcementAction,
    EnforcementActiveTimerState, EnforcementAdapterResultCode, EnforcementAuditEvent,
    EnforcementAuditEventKind, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementTimerEvent, EnforcementTimerEventKind,
    EnforcementUnavailableReason, EnforcementUnavailableStatus, ParentActionReference,
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

pub fn expired_timer_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    adapter_outcome: crate::enforcement_adapter::EnforcementAdapterOutcome,
) -> EnforcementBoundaryOutcome {
    transition_outcome_with_result(
        state,
        ids,
        timer_event_kind_for_expiry(adapter_outcome.status),
        adapter_outcome.status,
        TransitionResultOverride {
            adapter_result_code: adapter_outcome.adapter_result_code,
            rollback_state: adapter_outcome.rollback_state,
            unavailable_reason: adapter_outcome.unavailable_reason,
            failed_reason: adapter_outcome.failed_reason,
            rollback_token: adapter_outcome.rollback_token,
            parent_override: None,
        },
    )
}

struct TransitionResultOverride {
    adapter_result_code: EnforcementAdapterResultCode,
    rollback_state: EnforcementRollbackState,
    unavailable_reason: Option<String>,
    failed_reason: Option<String>,
    rollback_token: Option<String>,
    parent_override: Option<ParentActionReference>,
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
    transition_outcome_with_result(
        state,
        ids,
        timer_event_kind,
        status,
        TransitionResultOverride {
            adapter_result_code: EnforcementAdapterResultCode::NoOp,
            rollback_state: EnforcementRollbackState::NotRequired,
            unavailable_reason: None,
            failed_reason: None,
            rollback_token: None,
            parent_override,
        },
    )
}

fn transition_outcome_with_result(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
    status: EnforcementResultStatus,
    result_override: TransitionResultOverride,
) -> EnforcementBoundaryOutcome {
    let parent_override = result_override.parent_override.clone();
    let action = transition_action(&state.action, parent_override.clone());
    let result = transition_result(&action, &ids, status, &result_override);
    let timer_event = transition_timer_event(
        &action,
        &state.timer_event,
        &ids,
        timer_event_kind,
        unavailable_reason_from_transition(&result_override),
    );
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
    result_override: &TransitionResultOverride,
) -> EnforcementResult {
    let unavailable_status =
        unavailable_reason_from_transition(result_override).map(|unavailable_reason| {
            EnforcementUnavailableStatus {
                schema_version: action.schema_version.clone(),
                capability: action.capability.clone(),
                unavailable_reason,
                retryable: matches!(
                    unavailable_reason,
                    EnforcementUnavailableReason::AdapterUnavailable
                        | EnforcementUnavailableReason::AdapterError
                ),
                checked_at: action.capability.last_checked_at.clone(),
            }
        });

    EnforcementResult {
        schema_version: action.schema_version.clone(),
        result_id: ids.result_id.clone(),
        action_id: action.action_id.clone(),
        status,
        adapter_result_code: result_override.adapter_result_code,
        started_at: ids.observed_at.clone(),
        completed_at: Some(ids.observed_at.clone()),
        rollback_token: result_override
            .rollback_token
            .clone()
            .or_else(|| action.rollback_token.clone()),
        rollback_state: result_override.rollback_state,
        unavailable_reason: result_override.unavailable_reason.clone(),
        unavailable_status,
        failed_reason: result_override.failed_reason.clone(),
        next_check_at: next_check_at(action, status),
        capability: action.capability.clone(),
    }
}

fn transition_timer_event(
    action: &EnforcementAction,
    previous_timer: &EnforcementTimerEvent,
    ids: &EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
    unavailable_reason: Option<EnforcementUnavailableReason>,
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
        unavailable_reason,
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
        unavailable_status: result.unavailable_status.clone(),
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
        EnforcementTimerEventKind::Expired | EnforcementTimerEventKind::RestartRecovered => {
            action.expires_at.clone()
        }
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
        EnforcementResultStatus::Expired => EnforcementAuditEventKind::Expired,
        EnforcementResultStatus::Failed => EnforcementAuditEventKind::Failed,
        EnforcementResultStatus::RolledBack => EnforcementAuditEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementAuditEventKind::Cancelled,
        EnforcementResultStatus::Unavailable => EnforcementAuditEventKind::Unavailable,
        _ => EnforcementAuditEventKind::Attempted,
    }
}

fn timer_event_kind_for_expiry(status: EnforcementResultStatus) -> EnforcementTimerEventKind {
    match status {
        EnforcementResultStatus::Expired => EnforcementTimerEventKind::Expired,
        EnforcementResultStatus::Failed => EnforcementTimerEventKind::RecoveryNeeded,
        EnforcementResultStatus::RolledBack => EnforcementTimerEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementTimerEventKind::Cancelled,
        EnforcementResultStatus::Unavailable => EnforcementTimerEventKind::Unavailable,
        _ => EnforcementTimerEventKind::RecoveryNeeded,
    }
}

fn unavailable_reason_from_transition(
    result_override: &TransitionResultOverride,
) -> Option<EnforcementUnavailableReason> {
    result_override
        .unavailable_reason
        .as_deref()
        .and_then(unavailable_reason_from_protocol_str)
        .or_else(|| {
            (result_override.failed_reason.is_some())
                .then_some(EnforcementUnavailableReason::AdapterError)
        })
}

fn unavailable_reason_from_protocol_str(reason: &str) -> Option<EnforcementUnavailableReason> {
    match reason {
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM => {
            Some(EnforcementUnavailableReason::UnsupportedPlatform)
        }
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION => {
            Some(EnforcementUnavailableReason::UnsupportedAction)
        }
        enforcement_constants::UNAVAILABLE_MISSING_PERMISSION => {
            Some(EnforcementUnavailableReason::MissingPermission)
        }
        enforcement_constants::UNAVAILABLE_MISSING_DEPENDENCY => {
            Some(EnforcementUnavailableReason::MissingDependency)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_UNAVAILABLE => {
            Some(EnforcementUnavailableReason::AdapterUnavailable)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_ERROR => {
            Some(EnforcementUnavailableReason::AdapterError)
        }
        enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED => {
            Some(EnforcementUnavailableReason::ManualRequired)
        }
        _ => None,
    }
}
