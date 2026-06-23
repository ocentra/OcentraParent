use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewFindingKind, PolicyPreviewTargetState, PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_authority::PolicyManualReviewState;
use ocentra_policy_control_core::policy_preview::{
    policy_preview_schema_version, preview_parent_policy_before_save, PolicyPreviewExplanationCode,
    PolicyPreviewRequest, PolicyPreviewRequestId, PolicyPreviewSaveState, PolicyPreviewTargetInput,
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

fn sample_preview_request(
    candidate_version: u64,
    current_version: Option<u64>,
    preview_acknowledged: bool,
) -> TestResult<PolicyPreviewRequest> {
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
        current_document: current_version
            .map(|version| {
                sample_policy_document(
                    "policy-preview-current",
                    PolicySourceStatus::Confirmed,
                    version,
                )
            })
            .transpose()?,
        preview_acknowledged,
        target_inputs: vec![sample_target_input()?],
    })
}

fn sample_policy_document(
    document_id: &str,
    status: PolicySourceStatus,
    version: u64,
) -> TestResult<ParentPolicySourceDocument> {
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
    })
}

fn sample_target_input() -> TestResult<PolicyPreviewTargetInput> {
    Ok(PolicyPreviewTargetInput {
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Category,
            reference_id: test_ok!(
                PolicyTargetReferenceId::parse("category-gaming"),
                "policy target reference"
            ),
        },
        domain: PolicyConsumerDomain::App,
        state: PolicyPreviewTargetState::Supported,
        explanation_code: test_ok!(
            PolicyPreviewExplanationCode::parse("target-supported"),
            "preview explanation code"
        ),
    })
}

#[test]
fn policy_preview_request_serde_rejects_zero_schema_version() -> TestResult {
    let mut value = test_ok!(
        serde_json::to_value(sample_preview_request(4, Some(4), false)?),
        "preview request"
    );
    value["schema_version"] = serde_json::json!(0);

    let error = test_err!(
        serde_json::from_value::<PolicyPreviewRequest>(value),
        "policy preview schema version zero must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn stale_current_document_version_is_visible_and_blocks_save() -> TestResult {
    let request = sample_preview_request(4, Some(5), true)?;
    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview stale-source result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::Blocked);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(result.findings.len(), 1);
    assert_eq!(
        result.findings[0].kind,
        PolicyPreviewFindingKind::StaleSourceDocument
    );
    assert_eq!(result.findings[0].rule_ids, Vec::new());
    assert_eq!(result.findings[0].schedule_ids, Vec::new());
    assert_eq!(
        result.findings[0].explanation_code.as_str(),
        "stale-policy-version"
    );
    Ok(())
}

#[test]
fn matching_current_document_version_stays_ready_to_save() -> TestResult {
    let request = sample_preview_request(4, Some(4), true)?;
    let result = test_ok!(
        preview_parent_policy_before_save(&request),
        "policy preview matching-source result"
    );

    assert_eq!(result.save_state, PolicyPreviewSaveState::ReadyToSave);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
    assert_eq!(
        result.policy_version,
        test_ok!(PolicyVersion::new(4), "policy version")
    );
    Ok(())
}
