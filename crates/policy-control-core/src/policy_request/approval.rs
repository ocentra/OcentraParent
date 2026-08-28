#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    ChildPolicyRequest, ParentPolicyActorRole, ParentPolicyApproval, PolicyApprovalDecision,
    PolicySourceActorState,
};
use crate::policy_source::{
    assert_policy_utc_timestamp, policy_actor_role_name, policy_actor_state_name, PolicyRuleAction,
};

pub(crate) fn validate_parent_policy_approval(
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    assert_policy_utc_timestamp(
        policy_control::request::FIELD_TIMESTAMP,
        approval.decided_at.as_str(),
    )?;
    if let Some(override_expires_at) = approval.override_expires_at.as_ref() {
        assert_policy_utc_timestamp(
            policy_control::request::FIELD_TIMESTAMP,
            override_expires_at.as_str(),
        )?;
        if override_expires_at <= &approval.decided_at {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_TIMESTAMP,
                value: "approval-override-expiry-must-be-after-decision".to_string(),
            });
        }
    }
    assert_parent_actor_authority(
        approval.actor_role,
        approval.actor_state,
        policy_control::request::FIELD_ACTOR_ROLE,
        policy_control::request::FIELD_ACTOR_STATE,
    )?;

    if denied_or_expired_approval_carries_override_values(approval) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVAL_DECISION,
            value: policy_control::request::VALUE_DENY_OR_EXPIRE_CANNOT_CARRY_OVERRIDE_VALUES
                .to_string(),
        });
    }

    if modify_approval_missing_override_changes(approval) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVAL_DECISION,
            value: policy_control::request::VALUE_MODIFY_REQUIRES_CHANGED_OVERRIDE_VALUES
                .to_string(),
        });
    }

    Ok(())
}

pub(crate) fn assert_request_matches_approval(
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

    if approval.approved_bonus_minutes.is_some()
        && request.scope.request_kind != super::PolicyRequestKind::BonusTime
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: "only-bonus-time-requests-may-include-approved-minutes".to_string(),
        });
    }

    if request.scope.request_kind == super::PolicyRequestKind::BonusTime {
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

pub(crate) fn assert_parent_actor_authority(
    role: ParentPolicyActorRole,
    state: PolicySourceActorState,
    role_field: &'static str,
    state_field: &'static str,
) -> Result<(), EventingError> {
    if !matches!(
        role,
        ParentPolicyActorRole::Parent | ParentPolicyActorRole::CoParent
    ) {
        return Err(EventingError::InvalidValue {
            field: role_field,
            value: policy_actor_role_name(role).to_string(),
        });
    }

    if state != PolicySourceActorState::Active {
        return Err(EventingError::InvalidValue {
            field: state_field,
            value: policy_actor_state_name(state).to_string(),
        });
    }

    Ok(())
}

fn denied_or_expired_approval_carries_override_values(approval: &ParentPolicyApproval) -> bool {
    matches!(
        approval.decision,
        PolicyApprovalDecision::Deny | PolicyApprovalDecision::Expire
    ) && (approval.approved_action.is_some()
        || approval.approved_bonus_minutes.is_some()
        || approval.override_expires_at.is_some())
}

fn modify_approval_missing_override_changes(approval: &ParentPolicyApproval) -> bool {
    approval.decision == PolicyApprovalDecision::Modify
        && approval.approved_action.is_none()
        && approval.approved_bonus_minutes.is_none()
        && approval.override_expires_at.is_none()
}
