#[path = "policy_control_notification_support.rs"]
mod policy_control_notification_support;

use ocentra_child_notification_core::policy_control_notification::{
    build_policy_control_parent_notification, PolicyControlNotificationState,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState;

use policy_control_notification_support::{
    approved_override, approved_request, blocked_delivery, legacy_unverified_acknowledged_delivery,
    preview_request, queued_delivery, replay_rejected_request, retry_delivery,
};

#[test]
fn preview_only_request_stays_confirmation_gated() {
    let notification = build_policy_control_parent_notification(&preview_request(), None, None)
        .expect_value("preview notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::PreviewOnly
    );
    assert!(notification.source_approval_id.is_none());
    assert!(notification.source_override_id.is_none());
    assert!(notification.delivery_parent_visible_state.is_none());
}

#[test]
fn approved_request_and_queued_delivery_keep_override_and_audit_context() {
    let request = approved_request();
    let temporary_override = approved_override();
    let queued = queued_delivery();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&queued),
    )
    .expect_value("queued delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryPending
    );
    assert_eq!(
        notification
            .source_approval_id
            .as_ref()
            .expect_value("source approval id")
            .as_str(),
        "request-bonus-time-grant"
    );
    assert_eq!(
        notification
            .source_override_id
            .as_ref()
            .expect_value("source override id")
            .as_str(),
        "policy-override:request-bonus-time-grant"
    );
    assert_eq!(notification.audit_reference_ids.len(), 4);
}

#[test]
fn legacy_unverified_acknowledged_delivery_requires_manual_parent_action() {
    let request = approved_request();
    let temporary_override = approved_override();
    let legacy_acknowledged = legacy_unverified_acknowledged_delivery();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&legacy_acknowledged),
    )
    .expect_value("legacy acknowledged delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryManualRequired
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect_value("delivery parent visible state"),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(notification.audit_reference_ids.len(), 4);
}

#[test]
fn retry_and_partial_delivery_states_stay_parent_visible_as_degraded() {
    let request = approved_request();
    let temporary_override = approved_override();
    let retry = retry_delivery();

    let notification =
        build_policy_control_parent_notification(&request, Some(&temporary_override), Some(&retry))
            .expect_value("retry delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryDegraded
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect_value("delivery parent visible state"),
        PolicyDeliveryParentVisibleState::Degraded
    );
}

#[test]
fn blocked_delivery_states_surface_manual_required_notifications() {
    let request = approved_request();
    let temporary_override = approved_override();
    let blocked = blocked_delivery();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&blocked),
    )
    .expect_value("blocked delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryManualRequired
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect_value("delivery parent visible state"),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
}

#[test]
fn denied_request_cannot_fake_override_or_delivery() {
    let mut denied = approved_request();
    denied.status = PolicyRequestStatus::Denied;
    denied.resolved_at = Some(
        ocentra_policy_control_core::policy_request::PolicyRequestTimestamp::parse(
            "2026-06-13T20:05:00Z",
        )
        .expect_value("policy request timestamp"),
    );

    let error = build_policy_control_parent_notification(&denied, Some(&approved_override()), None)
        .expect_err_value("denied request must not carry override");
    assert!(error
        .to_string()
        .contains("policy_control_notification.override_id"));
}

#[test]
fn replay_rejected_request_is_rejected_for_parent_notification() {
    let replay_rejected = replay_rejected_request();

    let error = build_policy_control_parent_notification(&replay_rejected, None, None)
        .expect_err_value("replay-rejected status must not produce a parent notification");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.status",
            value: "replay-rejected".to_string(),
        }
    );
}
