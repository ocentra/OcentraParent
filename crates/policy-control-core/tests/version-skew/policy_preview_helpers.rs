use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewTargetState, PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_preview::{
    policy_preview_schema_version, PolicyPreviewExplanationCode, PolicyPreviewRequest,
    PolicyPreviewRequestId, PolicyPreviewTargetInput,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyChildProfileId,
    PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

pub(super) fn sample_preview_request(
    candidate_version: u64,
    current_version: Option<u64>,
    preview_acknowledged: bool,
) -> TestResult<PolicyPreviewRequest> {
    let current_document = match current_version {
        Some(version) => Some(sample_policy_document(
            "policy-preview-current",
            PolicySourceStatus::Confirmed,
            version,
        )?),
        None => None,
    };

    Ok(PolicyPreviewRequest {
        schema_version: test_ok!(
            policy_preview_schema_version(),
            "policy preview schema version"
        ),
        request_id: test_ok!(
            PolicyPreviewRequestId::parse("policy-preview-version-skew"),
            "policy preview request id"
        ),
        candidate_document: sample_policy_document(
            "policy-preview-candidate",
            PolicySourceStatus::Preview,
            candidate_version,
        )?,
        current_document,
        preview_acknowledged,
        target_inputs: vec![sample_target_input("policy-preview-version-skew")?],
    })
}

pub(super) fn sample_policy_document(
    document_id: impl std::fmt::Display,
    status: PolicySourceStatus,
    version: u64,
) -> TestResult<ParentPolicySourceDocument> {
    let document_id = document_id.to_string();
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse(document_id),
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
        status,
        child_profile_ids: vec![test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        )],
        device_ids: vec![test_ok!(
            PolicyDeviceId::parse("device-laptop"),
            "policy device id"
        )],
        rules: vec![sample_policy_rule()?],
        schedules: vec![sample_schedule_window()?],
        audit_reference_ids: Vec::new(),
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    })
}

fn sample_policy_rule() -> TestResult<ParentPolicyRule> {
    Ok(ParentPolicyRule {
        rule_id: test_ok!(
            PolicyRuleId::parse("rule-school-night-block"),
            "policy rule id"
        ),
        target: sample_rule_target("category-gaming")?,
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
    })
}

fn sample_rule_target(reference_id: impl std::fmt::Display) -> TestResult<PolicyRuleTarget> {
    Ok(PolicyRuleTarget {
        kind: PolicyTargetKind::Category,
        reference_id: test_ok!(
            PolicyTargetReferenceId::parse(reference_id.to_string()),
            "policy target reference"
        ),
    })
}

fn sample_schedule_window() -> TestResult<PolicyScheduleWindow> {
    Ok(PolicyScheduleWindow {
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

pub(super) fn sample_target_input(
    explanation_code: impl std::fmt::Display,
) -> TestResult<PolicyPreviewTargetInput> {
    let explanation_code = explanation_code.to_string();
    Ok(PolicyPreviewTargetInput {
        target: sample_rule_target("category-gaming")?,
        domain: PolicyConsumerDomain::App,
        state: PolicyPreviewTargetState::Supported,
        explanation_code: test_ok!(
            PolicyPreviewExplanationCode::parse(explanation_code),
            "preview explanation code"
        ),
    })
}
