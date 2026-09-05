#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::super::{ChildPolicyRequest, ParentPolicyApproval, PolicyRequestKind};
use crate::policy_source::PolicyRuleAction;

pub(super) fn validate(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    validate_identity(request, approval)?;
    validate_request_window(request, approval)?;
    validate_bonus_time(request, approval)
}

fn validate_identity(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    if request.request_id != approval.request_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUEST_ID,
            value: approval.request_id.as_str().to_string(),
        });
    }
    if request.household_id != approval.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_HOUSEHOLD_ID,
            value: approval.household_id.as_str().to_string(),
        });
    }
    if request.policy_version != approval.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_POLICY_VERSION,
            value: approval.policy_version.value().to_string(),
        });
    }
    Ok(())
}

fn validate_request_window(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    if request.status == PolicyRequestStatus::Expired {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_control::request::VALUE_EXPIRED_REQUEST_CANNOT_BE_APPROVED.to_string(),
        });
    }
    if approval.decided_at.as_str() < request.requested_at.as_str()
        || approval.decided_at.as_str() >= request.expires_at.as_str()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_TIMESTAMP,
            value: "approval-decision-must-be-within-request-window".to_string(),
        });
    }
    Ok(())
}

fn validate_bonus_time(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    if approval.approved_bonus_minutes.is_some()
        && request.scope.request_kind != PolicyRequestKind::BonusTime
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: "only-bonus-time-requests-may-include-approved-minutes".to_string(),
        });
    }

    if request.scope.request_kind == PolicyRequestKind::BonusTime {
        let approved_action = approval
            .approved_action
            .unwrap_or(request.scope.requested_action);
        if !matches!(
            approved_action,
            PolicyRuleAction::Allow | PolicyRuleAction::TimeLimit
        ) {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
                value: "bonus-time-approvals-require-allow-or-time-limit".to_string(),
            });
        }
    }
    Ok(())
}
