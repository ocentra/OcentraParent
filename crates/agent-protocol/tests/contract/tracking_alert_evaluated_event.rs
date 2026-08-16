use ocentra_eventing::envelope::DomainEvent;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAlertEvaluatedEvent, TrackingParentNotificationState,
};
use ocentra_parent_agent_protocol::TrackingAlertEvaluationId;
use ocentra_parent_agent_protocol::TrackingAlertSeverity;
use ocentra_parent_agent_protocol::TrackingChildDeviceId;
use ocentra_parent_agent_protocol::TrackingChildProfileId;
use ocentra_parent_agent_protocol::TrackingEvidenceRef;
use ocentra_parent_agent_protocol::TrackingPolicyRuleRef;
use ocentra_parent_agent_protocol::TrackingPolicyViolationId;
use ocentra_parent_agent_protocol::TrackingTimestamp;

#[test]
fn alert_evaluated_event_uses_tracking_contract_and_idempotency(
) -> Result<(), Box<dyn std::error::Error>> {
    let event = alert_evaluated_fixture(
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)?,
        TrackingParentNotificationState::Allowed,
        vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )?],
    )?;

    let contract = event.contract()?;
    let idempotency = event.idempotency_key()?;

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
    Ok(())
}

#[test]
fn alert_evaluated_event_serializes_suppressed_missing_evidence_state(
) -> Result<(), Box<dyn std::error::Error>> {
    let event = alert_evaluated_fixture(
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_INFO)?,
        TrackingParentNotificationState::SuppressedMissingEvidence,
        vec![],
    )?;

    let serialized = serde_json::to_value(&event)?;

    assert_eq!(
        serialized["severity"],
        constants::tracking_runtime::ALERT_SEVERITY_INFO
    );
    assert_eq!(
        serialized["parentNotificationState"],
        constants::tracking_runtime::PARENT_NOTIFICATION_STATE_SUPPRESSED_MISSING_EVIDENCE
    );
    assert_eq!(serialized["evidenceRefs"], serde_json::json!([]));
    Ok(())
}

fn alert_evaluated_fixture(
    severity: TrackingAlertSeverity,
    parent_notification_state: TrackingParentNotificationState,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> Result<TrackingAlertEvaluatedEvent, Box<dyn std::error::Error>> {
    Ok(TrackingAlertEvaluatedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )?,
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )?,
        alert_evaluation_id: TrackingAlertEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID,
        )?,
        source_policy_violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )?,
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )?,
        severity,
        parent_notification_state,
        evaluated_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)?,
        evidence_refs,
    })
}
