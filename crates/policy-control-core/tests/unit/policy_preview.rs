#[path = "policy_preview_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::{
    apply_fall_back_schedule_boundary, apply_spring_forward_schedule_boundary,
    sample_preview_request, sample_target_input,
};
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewFindingKind, PolicyPreviewTargetState,
};
use ocentra_policy_control_core::policy_authority::PolicyManualReviewState;
use ocentra_policy_control_core::policy_preview::{
    preview_parent_policy_before_save, PolicyPreviewSaveState,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyRule, PolicyReasonCode, PolicyRuleAction, PolicyRuleId,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTimezoneName,
};
#[test]
fn preview_must_be_acknowledged_before_save_is_ready() -> TestResult {
    let request = sample_preview_request(false)?;
    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::PreviewRequired);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
    Ok(())
}

#[test]
fn overlapping_rules_are_reported_as_visible_conflicts() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.candidate_document.rules.push(ParentPolicyRule {
        rule_id: test_ok!(PolicyRuleId::parse("rule-bedtime-warn"), "rule id"),
        target: request.candidate_document.rules[0].target.clone(),
        action: PolicyRuleAction::Warn,
        schedule_id: Some(test_ok!(
            PolicyScheduleId::parse("schedule-bedtime"),
            "schedule id"
        )),
        priority: 90,
        reason_code: test_ok!(PolicyReasonCode::parse("bedtime"), "reason code"),
        enabled: true,
    });
    request
        .candidate_document
        .schedules
        .push(PolicyScheduleWindow {
            schedule_id: test_ok!(PolicyScheduleId::parse("schedule-bedtime"), "schedule id"),
            timezone_name: test_ok!(
                PolicyTimezoneName::parse("America/Toronto"),
                "timezone name"
            ),
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
        });

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview conflict result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::OverlappingSchedule
    );
    assert_eq!(result.findings[0].rule_ids.len(), 2);
    assert_eq!(result.findings[0].schedule_ids.len(), 2);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    Ok(())
}

#[test]
fn timezone_boundary_conflict_is_visible_before_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.candidate_document.rules.push(ParentPolicyRule {
        rule_id: test_ok!(PolicyRuleId::parse("rule-bedtime-warn"), "rule id"),
        target: request.candidate_document.rules[0].target.clone(),
        action: PolicyRuleAction::Warn,
        schedule_id: Some(test_ok!(
            PolicyScheduleId::parse("schedule-bedtime"),
            "schedule id"
        )),
        priority: 90,
        reason_code: test_ok!(PolicyReasonCode::parse("preview-warning"), "reason code"),
        enabled: true,
    });
    request
        .candidate_document
        .schedules
        .push(PolicyScheduleWindow {
            schedule_id: test_ok!(PolicyScheduleId::parse("schedule-bedtime"), "schedule id"),
            timezone_name: test_ok!(
                PolicyTimezoneName::parse("America/Vancouver"),
                "timezone name"
            ),
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
        });

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview timezone-boundary result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::TimezoneBoundary
    );
    assert_eq!(result.findings[0].schedule_ids.len(), 2);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    Ok(())
}

#[test]
fn unsupported_target_state_is_visible_and_blocks_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Unsupported,
        "unsupported-platform",
    )?];

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview unsupported result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.target_results[0].state,
        PolicyPreviewTargetState::Unsupported
    );
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::UnsupportedTarget
    );
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    Ok(())
}

#[test]
fn manual_required_target_state_stays_visible_in_preview() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::ManualRequired,
        "device-offline-manual-review",
    )?];

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview manual result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.target_results[0].state,
        PolicyPreviewTargetState::ManualRequired
    );
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::ManualRequiredTarget
    );
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    Ok(())
}

#[test]
fn offline_target_state_stays_visible_and_blocks_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Offline,
        "offline-child",
    )?];

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview offline result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.target_results[0].state,
        PolicyPreviewTargetState::Offline
    );
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::OfflineTarget
    );
    assert_eq!(
        result.findings[0].explanation_code.as_str(),
        "offline-child"
    );
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    Ok(())
}

#[test]
fn stale_target_state_stays_visible_in_preview() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Stale,
        "stale-target-snapshot",
    )?];

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview stale result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.target_results[0].state,
        PolicyPreviewTargetState::Stale
    );
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::StaleTarget
    );
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    Ok(())
}

#[test]
fn nonexistent_local_time_finding_blocks_preview_before_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    apply_spring_forward_schedule_boundary(&mut request.candidate_document.schedules[0]);

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview nonexistent-local"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::NonexistentLocalTime
    );
    assert_eq!(result.findings[0].rule_ids.len(), 1);
    assert_eq!(result.findings[0].schedule_ids.len(), 1);
    assert_eq!(
        result.findings[0].rule_ids[0].as_str(),
        "rule-school-night-block"
    );
    assert_eq!(
        result.findings[0].schedule_ids[0].as_str(),
        "schedule-school-night"
    );
    assert_eq!(
        result.findings[0].explanation_code.as_str(),
        "nonexistent-local-time"
    );
    Ok(())
}

#[test]
fn ambiguous_local_time_finding_blocks_preview_before_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    apply_fall_back_schedule_boundary(&mut request.candidate_document.schedules[0]);

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview ambiguous-local"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::AmbiguousLocalTime
    );
    assert_eq!(result.findings[0].rule_ids.len(), 1);
    assert_eq!(result.findings[0].schedule_ids.len(), 1);
    assert_eq!(
        result.findings[0].rule_ids[0].as_str(),
        "rule-school-night-block"
    );
    assert_eq!(
        result.findings[0].schedule_ids[0].as_str(),
        "schedule-school-night"
    );
    assert_eq!(
        result.findings[0].explanation_code.as_str(),
        "ambiguous-local-time"
    );
    Ok(())
}

#[test]
fn manual_clock_source_surfaces_explicit_clock_skew_before_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.candidate_document.schedules[0]
        .time_budget
        .clock_source = PolicyScheduleClockSource::ManualRequired;

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview clock-skew"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(result.findings.len(), 1);
    assert_eq!(result.findings[0].kind, PolicyPreviewFindingKind::ClockSkew);
    assert_eq!(result.findings[0].rule_ids.len(), 1);
    assert_eq!(result.findings[0].schedule_ids.len(), 1);
    assert_eq!(
        result.findings[0].rule_ids[0].as_str(),
        "rule-school-night-block"
    );
    assert_eq!(
        result.findings[0].schedule_ids[0].as_str(),
        "schedule-school-night"
    );
    assert_eq!(result.findings[0].explanation_code.as_str(), "clock-skew");
    Ok(())
}

#[test]
fn child_device_clock_source_does_not_auto_surface_clock_skew_before_save() -> TestResult {
    let mut request = sample_preview_request(true)?;
    request.candidate_document.schedules[0]
        .time_budget
        .clock_source = PolicyScheduleClockSource::ChildDevice;

    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview child-device result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::ReadyToSave);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
    Ok(())
}
