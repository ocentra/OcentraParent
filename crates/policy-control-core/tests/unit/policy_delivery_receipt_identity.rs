use std::any::TypeId;

use super::policy_delivery_helpers as helpers;
use super::policy_delivery_receipt_helpers::{
    assert_unexpected_adapter_execution_receipt, execution_receipt_with_sequence,
};
use super::TestResult;
use helpers::{
    mutate_provenance_child_profile, mutate_provenance_device, mutate_provenance_domain,
    mutate_provenance_household, mutate_provenance_policy_version, mutate_provenance_reason_code,
    mutate_provenance_source_document, reason, sample_delivery_id, sample_delivery_target,
    sample_policy_source_document, sample_queued_delivery, transition,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, apply_policy_delivery_transition_without_execution_receipt,
    derive_policy_delivery_id, queue_policy_delivery, validate_policy_delivery_execution_receipt,
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliverySequence,
    PolicyDeliveryState, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, ParentPolicyDocumentId, PolicyConsumerDomain, PolicyScheduleId,
};

#[test]
fn policy_delivery_ids_are_distinct_opaque_types_with_delivery_specific_validation() -> TestResult {
    assert_ne!(
        TypeId::of::<PolicyDeliveryId>(),
        TypeId::of::<ParentPolicyDocumentId>()
    );
    assert_ne!(
        TypeId::of::<PolicyDeliveryAttemptId>(),
        TypeId::of::<PolicyScheduleId>()
    );

    let delivery_error = test_err!(
        PolicyDeliveryId::parse(""),
        "empty delivery id must be rejected"
    );
    assert_eq!(
        delivery_error,
        EventingError::EmptyValue {
            field: "policy_delivery.delivery_id",
        }
    );

    let attempt_error = test_err!(
        PolicyDeliveryAttemptId::parse(""),
        "empty attempt id must be rejected"
    );
    assert_eq!(
        attempt_error,
        EventingError::EmptyValue {
            field: "policy_delivery.attempt_id",
        }
    );
    Ok(())
}

#[test]
fn policy_delivery_id_is_derived_from_full_scope_and_is_stable() -> TestResult {
    let source = sample_policy_source_document()?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );
    let target = sample_delivery_target()?;
    let attempt_id = helpers::attempt("attempt-queued")?;
    let sequence = test_ok!(
        PolicyDeliverySequence::new(1),
        "policy delivery initial sequence"
    );
    let expected = test_ok!(
        derive_policy_delivery_id(&compiled, &target, &attempt_id, sequence,),
        "derived policy delivery id"
    );
    assert_eq!(expected, sample_delivery_id()?);
    assert_eq!(
        test_ok!(
            derive_policy_delivery_id(&compiled, &target, &attempt_id, sequence,),
            "rederived policy delivery id"
        ),
        expected
    );

    let mut source_document = source;
    source_document.document_id = ParentPolicyDocumentId::parse("policy-source-other")?;
    let alternate_compiled = test_ok!(
        compile_domain_policy_artifact(&source_document, PolicyConsumerDomain::Tracking),
        "compiled alternate domain policy artifact"
    );
    let alternate_source_id = test_ok!(
        derive_policy_delivery_id(&alternate_compiled, &target, &attempt_id, sequence,),
        "derived alternate source policy delivery id"
    );

    let mut alternate_target = sample_delivery_target()?;
    alternate_target.domain = PolicyConsumerDomain::Browser;
    let alternate_target_id = test_ok!(
        derive_policy_delivery_id(&compiled, &alternate_target, &attempt_id, sequence,),
        "derived alternate target policy delivery id"
    );
    let alternate_attempt_id = helpers::attempt("attempt-other")?;
    let alternate_attempt_delivery_id = test_ok!(
        derive_policy_delivery_id(&compiled, &target, &alternate_attempt_id, sequence,),
        "derived alternate attempt policy delivery id"
    );
    let alternate_sequence_delivery_id = test_ok!(
        derive_policy_delivery_id(
            &compiled,
            &target,
            &attempt_id,
            PolicyDeliverySequence::new(2)?,
        ),
        "derived alternate sequence policy delivery id"
    );

    let derived_ids = [
        expected.as_str(),
        alternate_source_id.as_str(),
        alternate_target_id.as_str(),
        alternate_attempt_delivery_id.as_str(),
        alternate_sequence_delivery_id.as_str(),
    ];
    let unique_count = derived_ids
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    assert_eq!(unique_count, derived_ids.len());

    Ok(())
}

#[test]
fn queue_preserves_caller_delivery_id_while_derivation_remains_opt_in() -> TestResult {
    let source = sample_policy_source_document()?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );
    let target = sample_delivery_target()?;
    let attempt_id = helpers::attempt("attempt-caller-selected")?;
    let sequence = test_ok!(
        PolicyDeliverySequence::new(1),
        "policy delivery initial sequence"
    );
    let derived_id = test_ok!(
        derive_policy_delivery_id(&compiled, &target, &attempt_id, sequence),
        "derived policy delivery id"
    );
    let caller_id = PolicyDeliveryId::parse("delivery-caller-selected")?;

    assert_ne!(caller_id, derived_id);

    let queued = test_ok!(
        queue_policy_delivery(
            &compiled,
            target,
            caller_id.clone(),
            attempt_id,
            vec![helpers::audit_ref("audit-caller-selected")?],
        ),
        "queue policy delivery with caller-selected id"
    );

    assert_eq!(queued.delivery_id, caller_id);
    Ok(())
}

#[test]
fn acknowledged_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let acknowledged_transition = transition(
        2,
        "attempt-acknowledged-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let acknowledged_receipt = helpers::execution_receipt(&queued, &acknowledged_transition);

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &acknowledged_transition, None,),
        "acknowledged delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for acknowledged".to_string(),
        }
    );

    test_ok!(
        validate_policy_delivery_execution_receipt(
            &queued,
            &acknowledged_transition,
            Some(&acknowledged_receipt),
        ),
        "acknowledged receipt evidence validates structurally"
    );
    Ok(())
}

#[test]
fn execution_receipt_matrix_rejects_unexpected_receipts_for_non_adapter_states() -> TestResult {
    let queued = sample_queued_delivery()?;
    let states = [
        PolicyDeliveryState::Queued,
        PolicyDeliveryState::Delivering,
        PolicyDeliveryState::Delivered,
        PolicyDeliveryState::Rejected,
        PolicyDeliveryState::Superseded,
        PolicyDeliveryState::Degraded,
        PolicyDeliveryState::Offline,
        PolicyDeliveryState::ExpiredBeforeDelivery,
        PolicyDeliveryState::RetryScheduled,
        PolicyDeliveryState::PartialDomainApply,
        PolicyDeliveryState::BlockedByPermission,
        PolicyDeliveryState::BlockedByCapability,
        PolicyDeliveryState::ManualRequired,
    ];

    for state in states {
        assert_unexpected_adapter_execution_receipt(&queued, state)?;
    }

    Ok(())
}

#[test]
fn ack_applied_and_rolled_back_receipts_require_explicit_adapter_provenance() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-receipt",
                PolicyDeliveryState::Delivered
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let cases = [
        PolicyDeliveryState::Acknowledged,
        PolicyDeliveryState::Applied,
        PolicyDeliveryState::RolledBack,
    ];

    for state in cases {
        let current = if state == PolicyDeliveryState::RolledBack {
            &delivered_record
        } else {
            &queued
        };
        let sequence = if state == PolicyDeliveryState::RolledBack {
            3
        } else {
            2
        };
        let mut transition = transition(sequence, format!("attempt-{state:?}-receipt"), state)?;
        if state == PolicyDeliveryState::RolledBack {
            transition.reason_code = Some(reason("adapter-failed")?);
            transition.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
        }
        let receipt = execution_receipt_with_sequence(current, &transition, sequence);

        test_ok!(
            validate_policy_delivery_execution_receipt(current, &transition, Some(&receipt)),
            "required adapter receipt should validate"
        );
    }

    Ok(())
}

#[test]
fn applied_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let applied_transition =
        transition(3, "attempt-applied-receipt", PolicyDeliveryState::Applied)?;

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &applied_transition, None),
        "applied delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for applied".to_string(),
        }
    );

    Ok(())
}

#[test]
fn rolled_back_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut rollback_transition = transition(
        4,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &rollback_transition, None),
        "rolled-back delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for rolled-back".to_string(),
        }
    );

    Ok(())
}

#[test]
fn bare_transition_apis_reject_every_receipt_required_state() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered-setup", PolicyDeliveryState::Delivered)?,
        ),
        "deliver setup transition"
    )
    .into_record();

    let acknowledged = transition(
        2,
        "attempt-acknowledged-without-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let applied_without_receipt = transition(
        2,
        "attempt-applied-without-receipt",
        PolicyDeliveryState::Applied,
    )?;
    let mut rolled_back = transition(
        3,
        "attempt-rolled-back-without-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rolled_back.reason_code = Some(reason("adapter-failed")?);
    rolled_back.rollback_reference_state = Some(PolicyDeliveryState::Delivered);

    assert_bare_transition_rejected(&queued, acknowledged, "acknowledged")?;
    assert_bare_transition_rejected(&queued, applied_without_receipt, "applied")?;
    assert_bare_transition_rejected(&delivered, rolled_back, "rolled-back")?;

    assert!(!queued.is_active());
    assert!(!delivered.is_active());
    assert_eq!(delivered.state, PolicyDeliveryState::Delivered);
    Ok(())
}

fn assert_bare_transition_rejected(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
    state_name: &str,
) -> TestResult {
    let compatibility_error = test_err!(
        apply_policy_delivery_transition(current, transition.clone()),
        "compatibility transition API must reject receipt-required state"
    );
    let explicit_error = test_err!(
        apply_policy_delivery_transition_without_execution_receipt(current, transition),
        "explicit non-receipt transition API must reject receipt-required state"
    );
    let expected = EventingError::InvalidValue {
        field: "policy_delivery.state",
        value: format!("missing adapter execution receipt for {state_name}"),
    };

    assert_eq!(compatibility_error, expected);
    assert_eq!(explicit_error, expected);
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_source_document_identity_mismatch() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-source-document",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-source-document",
        PolicyDeliveryState::Acknowledged,
    )?;
    let mut receipt = helpers::execution_receipt(&delivered_record, &acknowledged_transition);
    mutate_provenance_source_document(&mut receipt);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "source document provenance mismatch must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_source.document_id",
            value: "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_reason_code_identity_mismatch() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-reason-code",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let mut acknowledged_transition = transition(
        3,
        "attempt-acknowledged-reason-code",
        PolicyDeliveryState::Acknowledged,
    )?;
    acknowledged_transition.reason_code = Some(reason("acknowledged-reason")?);
    let mut receipt = helpers::execution_receipt(&delivered_record, &acknowledged_transition);
    mutate_provenance_reason_code(&mut receipt);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "reason code provenance mismatch must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "execution receipt identity mismatch: expected=transition, reported=execution-receipt"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_provenance_mismatches() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-provenance",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-provenance",
        PolicyDeliveryState::Acknowledged,
    )?;

    let provenance_cases: [helpers::ProvenanceCase; 6] = [
        (
            "policy_source.document_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_source_document,
        ),
        (
            "policy_source.household_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_household,
        ),
        (
            "policy_source.policy_version",
            format!(
                "expected policy version {} but receipt reported 8",
                delivered_record.policy_version.value()
            ),
            mutate_provenance_policy_version,
        ),
        (
            "policy_source.child_profile_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_child_profile,
        ),
        (
            "policy_source.device_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_device,
        ),
        (
            "policy_delivery.target.domain",
            "expected delivery domain tracking but receipt reported browser".to_string(),
            mutate_provenance_domain,
        ),
    ];

    for (field, value, mutate) in provenance_cases {
        let mut receipt = helpers::execution_receipt(&delivered_record, &acknowledged_transition);
        mutate(&mut receipt);

        let error = test_err!(
            validate_policy_delivery_execution_receipt(
                &delivered_record,
                &acknowledged_transition,
                Some(&receipt),
            ),
            "mismatched provenance must fail"
        );
        assert_eq!(error, EventingError::InvalidValue { field, value });
    }

    Ok(())
}

#[test]
fn execution_receipt_debug_redacts_sensitive_provenance() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut acknowledged_transition = transition(
        2,
        "attempt-debug-redaction",
        PolicyDeliveryState::Acknowledged,
    )?;
    acknowledged_transition.reason_code = Some(reason("reason-debug-redaction")?);
    let receipt = helpers::execution_receipt(&queued, &acknowledged_transition);

    let receipt_debug = format!("{receipt:?}");
    let sensitive_values = [
        "delivery-policy-household-default",
        "household-default",
        "policy-source-household-default",
        "child-primary",
        "device-laptop",
        "Tracking",
        "attempt-debug-redaction",
        "audit-attempt-debug-redaction-2",
        "reason-debug-redaction",
    ];

    for sensitive_value in sensitive_values {
        assert!(
            !receipt_debug.contains(sensitive_value),
            "debug output exposed sensitive value {sensitive_value}: {receipt_debug}"
        );
    }
    assert_eq!(
        receipt_debug,
        "PolicyDeliveryExecutionReceipt { delivery_id: \"<redacted>\", household_id: \"<redacted>\", policy_version: 3, source_document_id: \"<redacted>\", target: \"<redacted>\", attempt_id: \"<redacted>\", sequence: 2, state: Acknowledged, audit_reference_count: 1, reason_code_present: true, rollback_reference_state: None }"
    );
    Ok(())
}
