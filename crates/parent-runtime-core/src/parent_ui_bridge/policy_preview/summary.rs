use super::helpers::{policy_preview_has_conflict_finding, policy_preview_readable_value};
use super::*;

pub(super) fn policy_preview_summary_impl(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> String {
    if read_model.policy_request_origin.as_deref() == Some("assistant-draft")
        && read_model.policy_assistant_confirmation_state.as_deref() != Some("parent-confirmed")
    {
        return "Assistant draft remains preview-only until parent confirmation.".to_string();
    }

    if read_model.policy_preview_save_state.as_deref() == Some("blocked")
        || policy_preview_has_conflict_finding(read_model)
    {
        return "Preview is blocked and conflict details stay visible for parent review."
            .to_string();
    }

    if matches!(
        read_model.policy_preview_manual_review_state.as_deref(),
        Some("required")
    ) || matches!(
        read_model.policy_preview_target_state.as_deref(),
        Some("unsupported" | "manual-required" | "offline" | "stale")
    ) {
        return "Preview stays visible, but it is not ready to save.".to_string();
    }

    if read_model.policy_preview_save_state.as_deref() == Some("ready-to-save") {
        return "Preview is ready to save, but it is still not enforced.".to_string();
    }

    "Preview remains advisory and not enforced.".to_string()
}

pub(super) fn policy_preview_source_lifecycle_summary_impl(source_status: &str) -> String {
    match source_status {
        "delivered" => "Delivered is reported, but active enforcement is separate.".to_string(),
        "acknowledged" => {
            "Acknowledged delivery is reported, but active enforcement is separate.".to_string()
        }
        "active" | "partially-active" => {
            "Active lifecycle is adapter-owned and stays distinct from delivery or audit claims."
                .to_string()
        }
        _ => format!(
            "Source lifecycle: {}",
            policy_preview_readable_value(Some(source_status))
        ),
    }
}
