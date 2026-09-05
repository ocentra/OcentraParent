use super::*;

#[test]
fn approval_at_request_expiry_is_rejected() -> TestResult {
    let request = child_request()?;
    let mut approval = approval(&request, PolicyApprovalDecision::Grant)?;
    approval.decided_at = timestamp(EXPIRES_AT)?;

    let error = test_err!(
        resolve_parent_policy_approval(&request, approval, None),
        "approval at the request expiry cannot create an active override"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "approval-decision-must-be-within-request-window".to_string(),
        }
    );
    Ok(())
}

#[test]
fn request_expiry_requires_a_strictly_ordered_utc_window() -> TestResult {
    let mut request = child_request()?;
    request.expires_at = timestamp(REQUESTED_AT)?;

    let error = test_err!(
        register_child_policy_request(None, request),
        "request expiry must follow request creation"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "expires-at-must-be-after-requested-at".to_string(),
        }
    );
    Ok(())
}

#[test]
fn request_timestamps_reject_impossible_calendar_dates() -> TestResult {
    let mut request = child_request()?;
    request.requested_at = timestamp("2026-02-29T20:00:00Z")?;

    let error = test_err!(
        register_child_policy_request(None, request),
        "request timestamps must reject non-leap-day February 29"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "2026-02-29T20:00:00Z".to_string(),
        }
    );
    Ok(())
}

#[test]
fn bonus_minutes_cannot_be_attached_to_an_ask_parent_request() -> TestResult {
    let request = ChildPolicyRequest {
        scope: request_scope(PolicyRequestKind::AskParent, Some(10))?,
        ..child_request()?
    };

    let error = test_err!(
        register_child_policy_request(None, request),
        "bonus minutes must stay scoped to bonus-time requests"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.requested_bonus_minutes",
            value: "only-bonus-time-requests-may-include-minutes".to_string(),
        }
    );
    Ok(())
}

#[test]
fn bonus_time_requests_cannot_request_a_block_action() -> TestResult {
    let mut request = child_request()?;
    request.scope.requested_action = PolicyRuleAction::Block;

    let error = test_err!(
        register_child_policy_request(None, request),
        "bonus time cannot request a blocking action"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.requested_bonus_minutes",
            value: "bonus-time-requests-require-allow-or-time-limit".to_string(),
        }
    );
    Ok(())
}

#[test]
fn approval_expiry_and_bonus_action_must_stay_within_typed_bounds() -> TestResult {
    let request = child_request()?;
    let mut early_expiry = approval(&request, PolicyApprovalDecision::Grant)?;
    early_expiry.override_expires_at = Some(timestamp("2026-06-13T20:05:00Z")?);

    let error = test_err!(
        resolve_parent_policy_approval(&request, early_expiry, None),
        "override expiry must follow the approval decision"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "approval-override-expiry-must-be-after-decision".to_string(),
        }
    );

    let ask_parent_request = ChildPolicyRequest {
        scope: request_scope(PolicyRequestKind::AskParent, None)?,
        ..child_request()?
    };
    let mut invalid_bonus_approval = approval(&ask_parent_request, PolicyApprovalDecision::Grant)?;
    invalid_bonus_approval.approved_bonus_minutes =
        Some(test_ok!(PolicyDurationMinutes::new(10), "minutes"));
    let error = test_err!(
        resolve_parent_policy_approval(&ask_parent_request, invalid_bonus_approval, None),
        "approved bonus minutes must match the request kind"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.approved_bonus_minutes",
            value: "only-bonus-time-requests-may-include-approved-minutes".to_string(),
        }
    );
    Ok(())
}

#[test]
fn request_expiry_is_deadline_gated() -> TestResult {
    let request = child_request()?;
    let early_request_expiry = test_err!(
        expire_child_policy_request(
            &request,
            timestamp("2026-06-13T21:59:00Z")?,
            audit_ref("audit-request-early-expiry")?,
        ),
        "request cannot expire before its deadline"
    );
    assert_eq!(
        early_request_expiry,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "request-not-yet-expired".to_string(),
        }
    );
    Ok(())
}

#[test]
fn temporary_override_expiry_is_deadline_gated_and_idempotent() -> TestResult {
    let request = child_request()?;
    let resolution = test_ok!(
        resolve_parent_policy_approval(
            &request,
            approval(&request, PolicyApprovalDecision::Grant)?,
            None,
        ),
        "grant creates an active temporary override"
    );
    let active = test_some!(
        resolution.temporary_override.as_ref(),
        "active temporary override"
    );

    let early_error = test_err!(
        ocentra_policy_control_core::policy_request::expire_policy_temporary_override(
            active,
            &timestamp("2026-06-13T21:59:00Z")?,
            audit_ref("audit-override-early-expiry")?,
        ),
        "temporary override cannot expire before its deadline"
    );
    assert_eq!(
        early_error,
        EventingError::InvalidValue {
            field: "policy_request.timestamp",
            value: "override-not-yet-expired".to_string(),
        }
    );

    let expired = test_ok!(
        ocentra_policy_control_core::policy_request::expire_policy_temporary_override(
            active,
            &timestamp(EXPIRES_AT)?,
            audit_ref("audit-override-expired")?,
        ),
        "temporary override expires at its deadline"
    );
    let mut expected = active.clone();
    expected.state = PolicyOverrideState::Expired;
    expected
        .audit_reference_ids
        .push(audit_ref("audit-override-expired")?);
    assert_eq!(expired, expected);

    let replay = test_ok!(
        ocentra_policy_control_core::policy_request::expire_policy_temporary_override(
            &expired,
            &timestamp("2026-06-13T22:05:00Z")?,
            audit_ref("audit-override-expired-replay")?,
        ),
        "repeated temporary override expiry is idempotent"
    );
    assert_eq!(replay, expired);
    Ok(())
}
