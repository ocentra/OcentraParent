use std::fmt::Display;

use serde_json::Value;

use super::*;

pub(super) const TRACKING_STATUS_NOT_REPORTED: &str = "Not reported";
pub(super) const TRACKING_STATUS_UNAVAILABLE: &str = "Unavailable";

pub(super) fn tracking_overview_cards(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_family_dashboard_card(read_model_result, product_claim),
        tracking_unavailable_boundary_card(
            "report-policy-consumer-ui",
            "Tracking report policy consumer UI",
            "No Rust-owned tracking report-policy result is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "report-export-ui",
            "Tracking report export UI",
            "No Rust-owned tracking export result is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "notification-history-intent-ui",
            "Notification history intent UI",
            "No Rust-owned notification history result is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "parent-action-readiness-ui",
            "Parent action readiness UI",
            "No owner-authorized tracking action-readiness result is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "missing-device-ui",
            "Missing device UI",
            "No Rust-owned missing-device decision is supplied to this surface.",
            product_claim,
        ),
    ]
}

pub(super) fn tracking_device_action_cards(
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_unavailable_boundary_card(
            "child-check-in-request",
            "Child check-in request",
            "No authenticated child check-in request or receipt is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "child-runtime-ui",
            "Child runtime UI",
            "No authenticated child-runtime delivery or status is supplied to this surface.",
            product_claim,
        ),
        tracking_unavailable_boundary_card(
            "unsupported-manual-platform",
            "Unsupported manual platform",
            "No platform capability proof is supplied to this surface.",
            product_claim,
        ),
    ]
}

pub(super) fn tracking_family_dashboard_card(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let details = match read_model_result.and_then(|result| result.value.as_ref()) {
        Some(read_model) => vec![
            ("Status", tracking_read_model_state(read_model_result)),
            (
                "Visible devices",
                read_model.active_device_counts.len().to_string(),
            ),
            ("Active tracking rows", read_model.active_rows.to_string()),
            ("Tombstone rows", read_model.tombstone_rows.to_string()),
            (
                "Deleted evidence refs",
                read_model.deleted_evidence_reference_ids.len().to_string(),
            ),
            ("Product claim", product_claim.to_string()),
        ],
        None => vec![
            ("Status", tracking_read_model_state(read_model_result)),
            ("Visible devices", TRACKING_STATUS_NOT_REPORTED.to_string()),
            (
                "Active tracking rows",
                TRACKING_STATUS_NOT_REPORTED.to_string(),
            ),
            ("Tombstone rows", TRACKING_STATUS_NOT_REPORTED.to_string()),
            (
                "Deleted evidence refs",
                TRACKING_STATUS_NOT_REPORTED.to_string(),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    };
    tracking_card(
        "family-dashboard-rollup",
        "Family dashboard tracking rollup",
        details,
    )
}

pub(super) fn tracking_unavailable_boundary_card(
    key: &str,
    title: &str,
    reason: &str,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        key,
        title,
        vec![
            ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Reason", reason.to_string()),
            ("Product claim", product_claim.to_string()),
        ],
    )
}

pub(super) fn tracking_card(
    key: &str,
    title: &str,
    details: Vec<(&str, String)>,
) -> ParentTrackingStatusPanelCardSnapshot {
    ParentTrackingStatusPanelCardSnapshot {
        key: key.to_string(),
        title: title.to_string(),
        details: details
            .into_iter()
            .map(|(label, value)| tracking_detail(label, value))
            .collect(),
    }
}

pub(super) fn tracking_detail(
    label: &str,
    value: impl Into<String>,
) -> ParentTrackingStatusPanelDetailSnapshot {
    ParentTrackingStatusPanelDetailSnapshot {
        label: label.to_string(),
        value: value.into(),
    }
}

pub(super) fn tracking_read_model_state(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
) -> String {
    match read_model_result {
        Some(result) if result.ok && result.value.is_some() => "ready".to_string(),
        Some(result) => result
            .reason
            .as_ref()
            .map(serialized_enum_label)
            .unwrap_or_else(|| TRACKING_STATUS_UNAVAILABLE.to_string()),
        None => TRACKING_STATUS_UNAVAILABLE.to_string(),
    }
}

pub(super) fn tracking_count_summary(
    counts: &[ocentra_schema::parent_ui_bridge::ParentActivityTrackingReadModelCountSnapshot],
) -> String {
    if counts.is_empty() {
        return TRACKING_STATUS_UNAVAILABLE.to_string();
    }
    counts
        .iter()
        .map(|count| format!("{} ({})", count.value, count.count))
        .collect::<Vec<_>>()
        .join(" | ")
}

pub(super) fn tracking_refs<T>(values: &[T]) -> String
where
    T: Display,
{
    if values.is_empty() {
        TRACKING_STATUS_NOT_REPORTED.to_string()
    } else {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

pub(super) fn tracking_option_value<T>(value: Option<&T>) -> String
where
    T: Display + ?Sized,
{
    value
        .map(ToString::to_string)
        .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string())
}

pub(super) fn tracking_json_string(value: Option<&Value>, field: &str) -> String {
    value
        .and_then(|value| value.get(field))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string())
}
