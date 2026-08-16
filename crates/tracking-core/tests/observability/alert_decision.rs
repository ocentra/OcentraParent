use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingAlertSeverity, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingPolicyRuleRef, TrackingPolicySeverity, TrackingPolicyViolationId, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingPolicyViolationDetectedEvent;
use ocentra_tracking_core::alerting::{
    evaluate_tracking_alert, TrackingParentNotificationDecisionState,
};

#[test]
fn alert_decision_rate_limits_duplicate_parent_notifications_without_lowering_severity() {
    let violation = TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect_value(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingPolicySeverity::parse(
            constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
        )
        .expect_value(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
    };

    let decision = evaluate_tracking_alert(&violation, 1);

    assert_eq!(
        decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
            .expect_value(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
    );
    assert_eq!(
        decision.parent_notification_state,
        TrackingParentNotificationDecisionState::SuppressedDuplicate
    );
}
