use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, register_parent_policy_source_document,
    rollback_parent_policy_source_document, supersede_parent_policy_source_document,
    ParentPolicyActorRole, ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument,
    PolicyActorId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRetentionMetadata, PolicyRollbackRef, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
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

fn sample_policy_source_document(version: u64) -> TestResult<ParentPolicySourceDocument> {
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
        policy_version: test_ok!(PolicyVersion::new(version), "policy version"),
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
            time_budget: sample_policy_schedule_time_budget(),
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

fn sample_policy_source_payload(version: u64) -> TestResult<Value> {
    Ok(test_ok!(
        serde_json::to_value(sample_policy_source_document(version)?),
        "policy source payload"
    ))
}

#[test]
fn policy_source_serde_rejects_zero_schema_version() -> TestResult {
    let mut payload = sample_policy_source_payload(1)?;
    payload["schema_version"] = Value::from(0_u64);

    let error = test_err!(
        serde_json::from_value::<ParentPolicySourceDocument>(payload),
        "policy source schema version zero must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn policy_source_serde_rejects_schedule_payload_without_time_budget() -> TestResult {
    let mut payload = sample_policy_source_payload(1)?;
    let schedules = test_some!(
        payload.get_mut("schedules").and_then(Value::as_array_mut),
        "schedule payload array"
    );
    let schedule = test_some!(
        schedules.first_mut().and_then(Value::as_object_mut),
        "schedule payload object"
    );
    schedule.remove("time_budget");

    let error = test_err!(
        serde_json::from_value::<ParentPolicySourceDocument>(payload),
        "schedule payloads must include a time_budget block"
    );

    assert!(error.to_string().contains("time_budget"));
    Ok(())
}

#[test]
fn stale_policy_version_is_rejected_during_registration() -> TestResult {
    let existing = sample_policy_source_document(4)?;
    let candidate = sample_policy_source_document(3)?;

    let error = test_err!(
        register_parent_policy_source_document(Some(&existing), candidate),
        "older policy version cannot replace current source truth"
    );
    assert!(error.to_string().contains("stale policy version"));
    Ok(())
}

#[test]
fn supersede_rejects_non_newer_replacement_versions() -> TestResult {
    let current = sample_policy_source_document(4)?;

    let error = test_err!(
        supersede_parent_policy_source_document(
            &current,
            test_ok!(PolicyVersion::new(4), "policy version"),
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-superseded"),
                "policy audit ref"
            ),
        ),
        "replacement policy version must be newer than current source version"
    );

    assert!(error
        .to_string()
        .contains("policy_source.superseded_by_policy_version"));
    Ok(())
}

#[test]
fn rollback_rejects_non_older_restored_versions() -> TestResult {
    let current = sample_policy_source_document(4)?;
    let rollback_ref = PolicyRollbackRef {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        rolled_back_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id"
        ),
        rolled_back_policy_version: test_ok!(PolicyVersion::new(4), "policy version"),
        restored_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-previous"),
            "policy source document id"
        ),
        restored_policy_version: test_ok!(PolicyVersion::new(4), "policy version"),
    };

    let error = test_err!(
        rollback_parent_policy_source_document(
            &current,
            &rollback_ref,
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-rolled-back"),
                "policy audit ref"
            ),
        ),
        "restored policy version must be older than the rolled-back source version"
    );

    assert!(error
        .to_string()
        .contains("policy_source.rollback_ref.restored_policy_version"));
    Ok(())
}
