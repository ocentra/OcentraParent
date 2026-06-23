use super::TestResult;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryId;
use ocentra_policy_control_core::policy_event::{
    policy_event_contract_registry, policy_event_schema_version, PolicyEvent, PolicyEventKind,
    PolicyEventScope, PolicyEventSequence,
};
use ocentra_policy_control_core::policy_request::{
    PolicyApprovalId, PolicyOverrideId, PolicyRequestId,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

fn sample_event(kind: PolicyEventKind) -> TestResult<PolicyEvent> {
    let household_id = test_ok!(
        PolicyHouseholdId::parse("household-default"),
        "household id"
    );
    let source_document_id = test_ok!(
        ParentPolicyDocumentId::parse("policy-source-default"),
        "source document id"
    );
    let policy_version = test_ok!(PolicyVersion::new(5), "policy version");
    let scope = match kind {
        PolicyEventKind::DraftCreated
        | PolicyEventKind::PreviewRequested
        | PolicyEventKind::PreviewGenerated
        | PolicyEventKind::Confirmed
        | PolicyEventKind::VersionSuperseded
        | PolicyEventKind::CompilerRequested
        | PolicyEventKind::CompilerCompleted
        | PolicyEventKind::AuditRecorded
        | PolicyEventKind::DeadLetterRecorded
        | PolicyEventKind::ManualRequired => PolicyEventScope::SourceDocument {
            household_id,
            source_document_id,
            policy_version,
        },
        PolicyEventKind::AskParentRequested
        | PolicyEventKind::AskParentApproved
        | PolicyEventKind::AskParentDenied => PolicyEventScope::Request {
            household_id,
            request_id: test_ok!(
                PolicyRequestId::parse("policy-request-default"),
                "request id"
            ),
            child_profile_id: test_ok!(
                PolicyChildProfileId::parse("child-primary"),
                "child profile id"
            ),
            source_document_id,
            policy_version,
        },
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => {
            PolicyEventScope::Override {
                household_id,
                override_id: test_ok!(
                    PolicyOverrideId::parse("policy-override-default"),
                    "override id"
                ),
                approval_id: test_ok!(
                    PolicyApprovalId::parse("policy-approval-default"),
                    "approval id"
                ),
                request_id: test_ok!(
                    PolicyRequestId::parse("policy-request-default"),
                    "request id"
                ),
                source_document_id,
                policy_version,
            }
        }
        PolicyEventKind::DeliveryQueued
        | PolicyEventKind::DeliverySent
        | PolicyEventKind::DeliveryAcknowledged
        | PolicyEventKind::DeliveryRejected
        | PolicyEventKind::DeliveryExpired
        | PolicyEventKind::DeliveryRetryScheduled
        | PolicyEventKind::DomainApplied
        | PolicyEventKind::DomainPartial => PolicyEventScope::Delivery {
            household_id,
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
            source_document_id,
            policy_version,
        },
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => {
            PolicyEventScope::Rollback {
                household_id,
                rollback_ref: PolicyRollbackRef {
                    household_id: test_ok!(
                        PolicyHouseholdId::parse("household-default"),
                        "household id"
                    ),
                    rolled_back_document_id: test_ok!(
                        ParentPolicyDocumentId::parse("policy-source-default"),
                        "rolled back document id"
                    ),
                    rolled_back_policy_version: test_ok!(
                        PolicyVersion::new(5),
                        "rolled back policy version"
                    ),
                    restored_document_id: test_ok!(
                        ParentPolicyDocumentId::parse("policy-source-previous"),
                        "restored document id"
                    ),
                    restored_policy_version: test_ok!(
                        PolicyVersion::new(4),
                        "restored policy version"
                    ),
                },
            }
        }
    };

    Ok(PolicyEvent {
        schema_version: test_ok!(policy_event_schema_version(), "policy event schema version"),
        kind,
        sequence: test_ok!(PolicyEventSequence::new(1), "policy event sequence"),
        scope,
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-event"),
            "audit ref"
        )],
        reason_code: if matches!(
            kind,
            PolicyEventKind::DeliveryRejected
                | PolicyEventKind::DeliveryExpired
                | PolicyEventKind::DeliveryRetryScheduled
                | PolicyEventKind::DomainPartial
                | PolicyEventKind::AskParentDenied
                | PolicyEventKind::OverrideExpired
                | PolicyEventKind::ManualRequired
                | PolicyEventKind::RollbackApplied
        ) {
            Some(test_ok!(
                PolicyReasonCode::parse("manual-required"),
                "reason code"
            ))
        } else {
            None
        },
        dead_letter_reason: if matches!(kind, PolicyEventKind::DeadLetterRecorded) {
            Some(ocentra_policy_control_core::policy_event::PolicyEventDeadLetterReason::ReplayRejected)
        } else {
            None
        },
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
        serde_json::to_value(sample_event(PolicyEventKind::DeliveryQueued)?),
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
