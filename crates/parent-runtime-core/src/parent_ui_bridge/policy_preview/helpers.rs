use super::*;
use std::fmt::Display;

pub(super) fn policy_preview_unavailable_summary(
    event: Option<&ParentRouteEventSnapshot>,
) -> String {
    event
        .and_then(|value| value.payload.as_ref())
        .and_then(|payload| payload.get("reason"))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| "Policy preview read-model is unavailable.".to_string())
}

pub(super) fn policy_preview_is_replay_rejected(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> bool {
    read_model.and_then(|value| value.policy_request_status.as_deref()) == Some("replay-rejected")
}

pub(super) fn policy_preview_requires_parent_confirmation(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> bool {
    read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref())
        == Some("parent-confirmation-required")
}

pub(super) fn policy_preview_has_recorded_audit_decision(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> bool {
    let Some(read_model) = read_model else {
        return false;
    };
    if read_model.policy_audit_reference_id.is_none() {
        return false;
    }
    matches!(
        read_model.policy_request_status.as_deref(),
        Some("approved" | "modified" | "denied")
    )
}

pub(super) fn policy_preview_has_confirmed_controller_decision(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> bool {
    matches!(
        read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref()),
        Some("parent-confirmed")
    ) || matches!(
        read_model.and_then(|value| value.policy_request_status.as_deref()),
        Some("approved")
    )
}

pub(super) fn policy_preview_has_conflict_finding(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> bool {
    let finding_kinds = read_model
        .policy_preview_finding_kinds
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let explanation_code = read_model
        .policy_preview_target_explanation_code
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    finding_kinds.contains("conflict") || explanation_code.contains("conflict")
}

pub(super) fn policy_preview_parent_access_readable_value(
    parent_access_state: &ParentPortalParentAccessState,
) -> String {
    match parent_access_state {
        ParentPortalParentAccessState::ActiveController => "Active controller".to_string(),
        ParentPortalParentAccessState::ObserverOnly => "Observer only".to_string(),
        ParentPortalParentAccessState::Unauthenticated => "Unauthenticated".to_string(),
        ParentPortalParentAccessState::ProofMissing => "Proof missing".to_string(),
    }
}

pub(super) fn policy_preview_readable_value(value: Option<&str>) -> String {
    match value {
        Some(value) => policy_preview_readable_label(value)
            .unwrap_or(value)
            .to_string(),
        None => policy_preview_optional_value(None),
    }
}

pub(super) fn policy_preview_optional_numeric_value(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| policy_preview_optional_value(None))
}

pub(super) fn policy_preview_optional_value(value: Option<&str>) -> String {
    match value {
        Some(value) if !value.is_empty() => value.to_string(),
        _ => "Not reported".to_string(),
    }
}

pub(super) fn policy_preview_optional_display_value<T>(value: Option<&T>) -> String
where
    T: Display + ?Sized,
{
    value
        .map(ToString::to_string)
        .unwrap_or_else(|| "Not reported".to_string())
}

pub(super) fn policy_preview_required_readable_value(key: &str) -> String {
    policy_preview_readable_label(key)
        .unwrap_or(key)
        .to_string()
}

pub(super) fn policy_preview_readable_label(value: &str) -> Option<&'static str> {
    match value {
        "preview-required" => Some("Preview required"),
        "ready-to-save" => Some("Ready to save"),
        "blocked" => Some("Blocked"),
        "required" => Some("Required"),
        "not-required" => Some("Not required"),
        "supported" => Some("Supported"),
        "unsupported" => Some("Unsupported"),
        "manual-required" => Some("Manual required"),
        "offline" => Some("Offline"),
        "stale" => Some("Stale"),
        "draft" => Some("Draft"),
        "preview" => Some("Preview"),
        "confirmed" => Some("Confirmed"),
        "queued" => Some("Queued"),
        "delivered" => Some("Delivered"),
        "acknowledged" => Some("Acknowledged"),
        "active" => Some("Active"),
        "partially-active" => Some("Partially active"),
        "rejected" => Some("Rejected"),
        "superseded" => Some("Superseded"),
        "rolled-back" => Some("Rolled back"),
        "expired" => Some("Expired"),
        "parent-portal" => Some("Parent portal"),
        "parent-companion" => Some("Parent companion"),
        "ai-preview" => Some("AI preview"),
        "domain-cache" => Some("Domain cache"),
        "child" => Some("Child"),
        "assistant-draft" => Some("Assistant draft"),
        "parent-confirmation-required" => Some("Parent confirmation required"),
        "parent-confirmed" => Some("Parent confirmed"),
        "preview-only" => Some("Preview only"),
        "pending-parent-review" => Some("Pending parent review"),
        "approved" => Some("Approved"),
        "denied" => Some("Denied"),
        "modified" => Some("Modified"),
        "replay-rejected" => Some("Replay rejected"),
        "unavailable" => Some("Unavailable"),
        "disabled" => Some("Off"),
        "local-only" => Some("Local only"),
        "local-adapter-unavailable" => Some("Local adapter not connected"),
        _ => None,
    }
}

pub(super) fn policy_preview_reviewed_by_value(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> String {
    let actor_id = read_model
        .and_then(|value| value.policy_reviewed_by_actor_id.as_ref())
        .map(ToString::to_string);
    let actor_role = read_model.and_then(|value| value.policy_reviewed_by_actor_role.as_deref());

    match (actor_id.as_deref(), actor_role) {
        (None, None) => policy_preview_optional_value(None),
        (Some(actor_id), Some(actor_role)) => format!("{actor_id} ({actor_role})"),
        (Some(actor_id), None) => actor_id.to_string(),
        (None, Some(actor_role)) => actor_role.to_string(),
    }
}

pub(super) fn policy_preview_product_claim() -> String {
    "Policy preview is advisory parent-surface state only. It does not claim enforcement, adapter execution, provider delivery, or child-device application."
        .to_string()
}

pub(super) fn policy_preview_detail(
    label: &str,
    value: String,
) -> ParentPolicyPreviewPanelDetailSnapshot {
    ParentPolicyPreviewPanelDetailSnapshot {
        label: label.to_string(),
        value,
    }
}
