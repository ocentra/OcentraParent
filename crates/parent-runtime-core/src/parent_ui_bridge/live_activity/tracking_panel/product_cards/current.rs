use ocentra_parent_agent_protocol::{
    constants::activity_event_kind, tracking::read_model::TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
};
use ocentra_schema::parent_ui_bridge::ParentActivityTrackingReadModelSnapshot;

use super::super::*;

pub(super) fn latest_active_row(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
) -> Option<&ParentActivityTrackingReadModelRowSnapshot> {
    read_model.and_then(|value| {
        value
            .rows
            .iter()
            .find(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE)
    })
}

pub(super) fn tracking_current_device_card(
    row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let details = match row {
        Some(row) => vec![
            (
                "Status",
                tracking_option_value(row.capability_status.as_deref()),
            ),
            ("Device", row.device_id.to_string()),
            (
                "Child or place",
                row.subject_display_name
                    .clone()
                    .unwrap_or_else(|| row.subject_id.to_string()),
            ),
            ("Last observed", row.observed_at.clone()),
            ("Platform", row.platform.clone()),
            ("Source", row.observer.clone()),
            (
                "Latest activity",
                tracking_activity_label(&row.kind).to_string(),
            ),
            ("Evidence refs", tracking_refs(&row.evidence_reference_ids)),
            ("Product claim", product_claim.to_string()),
        ],
        None => vec![
            ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Device", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Child or place", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Last observed", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Reason", "No active tracking row is available.".to_string()),
            ("Product claim", product_claim.to_string()),
        ],
    };
    tracking_card("tracking-current-device", "Current child status", details)
}

pub(super) fn tracking_location_surface_card(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let location_row = read_model.and_then(|value| {
        value.rows.iter().find(|row| {
            row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
                && row.kind == activity_event_kind::LOCATION_OBSERVED
        })
    });
    let details = match location_row {
        Some(row) => reported_location_details(row, product_claim),
        None => unavailable_location_details(product_claim),
    };
    tracking_card("tracking-location-surface", "Last known location", details)
}

fn reported_location_details(
    row: &ParentActivityTrackingReadModelRowSnapshot,
    product_claim: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("Status", "Reported".to_string()),
        ("Device", row.device_id.to_string()),
        ("Last observed", row.observed_at.clone()),
        (
            "Location label",
            row.subject_display_name
                .clone()
                .unwrap_or_else(|| row.subject_id.to_string()),
        ),
        (
            "Accuracy",
            "Not projected by the service read model".to_string(),
        ),
        (
            "Map coordinates",
            "Not projected by the service read model".to_string(),
        ),
        (
            "Permission",
            "Not projected by the service read model".to_string(),
        ),
        ("Product claim", product_claim.to_string()),
    ]
}

fn unavailable_location_details(product_claim: &str) -> Vec<(&'static str, String)> {
    vec![
        ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
        ("Last observed", TRACKING_STATUS_NOT_REPORTED.to_string()),
        ("Location label", TRACKING_STATUS_NOT_REPORTED.to_string()),
        ("Accuracy", "Not supplied".to_string()),
        ("Map coordinates", "Not supplied".to_string()),
        ("Permission", "Not supplied".to_string()),
        (
            "Reason",
            "No active location-observed row is supplied by the service read model.".to_string(),
        ),
        ("Product claim", product_claim.to_string()),
    ]
}
