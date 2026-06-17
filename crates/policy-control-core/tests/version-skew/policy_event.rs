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

fn sample_event(kind: PolicyEventKind) -> PolicyEvent {
    let household_id = PolicyHouseholdId::parse("household-default").expect("household id");
    let source_document_id =
        ParentPolicyDocumentId::parse("policy-source-default").expect("source document id");
    let policy_version = PolicyVersion::new(5).expect("policy version");
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
            request_id: PolicyRequestId::parse("policy-request-default").expect("request id"),
            child_profile_id: PolicyChildProfileId::parse("child-primary")
                .expect("child profile id"),
            source_document_id,
            policy_version,
        },
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => {
            PolicyEventScope::Override {
                household_id,
                override_id: PolicyOverrideId::parse("policy-override-default")
                    .expect("override id"),
                approval_id: PolicyApprovalId::parse("policy-approval-default")
                    .expect("approval id"),
                request_id: PolicyRequestId::parse("policy-request-default").expect("request id"),
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
            delivery_id: PolicyDeliveryId::parse("policy-delivery-default").expect("delivery id"),
            child_profile_id: PolicyChildProfileId::parse("child-primary")
                .expect("child profile id"),
            device_id: PolicyDeviceId::parse("device-laptop").expect("device id"),
            domain: PolicyConsumerDomain::Tracking,
            source_document_id,
            policy_version,
        },
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => {
            PolicyEventScope::Rollback {
                household_id,
                rollback_ref: PolicyRollbackRef {
                    household_id: PolicyHouseholdId::parse("household-default")
                        .expect("household id"),
                    rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-default")
                        .expect("rolled back document id"),
                    rolled_back_policy_version: PolicyVersion::new(5)
                        .expect("rolled back policy version"),
                    restored_document_id: ParentPolicyDocumentId::parse("policy-source-previous")
                        .expect("restored document id"),
                    restored_policy_version: PolicyVersion::new(4)
                        .expect("restored policy version"),
                },
            }
        }
    };

    PolicyEvent {
        schema_version: policy_event_schema_version().expect("policy event schema version"),
        kind,
        sequence: PolicyEventSequence::new(1).expect("policy event sequence"),
        scope,
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-policy-event").expect("audit ref")
        ],
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
            Some(PolicyReasonCode::parse("manual-required").expect("reason code"))
        } else {
            None
        },
        dead_letter_reason: if matches!(kind, PolicyEventKind::DeadLetterRecorded) {
            Some(ocentra_policy_control_core::policy_event::PolicyEventDeadLetterReason::ReplayRejected)
        } else {
            None
        },
    }
}

#[test]
fn policy_event_schema_version_is_locked_to_one() {
    assert_eq!(
        policy_event_schema_version()
            .expect("policy event schema version")
            .value(),
        1
    );

    let registry = policy_event_contract_registry().expect("policy event contract registry");
    assert!(registry
        .descriptors()
        .all(|descriptor| descriptor.schema_version().value() == 1));
}

#[test]
fn policy_event_deserialization_rejects_zero_schema_version() {
    let mut payload = serde_json::to_value(sample_event(PolicyEventKind::DeliveryQueued))
        .expect("policy event payload");
    payload["schema_version"] = serde_json::json!(0);

    let error = serde_json::from_value::<PolicyEvent>(payload)
        .expect_err("zero schema version must be rejected");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}
