use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef, TrackingPolicyRuleRef,
    TrackingPolicySeverity, TrackingPolicyViolationId, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingPolicyViolationDetectedEvent;
use ocentra_tracking_core::parent_acknowledgement::record_parent_acknowledgement;

#[test]
fn parent_acknowledgement_is_idempotent_for_same_policy_violation() {
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

    let first = record_parent_acknowledgement(&violation);
    let second = record_parent_acknowledgement(&violation);

    assert_eq!(first.acknowledgement_id, second.acknowledgement_id);
    assert_eq!(
        first.source_policy_violation_id,
        second.source_policy_violation_id
    );
    assert_eq!(first.evidence_refs, second.evidence_refs);
}
