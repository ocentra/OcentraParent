use ocentra_parent_agent_protocol::constants::activity_event_kind;
use ocentra_schema::parent_ui_bridge::ParentActivityTrackingReadModelSnapshot;

use super::super::*;
use super::coverage::tracking_kind_count;

pub(super) fn tracking_child_surface_card(
    read_model: Option<&ParentActivityTrackingReadModelSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        "tracking-child-surface",
        "Child tracking surface",
        vec![
            ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
            (
                "Recorded check-ins",
                tracking_kind_count(
                    read_model,
                    activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
                ),
            ),
            ("Authenticated delivery", "Not supplied".to_string()),
            ("Location consent", "Not supplied".to_string()),
            ("Safe or help response", "Not supplied".to_string()),
            (
                "Reason",
                "The current service read model contains history, not an authenticated child-runtime status or action channel."
                    .to_string(),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    )
}

pub(super) fn tracking_action_readiness_card(
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        "tracking-action-readiness",
        "Tracking controls",
        vec![
            ("Status", TRACKING_STATUS_UNAVAILABLE.to_string()),
            (
                "Exception editor",
                "No owner-authorized mutation input".to_string(),
            ),
            (
                "Child check-in",
                "No authenticated delivery input".to_string(),
            ),
            (
                "Temporary live tracking",
                "No durable live-session input".to_string(),
            ),
            (
                "Missing-device action",
                "No owner-authorized decision input".to_string(),
            ),
            (
                "Notification preferences",
                "No writable preference input".to_string(),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    )
}
