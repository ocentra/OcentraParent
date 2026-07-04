use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_agent_protocol::constants::policy_control;
use ocentra_policy_control_core::policy_source::{
    assess_policy_source_compatibility, compile_domain_policy_artifact, latest_policy_audit_event,
    mark_parent_policy_source_document_active, parent_policy_source_schema_version,
    policy_enforcement_result_artifact, register_parent_policy_source_document,
    register_parent_policy_source_document_with_authority, rollback_parent_policy_source_document,
    supersede_parent_policy_source_document, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyDocumentCompatibilityState,
    PolicyEnforcementResultState, PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata,
    PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleDay, PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceActorAuthority, PolicySourceActorState, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

fn expect_invalid_value(
    error: EventingError,
    expected_field: impl std::fmt::Display,
    expected_value: impl std::fmt::Display,
) -> TestResult {
    let expected_field = expected_field.to_string();
    let expected_value = expected_value.to_string();
    match error {
        EventingError::InvalidValue { field, value } => {
            assert_eq!(field, expected_field);
            assert_eq!(value, expected_value);
            Ok(())
        }
        other => {
            Err(std::io::Error::other(format!("expected InvalidValue error, got {other:?}")).into())
        }
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

fn sample_policy_source_document() -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id"
        ),
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        policy_version: test_ok!(PolicyVersion::new(2), "policy version"),
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
            time_budget: sample_policy_schedule_time_budget(),
        }],
        audit_reference_ids: vec![
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-confirmed"),
                "policy audit ref"
            ),
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-activated"),
                "policy audit ref"
            ),
        ],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    })
}

fn sample_policy_source_authority() -> TestResult<PolicySourceActorAuthority> {
    Ok(PolicySourceActorAuthority {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
    })
}

fn sample_policy_rollback_ref() -> TestResult<PolicyRollbackRef> {
    Ok(PolicyRollbackRef {
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        rolled_back_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id"
        ),
        rolled_back_policy_version: test_ok!(PolicyVersion::new(2), "policy version"),
        restored_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-household-previous"),
            "policy source document id"
        ),
        restored_policy_version: test_ok!(PolicyVersion::new(1), "policy version"),
    })
}

#[test]
fn source_compatibility_reports_equal_schema_and_policy_versions_as_compatible() -> TestResult {
    let source = sample_policy_source_document()?;

    let report = test_ok!(
        assess_policy_source_compatibility(&source, source.schema_version, source.policy_version),
        "compatible policy source compatibility report"
    );

    assert_eq!(report.source_schema_version, source.schema_version);
    assert_eq!(report.supported_schema_version, source.schema_version);
    assert_eq!(report.source_policy_version, source.policy_version);
    assert_eq!(
        report.minimum_supported_policy_version,
        source.policy_version
    );
    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::Compatible
    );
    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::Compatible
    );
    Ok(())
}

#[test]
fn source_compatibility_requires_migration_for_older_schema_and_policy_versions() -> TestResult {
    let source = sample_policy_source_document()?;
    let supported_schema_version = test_ok!(
        SchemaVersion::new(source.schema_version.value() + 1),
        "supported schema version"
    );
    let minimum_supported_policy_version = test_ok!(
        PolicyVersion::new(source.policy_version.value() + 1),
        "minimum supported policy version"
    );

    let report = test_ok!(
        assess_policy_source_compatibility(
            &source,
            supported_schema_version,
            minimum_supported_policy_version,
        ),
        "migration-required policy source compatibility report"
    );

    assert_eq!(report.source_schema_version, source.schema_version);
    assert_eq!(report.supported_schema_version, supported_schema_version);
    assert_eq!(report.source_policy_version, source.policy_version);
    assert_eq!(
        report.minimum_supported_policy_version,
        minimum_supported_policy_version
    );
    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::MigrationRequired
    );
    Ok(())
}

#[test]
fn source_compatibility_marks_newer_schema_as_unsupported_and_policy_version_as_compatible(
) -> TestResult {
    let mut source = sample_policy_source_document()?;
    source.schema_version = test_ok!(
        SchemaVersion::new(source.schema_version.value() + 1),
        "newer source schema version"
    );
    let supported_schema_version = test_ok!(
        parent_policy_source_schema_version(),
        "supported policy source schema version"
    );

    let report = test_ok!(
        assess_policy_source_compatibility(
            &source,
            supported_schema_version,
            source.policy_version,
        ),
        "unsupported schema policy source compatibility report"
    );

    assert_eq!(report.source_schema_version, source.schema_version);
    assert_eq!(report.supported_schema_version, supported_schema_version);
    assert_eq!(report.source_policy_version, source.policy_version);
    assert_eq!(
        report.minimum_supported_policy_version,
        source.policy_version
    );
    assert_eq!(
        report.schema_state,
        PolicyDocumentCompatibilityState::Unsupported
    );
    assert_eq!(
        report.policy_version_state,
        PolicyDocumentCompatibilityState::Compatible
    );
    Ok(())
}

#[test]
fn parent_can_register_versioned_policy_source_of_truth() -> TestResult {
    let document = sample_policy_source_document()?;

    let registered = test_ok!(
        register_parent_policy_source_document(None, document.clone()),
        "registered parent policy source document"
    );
    let compiled = test_ok!(
        compile_domain_policy_artifact(&registered, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );
    let enforcement = test_ok!(
        policy_enforcement_result_artifact(
            &registered,
            PolicyEnforcementResultState::PendingDelivery,
        ),
        "policy enforcement result artifact"
    );
    let audit = test_ok!(latest_policy_audit_event(&registered), "policy audit event");

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
    Ok(())
}

#[test]
fn coparent_can_write_source_truth_but_child_and_support_cannot() -> TestResult {
    let mut coparent = sample_policy_source_document()?;
    coparent.actor_role = ParentPolicyActorRole::CoParent;
    coparent.actor_id = test_ok!(PolicyActorId::parse("actor-coparent"), "policy actor id");

    test_ok!(
        register_parent_policy_source_document(None, coparent),
        "co-parent can register parent policy source"
    );

    for (role, actor_id, expected_role_name) in [
        (ParentPolicyActorRole::Child, "actor-child", "child"),
        (ParentPolicyActorRole::Support, "actor-support", "support"),
    ] {
        let mut document = sample_policy_source_document()?;
        document.actor_role = role;
        document.actor_id = test_ok!(PolicyActorId::parse(actor_id), "policy actor id");

        let error = test_err!(
            register_parent_policy_source_document(None, document),
            "non-parent roles cannot register parent policy source"
        );
        assert!(error.to_string().contains(expected_role_name));
    }
    Ok(())
}

#[test]
fn ai_preview_and_domain_cache_cannot_become_source_truth() -> TestResult {
    for surface in [
        PolicySourceSurface::AiPreview,
        PolicySourceSurface::DomainCache,
    ] {
        let mut document = sample_policy_source_document()?;
        document.source_surface = surface;

        let error = test_err!(
            register_parent_policy_source_document(None, document),
            "non-parent surfaces cannot register parent policy source"
        );
        assert!(error.to_string().contains("policy_source.source_surface"));
    }
    Ok(())
}

#[test]
fn resolved_policy_states_require_audit_refs() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.audit_reference_ids.clear();
    document.status = PolicySourceStatus::Active;

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "active policy source requires audit refs"
    );
    assert!(error
        .to_string()
        .contains("policy_source.audit_reference_ids"));
    Ok(())
}

#[test]
fn rules_cannot_reference_unknown_schedule_ids() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.rules[0].schedule_id = Some(test_ok!(
        PolicyScheduleId::parse("schedule-weekend"),
        "policy schedule id"
    ));

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "policy source rejects unknown schedule refs"
    );
    assert!(error.to_string().contains("policy_source.rule.schedule_id"));
    Ok(())
}

#[test]
fn duplicate_household_truth_for_same_version_is_rejected() -> TestResult {
    let existing = sample_policy_source_document()?;
    let mut competing = sample_policy_source_document()?;
    competing.document_id = test_ok!(
        ParentPolicyDocumentId::parse("policy-source-household-shadow"),
        "document id"
    );

    let error = test_err!(
        register_parent_policy_source_document(Some(&existing), competing),
        "same household version cannot have competing source documents"
    );
    assert!(error.to_string().contains("duplicate source truth"));
    Ok(())
}

#[test]
fn wrong_household_actor_authority_cannot_register_source_truth() -> TestResult {
    let document = sample_policy_source_document()?;
    let mut authority = sample_policy_source_authority()?;
    authority.household_id = test_ok!(PolicyHouseholdId::parse("household-other"), "household id");

    let error = test_err!(
        register_parent_policy_source_document_with_authority(None, document, &authority),
        "wrong-household actor authority must be rejected"
    );
    assert!(error.to_string().contains("policy_source.household_id"));
    Ok(())
}

#[test]
fn mismatched_actor_authority_cannot_register_source_truth() -> TestResult {
    let document = sample_policy_source_document()?;
    let mut authority = sample_policy_source_authority()?;
    authority.actor_id = test_ok!(PolicyActorId::parse("actor-other"), "policy actor id");

    let error = test_err!(
        register_parent_policy_source_document_with_authority(None, document, &authority),
        "mismatched actor authority must be rejected"
    );
    assert!(error.to_string().contains("policy_source.actor_id"));
    Ok(())
}

#[test]
fn mismatched_role_authority_cannot_register_source_truth() -> TestResult {
    let document = sample_policy_source_document()?;
    let mut authority = sample_policy_source_authority()?;
    authority.actor_role = ParentPolicyActorRole::CoParent;

    let error = test_err!(
        register_parent_policy_source_document_with_authority(None, document, &authority),
        "mismatched actor role authority must be rejected"
    );
    assert!(error.to_string().contains("policy_source.actor_role"));
    Ok(())
}

#[test]
fn revoked_actor_authority_cannot_register_source_truth() -> TestResult {
    let document = sample_policy_source_document()?;
    let mut authority = sample_policy_source_authority()?;
    authority.actor_state = PolicySourceActorState::Revoked;

    let error = test_err!(
        register_parent_policy_source_document_with_authority(None, document, &authority),
        "revoked actor authority must be rejected"
    );
    assert!(error.to_string().contains("policy_source.actor_state"));
    Ok(())
}

#[test]
fn active_status_requires_acknowledged_delivery_for_every_target() -> TestResult {
    let document = sample_policy_source_document()?;
    let pending_delivery = test_ok!(
        policy_enforcement_result_artifact(
            &document,
            PolicyEnforcementResultState::PendingDelivery,
        ),
        "pending delivery artifact"
    );

    let error = test_err!(
        mark_parent_policy_source_document_active(&document, &[pending_delivery]),
        "active policy must reject pre-ack delivery state"
    );
    assert!(error.to_string().contains("acknowledged delivery"));

    let acknowledged_delivery = test_ok!(
        policy_enforcement_result_artifact(&document, PolicyEnforcementResultState::Acknowledged,),
        "acknowledged delivery artifact"
    );

    let activated = test_ok!(
        mark_parent_policy_source_document_active(&document, &[acknowledged_delivery]),
        "active policy after acknowledged delivery"
    );
    assert_eq!(activated.status, PolicySourceStatus::Active);
    Ok(())
}

#[test]
fn superseded_status_requires_newer_replacement_version_and_new_audit_ref() -> TestResult {
    let document = sample_policy_source_document()?;

    let same_version_error = test_err!(
        supersede_parent_policy_source_document(
            &document,
            document.policy_version,
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-superseded"),
                "policy audit ref"
            ),
        ),
        "same-version supersede must be rejected"
    );
    assert!(same_version_error
        .to_string()
        .contains("policy_source.superseded_by_policy_version"));

    let duplicate_audit_error = test_err!(
        supersede_parent_policy_source_document(
            &document,
            test_ok!(PolicyVersion::new(3), "policy version"),
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-activated"),
                "policy audit ref"
            ),
        ),
        "supersede must record a new audit reference"
    );
    assert!(duplicate_audit_error
        .to_string()
        .contains("policy_source.audit_reference_id"));

    let superseded = test_ok!(
        supersede_parent_policy_source_document(
            &document,
            test_ok!(PolicyVersion::new(3), "policy version"),
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-superseded"),
                "policy audit ref"
            ),
        ),
        "superseded policy source document"
    );

    assert_eq!(superseded.status, PolicySourceStatus::Superseded);
    assert_eq!(superseded.policy_version, document.policy_version);
    assert_eq!(
        superseded
            .superseded_by_policy_version
            .ok_or_else(|| std::io::Error::other("replacement policy version"))?
            .value(),
        3
    );
    assert_eq!(
        superseded
            .audit_reference_ids
            .last()
            .ok_or_else(|| std::io::Error::other("supersede audit ref"))?
            .as_str(),
        "audit-policy-superseded"
    );
    Ok(())
}

#[test]
fn rolled_back_status_requires_prior_version_reference_and_new_audit_ref() -> TestResult {
    let document = sample_policy_source_document()?;

    let mut invalid_ref = sample_policy_rollback_ref()?;
    invalid_ref.restored_policy_version = test_ok!(PolicyVersion::new(2), "policy version");

    let same_version_error = test_err!(
        rollback_parent_policy_source_document(
            &document,
            &invalid_ref,
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-rolled-back"),
                "policy audit ref"
            ),
        ),
        "rollback must reference an older restored policy version"
    );
    assert!(same_version_error
        .to_string()
        .contains("policy_source.rollback_ref.restored_policy_version"));

    let rollback_ref = sample_policy_rollback_ref()?;
    let duplicate_audit_error = test_err!(
        rollback_parent_policy_source_document(
            &document,
            &rollback_ref,
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-activated"),
                "policy audit ref"
            ),
        ),
        "rollback must record a new audit reference"
    );
    assert!(duplicate_audit_error
        .to_string()
        .contains("policy_source.audit_reference_id"));

    let rollback_ref = sample_policy_rollback_ref()?;
    let rolled_back = test_ok!(
        rollback_parent_policy_source_document(
            &document,
            &rollback_ref,
            test_ok!(
                PolicyAuditReferenceId::parse("audit-policy-rolled-back"),
                "policy audit ref"
            ),
        ),
        "rolled-back policy source document"
    );

    assert_eq!(rolled_back.status, PolicySourceStatus::RolledBack);
    assert_eq!(rolled_back.policy_version, document.policy_version);
    assert_eq!(
        rolled_back
            .rollback_ref
            .as_ref()
            .ok_or_else(|| std::io::Error::other("rollback reference"))?
            .restored_policy_version
            .value(),
        1
    );
    assert_eq!(
        rolled_back
            .audit_reference_ids
            .last()
            .ok_or_else(|| std::io::Error::other("rollback audit ref"))?
            .as_str(),
        "audit-policy-rolled-back"
    );
    Ok(())
}

#[test]
fn source_compile_helper_rejects_draft_and_preview_documents() -> TestResult {
    for status in [PolicySourceStatus::Draft, PolicySourceStatus::Preview] {
        let mut document = sample_policy_source_document()?;
        document.status = status;
        document.audit_reference_ids.clear();

        let error = test_err!(
            compile_domain_policy_artifact(&document, PolicyConsumerDomain::Tracking),
            "draft and preview source documents cannot compile artifacts"
        );
        assert!(error.to_string().contains("policy_source.status"));
    }
    Ok(())
}

#[test]
fn weekly_reset_requires_day() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schedules[0].time_budget.reset.kind = PolicyScheduleBudgetResetKind::Weekly;
    document.schedules[0].time_budget.reset.day = None;

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "weekly reset must require an explicit day"
    );

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_RESET_DAY,
        "missing-weekly-reset-day",
    )?;
    Ok(())
}

#[test]
fn daily_and_monthly_resets_reject_unexpected_day() -> TestResult {
    for reset_kind in [
        PolicyScheduleBudgetResetKind::Daily,
        PolicyScheduleBudgetResetKind::Monthly,
    ] {
        let mut document = sample_policy_source_document()?;
        document.schedules[0].time_budget.reset.kind = reset_kind;
        document.schedules[0].time_budget.reset.day = Some(PolicyScheduleDay::Monday);

        let error = test_err!(
            register_parent_policy_source_document(None, document),
            "non-weekly resets must reject an explicit day"
        );

        expect_invalid_value(
            error,
            policy_control::source::FIELD_SCHEDULE_RESET_DAY,
            "unexpected-reset-day",
        )?;
    }
    Ok(())
}

#[test]
fn discard_unused_carryover_rejects_max_minutes() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schedules[0].time_budget.carryover.mode =
        PolicyScheduleBudgetCarryoverMode::DiscardUnused;
    document.schedules[0].time_budget.carryover.max_minutes = Some(15);

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "discard-unused carryover must not accept max_minutes"
    );

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
        "discard-unused",
    )?;
    Ok(())
}

#[test]
fn cap_carryover_requires_positive_max_minutes() -> TestResult {
    for max_minutes in [None, Some(0)] {
        let mut document = sample_policy_source_document()?;
        document.schedules[0].time_budget.carryover.mode =
            PolicyScheduleBudgetCarryoverMode::CapCarryover;
        document.schedules[0].time_budget.carryover.max_minutes = max_minutes;

        let error = test_err!(
            register_parent_policy_source_document(None, document),
            "cap-carryover must require a positive max_minutes"
        );

        expect_invalid_value(
            error,
            policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
            "cap-carryover",
        )?;
    }
    Ok(())
}

#[test]
fn effective_until_must_be_after_effective_from() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schedules[0].time_budget.effective_until = Some("2026-01-01T00:00:00Z".to_string());

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "effective_until must be strictly after effective_from"
    );

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
        "2026-01-01T00:00:00Z",
    )?;
    Ok(())
}

#[test]
fn bonus_expiry_minutes_must_be_non_zero() -> TestResult {
    let mut document = sample_policy_source_document()?;
    document.schedules[0].time_budget.bonus_expiry_minutes = 0;

    let error = test_err!(
        register_parent_policy_source_document(None, document),
        "bonus expiry must be non-zero"
    );

    expect_invalid_value(
        error,
        policy_control::source::FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES,
        "0",
    )?;
    Ok(())
}
