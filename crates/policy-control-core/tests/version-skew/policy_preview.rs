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

fn sample_preview_request(
    candidate_version: u64,
    current_version: Option<u64>,
    preview_acknowledged: bool,
) -> PolicyPreviewRequest {
    PolicyPreviewRequest {
        schema_version: policy_preview_schema_version().expect("policy preview schema version"),
        request_id: PolicyPreviewRequestId::parse("policy-preview-version-skew")
            .expect("policy preview request id"),
        candidate_document: sample_policy_document(
            "policy-preview-candidate",
            PolicySourceDocumentStatus::Preview,
            candidate_version,
        ),
        current_document: current_version.map(|version| {
            sample_policy_document(
                "policy-preview-current",
                PolicySourceDocumentStatus::Confirmed,
                version,
            )
        }),
        preview_acknowledged,
        target_inputs: vec![sample_target_input()],
    }
}

fn sample_policy_document(
    document_id: &str,
    status: PolicySourceDocumentStatus,
    version: u64,
) -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse(document_id).expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(version).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("policy device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-school-night-block").expect("policy rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("policy target reference"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-school-night").expect("policy schedule id"),
            ),
            priority: 100,
            reason_code: PolicyReasonCode::parse("school-night").expect("policy reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night")
                .expect("policy schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto")
                .expect("policy timezone name"),
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

fn sample_target_input() -> PolicyPreviewTargetInput {
    PolicyPreviewTargetInput {
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect("policy target reference"),
        },
        domain: PolicyConsumerDomain::App,
        state: PolicyPreviewTargetState::Supported,
        explanation_code: PolicyPreviewExplanationCode::parse("target-supported")
            .expect("preview explanation code"),
    }
}

#[test]
fn policy_preview_request_serde_rejects_zero_schema_version() {
    let mut value =
        serde_json::to_value(sample_preview_request(4, Some(4), false)).expect("preview request");
    value["schema_version"] = serde_json::json!(0);

    let error = serde_json::from_value::<PolicyPreviewRequest>(value)
        .expect_err("policy preview schema version zero must be rejected");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn stale_current_document_version_is_visible_and_blocks_save() {
    let result = preview_parent_policy_before_save(&sample_preview_request(4, Some(5), true))
        .expect("policy preview stale-source result");

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
}

#[test]
fn matching_current_document_version_stays_ready_to_save() {
    let result = preview_parent_policy_before_save(&sample_preview_request(4, Some(4), true))
        .expect("policy preview matching-source result");

    assert_eq!(result.save_state, PolicyPreviewSaveState::ReadyToSave);
    assert_eq!(
        result.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
    assert!(result.findings.is_empty());
    assert_eq!(
        result.policy_version,
        PolicyVersion::new(4).expect("policy version")
    );
}
