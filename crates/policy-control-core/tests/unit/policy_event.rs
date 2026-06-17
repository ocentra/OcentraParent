use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryId;
use ocentra_policy_control_core::policy_event::{
    apply_policy_event_replay, policy_event_contract_registry, policy_event_family_namespace,
    policy_event_family_variants, policy_event_schema_version, PolicyEvent,
    PolicyEventApplyOutcome, PolicyEventDeadLetterReason, PolicyEventKind, PolicyEventScope,
    PolicyEventSequence, POLICY_EVENT_KINDS,
};
use ocentra_policy_control_core::policy_request::{
    PolicyApprovalId, PolicyOverrideId, PolicyRequestId,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(value).expect("policy audit ref")
}

fn sample_event(kind: PolicyEventKind, sequence: u64) -> PolicyEvent {
    sample_event_result(kind, sequence).expect("policy event fixture")
}

fn sample_event_result(kind: PolicyEventKind, sequence: u64) -> Result<PolicyEvent, EventingError> {
    let scope = sample_scope(kind)?;
    let reason_code = if kind_requires_reason(kind) {
        Some(PolicyReasonCode::parse(kind.reason_code_value())?)
    } else {
        None
    };
    let dead_letter_reason = if matches!(kind, PolicyEventKind::DeadLetterRecorded) {
        Some(PolicyEventDeadLetterReason::ReplayRejected)
    } else {
        None
    };

    Ok(PolicyEvent {
        schema_version: policy_event_schema_version()?,
        kind,
        sequence: PolicyEventSequence::new(sequence)?,
        scope,
        audit_reference_ids: vec![audit_ref("audit-policy-event")],
        reason_code,
        dead_letter_reason,
    })
}

fn sample_scope(kind: PolicyEventKind) -> Result<PolicyEventScope, EventingError> {
    let household_id = PolicyHouseholdId::parse("household-default")?;
    let source_document_id = ParentPolicyDocumentId::parse("policy-source-default")?;
    let policy_version = PolicyVersion::new(5)?;

    match kind {
        PolicyEventKind::DraftCreated
        | PolicyEventKind::PreviewRequested
        | PolicyEventKind::PreviewGenerated
        | PolicyEventKind::Confirmed
        | PolicyEventKind::VersionSuperseded
        | PolicyEventKind::CompilerRequested
        | PolicyEventKind::CompilerCompleted
        | PolicyEventKind::AuditRecorded
        | PolicyEventKind::DeadLetterRecorded
        | PolicyEventKind::ManualRequired => Ok(PolicyEventScope::SourceDocument {
            household_id,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::AskParentRequested
        | PolicyEventKind::AskParentApproved
        | PolicyEventKind::AskParentDenied => Ok(PolicyEventScope::Request {
            household_id,
            request_id: PolicyRequestId::parse("policy-request-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => {
            Ok(PolicyEventScope::Override {
                household_id,
                override_id: PolicyOverrideId::parse("policy-override-default")?,
                approval_id: PolicyApprovalId::parse("policy-approval-default")?,
                request_id: PolicyRequestId::parse("policy-request-default")?,
                source_document_id,
                policy_version,
            })
        }
        PolicyEventKind::DeliveryQueued
        | PolicyEventKind::DeliverySent
        | PolicyEventKind::DeliveryAcknowledged
        | PolicyEventKind::DeliveryRejected
        | PolicyEventKind::DeliveryExpired
        | PolicyEventKind::DeliveryRetryScheduled
        | PolicyEventKind::DomainApplied
        | PolicyEventKind::DomainPartial => Ok(PolicyEventScope::Delivery {
            household_id,
            delivery_id: PolicyDeliveryId::parse("policy-delivery-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            device_id: PolicyDeviceId::parse("device-laptop")?,
            domain: PolicyConsumerDomain::Tracking,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => {
            Ok(PolicyEventScope::Rollback {
                household_id,
                rollback_ref: PolicyRollbackRef {
                    household_id: PolicyHouseholdId::parse("household-default")?,
                    rolled_back_document_id: ParentPolicyDocumentId::parse(
                        "policy-source-default",
                    )?,
                    rolled_back_policy_version: PolicyVersion::new(5)?,
                    restored_document_id: ParentPolicyDocumentId::parse("policy-source-previous")?,
                    restored_policy_version: PolicyVersion::new(4)?,
                },
            })
        }
    }
}

fn kind_requires_reason(kind: PolicyEventKind) -> bool {
    matches!(
        kind,
        PolicyEventKind::DeliveryRejected
            | PolicyEventKind::DeliveryExpired
            | PolicyEventKind::DeliveryRetryScheduled
            | PolicyEventKind::DomainPartial
            | PolicyEventKind::AskParentDenied
            | PolicyEventKind::OverrideExpired
            | PolicyEventKind::ManualRequired
            | PolicyEventKind::RollbackApplied
    )
}

#[test]
fn policy_event_family_registry_lists_all_event_types() {
    let registry = policy_event_contract_registry().expect("policy event contract registry");
    let actual = registry
        .descriptors()
        .map(|descriptor| descriptor.event_type().as_str().to_string())
        .collect::<BTreeSet<_>>();
    let expected = POLICY_EVENT_KINDS
        .iter()
        .map(|kind| kind.event_type_name().to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
    assert_eq!(
        policy_event_family_namespace()
            .expect("policy event family namespace")
            .as_str(),
        "policy"
    );

    let variants = policy_event_family_variants().expect("policy event family variants");
    assert_eq!(variants.len(), POLICY_EVENT_KINDS.len());
    assert!(variants
        .iter()
        .all(|variant| variant.family.as_str() == "policy"));
    let variant_types = variants
        .iter()
        .map(|variant| variant.event_type.as_str())
        .collect::<Vec<_>>();
    let expected_types = POLICY_EVENT_KINDS
        .iter()
        .map(|kind| kind.event_type_name())
        .collect::<Vec<_>>();
    assert_eq!(variant_types, expected_types);
}

#[test]
fn policy_event_keys_and_contract_are_stable_for_delivery_events() {
    let event = sample_event(PolicyEventKind::DeliveryQueued, 3);

    let contract = event.contract().expect("policy event contract");
    assert_eq!(contract.event_type.as_str(), "policy.delivery.queued");
    assert_eq!(contract.schema_version.value(), 1);
    assert_eq!(
        event.aggregate_key().expect("policy aggregate key").as_str(),
        "policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5"
    );
    assert_eq!(
        event.idempotency_key().expect("policy idempotency key").as_str(),
        "policy-event:policy.delivery.queued|policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5|3|delivery|audit-policy-event|none|none"
    );
}

#[test]
fn policy_event_replay_tracks_duplicate_stale_and_conflicting_sequences() {
    let current = sample_event(PolicyEventKind::DeliveryQueued, 3);
    let current_record = current.replay_record().expect("policy event replay record");

    match apply_policy_event_replay(&current_record, &current).expect("duplicate replay") {
        PolicyEventApplyOutcome::Duplicate(record) => assert_eq!(record, current_record),
        other => panic!("expected duplicate replay outcome, got {other:?}"),
    }

    match apply_policy_event_replay(
        &current_record,
        &sample_event(PolicyEventKind::DeliveryQueued, 2),
    )
    .expect("stale replay")
    {
        PolicyEventApplyOutcome::Stale(record) => assert_eq!(record, current_record),
        other => panic!("expected stale replay outcome, got {other:?}"),
    }

    let error = apply_policy_event_replay(
        &current_record,
        &sample_event(PolicyEventKind::DeliverySent, 3),
    )
    .expect_err("conflicting same-sequence replay must fail");
    assert!(error
        .to_string()
        .contains("conflicting replay for sequence 3"));
}

#[test]
fn policy_event_redacted_summary_omits_private_identifiers() {
    let event = sample_event(PolicyEventKind::RollbackApplied, 5);
    let summary = event.redacted_summary();

    assert!(summary.contains("policy-event kind=policy.rollback.applied"));
    assert!(summary.contains("scope=rollback"));
    assert!(summary.contains("sequence=5"));
    assert!(!summary.contains("policy-source-default"));
    assert!(!summary.contains("policy-approval-default"));
    assert!(!summary.contains("policy-delivery-default"));

    let dead_letter = sample_event(PolicyEventKind::DeadLetterRecorded, 4);
    let dead_letter_summary = dead_letter.redacted_summary();
    assert!(dead_letter_summary.contains("dead-lettered"));
}

#[test]
fn policy_event_manual_required_and_dead_letter_payloads_remain_explicit() {
    let manual_required = sample_event(PolicyEventKind::ManualRequired, 1);
    assert_eq!(
        manual_required
            .reason_code
            .as_ref()
            .expect("manual-required reason code")
            .as_str(),
        "manual-required"
    );

    let dead_letter = sample_event(PolicyEventKind::DeadLetterRecorded, 1);
    assert_eq!(
        dead_letter.dead_letter_reason.expect("dead letter reason"),
        PolicyEventDeadLetterReason::ReplayRejected
    );
}
