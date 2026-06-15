use ocentra_policy_control_core::policy_conflict::{
    detect_policy_conflicts, has_blocking_policy_conflicts, PolicyConflictKind,
    PolicyConflictPrecedenceState, PolicyConflictSeverity,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
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
        document_id: ParentPolicyDocumentId::parse("policy-source-conflict")
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
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("device id")],
        rules: vec![
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-app-block").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::App,
                    reference_id: PolicyTargetReferenceId::parse("app-minecraft")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::Block,
                schedule_id: Some(PolicyScheduleId::parse("schedule-night").expect("schedule id")),
                priority: 100,
                reason_code: PolicyReasonCode::parse("school-night").expect("reason code"),
                enabled: true,
            },
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-app-warn").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::App,
                    reference_id: PolicyTargetReferenceId::parse("app-minecraft")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::Warn,
                schedule_id: Some(
                    PolicyScheduleId::parse("schedule-overlap").expect("schedule id"),
                ),
                priority: 90,
                reason_code: PolicyReasonCode::parse("preview-warning").expect("reason code"),
                enabled: true,
            },
        ],
        schedules: vec![
            PolicyScheduleWindow {
                schedule_id: PolicyScheduleId::parse("schedule-night").expect("schedule id"),
                timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
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
            },
            PolicyScheduleWindow {
                schedule_id: PolicyScheduleId::parse("schedule-overlap").expect("schedule id"),
                timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
                starts_at: "22:00".to_string(),
                ends_at: "06:00".to_string(),
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
            },
        ],
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-policy-confirmed").expect("audit ref")
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

fn sample_policy_rollback_ref() -> PolicyRollbackRef {
    PolicyRollbackRef {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-conflict")
            .expect("policy source document id"),
        rolled_back_policy_version: PolicyVersion::new(3).expect("policy version"),
        restored_document_id: ParentPolicyDocumentId::parse("policy-source-conflict-previous")
            .expect("policy source document id"),
        restored_policy_version: PolicyVersion::new(2).expect("policy version"),
    }
}

fn apply_spring_forward_schedule_boundary(schedule: &mut PolicyScheduleWindow) {
    schedule.starts_at = "02:15".to_string();
    schedule.ends_at = "03:30".to_string();
    schedule.time_budget.reset.local_time = "02:00".to_string();
    schedule.time_budget.effective_from = "2026-03-08T06:45:00Z".to_string();
    schedule.time_budget.effective_until = Some("2026-03-08T08:30:00Z".to_string());
}

fn apply_fall_back_schedule_boundary(schedule: &mut PolicyScheduleWindow) {
    schedule.starts_at = "01:30".to_string();
    schedule.ends_at = "01:45".to_string();
    schedule.time_budget.reset.local_time = "01:00".to_string();
    schedule.time_budget.effective_from = "2026-11-01T04:15:00Z".to_string();
    schedule.time_budget.effective_until = Some("2026-11-01T07:45:00Z".to_string());
}

fn assert_conflict_tracks_source_context(
    conflict: &ocentra_policy_control_core::policy_conflict::PolicyConflictRecord,
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

#[test]
fn higher_priority_rule_wins_for_overlapping_target_actions() {
    let document = sample_policy_source_document();

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::OverlappingActions);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::HigherPriorityWins
    );
    assert_eq!(
        conflicts[0].severity,
        PolicyConflictSeverity::ResolvedVisible
    );
    assert_eq!(
        conflicts[0]
            .winning_rule_id
            .as_ref()
            .expect("winning rule")
            .as_str(),
        "rule-app-block"
    );
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(!has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn equal_priority_overlap_requires_manual_review() {
    let mut document = sample_policy_source_document();
    document.rules[1].priority = document.rules[0].priority;

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::EqualPriority);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn timezone_mismatch_conflict_stays_explicit_and_manual_required() {
    let mut document = sample_policy_source_document();
    document.schedules[1].timezone_name =
        PolicyTimezoneName::parse("America/Vancouver").expect("timezone");

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::TimezoneBoundary);
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].schedule_ids.len(), 2);
    assert!(conflicts[0].winning_rule_id.is_none());
    assert!(conflicts[0].losing_rule_id.is_none());
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn device_targets_missing_from_household_inventory_are_blocking() {
    let mut document = sample_policy_source_document();
    document.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-device-curfew").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Device,
            reference_id: PolicyTargetReferenceId::parse("device-tablet").expect("target ref"),
        },
        action: PolicyRuleAction::Block,
        schedule_id: Some(PolicyScheduleId::parse("schedule-night").expect("schedule id")),
        priority: 110,
        reason_code: PolicyReasonCode::parse("device-curfew").expect("reason code"),
        enabled: true,
    });

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 2);
    assert!(conflicts.iter().any(|conflict| {
        conflict.kind == PolicyConflictKind::UnknownDeviceTarget
            && conflict.severity == PolicyConflictSeverity::Blocking
    }));
    conflicts
        .iter()
        .for_each(|conflict| assert_conflict_tracks_source_context(conflict, &document));
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn rolled_back_source_conflicts_preserve_rollback_context() {
    let mut document = sample_policy_source_document();
    document.status = PolicySourceDocumentStatus::RolledBack;
    document.rollback_ref = Some(sample_policy_rollback_ref());

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert_eq!(
        conflicts[0]
            .rollback_ref
            .as_ref()
            .expect("rollback ref")
            .restored_policy_version,
        PolicyVersion::new(2).expect("policy version")
    );
}

#[test]
fn disabled_rule_does_not_create_conflict_noise() {
    let mut document = sample_policy_source_document();
    document.rules[1].enabled = false;

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert!(conflicts.is_empty());
}

#[test]
fn nonexistent_local_time_stays_explicit_and_blocking() {
    let mut document = sample_policy_source_document();
    document.rules[1].enabled = false;
    apply_spring_forward_schedule_boundary(&mut document.schedules[0]);

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::NonexistentLocalTime);
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].schedule_ids.len(), 1);
    assert_eq!(conflicts[0].schedule_ids[0].as_str(), "schedule-night");
    assert!(conflicts[0].winning_rule_id.is_none());
    assert!(conflicts[0].losing_rule_id.is_none());
    assert_eq!(conflicts[0].reason_code.as_str(), "nonexistent-local-time");
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn ambiguous_local_time_stays_explicit_and_blocking() {
    let mut document = sample_policy_source_document();
    document.rules[1].enabled = false;
    apply_fall_back_schedule_boundary(&mut document.schedules[0]);

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::AmbiguousLocalTime);
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].schedule_ids.len(), 1);
    assert_eq!(conflicts[0].schedule_ids[0].as_str(), "schedule-night");
    assert!(conflicts[0].winning_rule_id.is_none());
    assert!(conflicts[0].losing_rule_id.is_none());
    assert_eq!(conflicts[0].reason_code.as_str(), "ambiguous-local-time");
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn manual_clock_source_stays_explicit_clock_skew_conflict() {
    let mut document = sample_policy_source_document();
    document.rules[1].enabled = false;
    document.schedules[0].time_budget.clock_source = PolicyScheduleClockSource::ManualRequired;

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::ClockSkew);
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].schedule_ids.len(), 1);
    assert_eq!(conflicts[0].schedule_ids[0].as_str(), "schedule-night");
    assert!(conflicts[0].winning_rule_id.is_none());
    assert!(conflicts[0].losing_rule_id.is_none());
    assert_eq!(conflicts[0].reason_code.as_str(), "clock-skew");
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
}

#[test]
fn child_device_clock_source_does_not_auto_create_clock_skew_conflict() {
    let mut document = sample_policy_source_document();
    document.rules[1].enabled = false;
    document.schedules[0].time_budget.clock_source = PolicyScheduleClockSource::ChildDevice;

    let conflicts = detect_policy_conflicts(&document).expect("policy conflicts");

    assert!(conflicts.is_empty());
    assert!(!has_blocking_policy_conflicts(&conflicts));
}
