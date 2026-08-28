#[path = "policy_conflict_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::{
    apply_fall_back_schedule_boundary, apply_spring_forward_schedule_boundary,
    assert_all_conflicts_track_source_context, assert_conflict_tracks_source_context,
    sample_policy_rollback_ref, sample_policy_source_document,
};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceStatus;
use ocentra_policy_control_core::policy_conflict::{
    detect_policy_conflicts, has_blocking_policy_conflicts, PolicyConflictKind,
    PolicyConflictPrecedenceState, PolicyConflictSeverity,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyRule, PolicyReasonCode, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleClockSource, PolicyScheduleId, PolicyTargetKind, PolicyTargetReferenceId,
    PolicyTimezoneName, PolicyVersion,
};
#[test]
fn higher_priority_rule_wins_for_overlapping_target_actions() -> TestResult {
    let document = sample_policy_source_document()?;

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

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
        test_some!(conflicts[0].winning_rule_id.as_ref(), "winning rule").as_str(),
        "rule-app-block"
    );
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(!has_blocking_policy_conflicts(&conflicts));
    Ok(())
}

#[test]
fn equal_priority_overlap_requires_manual_review() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].priority = document.rules[0].priority;

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::EqualPriority);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
    Ok(())
}

#[test]
fn timezone_mismatch_conflict_stays_explicit_and_manual_required() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schedules[1].timezone_name =
        test_ok!(PolicyTimezoneName::parse("America/Vancouver"), "timezone");

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

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
    Ok(())
}

#[test]
fn unsupported_timezone_stays_blocking_until_a_timezone_owner_is_available() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;
    document.schedules[0].timezone_name =
        test_ok!(PolicyTimezoneName::parse("Europe/London"), "timezone");

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].kind, PolicyConflictKind::TimezoneBoundary);
    assert_eq!(conflicts[0].severity, PolicyConflictSeverity::Blocking);
    assert_eq!(
        conflicts[0].precedence_state,
        PolicyConflictPrecedenceState::ManualRequired
    );
    assert_eq!(conflicts[0].reason_code.as_str(), "timezone-boundary");
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
    Ok(())
}

#[test]
fn device_targets_missing_from_household_inventory_are_blocking() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules.push(ParentPolicyRule {
        rule_id: test_ok!(PolicyRuleId::parse("rule-device-curfew"), "rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Device,
            reference_id: test_ok!(
                PolicyTargetReferenceId::parse("device-tablet"),
                "target ref"
            ),
        },
        action: PolicyRuleAction::Block,
        schedule_id: Some(test_ok!(
            PolicyScheduleId::parse("schedule-night"),
            "schedule id"
        )),
        priority: 110,
        reason_code: test_ok!(PolicyReasonCode::parse("device-curfew"), "reason code"),
        enabled: true,
    });

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert_eq!(conflicts.len(), 2);
    assert!(conflicts.iter().any(|conflict| {
        conflict.kind == PolicyConflictKind::UnknownDeviceTarget
            && conflict.severity == PolicyConflictSeverity::Blocking
    }));
    assert_all_conflicts_track_source_context(&conflicts, &document);
    assert!(has_blocking_policy_conflicts(&conflicts));
    Ok(())
}

#[test]
fn rolled_back_source_conflicts_preserve_rollback_context() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.status = PolicySourceStatus::RolledBack;
    document.rollback_ref = Some(sample_policy_rollback_ref()?);

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert_eq!(conflicts.len(), 1);
    assert_conflict_tracks_source_context(&conflicts[0], &document);
    assert_eq!(
        test_some!(conflicts[0].rollback_ref.as_ref(), "rollback ref").restored_policy_version,
        test_ok!(PolicyVersion::new(2), "policy version")
    );
    Ok(())
}

#[test]
fn disabled_rule_does_not_create_conflict_noise() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert!(conflicts.is_empty());
    Ok(())
}

#[test]
fn nonexistent_local_time_stays_explicit_and_blocking() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;
    apply_spring_forward_schedule_boundary(&mut document.schedules[0]);

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

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
    Ok(())
}

#[test]
fn ambiguous_local_time_stays_explicit_and_blocking() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;
    apply_fall_back_schedule_boundary(&mut document.schedules[0]);

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

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
    Ok(())
}

#[test]
fn manual_clock_source_stays_explicit_clock_skew_conflict() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;
    document.schedules[0].time_budget.clock_source = PolicyScheduleClockSource::ManualRequired;

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

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
    Ok(())
}

#[test]
fn child_device_clock_source_does_not_auto_create_clock_skew_conflict() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[1].enabled = false;
    document.schedules[0].time_budget.clock_source = PolicyScheduleClockSource::ChildDevice;

    let conflicts = test_ok!(detect_policy_conflicts(&document), "policy conflicts");

    assert!(conflicts.is_empty());
    assert!(!has_blocking_policy_conflicts(&conflicts));
    Ok(())
}
