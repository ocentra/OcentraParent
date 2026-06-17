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
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(3).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("policy device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-school-night-block").expect("policy rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("policy target reference"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-school-night").expect("policy schedule id"),
            ),
            priority: 100,
            reason_code: PolicyReasonCode::parse("school-night").expect("policy reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night")
                .expect("policy schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto")
                .expect("policy timezone name"),
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
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-policy-confirmed").expect("policy audit ref")
        ],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    }
}

fn sample_delivery_target() -> PolicyDeliveryTarget {
    PolicyDeliveryTarget {
        child_profile_id: PolicyChildProfileId::parse("child-primary").expect("child profile id"),
        device_id: PolicyDeviceId::parse("device-laptop").expect("policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    }
}

fn sample_policy_rollback_ref() -> PolicyRollbackRef {
    PolicyRollbackRef {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        rolled_back_policy_version: PolicyVersion::new(3).expect("policy version"),
        restored_document_id: ParentPolicyDocumentId::parse("policy-source-household-previous")
            .expect("policy source document id"),
        restored_policy_version: PolicyVersion::new(2).expect("policy version"),
    }
}

fn sample_queued_delivery() -> PolicyDeliveryRecord {
    let compiled = compile_domain_policy_artifact(
        &sample_policy_source_document(),
        PolicyConsumerDomain::Tracking,
    )
    .expect("compiled domain policy artifact");

    queue_policy_delivery(
        &compiled,
        sample_delivery_target(),
        PolicyDeliveryId::parse("delivery-policy-household-default").expect("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-queued").expect("policy attempt id"),
        vec![audit_ref("audit-policy-queued")],
    )
    .expect("queued policy delivery")
}

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(value).expect("policy audit ref")
}

fn reason(value: &str) -> PolicyReasonCode {
    PolicyReasonCode::parse(value).expect("policy reason code")
}

fn attempt(value: &str) -> PolicyDeliveryAttemptId {
    PolicyDeliveryAttemptId::parse(value).expect("policy attempt id")
}

fn transition(
    sequence: u64,
    attempt_id: &str,
    state: PolicyDeliveryState,
) -> PolicyDeliveryTransition {
    PolicyDeliveryTransition {
        attempt_id: attempt(attempt_id),
        sequence: PolicyDeliverySequence::new(sequence).expect("policy delivery sequence"),
        state,
        audit_reference_ids: vec![audit_ref(&format!("audit-{attempt_id}-{sequence}"))],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}

#[test]
fn queued_delivery_starts_pending_per_child_device_domain() {
    let queued = sample_queued_delivery();

    assert_eq!(queued.target.child_profile_id.as_str(), "child-primary");
    assert_eq!(queued.target.device_id.as_str(), "device-laptop");
    assert_eq!(queued.target.domain, PolicyConsumerDomain::Tracking);
    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert_eq!(
        queued.source_audit_reference_ids,
        vec![audit_ref("audit-policy-confirmed")]
    );
    assert!(queued.source_superseded_by_policy_version.is_none());
    assert!(queued.source_rollback_ref.is_none());
    assert_eq!(
        queued.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!queued.is_active());
}

#[test]
fn queued_delivery_preserves_source_lifecycle_metadata_separately_from_delivery_state() {
    let superseded_source = supersede_parent_policy_source_document(
        &sample_policy_source_document(),
        PolicyVersion::new(4).expect("policy version"),
        audit_ref("audit-policy-superseded"),
    )
    .expect("superseded policy source document");
    let superseded_compiled =
        compile_domain_policy_artifact(&superseded_source, PolicyConsumerDomain::Tracking)
            .expect("compiled superseded artifact");
    let superseded_delivery = queue_policy_delivery(
        &superseded_compiled,
        sample_delivery_target(),
        PolicyDeliveryId::parse("delivery-policy-superseded").expect("policy delivery id"),
        attempt("attempt-superseded-queued"),
        vec![audit_ref("audit-superseded-queued")],
    )
    .expect("queued superseded delivery");

    assert_eq!(
        superseded_delivery.source_audit_reference_ids,
        superseded_source.audit_reference_ids
    );
    assert_eq!(
        superseded_delivery
            .source_superseded_by_policy_version
            .expect("replacement policy version")
            .value(),
        4
    );
    assert!(superseded_delivery.source_rollback_ref.is_none());
    assert_eq!(
        superseded_delivery.audit_reference_ids,
        vec![audit_ref("audit-superseded-queued")]
    );
    assert!(superseded_delivery.superseded_by_policy_version.is_none());
    assert!(superseded_delivery.rollback_reference_state.is_none());

    let rolled_back_source = rollback_parent_policy_source_document(
        &sample_policy_source_document(),
        &sample_policy_rollback_ref(),
        audit_ref("audit-policy-rolled-back"),
    )
    .expect("rolled-back policy source document");
    let rolled_back_compiled =
        compile_domain_policy_artifact(&rolled_back_source, PolicyConsumerDomain::Tracking)
            .expect("compiled rolled-back artifact");
    let rolled_back_delivery = queue_policy_delivery(
        &rolled_back_compiled,
        sample_delivery_target(),
        PolicyDeliveryId::parse("delivery-policy-rolled-back").expect("policy delivery id"),
        attempt("attempt-rolled-back-queued"),
        vec![audit_ref("audit-rolled-back-queued")],
    )
    .expect("queued rolled-back delivery");

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
            .expect("source rollback ref")
            .restored_policy_version
            .value(),
        2
    );
    assert_eq!(
        rolled_back_delivery.audit_reference_ids,
        vec![audit_ref("audit-rolled-back-queued")]
    );
    assert!(rolled_back_delivery.superseded_by_policy_version.is_none());
    assert!(rolled_back_delivery.rollback_reference_state.is_none());
}

#[test]
fn duplicate_and_older_transitions_are_safe_noops() {
    let queued = sample_queued_delivery();
    let delivered = transition(2, "attempt-delivered", PolicyDeliveryState::Delivered);
    let delivered_record = apply_policy_delivery_transition(&queued, delivered.clone())
        .expect("deliver policy")
        .into_record();

    let duplicate = apply_policy_delivery_transition(&delivered_record, delivered)
        .expect("duplicate delivery is idempotent");
    let stale = apply_policy_delivery_transition(
        &delivered_record,
        transition(1, "attempt-stale", PolicyDeliveryState::Queued),
    )
    .expect("older queued replay is ignored");

    assert!(matches!(
        duplicate,
        PolicyDeliveryApplyOutcome::Duplicate(_)
    ));
    assert!(matches!(stale, PolicyDeliveryApplyOutcome::Stale(_)));
    assert_eq!(delivered_record.state, PolicyDeliveryState::Delivered);
}

#[test]
fn delivering_state_stays_pending_until_ack_or_apply() {
    let queued = sample_queued_delivery();
    let delivering = apply_policy_delivery_transition(
        &queued,
        transition(2, "attempt-delivering", PolicyDeliveryState::Delivering),
    )
    .expect("delivering transition")
    .into_record();

    assert_eq!(delivering.state, PolicyDeliveryState::Delivering);
    assert_eq!(
        delivering.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!delivering.is_active());
}

#[test]
fn acknowledged_delivery_stays_pending_and_is_not_active() {
    let queued = sample_queued_delivery();
    let acknowledged = apply_policy_delivery_transition(
        &queued,
        transition(2, "attempt-acknowledged", PolicyDeliveryState::Acknowledged),
    )
    .expect("acknowledge policy delivery")
    .into_record();

    assert_eq!(acknowledged.state, PolicyDeliveryState::Acknowledged);
    assert_eq!(
        acknowledged.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(acknowledged.reason_code.is_none());
    assert!(!acknowledged.is_active());
}

#[test]
fn offline_delivery_is_degraded_and_requires_reason_code() {
    let queued = sample_queued_delivery();
    let mut offline_transition = transition(2, "attempt-offline", PolicyDeliveryState::Offline);
    offline_transition.reason_code = Some(reason("network-offline"));

    let offline = apply_policy_delivery_transition(&queued, offline_transition)
        .expect("mark policy delivery offline")
        .into_record();

    assert_eq!(offline.state, PolicyDeliveryState::Offline);
    assert_eq!(
        offline.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        offline.reason_code,
        Some(reason("network-offline"))
    );
    assert!(!offline.is_active());
}

#[test]
fn queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output() {
    let queued = sample_queued_delivery();
    let payload = serde_json::to_value(&queued).expect("serialize policy delivery record");
    let debug = format!("{queued:?}");

    assert!(payload.get("child_profile_ids").is_none());
    assert!(payload.get("device_ids").is_none());
    assert!(payload.get("rules").is_none());
    assert!(payload.get("schedules").is_none());
    assert!(payload.get("retention").is_none());
    assert!(payload.get("source_audit_reference_ids").is_some());
    assert!(payload.get("target").is_some());
    assert!(!debug.contains("rule-school-night-block"));
    assert!(!debug.contains("schedule-school-night"));
    assert!(!debug.contains("school-night"));
}

#[test]
fn applied_transition_stays_active_when_intermediate_events_arrive_late() {
    let queued = sample_queued_delivery();
    let applied = apply_policy_delivery_transition(
        &queued,
        transition(4, "attempt-applied", PolicyDeliveryState::Applied),
    )
    .expect("applied transition can arrive before intermediate steps")
    .into_record();

    let stale_delivered = apply_policy_delivery_transition(
        &applied,
        transition(3, "attempt-delivered-late", PolicyDeliveryState::Delivered),
    )
    .expect("late delivered event is ignored");

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
}

#[test]
fn retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress() {
    let queued = sample_queued_delivery();
    let delivering = apply_policy_delivery_transition(
        &queued,
        transition(2, "attempt-delivering", PolicyDeliveryState::Delivering),
    )
    .expect("delivering transition")
    .into_record();

    let mut retry_transition = transition(3, "attempt-retry", PolicyDeliveryState::RetryScheduled);
    retry_transition.reason_code = Some(reason("adapter-timeout"));

    let retry = apply_policy_delivery_transition(&delivering, retry_transition)
        .expect("retry transition is accepted")
        .into_record();

    let mut partial_transition = transition(
        4,
        "attempt-partial",
        PolicyDeliveryState::PartialDomainApply,
    );
    partial_transition.reason_code = Some(reason("domain-subset-applied"));

    let partial = apply_policy_delivery_transition(&retry, partial_transition)
        .expect("partial-domain-apply transition is accepted")
        .into_record();

    let mut expired_transition = transition(
        5,
        "attempt-expired",
        PolicyDeliveryState::ExpiredBeforeDelivery,
    );
    expired_transition.reason_code = Some(reason("delivery-window-expired"));

    let expired = apply_policy_delivery_transition(&retry, expired_transition)
        .expect("expired transition is accepted")
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
            .expect("partial reason code")
            .as_str(),
        "domain-subset-applied"
    );
    assert_eq!(
        expired
            .reason_code
            .as_ref()
            .expect("expired reason code")
            .as_str(),
        "delivery-window-expired"
    );
    assert!(!partial.is_active());
    assert!(!expired.is_active());
}

#[test]
fn blocked_and_manual_required_transitions_require_reason_and_surface_manual_required() {
    let queued = sample_queued_delivery();
    let missing_reason_error = apply_policy_delivery_transition(
        &queued,
        transition(
            2,
            "attempt-blocked-permission-missing-reason",
            PolicyDeliveryState::BlockedByPermission,
        ),
    )
    .expect_err("blocked-by-permission without reason is invalid");
    assert!(missing_reason_error
        .to_string()
        .contains("policy_delivery.reason_code"));

    let mut blocked_permission = transition(
        2,
        "attempt-blocked-permission",
        PolicyDeliveryState::BlockedByPermission,
    );
    blocked_permission.reason_code = Some(reason("device-permission-lost"));
    let blocked_permission = apply_policy_delivery_transition(&queued, blocked_permission)
        .expect("blocked-by-permission transition")
        .into_record();

    let mut blocked_capability = transition(
        3,
        "attempt-blocked-capability",
        PolicyDeliveryState::BlockedByCapability,
    );
    blocked_capability.reason_code = Some(reason("adapter-capability-missing"));
    let blocked_capability =
        apply_policy_delivery_transition(&blocked_permission, blocked_capability)
            .expect("blocked-by-capability transition")
            .into_record();

    let mut manual_required = transition(
        4,
        "attempt-manual-required",
        PolicyDeliveryState::ManualRequired,
    );
    manual_required.reason_code = Some(reason("parent-confirmation-required"));
    let manual_required = apply_policy_delivery_transition(&blocked_capability, manual_required)
        .expect("manual-required transition")
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
}

#[test]
fn rejected_and_rolled_back_transitions_require_reason_and_reference_context() {
    let queued = sample_queued_delivery();

    let rejected_error = apply_policy_delivery_transition(
        &queued,
        transition(2, "attempt-rejected", PolicyDeliveryState::Rejected),
    )
    .expect_err("rejected transition without reason is invalid");
    assert!(rejected_error
        .to_string()
        .contains("policy_delivery.reason_code"));

    let applied = apply_policy_delivery_transition(
        &queued,
        transition(3, "attempt-applied", PolicyDeliveryState::Applied),
    )
    .expect("apply transition")
    .into_record();

    let mut rollback_transition =
        transition(4, "attempt-rollback", PolicyDeliveryState::RolledBack);
    rollback_transition.reason_code = Some(reason("adapter-failed"));
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let rolled_back = apply_policy_delivery_transition(&applied, rollback_transition)
        .expect("rollback transition")
        .into_record();

    assert_eq!(rolled_back.state, PolicyDeliveryState::RolledBack);
    assert_eq!(
        rolled_back.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        rolled_back
            .rollback_reference_state
            .expect("rollback reference state"),
        PolicyDeliveryState::Applied
    );
}

#[test]
fn superseded_transition_requires_newer_policy_version_and_blocks_regressions() {
    let queued = sample_queued_delivery();
    let applied = apply_policy_delivery_transition(
        &queued,
        transition(2, "attempt-applied", PolicyDeliveryState::Applied),
    )
    .expect("apply transition")
    .into_record();

    let mut invalid_superseded = transition(
        3,
        "attempt-superseded-invalid",
        PolicyDeliveryState::Superseded,
    );
    invalid_superseded.superseded_by_policy_version = Some(PolicyVersion::new(3).expect("version"));

    let invalid_error = apply_policy_delivery_transition(&applied, invalid_superseded)
        .expect_err("same-version supersede is invalid");
    assert!(invalid_error
        .to_string()
        .contains("policy_delivery.superseded_by_policy_version"));

    let mut superseded = transition(4, "attempt-superseded", PolicyDeliveryState::Superseded);
    superseded.superseded_by_policy_version = Some(PolicyVersion::new(4).expect("version"));

    let superseded_record = apply_policy_delivery_transition(&applied, superseded)
        .expect("superseded transition")
        .into_record();

    let regression_error = apply_policy_delivery_transition(
        &superseded_record,
        transition(5, "attempt-regression", PolicyDeliveryState::Delivered),
    )
    .expect_err("superseded delivery cannot regress to delivered");

    assert_eq!(superseded_record.state, PolicyDeliveryState::Superseded);
    assert_eq!(
        superseded_record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Superseded
    );
    assert!(regression_error.to_string().contains("invalid transition"));
}
