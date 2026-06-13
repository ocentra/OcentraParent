use ocentra_parent_agent_protocol::{
    constants, TrackingAlertSeverity, TrackingChildDeviceId, TrackingChildProfileId,
    TrackingEvidenceRef, TrackingPolicyRuleRef, TrackingPolicySeverity,
    TrackingPolicyViolationDetectedEvent, TrackingPolicyViolationId, TrackingTimestamp,
};
use ocentra_tracking_core::TrackingParentNotificationDecisionState;

#[test]
fn alert_decision_rate_limits_duplicate_parent_notifications() {
    let violation = TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
            .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID)
            .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingPolicySeverity::parse(constants::tracking_runtime::POLICY_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    };

    let decision = ocentra_tracking_core::evaluate_tracking_alert(&violation, 1);

    assert_eq!(
        decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_NONE)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_NONE)
    );
    assert_eq!(
        decision.parent_notification_state,
        TrackingParentNotificationDecisionState::Suppressed
    );
}
