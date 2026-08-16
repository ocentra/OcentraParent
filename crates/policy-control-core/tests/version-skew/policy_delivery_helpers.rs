use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_delivery::{
    queue_policy_delivery, PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord,
    PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version, ParentPolicyActorRole,
    ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

pub(super) fn sample_policy_source_document(
    version: u64,
) -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-delivery"),
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
        status: PolicySourceStatus::Confirmed,
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
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-confirmed"),
            "policy audit ref"
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

pub(super) fn sample_delivery_target() -> TestResult<PolicyDeliveryTarget> {
    Ok(PolicyDeliveryTarget {
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: test_ok!(PolicyDeviceId::parse("device-laptop"), "policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    })
}

pub(super) fn sample_queued_delivery() -> TestResult<PolicyDeliveryRecord> {
    let source = sample_policy_source_document(7)?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );

    Ok(test_ok!(
        queue_policy_delivery(
            &compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-version-skew"),
                "policy delivery id"
            ),
            test_ok!(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id"
            ),
            vec![audit_ref("audit-policy-queued")?],
        ),
        "queued policy delivery"
    ))
}

pub(super) fn sample_delivery_id() -> TestResult<PolicyDeliveryId> {
    let source = sample_policy_source_document(7)?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );
    let target = sample_delivery_target()?;
    let attempt_id = test_ok!(
        PolicyDeliveryAttemptId::parse("attempt-queued"),
        "policy attempt id"
    );
    let sequence = test_ok!(
        PolicyDeliverySequence::new(1),
        "policy delivery initial sequence"
    );

    Ok(
        ocentra_policy_control_core::policy_delivery::derive_policy_delivery_id(
            &compiled,
            &target,
            &attempt_id,
            sequence,
        )?,
    )
}

pub(super) fn audit_ref(value: impl std::fmt::Display) -> TestResult<PolicyAuditReferenceId> {
    Ok(test_ok!(
        PolicyAuditReferenceId::parse(value.to_string()),
        "policy audit ref"
    ))
}

pub(super) fn transition_or_context<T>(
    result: Result<T, EventingError>,
    context: impl std::fmt::Display,
) -> TestResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(std::io::Error::other(format!("{context}: {error}")).into()),
    }
}

pub(crate) fn transition(
    sequence: u64,
    attempt_id: impl std::fmt::Display,
    state: PolicyDeliveryState,
) -> TestResult<PolicyDeliveryTransition> {
    let attempt_id = attempt_id.to_string();
    Ok(PolicyDeliveryTransition {
        attempt_id: test_ok!(
            PolicyDeliveryAttemptId::parse(attempt_id.as_str()),
            "policy attempt id"
        ),
        sequence: test_ok!(
            PolicyDeliverySequence::new(sequence),
            "policy delivery sequence"
        ),
        state,
        audit_reference_ids: vec![audit_ref(format!("audit-{attempt_id}-{sequence}"))?],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    })
}
