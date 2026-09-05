#[path = "product_cards/boundaries.rs"]
mod boundaries;
#[path = "product_cards/coverage.rs"]
mod coverage;
#[path = "product_cards/current.rs"]
mod current;

use self::boundaries::{tracking_action_readiness_card, tracking_child_surface_card};
use self::coverage::{tracking_event_coverage_card, tracking_retention_custody_card};
use self::current::{tracking_current_device_card, tracking_location_surface_card};
use super::*;
use ocentra_parent_agent_protocol::constants::activity_event_kind;

pub(super) fn tracking_product_cards(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
    latest_row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    let read_model = read_model_result.and_then(|result| result.value.as_ref());
    let latest_active = current::latest_active_row(read_model);
    let mut cards = vec![
        tracking_family_dashboard_card(read_model_result, product_claim),
        tracking_current_device_card(latest_active, product_claim),
        tracking_location_surface_card(read_model, product_claim),
        tracking_event_coverage_card(read_model, product_claim),
        tracking_retention_custody_card(read_model, product_claim),
        tracking_child_surface_card(read_model, product_claim),
        tracking_action_readiness_card(product_claim),
    ];
    if let Some(result) = write_result {
        cards.push(tracking_retention_settings_card(
            Some(result),
            product_claim,
        ));
    }
    if let Some(row) = latest_row {
        cards.push(tracking_evidence_drawer_card(Some(row), product_claim));
    }
    cards
}

pub(super) fn tracking_activity_label(kind: &str) -> &'static str {
    match kind {
        activity_event_kind::LOCATION_OBSERVED => "Location update",
        activity_event_kind::TRACKING_ALERT_EVALUATED => "Tracking alert",
        activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED => "Geofence transition",
        activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED => "Expected-place status",
        activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED => "Child check-in",
        activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED => "Parent notification",
        activity_event_kind::TRACKING_RETENTION_DELETED => "Retention deletion",
        _ => "Tracking activity",
    }
}
