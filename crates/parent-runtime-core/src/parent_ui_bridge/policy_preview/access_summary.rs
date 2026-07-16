use super::helpers::{
    policy_preview_has_confirmed_controller_decision, policy_preview_has_recorded_audit_decision,
    policy_preview_is_replay_rejected, policy_preview_requires_parent_confirmation,
};
use super::*;

pub(super) fn policy_preview_access_summary_impl(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> String {
    if let Some(summary) = policy_preview_parent_access_summary_text_impl(parent_access_state) {
        return summary;
    }
    if policy_preview_is_replay_rejected(read_model) {
        return "The latest approval attempt was rejected as a replay, so no new override was created."
            .to_string();
    }
    if policy_preview_requires_parent_confirmation(read_model) {
        return "Controller authority is present, but parent confirmation is still required before any write."
            .to_string();
    }
    if policy_preview_has_recorded_audit_decision(read_model) {
        return "Controller review is recorded with reviewer and audit details, but delivery and enforcement remain separate states."
            .to_string();
    }
    if policy_preview_has_confirmed_controller_decision(read_model) {
        return "Controller confirmation is recorded, but delivery and enforcement remain separate states."
            .to_string();
    }
    "Controller authority is present, but the portal still treats this policy path as preview-only."
        .to_string()
}

pub(super) fn policy_preview_parent_access_summary_text_impl(
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<String> {
    match parent_access_state {
        ParentPortalParentAccessState::ObserverOnly => Some(
            "Observer-only parents can review policy explanation but cannot confirm or save writes."
                .to_string(),
        ),
        ParentPortalParentAccessState::Unauthenticated => Some(
            "Sign-in is required before reviewing or confirming policy changes.".to_string(),
        ),
        ParentPortalParentAccessState::ProofMissing => Some(
            "Parent authority proof is missing, so the portal cannot claim write permission."
                .to_string(),
        ),
        ParentPortalParentAccessState::ActiveController => None,
    }
}
