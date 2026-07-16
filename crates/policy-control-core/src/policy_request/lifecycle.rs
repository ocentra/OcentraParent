#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{
    approval::assert_parent_actor_authority,
    status::{duplicate_submission_key_value, policy_request_status_name},
    validation::{child_requests_match, validate_child_policy_request},
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, PolicyAuditReferenceId,
    PolicyRequestTimestamp,
};

pub(crate) fn register_child_policy_request(
    existing: Option<&ChildPolicyRequest>,
    candidate: ChildPolicyRequest,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(&candidate)?;
    if !matches!(
        candidate.status,
        PolicyRequestStatus::PreviewOnly | PolicyRequestStatus::PendingParentReview
    ) || candidate.resolved_approval_id.is_some()
        || candidate.resolved_at.is_some()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(candidate.status).to_string(),
        });
    }

    if let Some(current) = existing {
        if current.request_id == candidate.request_id
            && current.submission_key != candidate.submission_key
        {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_SUBMISSION_KEY,
                value: current.submission_key.as_str().to_string(),
            });
        }

        if current.submission_key == candidate.submission_key {
            if child_requests_match(current, &candidate) {
                return Ok(current.clone());
            }

            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_SUBMISSION_KEY,
                value: duplicate_submission_key_value(&current.submission_key),
            });
        }
    }

    Ok(candidate)
}

pub(crate) fn confirm_assistant_policy_request_preview(
    request: &ChildPolicyRequest,
    confirmation: AssistantPolicyRequestConfirmation,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(request)?;
    assert_parent_actor_authority(
        confirmation.actor_role,
        confirmation.actor_state,
        policy_control::request::FIELD_ACTOR_ROLE,
        policy_control::request::FIELD_ACTOR_STATE,
    )?;

    if request.origin != PolicyRequestOrigin::AssistantDraft
        || request.status != PolicyRequestStatus::PreviewOnly
        || request.assistant_confirmation_state
            != PolicyAssistantConfirmationState::ParentConfirmationRequired
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ASSISTANT_CONFIRMATION_STATE,
            value: policy_request_status_name(request.status).to_string(),
        });
    }

    let mut confirmed = request.clone();
    confirmed.status = PolicyRequestStatus::PendingParentReview;
    confirmed.assistant_confirmation_state = PolicyAssistantConfirmationState::ParentConfirmed;
    confirmed
        .audit_reference_ids
        .push(confirmation.audit_reference_id);
    validate_child_policy_request(&confirmed)?;
    Ok(confirmed)
}

pub(crate) fn expire_child_policy_request(
    request: &ChildPolicyRequest,
    expired_at: PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(request)?;
    if matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Denied | PolicyRequestStatus::Modified
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(request.status).to_string(),
        });
    }
    if request.status == PolicyRequestStatus::Expired {
        return Ok(request.clone());
    }

    let mut expired = request.clone();
    expired.status = PolicyRequestStatus::Expired;
    expired.resolved_at = Some(expired_at);
    expired.audit_reference_ids.push(audit_reference_id);
    validate_child_policy_request(&expired)?;
    Ok(expired)
}
