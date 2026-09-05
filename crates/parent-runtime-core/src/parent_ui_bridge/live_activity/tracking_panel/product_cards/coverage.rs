use ocentra_parent_agent_protocol::constants::activity_event_kind;
use ocentra_schema::parent_ui_bridge::ParentActivityTrackingReadModelSnapshot;

use super::super::*;

pub(super) fn tracking_event_coverage_card(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        "tracking-event-coverage",
        "Tracking activity coverage",
        vec![
            ("Status", tracking_model_availability(read_model)),
            (
                "Location updates",
                tracking_kind_count(read_model, activity_event_kind::LOCATION_OBSERVED),
            ),
            (
                "Geofence transitions",
                tracking_kind_count(
                    read_model,
                    activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
                ),
            ),
            (
                "Expected-place states",
                tracking_kind_count(
                    read_model,
                    activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
                ),
            ),
            (
                "Alerts",
                tracking_kind_count(read_model, activity_event_kind::TRACKING_ALERT_EVALUATED),
            ),
            (
                "Child check-ins",
                tracking_kind_count(
                    read_model,
                    activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
                ),
            ),
            (
                "Parent notifications",
                tracking_kind_count(
                    read_model,
                    activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
                ),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    )
}

pub(super) fn tracking_retention_custody_card(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let details = match read_model {
        Some(read_model) => vec![
            ("Status", "Read-only".to_string()),
            ("Custody", read_model.custody_label.clone()),
            ("Active rows", read_model.active_rows.to_string()),
            ("Deleted rows", read_model.tombstone_rows.to_string()),
            (
                "Last deletion",
                tracking_option_value(read_model.latest_tombstone_observed_at.as_deref()),
            ),
            (
                "Deleted evidence refs",
                tracking_refs(&read_model.deleted_evidence_reference_ids),
            ),
            ("Product claim", product_claim.to_string()),
        ],
        None => vec![
            ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Custody", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Active rows", "0".to_string()),
            ("Deleted rows", "0".to_string()),
            ("Last deletion", TRACKING_STATUS_NOT_REPORTED.to_string()),
            (
                "Deleted evidence refs",
                TRACKING_STATUS_NOT_REPORTED.to_string(),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    };
    tracking_card(
        "tracking-retention-custody",
        "Custody and retention",
        details,
    )
}

pub(super) fn tracking_kind_count(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
    kind: &str,
) -> String {
    let count = read_model
        .and_then(|value| {
            value
                .active_kind_counts
                .iter()
                .find(|count| count.value == kind)
        })
        .map(|count| count.count)
        .unwrap_or(0);
    if count == 0 {
        TRACKING_STATUS_NOT_REPORTED.to_string()
    } else {
        format!("{count} reported")
    }
}

fn tracking_model_availability(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
) -> String {
    read_model
        .map(|_| "Reported".to_string())
        .unwrap_or_else(|| TRACKING_STATUS_UNAVAILABLE.to_string())
}
