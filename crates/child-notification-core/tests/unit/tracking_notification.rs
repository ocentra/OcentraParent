use ocentra_parent_agent_protocol::{
    constants, TrackingChildDeviceId, TrackingChildProfileId, TrackingNotificationChannel,
    TrackingPolicyRuleRef, TrackingPolicySeverity,
    TrackingPolicyViolationDetectedEvent, tracking_evaluation_id_from_observation_id,
    tracking_evidence_ref_from_observation_id, tracking_notification_id_from_violation_id,
    tracking_violation_id_from_evaluation_and_rule_ref, TrackingObservationId,
};

fn tracking_policy_violation_fixture() -> TrackingPolicyViolationDetectedEvent {
    let observation_id = TrackingObservationId::parse(constants::tracking_runtime::DEFAULT_OBSERVATION_ID)
        .expect(constants::tracking_runtime::DEFAULT_OBSERVATION_ID);
    let policy_rule_ref = TrackingPolicyRuleRef::parse(
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
    )
    .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);

    TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: tracking_violation_id_from_evaluation_and_rule_ref(
            &tracking_evaluation_id_from_observation_id(&observation_id),
            &policy_rule_ref,
        ),
        policy_rule_ref,
        severity: TrackingPolicySeverity::parse(constants::tracking_runtime::POLICY_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        evidence_refs: vec![tracking_evidence_ref_from_observation_id(&observation_id)],
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
    assert_eq!(
        notification.notification_id,
        tracking_notification_id_from_violation_id(&violation.violation_id)
    );
    assert_eq!(notification.evidence_refs, violation.evidence_refs);
}
