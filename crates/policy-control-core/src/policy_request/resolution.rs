#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestStatus,
};
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    approval::{assert_request_matches_approval, validate_parent_policy_approval},
    decision::policy_request_status_for_approval,
    policy_request_schema_version,
    status::{policy_override_id_value, policy_request_status_name},
    validation::validate_child_policy_request,
    ChildPolicyRequest, ParentPolicyApproval, PolicyApprovalDecision, PolicyOverrideId,
    PolicyOverrideState, PolicyRequestKind, PolicyRequestResolution, PolicyTemporaryOverride,
};

pub(crate) fn resolve_parent_policy_approval(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyRequestResolution, EventingError> {
    validate_child_policy_request(request)?;
    validate_parent_policy_approval(&approval)?;
    assert_request_matches_approval(request, &approval)?;

    if request.assistant_confirmation_state
        == PolicyAssistantConfirmationState::ParentConfirmationRequired
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ASSISTANT_CONFIRMATION_STATE,
            value: policy_control::request::VALUE_ASSISTANT_PREVIEW_ONLY.to_string(),
        });
    }

    let expected_status = policy_request_status_for_approval(approval.decision);
    if let Some(resolved_approval_id) = request.resolved_approval_id.as_ref() {
        if resolved_approval_id != &approval.approval_id || request.status != expected_status {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_APPROVAL_ID,
                value: approval.approval_id.as_str().to_string(),
            });
        }

        return resolve_replayed_parent_policy_approval(request, &approval, existing_override);
    }

    resolve_new_parent_policy_approval(request, approval)
}

fn resolve_replayed_parent_policy_approval(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyRequestResolution, EventingError> {
    let replay_override = if matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified
    ) {
        let replay_override = existing_override.ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::request::FIELD_OVERRIDE_ID,
            value: policy_control::request::VALUE_MISSING_OVERRIDE_FOR_RESOLVED_APPROVAL_REPLAY
                .to_string(),
        })?;
        assert_override_matches(request, approval, replay_override)?;
        Some(replay_override.clone())
    } else {
        None
    };

    Ok(PolicyRequestResolution {
        request: request.clone(),
        temporary_override: replay_override,
    })
}

fn resolve_new_parent_policy_approval(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
) -> Result<PolicyRequestResolution, EventingError> {
    if request.status != PolicyRequestStatus::PendingParentReview {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(request.status).to_string(),
        });
    }

    let mut resolved_request = request.clone();
    let temporary_override = match approval.decision {
        PolicyApprovalDecision::Grant | PolicyApprovalDecision::Modify => {
            Some(build_policy_temporary_override(request, &approval)?)
        }
        PolicyApprovalDecision::Deny | PolicyApprovalDecision::Expire => None,
    };

    let ParentPolicyApproval {
        approval_id,
        decision,
        decided_at,
        audit_reference_id,
        ..
    } = approval;

    resolved_request.status = policy_request_status_for_approval(decision);
    resolved_request.resolved_approval_id = Some(approval_id);
    resolved_request.resolved_at = Some(decided_at);
    resolved_request
        .audit_reference_ids
        .push(audit_reference_id);
    validate_child_policy_request(&resolved_request)?;
    Ok(PolicyRequestResolution {
        request: resolved_request,
        temporary_override,
    })
}

fn build_policy_temporary_override(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<PolicyTemporaryOverride, EventingError> {
    let approved_action = approval
        .approved_action
        .unwrap_or(request.scope.requested_action);
    let approved_bonus_minutes = approval
        .approved_bonus_minutes
        .or(request.scope.requested_bonus_minutes);

    if request.scope.request_kind == PolicyRequestKind::BonusTime
        && approved_bonus_minutes.is_none()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_APPROVAL_REQUIRES_MINUTES.to_string(),
        });
    }

    Ok(PolicyTemporaryOverride {
        schema_version: policy_request_schema_version()?,
        override_id: PolicyOverrideId::parse(policy_override_id_value(&approval.approval_id))?,
        source_request_id: request.request_id.clone(),
        source_approval_id: approval.approval_id.clone(),
        household_id: request.household_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        device_id: request.device_id.clone(),
        source_document_id: request.source_document_id.clone(),
        policy_version: request.policy_version,
        request_kind: request.scope.request_kind,
        target: request.scope.target.clone(),
        approved_action,
        approved_bonus_minutes,
        effective_at: approval.decided_at.clone(),
        expires_at: approval
            .override_expires_at
            .clone()
            .unwrap_or_else(|| request.expires_at.clone()),
        state: PolicyOverrideState::Active,
        audit_reference_ids: vec![approval.audit_reference_id.clone()],
    })
}

fn assert_override_matches(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
    existing_override: &PolicyTemporaryOverride,
) -> Result<(), EventingError> {
    let expected_action = approval
        .approved_action
        .unwrap_or(request.scope.requested_action);
    let expected_minutes = approval
        .approved_bonus_minutes
        .or(request.scope.requested_bonus_minutes);
    let expected_expires_at = approval
        .override_expires_at
        .as_ref()
        .unwrap_or(&request.expires_at);

    if existing_override.source_request_id != request.request_id
        || existing_override.source_approval_id != approval.approval_id
        || existing_override.policy_version != request.policy_version
        || existing_override.approved_action != expected_action
        || existing_override.approved_bonus_minutes != expected_minutes
        || &existing_override.expires_at != expected_expires_at
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_OVERRIDE_ID,
            value: existing_override.override_id.as_str().to_string(),
        });
    }

    Ok(())
}
