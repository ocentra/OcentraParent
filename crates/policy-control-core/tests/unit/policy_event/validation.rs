use super::*;

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

    assert_reason_and_dead_letter_validation()?;
    Ok(())
}

fn assert_reason_and_dead_letter_validation() -> TestResult {
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
fn policy_event_rejects_rollback_scope_household_mismatch() -> TestResult {
    let mut event = sample_rollback_applied_event(1)?;
    match &mut event.scope {
        PolicyEventScope::Rollback { rollback_ref, .. } => {
            rollback_ref.household_id = test_ok!(
                PolicyHouseholdId::parse("household-other"),
                "other rollback household"
            );
        }
        scope => {
            return Err(
                std::io::Error::other(format!("expected rollback scope, got {scope:?}")).into(),
            );
        }
    }
    assert_eq!(
        test_err!(
            event.contract(),
            "rollback scope household mismatch must fail closed"
        ),
        EventingError::InvalidValue {
            field: "policy_event.scope",
            value: "rollback household mismatch".to_string(),
        }
    );
    Ok(())
}
