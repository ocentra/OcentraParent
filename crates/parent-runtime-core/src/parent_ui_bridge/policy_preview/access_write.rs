use super::*;

pub(super) fn policy_preview_access_write_authority_impl(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> String {
    match parent_access_state {
        ParentPortalParentAccessState::ObserverOnly => {
            return "Observer scope is read-only and cannot confirm or save policy writes."
                .to_string();
        }
        ParentPortalParentAccessState::Unauthenticated => {
            return "Sign-in required before any review or confirmation action.".to_string();
        }
        ParentPortalParentAccessState::ProofMissing => {
            return "Write authority is unavailable until household role proof is visible."
                .to_string();
        }
        ParentPortalParentAccessState::ActiveController => {}
    }

    if read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref())
        == Some("parent-confirmed")
    {
        return "Parent-confirmed preview is visible and the typed parent-confirmation request exists, but this does not prove downstream policy mutation, delivery, or active enforcement."
            .to_string();
    }
    if read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref())
        == Some("parent-confirmation-required")
    {
        return "Parent confirmation is required before any write.".to_string();
    }
    "Preview-only route; typed parent confirmation is available only through the bounded staged request flow and does not prove downstream policy mutation, delivery, or active enforcement.".to_string()
}
