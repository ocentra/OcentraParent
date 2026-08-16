#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_parent_agent_protocol::constants::policy_control;

use super::ChildPolicyRequest;

pub(crate) fn assert_request_origin_shape(
    request: &ChildPolicyRequest,
) -> Result<(), EventingError> {
    match request.origin {
        PolicyRequestOrigin::Child => assert_child_request_origin_shape(request),
        PolicyRequestOrigin::AssistantDraft => assert_assistant_draft_request_origin_shape(request),
    }
}

fn assert_child_request_origin_shape(request: &ChildPolicyRequest) -> Result<(), EventingError> {
    if request.assistant_preview_id.is_some()
        || request.assistant_confirmation_state != PolicyAssistantConfirmationState::NotRequired
        || request.status == PolicyRequestStatus::PreviewOnly
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ORIGIN,
            value: policy_control::request::VALUE_CHILD_REQUEST_CANNOT_BE_ASSISTANT_PREVIEW
                .to_string(),
        });
    }

    Ok(())
}

fn assert_assistant_draft_request_origin_shape(
    request: &ChildPolicyRequest,
) -> Result<(), EventingError> {
    if request.assistant_preview_id.is_none()
        || assistant_draft_confirmation_shape_is_invalid(request)
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ASSISTANT_PREVIEW_ID,
            value: policy_control::request::VALUE_ASSISTANT_DRAFT_REQUEST_MUST_STAY_PREVIEW_ONLY_UNTIL_PARENT_CONFIRMED
                .to_string(),
        });
    }

    Ok(())
}

fn assistant_draft_confirmation_shape_is_invalid(request: &ChildPolicyRequest) -> bool {
    match request.assistant_confirmation_state {
        PolicyAssistantConfirmationState::NotRequired => true,
        PolicyAssistantConfirmationState::ParentConfirmationRequired => !matches!(
            request.status,
            PolicyRequestStatus::PreviewOnly | PolicyRequestStatus::Expired
        ),
        PolicyAssistantConfirmationState::ParentConfirmed => {
            request.status == PolicyRequestStatus::PreviewOnly
        }
    }
}
