#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    build_policy_temporary_override, policy_request_status_name, validate_child_policy_request,
    ChildPolicyRequest, ParentPolicyApproval, PolicyApprovalDecision, PolicyRequestResolution,
    PolicyTemporaryOverride,
};

pub(crate) fn resolve_replayed_parent_policy_approval(
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

pub(crate) fn resolve_new_parent_policy_approval(
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

pub(crate) fn policy_request_status_for_approval(
    decision: PolicyApprovalDecision,
) -> PolicyRequestStatus {
    match decision {
        PolicyApprovalDecision::Grant => PolicyRequestStatus::Approved,
        PolicyApprovalDecision::Deny => PolicyRequestStatus::Denied,
        PolicyApprovalDecision::Modify => PolicyRequestStatus::Modified,
        PolicyApprovalDecision::Expire => PolicyRequestStatus::Expired,
    }
}
