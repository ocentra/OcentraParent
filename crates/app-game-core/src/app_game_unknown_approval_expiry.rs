use crate::app_game_unknown_approval_event::AppGameUnknownApprovalEvent;
use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownApprovalPersistenceState,
    AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalStatus,
};
use crate::app_game_unknown_approval_validation::{invalid_transition, require_text};

pub(crate) fn apply_expiry(
    current: &AppGameUnknownApprovalSnapshot,
    event: &AppGameUnknownApprovalEvent,
    audit_ref: &str,
) -> Result<AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalError> {
    if event.request_id != current.request.request_id {
        return invalid_transition("expiry request id mismatch");
    }
    if !current.status.accepts_parent_response() {
        return invalid_transition("terminal approval state cannot expire again");
    }
    if event.occurred_at_epoch_ms < current.request.expires_at_epoch_ms {
        return invalid_transition("expiry transition arrived before request expiry");
    }
    require_text(audit_ref, "app_game.unknown_approval.audit_ref")?;
    let mut snapshot = current.clone();
    snapshot.status = AppGameUnknownApprovalStatus::Expired;
    snapshot.response = None;
    snapshot.audit_refs.push(audit_ref.to_owned());
    snapshot.updated_at_epoch_ms = event.occurred_at_epoch_ms;
    snapshot.persistence_state = AppGameUnknownApprovalPersistenceState::Replayed;
    Ok(snapshot)
}
