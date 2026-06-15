use ocentra_policy_control_core::policy_authority::PolicyManualReviewState;
use ocentra_policy_control_core::policy_preview::{
    policy_preview_schema_version, preview_parent_policy_before_save, PolicyPreviewExplanationCode,
    PolicyPreviewFindingKind, PolicyPreviewRequest, PolicyPreviewRequestId, PolicyPreviewSaveState,
    PolicyPreviewTargetInput, PolicyPreviewTargetState,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyChildProfileId,
    PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

fn sample_preview_request(preview_acknowledged: bool) -> PolicyPreviewRequest {
    PolicyPreviewRequest {
        schema_version: policy_preview_schema_version().expect("policy preview schema version"),
        request_id: PolicyPreviewRequestId::parse("policy-preview-default")
            .expect("policy preview request id"),
        candidate_document: sample_policy_document(),
        current_document: None,
        preview_acknowledged,
        target_inputs: vec![sample_target_input(
            PolicyPreviewTargetState::Supported,
            "target-supported",
        )],
    }
}

fn sample_policy_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-draft-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(3).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Draft,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-school-night-block").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("target reference id"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-school-night").expect("schedule id"),
            ),
            priority: 100,
            reason_code: PolicyReasonCode::parse("school-night").expect("reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night").expect("schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone name"),
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
        audit_reference_ids: Vec::new(),
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    }
}

fn sample_target_input(
    state: PolicyPreviewTargetState,
    explanation_code: &str,
) -> PolicyPreviewTargetInput {
    PolicyPreviewTargetInput {
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect("target reference id"),
        },
        domain: PolicyConsumerDomain::App,
        state,
        explanation_code: PolicyPreviewExplanationCode::parse(explanation_code)
            .expect("preview explanation code"),
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

#[test]
fn preview_must_be_acknowledged_before_save_is_ready() {
    let result = preview_parent_policy_before_save(&sample_preview_request(false))
        .expect("policy preview result");

    assert_eq!(result.save_state, PolicyPreviewSaveState::PreviewRequired);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
}

#[test]
fn overlapping_rules_are_reported_as_visible_conflicts() {
    let mut request = sample_preview_request(true);
    request.candidate_document.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-bedtime-warn").expect("rule id"),
        target: request.candidate_document.rules[0].target.clone(),
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-bedtime").expect("schedule id")),
        priority: 90,
        reason_code: PolicyReasonCode::parse("bedtime").expect("reason code"),
        enabled: true,
    });
    request
        .candidate_document
        .schedules
        .push(PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-bedtime").expect("schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone name"),
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

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview conflict result");

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
}

#[test]
fn timezone_boundary_conflict_is_visible_before_save() {
    let mut request = sample_preview_request(true);
    request.candidate_document.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-bedtime-warn").expect("rule id"),
        target: request.candidate_document.rules[0].target.clone(),
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-bedtime").expect("schedule id")),
        priority: 90,
        reason_code: PolicyReasonCode::parse("preview-warning").expect("reason code"),
        enabled: true,
    });
    request
        .candidate_document
        .schedules
        .push(PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-bedtime").expect("schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Vancouver").expect("timezone name"),
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

    let result = preview_parent_policy_before_save(&request)
        .expect("policy preview timezone-boundary result");

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
}

#[test]
fn unsupported_target_state_is_visible_and_blocks_save() {
    let mut request = sample_preview_request(true);
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Unsupported,
        "unsupported-platform",
    )];

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview unsupported result");

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
}

#[test]
fn manual_required_target_state_stays_visible_in_preview() {
    let mut request = sample_preview_request(true);
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::ManualRequired,
        "device-offline-manual-review",
    )];

    let result = preview_parent_policy_before_save(&request).expect("policy preview manual result");

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
}

#[test]
fn offline_target_state_stays_visible_and_blocks_save() {
    let mut request = sample_preview_request(true);
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Offline,
        "offline-child",
    )];

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview offline result");

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
}

#[test]
fn stale_target_state_stays_visible_in_preview() {
    let mut request = sample_preview_request(true);
    request.target_inputs = vec![sample_target_input(
        PolicyPreviewTargetState::Stale,
        "stale-target-snapshot",
    )];

    let result = preview_parent_policy_before_save(&request).expect("policy preview stale result");

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
}

#[test]
fn nonexistent_local_time_finding_blocks_preview_before_save() {
    let mut request = sample_preview_request(true);
    apply_spring_forward_schedule_boundary(&mut request.candidate_document.schedules[0]);

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview nonexistent-local");

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
}

#[test]
fn ambiguous_local_time_finding_blocks_preview_before_save() {
    let mut request = sample_preview_request(true);
    apply_fall_back_schedule_boundary(&mut request.candidate_document.schedules[0]);

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview ambiguous-local");

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
}

#[test]
fn manual_clock_source_surfaces_explicit_clock_skew_before_save() {
    let mut request = sample_preview_request(true);
    request.candidate_document.schedules[0]
        .time_budget
        .clock_source = PolicyScheduleClockSource::ManualRequired;

    let result = preview_parent_policy_before_save(&request).expect("policy preview clock-skew");

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
}

#[test]
fn child_device_clock_source_does_not_auto_surface_clock_skew_before_save() {
    let mut request = sample_preview_request(true);
    request.candidate_document.schedules[0]
        .time_budget
        .clock_source = PolicyScheduleClockSource::ChildDevice;

    let result =
        preview_parent_policy_before_save(&request).expect("policy preview child-device result");

    assert_eq!(result.save_state, PolicyPreviewSaveState::ReadyToSave);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
}
