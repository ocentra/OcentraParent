use crate::app_game_unknown_approval_event::{
    AppGameUnknownApprovalEvent, AppGameUnknownApprovalTransition,
};
use crate::app_game_unknown_approval_expiry::apply_expiry;
use crate::app_game_unknown_approval_response::apply_parent_response;
use crate::app_game_unknown_approval_types::{
    AppGameUnknownAdapterDispatchState, AppGameUnknownApprovalError,
    AppGameUnknownApprovalPersistenceState, AppGameUnknownApprovalSnapshot,
    AppGameUnknownApprovalStatus,
};
use crate::app_game_unknown_approval_validation::{
    invalid_transition, validate_optional_refs, validate_unknown_candidate,
};

pub(crate) fn apply_unknown_approval_event(
    snapshot: Option<&AppGameUnknownApprovalSnapshot>,
    event: &AppGameUnknownApprovalEvent,
) -> Result<AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalError> {
    match (snapshot, &event.transition) {
        (None, AppGameUnknownApprovalTransition::RequestOpened { request }) => {
            open_snapshot(event, request)
        }
        (None, _) => invalid_transition("approval history must begin with request-opened"),
        (Some(_), AppGameUnknownApprovalTransition::RequestOpened { .. }) => {
            invalid_transition("approval request cannot be opened twice")
        }
        (Some(current), AppGameUnknownApprovalTransition::ParentResponded { .. }) => {
            apply_parent_response(current, event)
        }
        (Some(current), AppGameUnknownApprovalTransition::RequestExpired { audit_ref }) => {
            apply_expiry(current, event, audit_ref)
        }
    }
}

fn open_snapshot(
    event: &AppGameUnknownApprovalEvent,
    request: &crate::app_game_unknown_approval_types::AppGameUnknownApprovalRequest,
) -> Result<AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalError> {
    if request.request_id != event.request_id {
        return invalid_transition("opened request id does not match event aggregate");
    }
    validate_unknown_candidate(&request.candidate)?;
    validate_optional_refs(
        &request.child_reason_refs,
        "app_game.unknown_approval.child_reason_refs",
    )?;
    if request.expires_at_epoch_ms <= request.candidate.observed_at_epoch_ms {
        return invalid_transition("request expiry must be after candidate observation");
    }
    if event.occurred_at_epoch_ms != request.candidate.observed_at_epoch_ms {
        return invalid_transition("request event time must match candidate observation");
    }
    Ok(AppGameUnknownApprovalSnapshot {
        request: request.clone(),
        status: AppGameUnknownApprovalStatus::Pending,
        response: None,
        actor_ref: None,
        evidence_refs: request.candidate.evidence_refs.clone(),
        child_reason_refs: request.child_reason_refs.clone(),
        child_status_refs: request.candidate.child_status_refs.clone(),
        audit_refs: Vec::new(),
        override_ref: None,
        decision_expires_at_epoch_ms: None,
        updated_at_epoch_ms: event.occurred_at_epoch_ms,
        persistence_state: AppGameUnknownApprovalPersistenceState::Replayed,
        adapter_dispatch_state: AppGameUnknownAdapterDispatchState::NotDispatched,
    })
}
