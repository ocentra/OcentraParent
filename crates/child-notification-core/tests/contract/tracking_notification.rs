use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_evaluation_id_from_observation_id, tracking_evidence_ref_from_observation_id,
    tracking_notification_id_from_violation_id, tracking_violation_id_from_evaluation_and_rule_ref,
    TrackingChildDeviceId, TrackingChildProfileId, TrackingNotificationChannel,
    TrackingObservationId, TrackingPolicyRuleRef, TrackingPolicySeverity, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingPolicyViolationDetectedEvent;

fn tracking_policy_violation_fixture() -> TrackingPolicyViolationDetectedEvent {
    let observation_id =
        TrackingObservationId::parse(constants::tracking_runtime::DEFAULT_OBSERVATION_ID)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVATION_ID);
    let policy_rule_ref =
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .expect_value(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);

    TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: tracking_violation_id_from_evaluation_and_rule_ref(
            &tracking_evaluation_id_from_observation_id(&observation_id),
            &policy_rule_ref,
        ),
        policy_rule_ref,
        severity: TrackingPolicySeverity::parse(
            constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
        )
        .expect_value(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![tracking_evidence_ref_from_observation_id(&observation_id)],
    }
}

#[test]
fn tracking_notification_keeps_policy_violation_as_source_authority() {
    let violation = tracking_policy_violation_fixture();

    let notification =
        ocentra_child_notification_core::tracking_notification::request_parent_notification_from_policy_violation(
            &violation,
        );

    assert_eq!(
        notification.channel,
        TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect_value(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL)
    );
    assert_eq!(
        notification.source_policy_violation_id,
        violation.violation_id
    );
    assert_eq!(
        notification.notification_id,
        tracking_notification_id_from_violation_id(&violation.violation_id)
    );
    assert_eq!(notification.requested_at, violation.detected_at);
    assert_eq!(notification.evidence_refs, violation.evidence_refs);
}
