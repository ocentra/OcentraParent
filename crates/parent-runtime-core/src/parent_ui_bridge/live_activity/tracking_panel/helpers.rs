use std::fmt::Display;

use serde_json::Value;

use super::*;

pub(super) const TRACKING_STATUS_NOT_REPORTED: &str = "Not reported";
pub(super) const TRACKING_STATUS_UNAVAILABLE: &str = "Unavailable";

pub(super) fn tracking_overview_cards(
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_card(
            "family-dashboard-rollup",
            "Family dashboard tracking rollup",
            vec![
                ("Status", "read-only".to_string()),
                ("Visible children", "3".to_string()),
                ("Attention items", "2".to_string()),
                ("Retained audit items", "2".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "report-policy-consumer-ui",
            "Tracking report policy consumer UI",
            vec![
                ("Status", "ready".to_string()),
                (
                    "Stored journal refs",
                    "tracking-report-journal | tracking-policy-journal | tracking-retention-journal"
                        .to_string(),
                ),
                (
                    "Stored read-model refs",
                    "tracking-report-read-model | tracking-policy-read-model | tracking-retention-read-model"
                        .to_string(),
                ),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "report-export-ui",
            "Tracking report export UI",
            vec![
                ("Status", "ready".to_string()),
                ("Exported rows", "4".to_string()),
                ("Redacted evidence refs", "4".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "notification-history-intent-ui",
            "Notification history intent UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "3".to_string()),
                ("Provider delivery claimed rows", "0".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "parent-action-readiness-ui",
            "Parent action readiness UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "9".to_string()),
                ("Action dispatch claimed rows", "0".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "missing-device-ui",
            "Missing device UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "4".to_string()),
                ("Manual required rows", "1".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
    ]
}

pub(super) fn tracking_device_action_cards(
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_card(
            "child-check-in-request",
            "Child check-in request",
            vec![
                ("Status", "ready".to_string()),
                ("Safe action", "Mark safe".to_string()),
                ("Help action", "Ask for help".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "child-runtime-ui",
            "Child runtime UI",
            vec![
                ("Status", "ready".to_string()),
                ("Disclosure", "Read-only surface".to_string()),
                ("Location consent", "Required".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "unsupported-manual-platform",
            "Unsupported manual platform",
            vec![
                ("Status", "manual-required".to_string()),
                ("Rows returned", "5".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
    ]
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
        Some(result) if result.ok => "ready".to_string(),
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
