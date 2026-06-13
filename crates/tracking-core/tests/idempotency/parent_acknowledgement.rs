use ocentra_parent_agent_protocol::{
    constants, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingPolicyRuleRef, TrackingPolicySeverity, TrackingPolicyViolationDetectedEvent,
    TrackingPolicyViolationId,
};

#[test]
fn parent_acknowledgement_is_idempotent_for_same_policy_violation() {
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
        evidence_refs: vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    };

    let first = ocentra_tracking_core::record_parent_acknowledgement(&violation);
    let second = ocentra_tracking_core::record_parent_acknowledgement(&violation);

    assert_eq!(first.acknowledgement_id, second.acknowledgement_id);
    assert_eq!(first.source_policy_violation_id, second.source_policy_violation_id);
    assert_eq!(first.evidence_refs, second.evidence_refs);
}
