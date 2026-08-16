use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownApprovalSnapshot, AppGameUnknownParentResponse,
};
use crate::app_game_unknown_approval_validation::{invalid_transition, require_optional_text};

pub(crate) fn validate_response_specifics(
    snapshot: &AppGameUnknownApprovalSnapshot,
    response: AppGameUnknownParentResponse,
    override_ref: &Option<String>,
    decision_expires_at_epoch_ms: Option<u64>,
    occurred_at_epoch_ms: u64,
) -> Result<(), AppGameUnknownApprovalError> {
    validate_allow_once(
        snapshot,
        response,
        decision_expires_at_epoch_ms,
        occurred_at_epoch_ms,
    )?;
    if response == AppGameUnknownParentResponse::AllowCategory
        && snapshot.request.candidate.category_candidate_ref.is_none()
    {
        return invalid_transition("allow-category requires a category candidate ref");
    }
    validate_override(response, override_ref)
}

fn validate_allow_once(
    snapshot: &AppGameUnknownApprovalSnapshot,
    response: AppGameUnknownParentResponse,
    decision_expires_at_epoch_ms: Option<u64>,
    occurred_at_epoch_ms: u64,
) -> Result<(), AppGameUnknownApprovalError> {
    if response != AppGameUnknownParentResponse::AllowOnce {
        return Ok(());
    }
    let Some(decision_expiry) = decision_expires_at_epoch_ms else {
        return invalid_transition("allow-once requires a decision expiry");
    };
    if decision_expiry <= occurred_at_epoch_ms
        || decision_expiry > snapshot.request.expires_at_epoch_ms
    {
        return invalid_transition("allow-once expiry must remain inside request lifetime");
    }
    Ok(())
}

fn validate_override(
    response: AppGameUnknownParentResponse,
    override_ref: &Option<String>,
) -> Result<(), AppGameUnknownApprovalError> {
    if response == AppGameUnknownParentResponse::Override {
        return require_optional_text(override_ref, "app_game.unknown_approval.override_ref");
    }
    if override_ref.is_some() {
        return invalid_transition("override ref is only valid for an override response");
    }
    Ok(())
}
