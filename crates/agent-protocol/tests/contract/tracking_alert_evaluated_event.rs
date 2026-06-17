use ocentra_eventing::envelope::DomainEvent;
use ocentra_parent_agent_protocol::{
    constants, TrackingAlertEvaluatedEvent, TrackingAlertEvaluationId, TrackingAlertSeverity,
    TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingParentNotificationState, TrackingPolicyRuleRef, TrackingPolicyViolationId,
    TrackingTimestamp,
};

#[test]
fn alert_evaluated_event_uses_tracking_contract_and_idempotency() {
    let event = alert_evaluated_fixture(
        constants::tracking_runtime::ALERT_SEVERITY_WATCH,
        TrackingParentNotificationState::Allowed,
        vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    );

    let contract = event
        .contract()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let idempotency = event
        .idempotency_key()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE
    );
    assert_eq!(
        idempotency.as_str(),
        format!(
            "{}:{}",
            constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE,
            constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID
        )
    );
}

#[test]
fn alert_evaluated_event_serializes_suppressed_missing_evidence_state() {
    let event = alert_evaluated_fixture(
        constants::tracking_runtime::ALERT_SEVERITY_INFO,
        TrackingParentNotificationState::SuppressedMissingEvidence,
        vec![],
    );

    let serialized = serde_json::to_value(&event).expect("tracking alert event serializes");

    assert_eq!(
        serialized["severity"],
        constants::tracking_runtime::ALERT_SEVERITY_INFO
    );
    assert_eq!(
        serialized["parentNotificationState"],
        constants::tracking_runtime::PARENT_NOTIFICATION_STATE_SUPPRESSED_MISSING_EVIDENCE
    );
    assert_eq!(serialized["evidenceRefs"], serde_json::json!([]));
}

fn alert_evaluated_fixture(
    severity: &'static str,
    parent_notification_state: TrackingParentNotificationState,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> TrackingAlertEvaluatedEvent {
    TrackingAlertEvaluatedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        alert_evaluation_id: TrackingAlertEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID),
        source_policy_violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingAlertSeverity::parse(severity).expect(severity),
        parent_notification_state,
        evaluated_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs,
    }
}
