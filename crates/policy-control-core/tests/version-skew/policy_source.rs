use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, register_parent_policy_source_document,
    rollback_parent_policy_source_document, supersede_parent_policy_source_document,
    ParentPolicyActorRole, ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument,
    PolicyActorId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRetentionMetadata, PolicyRollbackRef, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};
use serde_json::Value;

fn sample_policy_schedule_time_budget() -> PolicyScheduleTimeBudget {
    PolicyScheduleTimeBudget {
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
    }
}

fn sample_policy_source_document(version: u64) -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(version).expect("policy version"),
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
            time_budget: sample_policy_schedule_time_budget(),
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

fn sample_policy_source_payload(version: u64) -> Value {
    serde_json::to_value(sample_policy_source_document(version)).expect("policy source payload")
}

#[test]
fn policy_source_serde_rejects_zero_schema_version() {
    let mut payload = sample_policy_source_payload(1);
    payload["schema_version"] = Value::from(0_u64);

    let error = serde_json::from_value::<ParentPolicySourceDocument>(payload)
        .expect_err("policy source schema version zero must be rejected");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn policy_source_serde_rejects_schedule_payload_without_time_budget() {
    let mut payload = sample_policy_source_payload(1);
    let schedules = payload
        .get_mut("schedules")
        .and_then(Value::as_array_mut)
        .expect("schedule payload array");
    let schedule = schedules
        .first_mut()
        .and_then(Value::as_object_mut)
        .expect("schedule payload object");
    schedule.remove("time_budget");

    let error = serde_json::from_value::<ParentPolicySourceDocument>(payload)
        .expect_err("schedule payloads must include a time_budget block");

    assert!(error.to_string().contains("time_budget"));
}

#[test]
fn stale_policy_version_is_rejected_during_registration() {
    let existing = sample_policy_source_document(4);
    let candidate = sample_policy_source_document(3);

    let error = register_parent_policy_source_document(Some(&existing), candidate)
        .expect_err("older policy version cannot replace current source truth");
    assert!(error.to_string().contains("stale policy version"));
}

#[test]
fn supersede_rejects_non_newer_replacement_versions() {
    let current = sample_policy_source_document(4);

    let error = supersede_parent_policy_source_document(
        &current,
        PolicyVersion::new(4).expect("policy version"),
        PolicyAuditReferenceId::parse("audit-policy-superseded").expect("policy audit ref"),
    )
    .expect_err("replacement policy version must be newer than current source version");

    assert!(error
        .to_string()
        .contains("policy_source.superseded_by_policy_version"));
}

#[test]
fn rollback_rejects_non_older_restored_versions() {
    let current = sample_policy_source_document(4);
    let rollback_ref = PolicyRollbackRef {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        rolled_back_policy_version: PolicyVersion::new(4).expect("policy version"),
        restored_document_id: ParentPolicyDocumentId::parse("policy-source-household-previous")
            .expect("policy source document id"),
        restored_policy_version: PolicyVersion::new(4).expect("policy version"),
    };

    let error = rollback_parent_policy_source_document(
        &current,
        &rollback_ref,
        PolicyAuditReferenceId::parse("audit-policy-rolled-back").expect("policy audit ref"),
    )
    .expect_err("restored policy version must be older than the rolled-back source version");

    assert!(error
        .to_string()
        .contains("policy_source.rollback_ref.restored_policy_version"));
}
