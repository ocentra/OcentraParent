use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryApplyOutcome,
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryParentVisibleState,
    PolicyDeliveryRecord, PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTarget,
    PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version,
    rollback_parent_policy_source_document, supersede_parent_policy_source_document,
    ParentPolicyActorRole, ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument,
    PolicyActorId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata,
    PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

fn sample_policy_source_document() -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id"
        ),
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        policy_version: test_ok!(PolicyVersion::new(3), "policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceStatus::Confirmed,
        child_profile_ids: vec![test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        )],
        device_ids: vec![test_ok!(
            PolicyDeviceId::parse("device-laptop"),
            "policy device id"
        )],
        rules: vec![ParentPolicyRule {
            rule_id: test_ok!(
                PolicyRuleId::parse("rule-school-night-block"),
                "policy rule id"
            ),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("category-gaming"),
                    "policy target reference"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id"
            )),
            priority: 100,
            reason_code: test_ok!(
                PolicyReasonCode::parse("school-night"),
                "policy reason code"
            ),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id"
            ),
            timezone_name: test_ok!(
                PolicyTimezoneName::parse("America/Toronto"),
                "policy timezone name"
            ),
            starts_at: "21:00".to_string(),
            ends_at: "07:00".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 120,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "00:00".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::DiscardUnused,
                    max_minutes: None,
                },
                grace_period_minutes: 5,
                effective_from: "2026-01-01T00:00:00Z".to_string(),
                effective_until: None,
                bonus_expiry_minutes: 30,
                clock_source: PolicyScheduleClockSource::TrustedService,
                offline_recovery: PolicyScheduleOfflineRecovery::RecomputeFromJournal,
            },
        }],
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-confirmed"),
            "policy audit ref"
        )],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    })
}

fn sample_delivery_target() -> TestResult<PolicyDeliveryTarget> {
    Ok(PolicyDeliveryTarget {
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: test_ok!(PolicyDeviceId::parse("device-laptop"), "policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    })
}

fn sample_policy_rollback_ref() -> TestResult<PolicyRollbackRef> {
    Ok(PolicyRollbackRef {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        rolled_back_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id"
        ),
        rolled_back_policy_version: test_ok!(PolicyVersion::new(3), "policy version"),
        restored_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-previous"),
            "policy source document id"
        ),
        restored_policy_version: test_ok!(PolicyVersion::new(2), "policy version"),
    })
}

fn sample_queued_delivery() -> TestResult<PolicyDeliveryRecord> {
    let source = sample_policy_source_document()?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );

    Ok(test_ok!(
        queue_policy_delivery(
            &compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-policy-household-default"),
                "policy delivery id"
            ),
            test_ok!(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id"
            ),
            vec![audit_ref("audit-policy-queued")?],
        ),
        "queued policy delivery"
    ))
}

fn audit_ref(value: &str) -> TestResult<PolicyAuditReferenceId> {
    Ok(test_ok!(
        PolicyAuditReferenceId::parse(value),
        "policy audit ref"
    ))
}

fn reason(value: &str) -> TestResult<PolicyReasonCode> {
    Ok(test_ok!(
        PolicyReasonCode::parse(value),
        "policy reason code"
    ))
}

fn attempt(value: &str) -> TestResult<PolicyDeliveryAttemptId> {
    Ok(test_ok!(
        PolicyDeliveryAttemptId::parse(value),
        "policy attempt id"
    ))
}

fn transition(
    sequence: u64,
    attempt_id: &str,
    state: PolicyDeliveryState,
) -> TestResult<PolicyDeliveryTransition> {
    Ok(PolicyDeliveryTransition {
        attempt_id: attempt(attempt_id)?,
        sequence: test_ok!(
            PolicyDeliverySequence::new(sequence),
            "policy delivery sequence"
        ),
        state,
        audit_reference_ids: vec![audit_ref(&format!("audit-{attempt_id}-{sequence}"))?],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    })
}

#[test]
fn queued_delivery_starts_pending_per_child_device_domain() -> TestResult {
    let queued = sample_queued_delivery()?;

    assert_eq!(queued.target.child_profile_id.as_str(), "child-primary");
    assert_eq!(queued.target.device_id.as_str(), "device-laptop");
    assert_eq!(queued.target.domain, PolicyConsumerDomain::Tracking);
    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert_eq!(
        queued.source_audit_reference_ids,
        vec![audit_ref("audit-policy-confirmed")?]
    );
    assert!(queued.source_superseded_by_policy_version.is_none());
    assert!(queued.source_rollback_ref.is_none());
    assert_eq!(
        queued.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!queued.is_active());
    Ok(())
}

#[test]
fn queued_delivery_preserves_source_lifecycle_metadata_separately_from_delivery_state() -> TestResult
{
    let superseded_source = test_ok!(
        supersede_parent_policy_source_document(
            &sample_policy_source_document()?,
            test_ok!(PolicyVersion::new(4), "policy version"),
            audit_ref("audit-policy-superseded")?,
        ),
        "superseded policy source document"
    );
    let superseded_compiled = test_ok!(
        compile_domain_policy_artifact(&superseded_source, PolicyConsumerDomain::Tracking),
        "compiled superseded artifact"
    );
    let superseded_delivery = test_ok!(
        queue_policy_delivery(
            &superseded_compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-policy-superseded"),
                "policy delivery id"
            ),
            attempt("attempt-superseded-queued")?,
            vec![audit_ref("audit-superseded-queued")?],
        ),
        "queued superseded delivery"
    );

    assert_eq!(
        superseded_delivery.source_audit_reference_ids,
        superseded_source.audit_reference_ids
    );
    assert_eq!(
        superseded_delivery
            .source_superseded_by_policy_version
            .ok_or_else(|| std::io::Error::other("replacement policy version"))?
            .value(),
        4
    );
    assert!(superseded_delivery.source_rollback_ref.is_none());
    assert_eq!(
        superseded_delivery.audit_reference_ids,
        vec![audit_ref("audit-superseded-queued")?]
    );
    assert!(superseded_delivery.superseded_by_policy_version.is_none());
    assert!(superseded_delivery.rollback_reference_state.is_none());

    let rollback_ref = sample_policy_rollback_ref()?;
    let rolled_back_source = test_ok!(
        rollback_parent_policy_source_document(
            &sample_policy_source_document()?,
            &rollback_ref,
            audit_ref("audit-policy-rolled-back")?,
        ),
        "rolled-back policy source document"
    );
    let rolled_back_compiled = test_ok!(
        compile_domain_policy_artifact(&rolled_back_source, PolicyConsumerDomain::Tracking),
        "compiled rolled-back artifact"
    );
    let rolled_back_delivery = test_ok!(
        queue_policy_delivery(
            &rolled_back_compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-policy-rolled-back"),
                "policy delivery id"
            ),
            attempt("attempt-rolled-back-queued")?,
            vec![audit_ref("audit-rolled-back-queued")?],
        ),
        "queued rolled-back delivery"
    );

    assert_eq!(
        rolled_back_delivery.source_audit_reference_ids,
        rolled_back_source.audit_reference_ids
    );
    assert!(rolled_back_delivery
        .source_superseded_by_policy_version
        .is_none());
    assert_eq!(
        rolled_back_delivery
            .source_rollback_ref
            .as_ref()
            .ok_or_else(|| std::io::Error::other("source rollback ref"))?
            .restored_policy_version
            .value(),
        2
    );
    assert_eq!(
        rolled_back_delivery.audit_reference_ids,
        vec![audit_ref("audit-rolled-back-queued")?]
    );
    assert!(rolled_back_delivery.superseded_by_policy_version.is_none());
    assert!(rolled_back_delivery.rollback_reference_state.is_none());
    Ok(())
}

#[test]
fn duplicate_and_older_transitions_are_safe_noops() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered.clone()),
        "deliver policy"
    )
    .into_record();

    let duplicate = test_ok!(
        apply_policy_delivery_transition(&delivered_record, delivered),
        "duplicate delivery is idempotent"
    );
    let stale = apply_policy_delivery_transition(
        &delivered_record,
        transition(1, "attempt-stale", PolicyDeliveryState::Queued)?,
    )
    .map_err(|error| std::io::Error::other(format!("older queued replay is ignored: {error}")))?;

    assert!(matches!(
        duplicate,
        PolicyDeliveryApplyOutcome::Duplicate(_)
    ));
    assert!(matches!(stale, PolicyDeliveryApplyOutcome::Stale(_)));
    assert_eq!(delivered_record.state, PolicyDeliveryState::Delivered);
    Ok(())
}

#[test]
fn conflicting_same_sequence_replay_is_rejected() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered),
        "deliver policy"
    )
    .into_record();

    let conflict = test_err!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(
                2,
                "attempt-acknowledged-conflict",
                PolicyDeliveryState::Acknowledged,
            )?,
        ),
        "same-sequence replay with changed state must be rejected"
    );

    assert_eq!(
        conflict,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "conflicting replay for sequence 2 on delivery-policy-household-default"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn delivering_state_stays_pending_until_ack_or_apply() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivering = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivering", PolicyDeliveryState::Delivering)?,
        ),
        "delivering transition"
    )
    .into_record();

    assert_eq!(delivering.state, PolicyDeliveryState::Delivering);
    assert_eq!(
        delivering.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!delivering.is_active());
    Ok(())
}

#[test]
fn acknowledged_delivery_stays_pending_and_is_not_active() -> TestResult {
    let queued = sample_queued_delivery()?;
    let acknowledged = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-acknowledged", PolicyDeliveryState::Acknowledged)?,
        ),
        "acknowledge policy delivery"
    )
    .into_record();

    assert_eq!(acknowledged.state, PolicyDeliveryState::Acknowledged);
    assert_eq!(
        acknowledged.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(acknowledged.reason_code.is_none());
    assert!(!acknowledged.is_active());
    Ok(())
}

#[test]
fn offline_delivery_is_degraded_and_requires_reason_code() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut offline_transition = transition(2, "attempt-offline", PolicyDeliveryState::Offline)?;
    offline_transition.reason_code = Some(reason("network-offline")?);

    let offline = test_ok!(
        apply_policy_delivery_transition(&queued, offline_transition),
        "mark policy delivery offline"
    )
    .into_record();

    assert_eq!(offline.state, PolicyDeliveryState::Offline);
    assert_eq!(
        offline.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(offline.reason_code, Some(reason("network-offline")?));
    assert!(!offline.is_active());
    Ok(())
}

#[test]
fn queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let payload = test_ok!(
        serde_json::to_value(&queued),
        "serialize policy delivery record"
    );
    let debug = format!("{queued:?}");

    assert!(payload.get("child_profile_ids").is_none());
    assert!(payload.get("device_ids").is_none());
    assert!(payload.get("rules").is_none());
    assert!(payload.get("schedules").is_none());
    assert!(payload.get("retention").is_none());
    assert_eq!(
        payload["source_audit_reference_ids"],
        serde_json::json!(["audit-policy-confirmed"])
    );
    assert_eq!(payload["target"]["child_profile_id"], "child-primary");
    assert_eq!(debug.find("rule-school-night-block"), None);
    assert_eq!(debug.find("schedule-school-night"), None);
    assert_eq!(debug.find("school-night"), None);
    Ok(())
}

#[test]
fn applied_transition_stays_active_when_intermediate_events_arrive_late() -> TestResult {
    let queued = sample_queued_delivery()?;
    let applied = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(4, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "applied transition can arrive before intermediate steps"
    )
    .into_record();

    let stale_delivered = test_ok!(
        apply_policy_delivery_transition(
            &applied,
            transition(3, "attempt-delivered-late", PolicyDeliveryState::Delivered)?,
        ),
        "late delivered event is ignored"
    );

    assert!(matches!(
        stale_delivered,
        PolicyDeliveryApplyOutcome::Stale(_)
    ));
    assert_eq!(applied.state, PolicyDeliveryState::Applied);
    assert!(applied.is_active());
    assert_eq!(
        applied.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Applied
    );
    Ok(())
}

#[test]
fn retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let delivering = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivering", PolicyDeliveryState::Delivering)?,
        ),
        "delivering transition"
    )
    .into_record();

    let mut retry_transition = transition(3, "attempt-retry", PolicyDeliveryState::RetryScheduled)?;
    retry_transition.reason_code = Some(reason("adapter-timeout")?);

    let retry = test_ok!(
        apply_policy_delivery_transition(&delivering, retry_transition),
        "retry transition is accepted"
    )
    .into_record();

    let mut partial_transition = transition(
        4,
        "attempt-partial",
        PolicyDeliveryState::PartialDomainApply,
    )?;
    partial_transition.reason_code = Some(reason("domain-subset-applied")?);

    let partial = test_ok!(
        apply_policy_delivery_transition(&retry, partial_transition),
        "partial-domain-apply transition is accepted"
    )
    .into_record();

    let mut expired_transition = transition(
        5,
        "attempt-expired",
        PolicyDeliveryState::ExpiredBeforeDelivery,
    )?;
    expired_transition.reason_code = Some(reason("delivery-window-expired")?);

    let expired = test_ok!(
        apply_policy_delivery_transition(&retry, expired_transition),
        "expired transition is accepted"
    )
    .into_record();

    assert_eq!(
        retry.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        partial.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        expired.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        partial
            .reason_code
            .as_ref()
            .ok_or_else(|| std::io::Error::other("partial reason code"))?
            .as_str(),
        "domain-subset-applied"
    );
    assert_eq!(
        expired
            .reason_code
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expired reason code"))?
            .as_str(),
        "delivery-window-expired"
    );
    assert!(!partial.is_active());
    assert!(!expired.is_active());
    Ok(())
}

#[test]
fn blocked_and_manual_required_transitions_require_reason_and_surface_manual_required() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let missing_reason_error = test_err!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-blocked-permission-missing-reason",
                PolicyDeliveryState::BlockedByPermission,
            )?,
        ),
        "blocked-by-permission without reason is invalid"
    );
    assert_eq!(
        missing_reason_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "missing reason code for blocked-by-permission".to_string(),
        }
    );

    let mut blocked_permission = transition(
        2,
        "attempt-blocked-permission",
        PolicyDeliveryState::BlockedByPermission,
    )?;
    blocked_permission.reason_code = Some(reason("device-permission-lost")?);
    let blocked_permission = test_ok!(
        apply_policy_delivery_transition(&queued, blocked_permission),
        "blocked-by-permission transition"
    )
    .into_record();

    let mut blocked_capability = transition(
        3,
        "attempt-blocked-capability",
        PolicyDeliveryState::BlockedByCapability,
    )?;
    blocked_capability.reason_code = Some(reason("adapter-capability-missing")?);
    let blocked_capability = test_ok!(
        apply_policy_delivery_transition(&blocked_permission, blocked_capability),
        "blocked-by-capability transition"
    )
    .into_record();

    let mut manual_required = transition(
        4,
        "attempt-manual-required",
        PolicyDeliveryState::ManualRequired,
    )?;
    manual_required.reason_code = Some(reason("parent-confirmation-required")?);
    let manual_required = test_ok!(
        apply_policy_delivery_transition(&blocked_capability, manual_required),
        "manual-required transition"
    )
    .into_record();

    assert_eq!(
        blocked_permission.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        blocked_capability.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        manual_required.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert!(!manual_required.is_active());
    Ok(())
}

#[test]
fn rejected_and_rolled_back_transitions_require_reason_and_reference_context() -> TestResult {
    let queued = sample_queued_delivery()?;

    let rejected_error = test_err!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-rejected", PolicyDeliveryState::Rejected)?,
        ),
        "rejected transition without reason is invalid"
    );
    assert_eq!(
        rejected_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "missing reason code for rejected".to_string(),
        }
    );

    let applied = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(3, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "apply transition"
    )
    .into_record();

    let mut rollback_transition =
        transition(4, "attempt-rollback", PolicyDeliveryState::RolledBack)?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let rolled_back = test_ok!(
        apply_policy_delivery_transition(&applied, rollback_transition),
        "rollback transition"
    )
    .into_record();

    assert_eq!(rolled_back.state, PolicyDeliveryState::RolledBack);
    assert_eq!(
        rolled_back.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        rolled_back
            .rollback_reference_state
            .ok_or_else(|| std::io::Error::other("rollback reference state"))?,
        PolicyDeliveryState::Applied
    );
    Ok(())
}

#[test]
fn superseded_transition_requires_newer_policy_version_and_blocks_regressions() -> TestResult {
    let queued = sample_queued_delivery()?;
    let applied = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "apply transition"
    )
    .into_record();

    let mut invalid_superseded = transition(
        3,
        "attempt-superseded-invalid",
        PolicyDeliveryState::Superseded,
    )?;
    invalid_superseded.superseded_by_policy_version =
        Some(test_ok!(PolicyVersion::new(3), "version"));

    let invalid_error = test_err!(
        apply_policy_delivery_transition(&applied, invalid_superseded),
        "same-version supersede is invalid"
    );
    assert_eq!(
        invalid_error,
        EventingError::InvalidValue {
            field: "policy_delivery.superseded_by_policy_version",
            value: "replacement policy version 3 must be newer than 3".to_string(),
        }
    );

    let mut superseded = transition(4, "attempt-superseded", PolicyDeliveryState::Superseded)?;
    superseded.superseded_by_policy_version = Some(test_ok!(PolicyVersion::new(4), "version"));

    let superseded_record = test_ok!(
        apply_policy_delivery_transition(&applied, superseded),
        "superseded transition"
    )
    .into_record();

    let regression_error = test_err!(
        apply_policy_delivery_transition(
            &superseded_record,
            transition(5, "attempt-regression", PolicyDeliveryState::Delivered)?,
        ),
        "superseded delivery cannot regress to delivered"
    );

    assert_eq!(superseded_record.state, PolicyDeliveryState::Superseded);
    assert_eq!(
        superseded_record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Superseded
    );
    assert_eq!(
        regression_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "invalid transition superseded -> delivered".to_string(),
        }
    );
    Ok(())
}

#[test]
fn superseded_before_ack_stays_superseded_and_never_becomes_active() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "delivered transition"
    )
    .into_record();

    let mut superseded = transition(
        3,
        "attempt-superseded-before-ack",
        PolicyDeliveryState::Superseded,
    )?;
    superseded.superseded_by_policy_version = Some(test_ok!(PolicyVersion::new(4), "version"));

    let superseded_record = test_ok!(
        apply_policy_delivery_transition(&delivered, superseded),
        "superseded before ack transition"
    )
    .into_record();

    assert_eq!(superseded_record.state, PolicyDeliveryState::Superseded);
    assert_eq!(
        superseded_record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Superseded
    );
    assert_eq!(
        superseded_record
            .superseded_by_policy_version
            .ok_or_else(|| std::io::Error::other("replacement policy version"))?
            .value(),
        4
    );
    assert!(!superseded_record.is_active());
    Ok(())
}
