#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    origin::assert_request_origin_shape, status::policy_request_status_name, ChildPolicyRequest,
    PolicyRequestKind, PolicyRequestScope,
};

pub(crate) fn validate_child_policy_request(
    request: &ChildPolicyRequest,
) -> Result<(), EventingError> {
    assert_non_empty_audit_refs(
        &request.audit_reference_ids,
        policy_control::request::FIELD_AUDIT_REFERENCE_IDS,
    )?;
    assert_request_status_supported(request.status)?;
    assert_request_scope(&request.scope)?;
    assert_request_origin_shape(request)?;
    assert_request_resolution_shape(request)?;
    Ok(())
}

pub(crate) fn child_requests_match(left: &ChildPolicyRequest, right: &ChildPolicyRequest) -> bool {
    left.household_id == right.household_id
        && left.child_profile_id == right.child_profile_id
        && left.device_id == right.device_id
        && left.source_document_id == right.source_document_id
        && left.policy_version == right.policy_version
        && left.origin == right.origin
        && left.assistant_preview_id == right.assistant_preview_id
        && left.assistant_confirmation_state == right.assistant_confirmation_state
        && left.status == right.status
        && left.scope == right.scope
        && left.requested_at == right.requested_at
        && left.expires_at == right.expires_at
}

pub(crate) fn assert_request_resolution_shape(
    request: &ChildPolicyRequest,
) -> Result<(), EventingError> {
    let resolved = request.resolved_approval_id.is_some() || request.resolved_at.is_some();
    if matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Denied | PolicyRequestStatus::Modified
    ) && (request.resolved_approval_id.is_none() || request.resolved_at.is_none())
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_RESOLVED_APPROVAL_ID,
            value: policy_request_status_name(request.status).to_string(),
        });
    }
    if request.status == PolicyRequestStatus::PreviewOnly && resolved {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_control::request::VALUE_PREVIEW_ONLY_REQUEST_CANNOT_BE_RESOLVED
                .to_string(),
        });
    }
    Ok(())
}

fn assert_request_scope(scope: &PolicyRequestScope) -> Result<(), EventingError> {
    if scope.request_kind == PolicyRequestKind::BonusTime && scope.requested_bonus_minutes.is_none()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUESTED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_REQUEST_REQUIRES_MINUTES.to_string(),
        });
    }
    Ok(())
}

fn assert_request_status_supported(status: PolicyRequestStatus) -> Result<(), EventingError> {
    if status == PolicyRequestStatus::ReplayRejected {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(status).to_string(),
        });
    }

    Ok(())
}

fn assert_non_empty_audit_refs(
    audit_reference_ids: &[super::PolicyAuditReferenceId],
    field: &'static str,
) -> Result<(), EventingError> {
    if audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field,
            value: policy_control::request::VALUE_MISSING_AUDIT_REFERENCE.to_string(),
        });
    }
    Ok(())
}
