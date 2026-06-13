use ocentra_parent_agent_protocol::{
    constants, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingNotificationChannel, TrackingPolicyRuleRef, TrackingPolicySeverity,
    TrackingPolicyViolationDetectedEvent, TrackingPolicyViolationId,
};

fn tracking_policy_violation_fixture() -> TrackingPolicyViolationDetectedEvent {
    TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
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
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
    }
}

#[test]
fn tracking_notification_keeps_policy_violation_as_source_authority() {
    let violation = tracking_policy_violation_fixture();

    let notification =
        ocentra_child_notification_core::request_parent_notification_from_policy_violation(
            &violation,
        );

    assert_eq!(
        notification.channel,
        TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL)
    );
    assert_eq!(
        notification.source_policy_violation_id,
        violation.violation_id
    );
    assert_eq!(notification.evidence_refs, violation.evidence_refs);
}
