use super::TestResult;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryId;
use ocentra_policy_control_core::policy_event::{
    policy_event_contract_registry, policy_event_schema_version, PolicyEvent, PolicyEventKind,
    PolicyEventScope, PolicyEventSequence,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyVersion,
};

fn sample_delivery_scope(version: u64) -> TestResult<PolicyEventScope> {
    Ok(PolicyEventScope::Delivery {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        delivery_id: test_ok!(
            PolicyDeliveryId::parse("policy-delivery-default"),
            "delivery id"
        ),
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: test_ok!(PolicyDeviceId::parse("device-laptop"), "device id"),
        domain: PolicyConsumerDomain::Tracking,
        source_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-default"),
            "source document id"
        ),
        policy_version: test_ok!(PolicyVersion::new(version), "policy version"),
    })
}

fn sample_delivery_queued_event(version: u64) -> TestResult<PolicyEvent> {
    Ok(PolicyEvent {
        schema_version: test_ok!(policy_event_schema_version(), "policy event schema version"),
        kind: PolicyEventKind::DeliveryQueued,
        sequence: test_ok!(PolicyEventSequence::new(1), "policy event sequence"),
        scope: sample_delivery_scope(version)?,
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-event"),
            "audit ref"
        )],
        reason_code: None,
        dead_letter_reason: None,
    })
}

#[test]
fn policy_event_schema_version_is_locked_to_one() -> TestResult {
    assert_eq!(
        test_ok!(policy_event_schema_version(), "policy event schema version").value(),
        1
    );

    let registry = test_ok!(
        policy_event_contract_registry(),
        "policy event contract registry"
    );
    assert!(registry
        .descriptors()
        .all(|descriptor| descriptor.schema_version().value() == 1));
    Ok(())
}

#[test]
fn policy_event_deserialization_rejects_zero_schema_version() -> TestResult {
    let mut payload = test_ok!(
        serde_json::to_value(sample_delivery_queued_event(7)?),
        "policy event payload"
    );
    payload["schema_version"] = serde_json::json!(0);

    let error = test_err!(
        serde_json::from_value::<PolicyEvent>(payload),
        "zero schema version must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}
