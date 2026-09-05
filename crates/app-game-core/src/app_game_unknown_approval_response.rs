use crate::app_game_unknown_approval_event::{
    AppGameUnknownApprovalEvent, AppGameUnknownApprovalTransition,
};
use crate::app_game_unknown_approval_response_validation::validate_response_specifics;
use crate::app_game_unknown_approval_status::response_status;
use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownApprovalPersistenceState,
    AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalStatus,
};
use crate::app_game_unknown_approval_validation::{
    invalid_transition, require_refs, require_text, validate_optional_refs,
};

pub(crate) fn apply_parent_response(
    current: &AppGameUnknownApprovalSnapshot,
    event: &AppGameUnknownApprovalEvent,
) -> Result<AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalError> {
    let AppGameUnknownApprovalTransition::ParentResponded {
        actor_ref,
        response,
        capability_state,
        evidence_refs,
        child_reason_refs,
        child_status_refs,
        audit_ref,
        override_ref,
        decision_expires_at_epoch_ms,
    } = &event.transition
    else {
        return invalid_transition("expected parent response transition");
    };
    validate_common_response(current, event, actor_ref, evidence_refs, audit_ref)?;
    validate_optional_refs(
        child_reason_refs,
        "app_game.unknown_approval.child_reason_refs",
    )?;
    validate_optional_refs(
        child_status_refs,
        "app_game.unknown_approval.child_status_refs",
    )?;
    if current.status == AppGameUnknownApprovalStatus::AwaitingChildReason
        && child_reason_refs.is_empty()
    {
        return invalid_transition("follow-up response requires child reason refs");
    }
    validate_response_specifics(
        current,
        *response,
        override_ref,
        *decision_expires_at_epoch_ms,
        event.occurred_at_epoch_ms,
    )?;

    let mut snapshot = current.clone();
    snapshot.status = response_status(*response, *capability_state);
    snapshot.response = Some(*response);
    snapshot.actor_ref = Some(actor_ref.clone());
    snapshot.evidence_refs = evidence_refs.clone();
    update_child_refs(&mut snapshot, child_reason_refs, child_status_refs);
    snapshot.audit_refs.push(audit_ref.clone());
    snapshot.override_ref.clone_from(override_ref);
    snapshot.decision_expires_at_epoch_ms = *decision_expires_at_epoch_ms;
    snapshot.updated_at_epoch_ms = event.occurred_at_epoch_ms;
    snapshot.persistence_state = AppGameUnknownApprovalPersistenceState::Replayed;
    Ok(snapshot)
}

fn validate_common_response(
    snapshot: &AppGameUnknownApprovalSnapshot,
    event: &AppGameUnknownApprovalEvent,
    actor_ref: &str,
    evidence_refs: &[String],
    audit_ref: &str,
) -> Result<(), AppGameUnknownApprovalError> {
    if event.request_id != snapshot.request.request_id {
        return invalid_transition("parent response request id mismatch");
    }
    if !snapshot.status.accepts_parent_response() {
        return invalid_transition("terminal approval state rejects another parent response");
    }
    if event.occurred_at_epoch_ms < snapshot.updated_at_epoch_ms {
        return invalid_transition("parent response predates current approval state");
    }
    if event.occurred_at_epoch_ms >= snapshot.request.expires_at_epoch_ms {
        return invalid_transition("parent response arrived at or after request expiry");
    }
    require_text(actor_ref, "app_game.unknown_approval.actor_ref")?;
    require_text(audit_ref, "app_game.unknown_approval.audit_ref")?;
    require_refs(evidence_refs, "app_game.unknown_approval.evidence_refs")
}

fn update_child_refs(
    snapshot: &mut AppGameUnknownApprovalSnapshot,
    child_reason_refs: &[String],
    child_status_refs: &[String],
) {
    if !child_reason_refs.is_empty() {
        snapshot.child_reason_refs = child_reason_refs.to_vec();
    }
    if !child_status_refs.is_empty() {
        snapshot.child_status_refs = child_status_refs.to_vec();
    }
}
