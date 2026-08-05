use super::*;
use std::fmt::Display;

const POLICY_PREVIEW_READABLE_LABELS: &[(&str, &str)] = &[
    ("preview-required", "Preview required"),
    ("ready-to-save", "Ready to save"),
    ("blocked", "Blocked"),
    ("required", "Required"),
    ("not-required", "Not required"),
    ("supported", "Supported"),
    ("unsupported", "Unsupported"),
    ("manual-required", "Manual required"),
    ("offline", "Offline"),
    ("stale", "Stale"),
    ("draft", "Draft"),
    ("preview", "Preview"),
    ("confirmed", "Confirmed"),
    ("queued", "Queued"),
    ("delivered", "Delivered"),
    ("acknowledged", "Acknowledged"),
    ("active", "Active"),
    ("partially-active", "Partially active"),
    ("rejected", "Rejected"),
    ("superseded", "Superseded"),
    ("rolled-back", "Rolled back"),
    ("expired", "Expired"),
    ("parent-portal", "Parent portal"),
    ("parent-companion", "Parent companion"),
    ("ai-preview", "AI preview"),
    ("domain-cache", "Domain cache"),
    ("child", "Child"),
    ("assistant-draft", "Assistant draft"),
    (
        "parent-confirmation-required",
        "Parent confirmation required",
    ),
    ("parent-confirmed", "Parent confirmed"),
    ("preview-only", "Preview only"),
    ("pending-parent-review", "Pending parent review"),
    ("approved", "Approved"),
    ("denied", "Denied"),
    ("modified", "Modified"),
    ("replay-rejected", "Replay rejected"),
    ("unavailable", "Unavailable"),
    ("disabled", "Off"),
    ("local-only", "Local only"),
    ("local-adapter-unavailable", "Local adapter not connected"),
];
const POLICY_PREVIEW_PARENT_ACCESS_LABELS: &[(&str, &str)] = &[
    ("active-controller", "Active controller"),
    ("observer-only", "Observer only"),
    ("unauthenticated", "Unauthenticated"),
    ("proof-missing", "Proof missing"),
];
const POLICY_PREVIEW_CONFLICT_FINDING_KINDS: &[&str] = &[
    "schedule-conflict",
    "overlapping-schedule",
    "timezone-boundary",
    "ambiguous-local-time",
    "nonexistent-local-time",
    "clock-skew",
];
const POLICY_PREVIEW_CONFLICT_EXPLANATION_CODES: &[&str] = &[
    "schedule-conflict",
    "overlapping-schedule",
    "schedule-timezone-boundary",
    "timezone-boundary-conflict",
    "ambiguous-local-time",
    "nonexistent-local-time",
    "clock-skew",
];

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
    policy_preview_has_protocol_value(
        read_model.policy_preview_finding_kinds.as_deref(),
        POLICY_PREVIEW_CONFLICT_FINDING_KINDS,
    ) || policy_preview_has_protocol_value(
        read_model.policy_preview_target_explanation_code.as_deref(),
        POLICY_PREVIEW_CONFLICT_EXPLANATION_CODES,
    )
}

fn policy_preview_has_protocol_value(value: Option<&str>, expected_values: &[&str]) -> bool {
    value
        .into_iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .any(|value| {
            expected_values
                .iter()
                .any(|expected| value.eq_ignore_ascii_case(expected))
        })
}

pub(super) fn policy_preview_parent_access_readable_value(
    parent_access_state: &ParentPortalParentAccessState,
) -> String {
    let value = serialized_enum_label(parent_access_state);
    POLICY_PREVIEW_PARENT_ACCESS_LABELS
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or(value)
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
    POLICY_PREVIEW_READABLE_LABELS
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, label)| *label)
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
