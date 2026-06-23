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
fn review_policy_violation_maps_to_watch_alert_when_notification_is_allowed() {
    let decision = evaluate_tracking_alert(
        &tracking_policy_violation(
            constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
            vec![tracking_evidence_ref()],
        ),
        0,
    );

    assert_eq!(
        decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
            .unwrap_or_else(|_| unreachable!(
                "{}",
                constants::tracking_runtime::ALERT_SEVERITY_WATCH
            ))
    );
    assert_eq!(
        decision.parent_notification_state,
        TrackingParentNotificationDecisionState::Allowed
    );
}

#[test]
fn duplicate_alert_suppression_preserves_urgent_severity() {
    let decision = evaluate_tracking_alert(
        &tracking_policy_violation(
            constants::tracking_runtime::POLICY_SEVERITY_URGENT,
            vec![tracking_evidence_ref()],
        ),
        2,
    );

    assert_eq!(
        decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_URGENT)
            .unwrap_or_else(|_| unreachable!(
                "{}",
                constants::tracking_runtime::ALERT_SEVERITY_URGENT
            ))
    );
    assert_eq!(
        decision.parent_notification_state,
        TrackingParentNotificationDecisionState::SuppressedDuplicate
    );
}

#[test]
fn missing_evidence_is_downgraded_to_info_and_suppressed() {
    let decision = evaluate_tracking_alert(
        &tracking_policy_violation(
            constants::tracking_runtime::POLICY_SEVERITY_CRITICAL,
            vec![],
        ),
        0,
    );

    assert_eq!(
        decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_INFO)
            .unwrap_or_else(|_| unreachable!(
                "{}",
                constants::tracking_runtime::ALERT_SEVERITY_INFO
            ))
    );
    assert_eq!(
        decision.parent_notification_state,
        TrackingParentNotificationDecisionState::SuppressedMissingEvidence
    );
}

#[test]
fn warning_and_critical_policy_severities_map_through_to_alert_severity() {
    let warning = evaluate_tracking_alert(
        &tracking_policy_violation(
            constants::tracking_runtime::POLICY_SEVERITY_WARNING,
            vec![tracking_evidence_ref()],
        ),
        0,
    );
    let critical = evaluate_tracking_alert(
        &tracking_policy_violation(
            constants::tracking_runtime::POLICY_SEVERITY_CRITICAL,
            vec![tracking_evidence_ref()],
        ),
        0,
    );

    assert_eq!(
        warning.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WARNING)
            .unwrap_or_else(|_| unreachable!(
                "{}",
                constants::tracking_runtime::ALERT_SEVERITY_WARNING
            ))
    );
    assert_eq!(
        critical.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_CRITICAL)
            .unwrap_or_else(|_| unreachable!(
                "{}",
                constants::tracking_runtime::ALERT_SEVERITY_CRITICAL
            ))
    );
}

fn tracking_policy_violation(
    severity: &'static str,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> TrackingPolicyViolationDetectedEvent {
    TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .unwrap_or_else(|_| {
            unreachable!("{}", constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
        }),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .unwrap_or_else(|_| {
            unreachable!("{}", constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID)
        }),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .unwrap_or_else(|_| {
            unreachable!(
                "{}",
                constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID
            )
        }),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .unwrap_or_else(|_| {
            unreachable!(
                "{}",
                constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
            )
        }),
        severity: TrackingPolicySeverity::parse(severity)
            .unwrap_or_else(|_| unreachable!("{}", severity)),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .unwrap_or_else(|_| {
                unreachable!("{}", constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            }),
        evidence_refs,
    }
}

fn tracking_evidence_ref() -> TrackingEvidenceRef {
    TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
        .unwrap_or_else(|_| unreachable!("{}", constants::tracking_runtime::DEFAULT_EVIDENCE_REF))
}
