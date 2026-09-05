#![forbid(unsafe_code)]

mod request_match;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    ChildPolicyRequest, ParentPolicyActorRole, ParentPolicyApproval, PolicyApprovalDecision,
    PolicySourceActorState,
};
use crate::policy_source::{
    assert_policy_utc_timestamp, policy_actor_role_name, policy_actor_state_name,
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
    request_match::validate(request, approval)
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
