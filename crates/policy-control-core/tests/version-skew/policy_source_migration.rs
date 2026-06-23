use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_source::{
    assess_policy_source_compatibility, parent_policy_source_schema_version, ParentPolicyActorRole,
    ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId, PolicyDocumentCompatibilityState,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

use ocentra_eventing::ids::SchemaVersion;
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
        policy_version: test_ok!(PolicyVersion::new(6), "policy version"),
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
            "device id"
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
                    "target reference"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 100,
            reason_code: test_ok!(PolicyReasonCode::parse("school-night"), "reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: test_ok!(PolicyScheduleId::parse("schedule-school-night"), "schedule"),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Toronto"), "timezone"),
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

fn sample_policy_source_payload() -> TestResult<Value> {
    Ok(test_ok!(
        serde_json::to_value(sample_policy_source_document()?),
        "policy source payload"
    ))
}

#[test]
fn older_schema_version_is_marked_for_migration() -> TestResult {
    let document = sample_policy_source_document()?;
    let supported_schema_version = test_ok!(SchemaVersion::new(2), "supported schema version");

    let report = test_ok!(
        assess_policy_source_compatibility(
            &document,
            supported_schema_version,
            test_ok!(PolicyVersion::new(6), "minimum supported policy version"),
        ),
        "compatibility report"
    );

    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::Compatible
    );
    Ok(())
}

#[test]
fn future_schema_version_is_rejected_as_unsupported() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schema_version = test_ok!(SchemaVersion::new(2), "future schema version");

    let report = test_ok!(
        assess_policy_source_compatibility(
            &document,
            test_ok!(
                parent_policy_source_schema_version(),
                "supported schema version"
            ),
            test_ok!(PolicyVersion::new(6), "minimum supported policy version"),
        ),
        "compatibility report"
    );

    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::Unsupported
    );
    Ok(())
}

#[test]
fn stale_policy_version_is_marked_for_migration() -> TestResult {
    let document = sample_policy_source_document()?;

    let report = test_ok!(
        assess_policy_source_compatibility(
            &document,
            test_ok!(
                parent_policy_source_schema_version(),
                "supported schema version"
            ),
            test_ok!(PolicyVersion::new(7), "minimum supported policy version"),
        ),
        "compatibility report"
    );

    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
    Ok(())
}

#[test]
fn compatibility_input_rejects_schedule_payload_without_time_budget() -> TestResult {
    let mut payload = sample_policy_source_payload()?;
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
        "compatibility input must preserve schedule time budgets"
    );

    assert!(error.to_string().contains("time_budget"));
    Ok(())
}
