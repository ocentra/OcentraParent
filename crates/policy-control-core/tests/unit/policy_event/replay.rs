use super::*;

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
