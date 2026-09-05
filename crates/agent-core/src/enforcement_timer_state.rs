use ocentra_parent_agent_protocol::{
    constants,
    enforcement::{
        AppGameTimerSessionBinding, EnforcementActiveTimerState, EnforcementResultStatus,
        EnforcementTimerEventKind, ParentActionReference,
    },
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

/// Returns whether a persisted timer state still describes one coherent,
/// active transition.  Timer state is reconstructed from several protocol
/// records, so accepting a parseable file is not enough to make restart
/// recovery safe.
pub fn active_timer_state_is_consistent(state: &EnforcementActiveTimerState) -> bool {
    let action = &state.action;
    let result = &state.result;
    let audit_event = &state.audit_event;
    let timer_event = &state.timer_event;

    !action.action_id.trim().is_empty()
        && !state.stored_at.trim().is_empty()
        && state.schema_version == action.schema_version
        && state.state_id
            == enforcement_timer_state_helpers::active_timer_state_id(&action.action_id)
        && result.schema_version == action.schema_version
        && result.action_id == action.action_id
        && result.capability == action.capability
        && audit_event.schema_version == action.schema_version
        && audit_event.action.eq(action)
        && audit_event.result.eq(result)
        && audit_event.capability == result.capability
        && audit_event.unavailable_status == result.unavailable_status
        && audit_event.evidence_references == action.evidence_references
        && timer_event.schema_version == action.schema_version
        && timer_event.action_id == action.action_id
        && timer_event.policy_decision_id == action.policy_decision_id
        && timer_event.evidence_references == action.evidence_references
        && timer_event.rollback_token == action.rollback_token
        && timer_event.unavailable_reason.is_none()
        && timer_event.recovered_after_restart
            == (timer_event.timer_event_kind == EnforcementTimerEventKind::RestartRecovered)
        && enforcement_timer_state_helpers::active_timer_event(timer_event, result)
        && active_timer_state_timestamps_are_consistent(state)
}

fn active_timer_state_timestamps_are_consistent(state: &EnforcementActiveTimerState) -> bool {
    parsed_timer_state_timestamps(state).is_some_and(|timestamps| timestamps.are_consistent())
}

type TimerTimestamp = chrono::DateTime<chrono::FixedOffset>;

struct ParsedTimerStateTimestamps {
    action_requested_at: TimerTimestamp,
    action_expires_at: TimerTimestamp,
    stored_at: TimerTimestamp,
    result_started_at: TimerTimestamp,
    result_completed_at: Option<TimerTimestamp>,
    result_next_check_at: Option<TimerTimestamp>,
    audit_observed_at: TimerTimestamp,
    timer_scheduled_at: TimerTimestamp,
    timer_effective_at: TimerTimestamp,
}

impl ParsedTimerStateTimestamps {
    fn are_consistent(&self) -> bool {
        self.action_requested_at < self.action_expires_at
            && self.result_started_at >= self.action_requested_at
            && self.timer_scheduled_at == self.action_requested_at
            && self.timer_effective_at == self.action_expires_at
            && self.stored_at >= self.result_started_at
            && self.stored_at >= self.audit_observed_at
            && self.audit_observed_at >= self.result_started_at
            && self
                .result_completed_at
                .as_ref()
                .is_none_or(|completed_at| *completed_at >= self.result_started_at)
            && self
                .result_next_check_at
                .as_ref()
                .is_none_or(|next_check_at| *next_check_at == self.action_expires_at)
    }
}

fn parsed_timer_state_timestamps(
    state: &EnforcementActiveTimerState,
) -> Option<ParsedTimerStateTimestamps> {
    let action_requested_at = parse_timer_timestamp(&state.action.requested_at)?;
    let action_expires_at = parse_required_timer_timestamp(state.action.expires_at.as_deref())?;
    let stored_at = parse_timer_timestamp(&state.stored_at)?;
    let result_started_at = parse_timer_timestamp(&state.result.started_at)?;
    let result_completed_at = parse_optional_timer_timestamp(state.result.completed_at.as_deref())?;
    let result_next_check_at =
        parse_optional_timer_timestamp(state.result.next_check_at.as_deref())?;
    let audit_observed_at = parse_timer_timestamp(&state.audit_event.observed_at)?;
    let timer_scheduled_at = parse_timer_timestamp(&state.timer_event.scheduled_at)?;
    let timer_effective_at =
        parse_required_timer_timestamp(state.timer_event.effective_at.as_deref())?;

    validate_related_timer_timestamps(state)?;
    Some(ParsedTimerStateTimestamps {
        action_requested_at,
        action_expires_at,
        stored_at,
        result_started_at,
        result_completed_at,
        result_next_check_at,
        audit_observed_at,
        timer_scheduled_at,
        timer_effective_at,
    })
}

fn validate_related_timer_timestamps(state: &EnforcementActiveTimerState) -> Option<()> {
    parse_timer_timestamp(&state.action.capability.last_checked_at)?;
    parse_optional_timer_timestamp(
        state
            .action
            .parent_approval
            .as_ref()
            .map(|approval| approval.created_at.as_str()),
    )?;
    parse_optional_timer_timestamp(
        state
            .result
            .unavailable_status
            .as_ref()
            .map(|status| status.checked_at.as_str()),
    )?;
    parse_optional_timer_timestamp(
        state
            .app_game_session
            .as_ref()
            .map(|session| session.last_observed_at.as_str()),
    )?;
    Some(())
}

fn parse_required_timer_timestamp(value: Option<&str>) -> Option<TimerTimestamp> {
    value.and_then(parse_timer_timestamp)
}

fn parse_optional_timer_timestamp(value: Option<&str>) -> Option<Option<TimerTimestamp>> {
    match value {
        Some(value) => parse_timer_timestamp(value).map(Some),
        None => Some(None),
    }
}

fn parse_timer_timestamp(value: &str) -> Option<TimerTimestamp> {
    let value = value.trim();
    (!value.is_empty())
        .then(|| chrono::DateTime::parse_from_rfc3339(value).ok())
        .flatten()
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

pub fn expiring_timer_before_dispatch_outcome(
    state: &EnforcementActiveTimerState,
    ids: EnforcementTimerTransitionIds,
) -> EnforcementBoundaryOutcome {
    let before_dispatch_ids = EnforcementTimerTransitionIds {
        result_id: before_dispatch_id(&ids.result_id),
        audit_event_id: before_dispatch_id(&ids.audit_event_id),
        timer_event_id: before_dispatch_id(&ids.timer_event_id),
        observed_at: ids.observed_at,
    };
    enforcement_timer_state_transition::transition_outcome(
        state,
        before_dispatch_ids,
        EnforcementTimerEventKind::Expired,
        EnforcementResultStatus::WouldEnforce,
        None,
    )
}

fn before_dispatch_id(value: &str) -> String {
    format!(
        "{}{}",
        constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX,
        value
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
