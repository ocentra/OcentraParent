use super::TestResult;
use std::collections::BTreeSet;

use ocentra_eventing::envelope::{EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    CausationId, CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService,
};
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryId;
use ocentra_policy_control_core::policy_event::{
    apply_policy_event_replay, policy_event_contract_registry, policy_event_family_namespace,
    policy_event_family_variants, policy_event_schema_version, PolicyEvent,
    PolicyEventApplyOutcome, PolicyEventDeadLetterReason, PolicyEventKind, PolicyEventScope,
    PolicyEventSequence, POLICY_EVENT_KINDS,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

fn audit_ref(value: impl std::fmt::Display) -> TestResult<PolicyAuditReferenceId> {
    Ok(test_ok!(
        PolicyAuditReferenceId::parse(value.to_string()),
        "policy audit ref"
    ))
}

fn sample_policy_event(
    kind: PolicyEventKind,
    sequence: u64,
    scope: PolicyEventScope,
    reason_code: Option<PolicyReasonCode>,
    dead_letter_reason: Option<PolicyEventDeadLetterReason>,
) -> TestResult<PolicyEvent> {
    Ok(PolicyEvent {
        schema_version: test_ok!(policy_event_schema_version(), "policy event schema version"),
        kind,
        sequence: test_ok!(PolicyEventSequence::new(sequence), "policy event sequence"),
        scope,
        audit_reference_ids: vec![audit_ref("audit-policy-event")?],
        reason_code,
        dead_letter_reason,
    })
}

fn source_document_scope() -> TestResult<PolicyEventScope> {
    Ok(PolicyEventScope::SourceDocument {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "policy household id"
        ),
        source_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-default"),
            "policy source document id"
        ),
        policy_version: test_ok!(PolicyVersion::new(5), "policy version"),
    })
}

fn delivery_scope() -> TestResult<PolicyEventScope> {
    Ok(PolicyEventScope::Delivery {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "policy household id"
        ),
        delivery_id: test_ok!(
            PolicyDeliveryId::parse("policy-delivery-default"),
            "policy delivery id"
        ),
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: test_ok!(PolicyDeviceId::parse("device-laptop"), "device id"),
        domain: PolicyConsumerDomain::Tracking,
        source_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-default"),
            "policy source document id"
        ),
        policy_version: test_ok!(PolicyVersion::new(5), "policy version"),
    })
}

fn rollback_scope() -> TestResult<PolicyEventScope> {
    Ok(PolicyEventScope::Rollback {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "policy household id"
        ),
        rollback_ref: PolicyRollbackRef {
            household_id: test_ok!(
                PolicyHouseholdId::parse("household-default"),
                "policy household id"
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
            restored_policy_version: test_ok!(PolicyVersion::new(4), "restored policy version"),
        },
    })
}

fn sample_delivery_queued_event(sequence: u64) -> TestResult<PolicyEvent> {
    sample_policy_event(
        PolicyEventKind::DeliveryQueued,
        sequence,
        delivery_scope()?,
        None,
        None,
    )
}

fn sample_delivery_sent_event(sequence: u64) -> TestResult<PolicyEvent> {
    sample_policy_event(
        PolicyEventKind::DeliverySent,
        sequence,
        delivery_scope()?,
        None,
        None,
    )
}

fn sample_rollback_applied_event(sequence: u64) -> TestResult<PolicyEvent> {
    sample_policy_event(
        PolicyEventKind::RollbackApplied,
        sequence,
        rollback_scope()?,
        Some(test_ok!(
            PolicyReasonCode::parse("manual-required"),
            "reason code"
        )),
        None,
    )
}

fn sample_dead_letter_recorded_event(sequence: u64) -> TestResult<PolicyEvent> {
    sample_policy_event(
        PolicyEventKind::DeadLetterRecorded,
        sequence,
        source_document_scope()?,
        None,
        Some(PolicyEventDeadLetterReason::ReplayRejected),
    )
}

fn sample_manual_required_event(sequence: u64) -> TestResult<PolicyEvent> {
    sample_policy_event(
        PolicyEventKind::ManualRequired,
        sequence,
        source_document_scope()?,
        Some(test_ok!(
            PolicyReasonCode::parse("manual-required"),
            "reason code"
        )),
        None,
    )
}

#[test]
fn policy_event_family_registry_lists_all_event_types() -> TestResult {
    let registry = test_ok!(
        policy_event_contract_registry(),
        "policy event contract registry"
    );
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
            .map_err(|error| {
                std::io::Error::other(format!("policy event family namespace: {error}"))
            })?
            .as_str(),
        "policy"
    );

    let variants = test_ok!(
        policy_event_family_variants(),
        "policy event family variants"
    );
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
    Ok(())
}

#[test]
fn policy_event_envelope_preserves_causation_correlation_and_deterministic_keys() -> TestResult {
    let event = sample_delivery_queued_event(3)?;
    let aggregate_key = test_ok!(event.aggregate_key(), "policy aggregate key");
    let idempotency_key = test_ok!(event.idempotency_key(), "policy idempotency key");
    let correlation_id = test_ok!(
        CorrelationId::parse("correlation-policy-event-1"),
        "policy event correlation id"
    );
    let causation_id = test_ok!(
        CausationId::parse("causation-policy-event-1"),
        "policy event causation id"
    );
    let metadata = EventMetadata::new(
        correlation_id.clone(),
        EventSource::new(
            test_ok!(EventCustody::parse("local"), "event custody"),
            test_ok!(RuntimeRole::parse("policy-control-plane"), "runtime role"),
            test_ok!(
                SourceService::parse("policy-control-plane"),
                "source service"
            ),
            test_ok!(
                SourceComponent::parse("policy-event-test"),
                "source component"
            ),
            test_ok!(
                RuntimeInstanceId::parse("instance-1"),
                "runtime instance id"
            ),
        ),
    )
    .with_causation_id(causation_id.clone());

    let envelope = test_ok!(
        EventEnvelope::from_event(event.clone(), metadata),
        "policy event envelope"
    );

    assert_eq!(
        envelope.contract().event_type.as_str(),
        event.kind.event_type_name()
    );
    assert_eq!(envelope.contract().schema_version, event.schema_version);
    assert_eq!(envelope.aggregate_key(), &aggregate_key);
    assert_eq!(envelope.idempotency_key(), &idempotency_key);
    assert_eq!(envelope.correlation_id(), &correlation_id);
    assert_eq!(
        test_some!(envelope.causation_id(), "policy event causation"),
        &causation_id
    );
    Ok(())
}

#[test]
fn policy_event_consistency_rejects_invalid_scope_audit_reason_and_dead_letter_state() -> TestResult
{
    let mut wrong_scope = sample_delivery_queued_event(1)?;
    wrong_scope.scope = source_document_scope()?;
    assert_eq!(
        test_err!(wrong_scope.contract(), "mismatched policy event scope"),
        EventingError::InvalidValue {
            field: "policy_event.scope",
            value: "scope does not match event kind: expected delivery, received source-document"
                .to_string(),
        }
    );

    let mut missing_audit = sample_delivery_queued_event(1)?;
    missing_audit.audit_reference_ids.clear();
    assert_eq!(
        test_err!(
            missing_audit.idempotency_key(),
            "missing policy event audit refs"
        ),
        EventingError::InvalidValue {
            field: "policy_event.audit_reference_ids",
            value: "missing audit references".to_string(),
        }
    );

    let mut duplicate_audit = sample_delivery_queued_event(1)?;
    duplicate_audit
        .audit_reference_ids
        .push(duplicate_audit.audit_reference_ids[0].clone());
    assert_eq!(
        test_err!(
            duplicate_audit.contract(),
            "duplicate policy event audit refs"
        ),
        EventingError::InvalidValue {
            field: "policy_event.audit_reference_ids",
            value: "duplicate audit reference".to_string(),
        }
    );

    let missing_reason = sample_policy_event(
        PolicyEventKind::DeliveryRejected,
        1,
        delivery_scope()?,
        None,
        None,
    )?;
    assert_eq!(
        test_err!(missing_reason.contract(), "missing policy event reason"),
        EventingError::InvalidValue {
            field: "policy_event.reason_code",
            value: "missing reason code for delivery-rejected".to_string(),
        }
    );

    let mut unexpected_reason = sample_delivery_queued_event(1)?;
    unexpected_reason.reason_code = Some(test_ok!(
        PolicyReasonCode::parse("caller-supplied-reason"),
        "unexpected policy event reason"
    ));
    assert_eq!(
        test_err!(
            unexpected_reason.contract(),
            "unexpected policy event reason"
        ),
        EventingError::InvalidValue {
            field: "policy_event.reason_code",
            value: "unexpected reason code for policy.delivery.queued".to_string(),
        }
    );

    let mut missing_dead_letter_reason = sample_dead_letter_recorded_event(1)?;
    missing_dead_letter_reason.dead_letter_reason = None;
    assert_eq!(
        test_err!(
            missing_dead_letter_reason.contract(),
            "missing policy event dead letter reason"
        ),
        EventingError::InvalidValue {
            field: "policy_event.dead_letter_reason",
            value: "dead-letter reason required".to_string(),
        }
    );

    let mut hidden_dead_letter = sample_delivery_queued_event(1)?;
    hidden_dead_letter.dead_letter_reason = Some(PolicyEventDeadLetterReason::ManualRequired);
    assert_eq!(
        test_err!(
            hidden_dead_letter.contract(),
            "unexpected policy event dead letter reason"
        ),
        EventingError::InvalidValue {
            field: "policy_event.dead_letter_reason",
            value: "dead-letter reason only valid for policy.dead-letter.recorded".to_string(),
        }
    );
    Ok(())
}

#[test]
fn policy_event_keys_and_contract_are_stable_for_delivery_events() -> TestResult {
    let event = sample_delivery_queued_event(3)?;

    let contract = test_ok!(event.contract(), "policy event contract");
    assert_eq!(contract.event_type.as_str(), "policy.delivery.queued");
    assert_eq!(contract.schema_version.value(), 1);
    assert_eq!(
        test_ok!(event.aggregate_key(), "policy aggregate key").as_str(),
        "policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5"
    );
    assert_eq!(
        test_ok!(event.idempotency_key(), "policy idempotency key").as_str(),
        "policy-event:policy.delivery.queued|policy-delivery:household-default:policy-delivery-default:child-primary:device-laptop:tracking:5|3|delivery|audit-policy-event|none|none"
    );
    Ok(())
}

#[test]
fn policy_event_replay_tracks_duplicate_stale_and_conflicting_sequences() -> TestResult {
    let current = sample_delivery_queued_event(3)?;
    let current_record = test_ok!(current.replay_record(), "policy event replay record");

    match test_ok!(
        apply_policy_event_replay(&current_record, &current),
        "duplicate replay"
    ) {
        PolicyEventApplyOutcome::Duplicate(record) => assert_eq!(record, current_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected duplicate replay outcome, got {other:?}"
            ))
            .into());
        }
    }

    match apply_policy_event_replay(&current_record, &sample_delivery_queued_event(2)?)
        .map_err(|error| std::io::Error::other(format!("stale replay: {error}")))?
    {
        PolicyEventApplyOutcome::Stale(record) => assert_eq!(record, current_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected stale replay outcome, got {other:?}"
            ))
            .into());
        }
    }

    let error = test_err!(
        apply_policy_event_replay(&current_record, &sample_delivery_sent_event(3)?),
        "conflicting same-sequence replay must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "conflicting replay for sequence 3 on policy.delivery.queued".to_string(),
        }
    );
    Ok(())
}

#[test]
fn policy_event_replay_aggregate_mismatch_redacts_private_identity() -> TestResult {
    let current = sample_delivery_queued_event(1)?;
    let current_record = test_ok!(current.replay_record(), "policy event replay record");
    let mut next = sample_delivery_queued_event(2)?;
    match &mut next.scope {
        PolicyEventScope::Delivery { household_id, .. } => {
            *household_id = test_ok!(
                PolicyHouseholdId::parse("household-private-child"),
                "private household id"
            );
        }
        scope => {
            return Err(
                std::io::Error::other(format!("expected delivery scope, got {scope:?}")).into(),
            );
        }
    }

    assert_eq!(
        test_err!(
            apply_policy_event_replay(&current_record, &next),
            "mismatched policy event aggregate"
        ),
        EventingError::InvalidValue {
            field: "policy_event.aggregate_key",
            value: "[redacted mismatch]".to_string(),
        }
    );
    Ok(())
}

#[test]
fn policy_event_redacted_summary_omits_private_identifiers() -> TestResult {
    let event = sample_rollback_applied_event(5)?;
    let summary = event.redacted_summary();

    assert_eq!(
        summary,
        "policy-event kind=policy.rollback.applied scope=rollback sequence=5"
    );
    assert_eq!(summary.find("policy-source-default"), None);
    assert_eq!(summary.find("policy-approval-default"), None);
    assert_eq!(summary.find("policy-delivery-default"), None);

    let dead_letter = sample_dead_letter_recorded_event(4)?;
    let dead_letter_summary = dead_letter.redacted_summary();
    assert_eq!(
        dead_letter_summary,
        "policy-event kind=policy.dead-letter.recorded scope=source-document sequence=4 dead-lettered"
    );
    Ok(())
}

#[test]
fn policy_event_manual_required_and_dead_letter_payloads_remain_explicit() -> TestResult {
    let manual_required = sample_manual_required_event(1)?;
    assert_eq!(
        test_some!(
            manual_required.reason_code.as_ref(),
            "manual-required reason code"
        )
        .as_str(),
        "manual-required"
    );

    let dead_letter = sample_dead_letter_recorded_event(1)?;
    assert_eq!(
        test_some!(dead_letter.dead_letter_reason, "dead letter reason"),
        PolicyEventDeadLetterReason::ReplayRejected
    );
    Ok(())
}
