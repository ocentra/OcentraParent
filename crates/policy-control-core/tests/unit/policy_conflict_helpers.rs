use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_conflict::PolicyConflictRecord;
use ocentra_policy_control_core::policy_conflict::{detect_policy_conflicts, PolicyConflictKind};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

pub(super) fn sample_policy_source_document() -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-conflict"),
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
            "device id"
        )],
        rules: vec![
            sample_policy_rule(
                "rule-app-block",
                "schedule-night",
                100,
                "school-night",
                PolicyRuleAction::Block,
            )?,
            sample_policy_rule(
                "rule-app-warn",
                "schedule-overlap",
                90,
                "preview-warning",
                PolicyRuleAction::Warn,
            )?,
        ],
        schedules: vec![
            sample_schedule_window("schedule-night", "21:00", "07:00")?,
            sample_schedule_window("schedule-overlap", "22:00", "06:00")?,
        ],
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-confirmed"),
            "audit ref"
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

fn sample_policy_rule(
    rule_id: impl std::fmt::Display,
    schedule_id: impl std::fmt::Display,
    priority: u16,
    reason_code: impl std::fmt::Display,
    action: PolicyRuleAction,
) -> TestResult<ParentPolicyRule> {
    Ok(ParentPolicyRule {
        rule_id: test_ok!(PolicyRuleId::parse(rule_id.to_string()), "rule id"),
        target: sample_rule_target("app-minecraft")?,
        action,
        schedule_id: Some(test_ok!(
            PolicyScheduleId::parse(schedule_id.to_string()),
            "schedule id"
        )),
        priority,
        reason_code: test_ok!(
            PolicyReasonCode::parse(reason_code.to_string()),
            "reason code"
        ),
        enabled: true,
    })
}

fn sample_rule_target(reference_id: impl std::fmt::Display) -> TestResult<PolicyRuleTarget> {
    Ok(PolicyRuleTarget {
        kind: PolicyTargetKind::App,
        reference_id: test_ok!(
            PolicyTargetReferenceId::parse(reference_id.to_string()),
            "target ref"
        ),
    })
}

fn sample_schedule_window(
    schedule_id: impl std::fmt::Display,
    starts_at: impl std::fmt::Display,
    ends_at: impl std::fmt::Display,
) -> TestResult<PolicyScheduleWindow> {
    Ok(PolicyScheduleWindow {
        schedule_id: test_ok!(
            PolicyScheduleId::parse(schedule_id.to_string()),
            "schedule id"
        ),
        timezone_name: test_ok!(PolicyTimezoneName::parse("America/Toronto"), "timezone"),
        starts_at: starts_at.to_string(),
        ends_at: ends_at.to_string(),
        time_budget: sample_time_budget(),
    })
}

fn sample_time_budget() -> PolicyScheduleTimeBudget {
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

pub(super) fn sample_policy_rollback_ref() -> TestResult<PolicyRollbackRef> {
    Ok(PolicyRollbackRef {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        rolled_back_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-conflict"),
            "policy source document id"
        ),
        rolled_back_policy_version: test_ok!(PolicyVersion::new(3), "policy version"),
        restored_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-conflict-previous"),
            "policy source document id"
        ),
        restored_policy_version: test_ok!(PolicyVersion::new(2), "policy version"),
    })
}

pub(super) fn apply_spring_forward_schedule_boundary(schedule: &mut PolicyScheduleWindow) {
    schedule.starts_at = "02:15".to_string();
    schedule.ends_at = "03:30".to_string();
    schedule.time_budget.reset.local_time = "02:00".to_string();
    schedule.time_budget.effective_from = "2026-03-08T06:45:00Z".to_string();
    schedule.time_budget.effective_until = Some("2026-03-08T08:30:00Z".to_string());
}

pub(super) fn apply_fall_back_schedule_boundary(schedule: &mut PolicyScheduleWindow) {
    schedule.starts_at = "01:30".to_string();
    schedule.ends_at = "01:45".to_string();
    schedule.time_budget.reset.local_time = "01:00".to_string();
    schedule.time_budget.effective_from = "2026-11-01T04:15:00Z".to_string();
    schedule.time_budget.effective_until = Some("2026-11-01T07:45:00Z".to_string());
}

pub(super) fn assert_conflict_tracks_source_context(
    conflict: &PolicyConflictRecord,
    document: &ParentPolicySourceDocument,
) {
    assert_eq!(conflict.source_document_id, document.document_id);
    assert_eq!(conflict.source_policy_version, document.policy_version);
    assert_eq!(conflict.audit_reference_ids, document.audit_reference_ids);
    assert_eq!(
        conflict.superseded_by_policy_version,
        document.superseded_by_policy_version
    );
    assert_eq!(conflict.rollback_ref, document.rollback_ref);
}

pub(super) fn assert_all_conflicts_track_source_context(
    conflicts: &[PolicyConflictRecord],
    document: &ParentPolicySourceDocument,
) {
    for conflict in conflicts {
        assert_conflict_tracks_source_context(conflict, document);
    }
}

#[test]
fn conflict_fixture_exercises_a_real_cross_midnight_overlap() -> TestResult {
    let document = sample_policy_source_document()?;
    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::OverlappingActions);
    assert_eq!(conflicts[0].schedule_ids.len(), 2);
    assert_all_conflicts_track_source_context(&conflicts, &document);
    Ok(())
}
