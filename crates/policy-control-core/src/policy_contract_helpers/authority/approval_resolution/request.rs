#![forbid(unsafe_code)]

use super::super::{
    assert_utc_timestamp, validate_policy_schedule_boundary, PolicyContractApprovalKind,
    PolicyContractApprovalRequest, PolicyContractValidationResult,
};

pub(crate) fn validate_policy_approval_request(
    request: &PolicyContractApprovalRequest,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&request.requested_at, "approval.requestedAt")?;
    assert_utc_timestamp(&request.expires_at, "approval.expiresAt")?;
    if request.expires_at <= request.requested_at {
        return Err("approval.expiresAt must be after approval.requestedAt".into());
    }

    if let Some(schedule_boundary) = &request.schedule_boundary {
        validate_policy_schedule_boundary(schedule_boundary)?;
    }

    match request.kind {
        PolicyContractApprovalKind::BonusTime => {
            if request.requested_bonus_time_minutes.unwrap_or(0) == 0 {
                return Err(
                    "bonus-time requests must include a positive requestedBonusTimeMinutes value"
                        .into(),
                );
            }
            let Some(schedule_boundary) = &request.schedule_boundary else {
                return Err("bonus-time requests must include scheduleBoundary details".into());
            };
            if schedule_boundary.time_budget.is_none() {
                return Err(
                    "bonus-time requests must include scheduleBoundary.timeBudget details".into(),
                );
            }
        }
        PolicyContractApprovalKind::AskParent | PolicyContractApprovalKind::TemporaryOverride => {
            if request.requested_bonus_time_minutes.is_some() {
                return Err(
                    "only bonus-time requests may include requestedBonusTimeMinutes".into(),
                );
            }
        }
    }

    Ok(())
}
