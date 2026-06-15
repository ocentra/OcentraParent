use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, latest_policy_audit_event,
    mark_parent_policy_source_document_active, parent_policy_source_schema_version,
    policy_enforcement_result_artifact, register_parent_policy_source_document,
    register_parent_policy_source_document_with_authority, rollback_parent_policy_source_document,
    supersede_parent_policy_source_document, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyEnforcementResultState,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRollbackRef,
    PolicyRuleAction, PolicyRuleId, PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode,
    PolicyScheduleBudgetCarryoverRule, PolicyScheduleBudgetResetKind,
    PolicyScheduleBudgetResetRule, PolicyScheduleClockSource, PolicyScheduleDay, PolicyScheduleId,
    PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget, PolicyScheduleWindow,
    PolicySourceActorAuthority, PolicySourceActorState, PolicySourceDocumentStatus,
    PolicySourceWriteSurface, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

fn expect_invalid_value(error: EventingError, expected_field: &'static str, expected_value: &str) {
    match error {
        EventingError::InvalidValue { field, value } => {
            assert_eq!(field, expected_field);
            assert_eq!(value, expected_value);
        }
        other => panic!("expected InvalidValue error, got {other:?}"),
    }
}

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
        policy_version: PolicyVersion::new(2).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
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
            time_budget: sample_policy_schedule_time_budget(),
        }],
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-policy-confirmed").expect("policy audit ref"),
            PolicyAuditReferenceId::parse("audit-policy-activated").expect("policy audit ref"),
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

fn sample_policy_source_authority() -> PolicySourceActorAuthority {
    PolicySourceActorAuthority {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
    }
}

fn sample_policy_rollback_ref() -> PolicyRollbackRef {
    PolicyRollbackRef {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        rolled_back_policy_version: PolicyVersion::new(2).expect("policy version"),
        restored_document_id: ParentPolicyDocumentId::parse("policy-source-household-previous")
            .expect("policy source document id"),
        restored_policy_version: PolicyVersion::new(1).expect("policy version"),
    }
}

#[test]
fn parent_can_register_versioned_policy_source_of_truth() {
    let document = sample_policy_source_document();

    let registered = register_parent_policy_source_document(None, document.clone())
        .expect("registered parent policy source document");
    let compiled = compile_domain_policy_artifact(&registered, PolicyConsumerDomain::Tracking)
        .expect("compiled domain policy artifact");
    let enforcement = policy_enforcement_result_artifact(
        &registered,
        PolicyEnforcementResultState::PendingDelivery,
    )
    .expect("policy enforcement result artifact");
    let audit = latest_policy_audit_event(&registered).expect("policy audit event");

    assert_eq!(registered.household_id, document.household_id);
    assert_eq!(registered.policy_version, document.policy_version);
    assert_eq!(registered.schedules, document.schedules);
    assert_eq!(compiled.rule_count, 1);
    assert_eq!(compiled.domain, PolicyConsumerDomain::Tracking);
    assert_eq!(compiled.source_document_id, registered.document_id);
    assert_eq!(compiled.schedules, registered.schedules);
    assert_eq!(compiled.audit_reference_ids.len(), 2);
    assert!(compiled.superseded_by_policy_version.is_none());
    assert!(compiled.rollback_ref.is_none());
    assert_eq!(enforcement.audit_reference_ids.len(), 2);
    assert_eq!(audit.audit_reference_id.as_str(), "audit-policy-activated");
}

#[test]
fn coparent_can_write_source_truth_but_child_and_support_cannot() {
    let mut coparent = sample_policy_source_document();
    coparent.actor_role = ParentPolicyActorRole::CoParent;
    coparent.actor_id = PolicyActorId::parse("actor-coparent").expect("policy actor id");

    register_parent_policy_source_document(None, coparent)
        .expect("co-parent can register parent policy source");

    for (role, actor_id, expected_role_name) in [
        (ParentPolicyActorRole::Child, "actor-child", "child"),
        (ParentPolicyActorRole::Support, "actor-support", "support"),
    ] {
        let mut document = sample_policy_source_document();
        document.actor_role = role;
        document.actor_id = PolicyActorId::parse(actor_id).expect("policy actor id");

        let error = register_parent_policy_source_document(None, document)
            .expect_err("non-parent roles cannot register parent policy source");
        assert!(error.to_string().contains(expected_role_name));
    }
}

#[test]
fn ai_preview_and_domain_cache_cannot_become_source_truth() {
    for surface in [
        PolicySourceWriteSurface::AiPreview,
        PolicySourceWriteSurface::DomainCache,
    ] {
        let mut document = sample_policy_source_document();
        document.source_surface = surface;

        let error = register_parent_policy_source_document(None, document)
            .expect_err("non-parent surfaces cannot register parent policy source");
        assert!(error.to_string().contains("policy_source.source_surface"));
    }
}

#[test]
fn resolved_policy_states_require_audit_refs() {
    let mut document = sample_policy_source_document();
    document.audit_reference_ids.clear();
    document.status = PolicySourceDocumentStatus::Active;

    let error = register_parent_policy_source_document(None, document)
        .expect_err("active policy source requires audit refs");
    assert!(error
        .to_string()
        .contains("policy_source.audit_reference_ids"));
}

#[test]
fn rules_cannot_reference_unknown_schedule_ids() {
    let mut document = sample_policy_source_document();
    document.rules[0].schedule_id =
        Some(PolicyScheduleId::parse("schedule-weekend").expect("policy schedule id"));

    let error = register_parent_policy_source_document(None, document)
        .expect_err("policy source rejects unknown schedule refs");
    assert!(error.to_string().contains("policy_source.rule.schedule_id"));
}

#[test]
fn duplicate_household_truth_for_same_version_is_rejected() {
    let existing = sample_policy_source_document();
    let mut competing = sample_policy_source_document();
    competing.document_id =
        ParentPolicyDocumentId::parse("policy-source-household-shadow").expect("document id");

    let error = register_parent_policy_source_document(Some(&existing), competing)
        .expect_err("same household version cannot have competing source documents");
    assert!(error.to_string().contains("duplicate source truth"));
}

#[test]
fn wrong_household_actor_authority_cannot_register_source_truth() {
    let document = sample_policy_source_document();
    let mut authority = sample_policy_source_authority();
    authority.household_id = PolicyHouseholdId::parse("household-other").expect("household id");

    let error = register_parent_policy_source_document_with_authority(None, document, &authority)
        .expect_err("wrong-household actor authority must be rejected");
    assert!(error.to_string().contains("policy_source.household_id"));
}

#[test]
fn mismatched_actor_authority_cannot_register_source_truth() {
    let document = sample_policy_source_document();
    let mut authority = sample_policy_source_authority();
    authority.actor_id = PolicyActorId::parse("actor-other").expect("policy actor id");

    let error = register_parent_policy_source_document_with_authority(None, document, &authority)
        .expect_err("mismatched actor authority must be rejected");
    assert!(error.to_string().contains("policy_source.actor_id"));
}

#[test]
fn mismatched_role_authority_cannot_register_source_truth() {
    let document = sample_policy_source_document();
    let mut authority = sample_policy_source_authority();
    authority.actor_role = ParentPolicyActorRole::CoParent;

    let error = register_parent_policy_source_document_with_authority(None, document, &authority)
        .expect_err("mismatched actor role authority must be rejected");
    assert!(error.to_string().contains("policy_source.actor_role"));
}

#[test]
fn revoked_actor_authority_cannot_register_source_truth() {
    let document = sample_policy_source_document();
    let mut authority = sample_policy_source_authority();
    authority.actor_state = PolicySourceActorState::Revoked;

    let error = register_parent_policy_source_document_with_authority(None, document, &authority)
        .expect_err("revoked actor authority must be rejected");
    assert!(error.to_string().contains("policy_source.actor_state"));
}

#[test]
fn active_status_requires_acknowledged_delivery_for_every_target() {
    let document = sample_policy_source_document();
    let pending_delivery = policy_enforcement_result_artifact(
        &document,
        PolicyEnforcementResultState::PendingDelivery,
    )
    .expect("pending delivery artifact");

    let error = mark_parent_policy_source_document_active(&document, &[pending_delivery])
        .expect_err("active policy must reject pre-ack delivery state");
    assert!(error.to_string().contains("acknowledged delivery"));

    let acknowledged_delivery =
        policy_enforcement_result_artifact(&document, PolicyEnforcementResultState::Acknowledged)
            .expect("acknowledged delivery artifact");

    let activated = mark_parent_policy_source_document_active(&document, &[acknowledged_delivery])
        .expect("active policy after acknowledged delivery");
    assert_eq!(activated.status, PolicySourceDocumentStatus::Active);
}

#[test]
fn superseded_status_requires_newer_replacement_version_and_new_audit_ref() {
    let document = sample_policy_source_document();

    let same_version_error = supersede_parent_policy_source_document(
        &document,
        document.policy_version,
        PolicyAuditReferenceId::parse("audit-policy-superseded").expect("policy audit ref"),
    )
    .expect_err("same-version supersede must be rejected");
    assert!(same_version_error
        .to_string()
        .contains("policy_source.superseded_by_policy_version"));

    let duplicate_audit_error = supersede_parent_policy_source_document(
        &document,
        PolicyVersion::new(3).expect("policy version"),
        PolicyAuditReferenceId::parse("audit-policy-activated").expect("policy audit ref"),
    )
    .expect_err("supersede must record a new audit reference");
    assert!(duplicate_audit_error
        .to_string()
        .contains("policy_source.audit_reference_id"));

    let superseded = supersede_parent_policy_source_document(
        &document,
        PolicyVersion::new(3).expect("policy version"),
        PolicyAuditReferenceId::parse("audit-policy-superseded").expect("policy audit ref"),
    )
    .expect("superseded policy source document");

    assert_eq!(superseded.status, PolicySourceDocumentStatus::Superseded);
    assert_eq!(superseded.policy_version, document.policy_version);
    assert_eq!(
        superseded
            .superseded_by_policy_version
            .expect("replacement policy version")
            .value(),
        3
    );
    assert_eq!(
        superseded
            .audit_reference_ids
            .last()
            .expect("supersede audit ref")
            .as_str(),
        "audit-policy-superseded"
    );
}

#[test]
fn rolled_back_status_requires_prior_version_reference_and_new_audit_ref() {
    let document = sample_policy_source_document();

    let mut invalid_ref = sample_policy_rollback_ref();
    invalid_ref.restored_policy_version = PolicyVersion::new(2).expect("policy version");

    let same_version_error = rollback_parent_policy_source_document(
        &document,
        &invalid_ref,
        PolicyAuditReferenceId::parse("audit-policy-rolled-back").expect("policy audit ref"),
    )
    .expect_err("rollback must reference an older restored policy version");
    assert!(same_version_error
        .to_string()
        .contains("policy_source.rollback_ref.restored_policy_version"));

    let duplicate_audit_error = rollback_parent_policy_source_document(
        &document,
        &sample_policy_rollback_ref(),
        PolicyAuditReferenceId::parse("audit-policy-activated").expect("policy audit ref"),
    )
    .expect_err("rollback must record a new audit reference");
    assert!(duplicate_audit_error
        .to_string()
        .contains("policy_source.audit_reference_id"));

    let rolled_back = rollback_parent_policy_source_document(
        &document,
        &sample_policy_rollback_ref(),
        PolicyAuditReferenceId::parse("audit-policy-rolled-back").expect("policy audit ref"),
    )
    .expect("rolled-back policy source document");

    assert_eq!(rolled_back.status, PolicySourceDocumentStatus::RolledBack);
    assert_eq!(rolled_back.policy_version, document.policy_version);
    assert_eq!(
        rolled_back
            .rollback_ref
            .as_ref()
            .expect("rollback reference")
            .restored_policy_version
            .value(),
        1
    );
    assert_eq!(
        rolled_back
            .audit_reference_ids
            .last()
            .expect("rollback audit ref")
            .as_str(),
        "audit-policy-rolled-back"
    );
}

#[test]
fn source_compile_helper_rejects_draft_and_preview_documents() {
    for status in [
        PolicySourceDocumentStatus::Draft,
        PolicySourceDocumentStatus::Preview,
    ] {
        let mut document = sample_policy_source_document();
        document.status = status;
        document.audit_reference_ids.clear();

        let error = compile_domain_policy_artifact(&document, PolicyConsumerDomain::Tracking)
            .expect_err("draft and preview source documents cannot compile artifacts");
        assert!(error.to_string().contains("policy_source.status"));
    }
}

#[test]
fn weekly_reset_requires_day() {
    let mut document = sample_policy_source_document();
    document.schedules[0].time_budget.reset.kind = PolicyScheduleBudgetResetKind::Weekly;
    document.schedules[0].time_budget.reset.day = None;

    let error = register_parent_policy_source_document(None, document)
        .expect_err("weekly reset must require an explicit day");

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_RESET_DAY,
        "missing-weekly-reset-day",
    );
}

#[test]
fn daily_and_monthly_resets_reject_unexpected_day() {
    for reset_kind in [
        PolicyScheduleBudgetResetKind::Daily,
        PolicyScheduleBudgetResetKind::Monthly,
    ] {
        let mut document = sample_policy_source_document();
        document.schedules[0].time_budget.reset.kind = reset_kind;
        document.schedules[0].time_budget.reset.day = Some(PolicyScheduleDay::Monday);

        let error = register_parent_policy_source_document(None, document)
            .expect_err("non-weekly resets must reject an explicit day");

        expect_invalid_value(
            error,
            policy_control::source::FIELD_SCHEDULE_RESET_DAY,
            "unexpected-reset-day",
        );
    }
}

#[test]
fn discard_unused_carryover_rejects_max_minutes() {
    let mut document = sample_policy_source_document();
    document.schedules[0].time_budget.carryover.mode =
        PolicyScheduleBudgetCarryoverMode::DiscardUnused;
    document.schedules[0].time_budget.carryover.max_minutes = Some(15);

    let error = register_parent_policy_source_document(None, document)
        .expect_err("discard-unused carryover must not accept max_minutes");

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
        "discard-unused",
    );
}

#[test]
fn cap_carryover_requires_positive_max_minutes() {
    for max_minutes in [None, Some(0)] {
        let mut document = sample_policy_source_document();
        document.schedules[0].time_budget.carryover.mode =
            PolicyScheduleBudgetCarryoverMode::CapCarryover;
        document.schedules[0].time_budget.carryover.max_minutes = max_minutes;

        let error = register_parent_policy_source_document(None, document)
            .expect_err("cap-carryover must require a positive max_minutes");

        expect_invalid_value(
            error,
            policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
            "cap-carryover",
        );
    }
}

#[test]
fn effective_until_must_be_after_effective_from() {
    let mut document = sample_policy_source_document();
    document.schedules[0].time_budget.effective_until = Some("2026-01-01T00:00:00Z".to_string());

    let error = register_parent_policy_source_document(None, document)
        .expect_err("effective_until must be strictly after effective_from");

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
        "2026-01-01T00:00:00Z",
    );
}

#[test]
fn bonus_expiry_minutes_must_be_non_zero() {
    let mut document = sample_policy_source_document();
    document.schedules[0].time_budget.bonus_expiry_minutes = 0;

    let error = register_parent_policy_source_document(None, document)
        .expect_err("bonus expiry must be non-zero");

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES,
        "0",
    );
}
