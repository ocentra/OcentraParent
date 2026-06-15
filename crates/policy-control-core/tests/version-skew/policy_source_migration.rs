use ocentra_policy_control_core::policy_source::{
    assess_policy_source_compatibility, parent_policy_source_schema_version, ParentPolicyActorRole,
    ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId, PolicyDocumentCompatibilityState,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

use ocentra_eventing::SchemaVersion;
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

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(6).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-school-night-block").expect("policy rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("target reference"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 100,
            reason_code: PolicyReasonCode::parse("school-night").expect("reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
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

fn sample_policy_source_payload() -> Value {
    serde_json::to_value(sample_policy_source_document()).expect("policy source payload")
}

#[test]
fn older_schema_version_is_marked_for_migration() {
    let document = sample_policy_source_document();
    let supported_schema_version = SchemaVersion::new(2).expect("supported schema version");

    let report = assess_policy_source_compatibility(
        &document,
        supported_schema_version,
        PolicyVersion::new(6).expect("minimum supported policy version"),
    )
    .expect("compatibility report");

    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::Compatible
    );
}

#[test]
fn future_schema_version_is_rejected_as_unsupported() {
    let mut document = sample_policy_source_document();
    document.schema_version = SchemaVersion::new(2).expect("future schema version");

    let report = assess_policy_source_compatibility(
        &document,
        parent_policy_source_schema_version().expect("supported schema version"),
        PolicyVersion::new(6).expect("minimum supported policy version"),
    )
    .expect("compatibility report");

    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::Unsupported
    );
}

#[test]
fn stale_policy_version_is_marked_for_migration() {
    let document = sample_policy_source_document();

    let report = assess_policy_source_compatibility(
        &document,
        parent_policy_source_schema_version().expect("supported schema version"),
        PolicyVersion::new(7).expect("minimum supported policy version"),
    )
    .expect("compatibility report");

    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
}

#[test]
fn compatibility_input_rejects_schedule_payload_without_time_budget() {
    let mut payload = sample_policy_source_payload();
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
        .expect_err("compatibility input must preserve schedule time budgets");

    assert!(error.to_string().contains("time_budget"));
}
