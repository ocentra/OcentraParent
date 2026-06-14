use ocentra_parent_agent_protocol::{
    child_domain_child_device_id, child_domain_child_profile_id, child_domain_evidence_ref,
    child_domain_notification_id_from_policy_violation_id, child_domain_observed_at,
    child_domain_policy_rule_ref, child_domain_policy_severity, child_domain_policy_violation_id,
    ChildDomainEventType, ChildDomainPolicyRuleKind, ChildDomainPolicySeverityKind,
    ChildDomainPolicyViolationDetectedEvent, ChildDomainRefSuffix, ChildRuntimeDomain,
};

#[test]
fn child_domain_notification_preserves_policy_violation_source() {
    let violation = ChildDomainPolicyViolationDetectedEvent {
        event_type: ChildDomainEventType::policy_violation_detected(),
        domain: ChildRuntimeDomain::AppGame,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        violation_id: child_domain_policy_violation_id(
            ChildRuntimeDomain::AppGame,
            ChildDomainRefSuffix::DefaultPolicyViolation,
        ),
        policy_rule_ref: child_domain_policy_rule_ref(ChildDomainPolicyRuleKind::Default),
        severity: child_domain_policy_severity(ChildDomainPolicySeverityKind::Review),
        detected_at: child_domain_observed_at(),
        evidence_refs: vec![child_domain_evidence_ref(
            ChildRuntimeDomain::AppGame,
            ChildDomainRefSuffix::DefaultEvidence,
        )],
    };

    let notification =
        ocentra_child_notification_core::request_child_domain_parent_notification(&violation);

    assert_eq!(
        notification.event_type,
        ChildDomainEventType::notification_requested()
    );
    assert_eq!(
        notification.source_policy_violation_id,
        violation.violation_id
    );
    assert_eq!(
        notification.notification_id,
        child_domain_notification_id_from_policy_violation_id(&violation.violation_id)
    );
    assert_eq!(notification.requested_at, violation.detected_at);
    assert_eq!(notification.evidence_refs, violation.evidence_refs);
}

#[test]
fn child_domain_notification_canonicalizes_duplicate_evidence_refs() {
    let evidence_ref = child_domain_evidence_ref(
        ChildRuntimeDomain::AppGame,
        ChildDomainRefSuffix::DefaultEvidence,
    );
    let violation = ChildDomainPolicyViolationDetectedEvent {
        event_type: ChildDomainEventType::policy_violation_detected(),
        domain: ChildRuntimeDomain::AppGame,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        violation_id: child_domain_policy_violation_id(
            ChildRuntimeDomain::AppGame,
            ChildDomainRefSuffix::DefaultPolicyViolation,
        ),
        policy_rule_ref: child_domain_policy_rule_ref(ChildDomainPolicyRuleKind::Default),
        severity: child_domain_policy_severity(ChildDomainPolicySeverityKind::Review),
        detected_at: child_domain_observed_at(),
        evidence_refs: vec![evidence_ref.clone(), evidence_ref],
    };

    let notification =
        ocentra_child_notification_core::request_child_domain_parent_notification(&violation);

    assert_eq!(notification.requested_at, violation.detected_at);
    assert_eq!(notification.evidence_refs.len(), 1);
}
