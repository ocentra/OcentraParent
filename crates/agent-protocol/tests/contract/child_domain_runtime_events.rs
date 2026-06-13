use ocentra_eventing::{DomainEvent, EventingError};
use ocentra_parent_agent_protocol::{
    child_domain_ai_analysis_completed_event, child_domain_ai_analysis_requested_event,
    child_domain_evidence_recorded_event, child_domain_notification_requested_event,
    child_domain_notification_id_from_policy_violation_id,
    child_domain_observed_event, child_domain_policy_evaluation_requested_from_ai_result_event,
    child_domain_policy_violation_id_from_policy_request_id,
    child_domain_policy_violation_detected_event, constants, ChildDomainEventType,
    ChildDomainObservedEvent, ChildRuntimeDomain, AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn child_domain_events_expose_eventing_contract_keys_without_local_shape_duplication() {
    let observed =
        child_domain_observed_event(ChildRuntimeDomain::Browser.default_observed_profile());
    let evidence = child_domain_evidence_recorded_event(&observed);
    let ai_requested = child_domain_ai_analysis_requested_event(&evidence);
    let ai_completed = child_domain_ai_analysis_completed_event(&ai_requested);
    let policy_requested =
        child_domain_policy_evaluation_requested_from_ai_result_event(&ai_completed);
    let violation = child_domain_policy_violation_detected_event(&policy_requested);
    let notification = child_domain_notification_requested_event(&violation);

    let observed_contract = observed.contract().expect("observed contract");
    let notification_contract = notification.contract().expect("notification contract");

    assert_eq!(
        observed_contract.event_type.as_str(),
        constants::child_domain_runtime::BROWSER_OBSERVED_EVENT_TYPE
    );
    assert_eq!(
        observed_contract.schema_version.value(),
        AGENT_PROTOCOL_SCHEMA_VERSION
    );
    assert_eq!(
        notification_contract.event_type.as_str(),
        constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE
    );
    assert!(observed
        .aggregate_key()
        .expect("observed aggregate")
        .as_str()
        .contains(ChildRuntimeDomain::Browser.as_contract_text()));
    assert!(notification
        .idempotency_key()
        .expect("notification idempotency")
        .as_str()
        .contains(notification.notification_id.as_str()));
    assert_eq!(
        violation.violation_id,
        child_domain_policy_violation_id_from_policy_request_id(
            &policy_requested.policy_request_id
        )
    );
    assert_eq!(
        notification.notification_id,
        child_domain_notification_id_from_policy_violation_id(&violation.violation_id)
    );
}

#[test]
fn child_domain_policy_and_notification_helpers_canonicalize_evidence_refs() {
    let observed =
        child_domain_observed_event(ChildRuntimeDomain::Browser.default_observed_profile());
    let evidence = child_domain_evidence_recorded_event(&observed);
    let ai_requested = child_domain_ai_analysis_requested_event(&evidence);
    let ai_completed = child_domain_ai_analysis_completed_event(&ai_requested);
    let mut policy_requested =
        child_domain_policy_evaluation_requested_from_ai_result_event(&ai_completed);
    let duplicate = policy_requested
        .evidence_refs
        .first()
        .expect("policy evidence ref")
        .clone();
    policy_requested.evidence_refs.push(duplicate);

    let violation = child_domain_policy_violation_detected_event(&policy_requested);
    let notification = child_domain_notification_requested_event(&violation);

    assert_eq!(violation.evidence_refs.len(), 1);
    assert_eq!(notification.evidence_refs.len(), 1);
    assert_eq!(notification.evidence_refs, violation.evidence_refs);
}

#[test]
fn child_domain_event_type_deserialization_rejects_unknown_protocol_text() {
    let payload = serde_json::json!({
        "eventType": "browser.local-lookalike.event",
        "domain": constants::child_domain_runtime::DOMAIN_BROWSER,
        "childDeviceId": constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID,
        "childProfileId": constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID,
        "observationId": "browser:observation-default",
        "subjectRef": "browser:active-url",
        "observedState": constants::child_domain_runtime::SIGNAL_REQUIRES_AI,
        "observedAt": constants::child_domain_runtime::DEFAULT_OBSERVED_AT,
        "aiAnalysisRequirement": "required",
        "policyEvaluationRequirement": "required"
    });

    let result = serde_json::from_value::<ChildDomainObservedEvent>(payload);

    assert!(result.is_err());
}

#[test]
fn child_domain_event_type_parse_rejects_empty_and_unknown_values() {
    assert!(matches!(
        ChildDomainEventType::parse(" "),
        Err(EventingError::EmptyValue { .. })
    ));
    assert!(matches!(
        ChildDomainEventType::parse("screen.local-unowned.event"),
        Err(EventingError::InvalidValue { .. })
    ));
}
