use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_agent_protocol::constants::policy_control;
use ocentra_policy_control_core::policy_compiler::{
    compile_browser_policy, compile_domain_policy_with_support_matrix,
    compile_enforcement_policy_hints, compile_notification_ask_parent_policy,
    compile_screen_policy, policy_compiler_schema_version, DomainCompiledPolicyArtifact,
    PolicyCompilerCapabilityState, PolicyCompilerDomain, PolicyCompilerSupportMatrix,
    PolicyCompilerSupportMatrixRow,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};
use serde_json::Value;

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

fn sample_policy_source_document(version: u64) -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-compiler"),
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
            "device id"
        )],
        rules: vec![ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-site-block"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-youtube"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 100,
            reason_code: test_ok!(PolicyReasonCode::parse("bedtime"), "reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: test_ok!(PolicyScheduleId::parse("schedule-school-night"), "schedule"),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Toronto"), "timezone"),
            starts_at: "21:00".to_string(),
            ends_at: "07:00".to_string(),
            time_budget: sample_policy_schedule_time_budget(),
        }],
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-compiler-source"),
            "audit ref"
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

fn sample_screen_status_source_document(version: u64) -> TestResult<ParentPolicySourceDocument> {
    let mut source = sample_policy_source_document(version)?;
    source.rules = vec![
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-screen-app-review"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::App,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("app-minecraft"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::TimeLimit,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 100,
            reason_code: test_ok!(PolicyReasonCode::parse("screen-app-review"), "reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-screen-site-review"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-youtube"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 90,
            reason_code: test_ok!(PolicyReasonCode::parse("screen-site-review"), "reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-screen-device-ready"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Device,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("device-laptop"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Warn,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 80,
            reason_code: test_ok!(
                PolicyReasonCode::parse("screen-device-ready"),
                "reason code"
            ),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-screen-category-ready"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("category-social"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::AskParent,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "schedule"
            )),
            priority: 70,
            reason_code: test_ok!(
                PolicyReasonCode::parse("screen-category-ready"),
                "reason code"
            ),
            enabled: true,
        },
    ];
    Ok(source)
}

fn sample_time_boundary_policy_source_document(
    version: u64,
) -> TestResult<ParentPolicySourceDocument> {
    let mut source = sample_policy_source_document(version)?;
    source.rules = vec![
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-dst-spring-forward"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-bedtime-spring-forward"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-dst-spring-forward"),
                "schedule"
            )),
            priority: 120,
            reason_code: test_ok!(PolicyReasonCode::parse("dst-spring-forward"), "reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-dst-fall-back"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-bedtime-fall-back"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::TimeLimit,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-dst-fall-back"),
                "schedule"
            )),
            priority: 110,
            reason_code: test_ok!(PolicyReasonCode::parse("dst-fall-back"), "reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-child-device-clock"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-child-device-clock"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Warn,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-child-device-clock"),
                "schedule"
            )),
            priority: 100,
            reason_code: test_ok!(PolicyReasonCode::parse("child-device-clock"), "reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: test_ok!(PolicyRuleId::parse("rule-manual-clock-review"), "rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("site-manual-clock-review"),
                    "target ref"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-manual-clock-review"),
                "schedule"
            )),
            priority: 90,
            reason_code: test_ok!(
                PolicyReasonCode::parse("manual-clock-review"),
                "reason code"
            ),
            enabled: true,
        },
    ];
    source.schedules = vec![
        PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-dst-spring-forward"),
                "schedule"
            ),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Toronto"), "timezone"),
            starts_at: "02:15".to_string(),
            ends_at: "03:30".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 90,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "02:00".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::CapCarryover,
                    max_minutes: Some(45),
                },
                grace_period_minutes: 10,
                effective_from: "2026-03-08T06:45:00Z".to_string(),
                effective_until: Some("2026-03-08T08:30:00Z".to_string()),
                bonus_expiry_minutes: 15,
                clock_source: PolicyScheduleClockSource::TrustedService,
                offline_recovery: PolicyScheduleOfflineRecovery::ResumeRemaining,
            },
        },
        PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-dst-fall-back"),
                "schedule"
            ),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Toronto"), "timezone"),
            starts_at: "01:15".to_string(),
            ends_at: "01:45".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 120,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "01:00".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::CarryForward,
                    max_minutes: None,
                },
                grace_period_minutes: 20,
                effective_from: "2026-11-01T04:15:00Z".to_string(),
                effective_until: Some("2026-11-01T07:45:00Z".to_string()),
                bonus_expiry_minutes: 90,
                clock_source: PolicyScheduleClockSource::TrustedService,
                offline_recovery: PolicyScheduleOfflineRecovery::RecomputeFromJournal,
            },
        },
        PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-child-device-clock"),
                "schedule"
            ),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Los_Angeles"), "timezone"),
            starts_at: "20:00".to_string(),
            ends_at: "21:00".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 60,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "00:05".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::DiscardUnused,
                    max_minutes: None,
                },
                grace_period_minutes: 2,
                effective_from: "2026-02-01T04:00:00Z".to_string(),
                effective_until: Some("2026-02-01T08:00:00Z".to_string()),
                bonus_expiry_minutes: 5,
                clock_source: PolicyScheduleClockSource::ChildDevice,
                offline_recovery: PolicyScheduleOfflineRecovery::ResumeRemaining,
            },
        },
        PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-manual-clock-review"),
                "schedule"
            ),
            timezone_name: test_ok!(PolicyTimezoneName::parse("America/Winnipeg"), "timezone"),
            starts_at: "19:30".to_string(),
            ends_at: "21:30".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 30,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "00:30".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::CapCarryover,
                    max_minutes: Some(30),
                },
                grace_period_minutes: 1,
                effective_from: "2026-02-15T01:30:00Z".to_string(),
                effective_until: Some("2026-02-15T06:30:00Z".to_string()),
                bonus_expiry_minutes: 10,
                clock_source: PolicyScheduleClockSource::ManualRequired,
                offline_recovery: PolicyScheduleOfflineRecovery::ManualRequired,
            },
        },
    ];
    Ok(source)
}

fn support_matrix_row(
    target_kind: PolicyTargetKind,
    capability_state: PolicyCompilerCapabilityState,
) -> PolicyCompilerSupportMatrixRow {
    PolicyCompilerSupportMatrixRow {
        target_kind,
        capability_state,
    }
}

fn sample_browser_support_matrix() -> PolicyCompilerSupportMatrix {
    PolicyCompilerSupportMatrix {
        domain: PolicyCompilerDomain::Browser,
        rows: vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Unsupported,
            ),
        ],
    }
}

#[test]
fn policy_compiler_schema_version_is_nonzero() -> TestResult {
    let version = test_ok!(policy_compiler_schema_version(), "compiler schema version");
    assert_eq!(version.value(), 1);
    Ok(())
}

#[test]
fn compiled_artifact_serialization_preserves_schedule_payload() -> TestResult {
    let source = sample_policy_source_document(5)?;
    let artifact = test_ok!(
        compile_browser_policy(&source, source.policy_version),
        "browser policy artifact"
    );
    let payload = test_ok!(serde_json::to_value(&artifact), "compiled artifact payload");
    let schedules = payload
        .get("schedules")
        .and_then(Value::as_array)
        .ok_or_else(|| std::io::Error::other("serialized schedules array"))?;

    assert_eq!(schedules.len(), 1);
    assert_eq!(payload["rules"][0]["schedule_id"], "schedule-school-night");
    assert_eq!(
        payload["schedules"][0]["schedule_id"],
        "schedule-school-night"
    );
    assert_eq!(
        payload["compiled_artifact_id"],
        "policy-compiler:browser:policy-source-compiler:5"
    );
    assert_eq!(payload["delivery_target"]["domain"], "browser");
    assert_eq!(payload["support_matrix"]["domain"], "browser");
    assert_eq!(payload["rules"][0]["capability_state"], "supported");
    assert_eq!(
        payload["delivery_target"]["child_profile_ids"][0],
        "child-primary"
    );
    assert_eq!(payload["delivery_target"]["device_ids"][0], "device-laptop");
    assert_eq!(
        payload["evidence_custody_requirements"]["export_allowed"],
        true
    );
    assert_eq!(
        payload["evidence_custody_requirements"]["delete_allowed"],
        true
    );
    assert_eq!(
        payload["evidence_custody_requirements"]["sync_allowed"],
        false
    );
    assert_eq!(
        payload["no_claim_labels"][0],
        policy_control::compiler::NO_CLAIM_COMPILED_ARTIFACT_NOT_SOURCE_TRUTH
    );
    assert_eq!(
        payload["no_claim_labels"][4],
        policy_control::compiler::NO_CLAIM_PLATFORM_SUPPORT
    );
    assert!(payload["schedules"][0]["time_budget"].is_object());
    assert_eq!(
        payload["schedules"][0]["time_budget"]["budget_window_minutes"],
        120
    );
    Ok(())
}

#[test]
fn compiled_artifact_round_trips_wp07_time_boundary_schedule_payload() -> TestResult {
    let source = sample_time_boundary_policy_source_document(5)?;
    let artifact = test_ok!(
        compile_browser_policy(&source, source.policy_version),
        "browser policy artifact"
    );
    let payload = test_ok!(serde_json::to_value(&artifact), "compiled artifact payload");
    let schedules = payload
        .get("schedules")
        .and_then(Value::as_array)
        .ok_or_else(|| std::io::Error::other("serialized schedules array"))?;

    assert_eq!(schedules.len(), 4);
    assert_eq!(schedules[0]["starts_at"], "02:15");
    assert_eq!(schedules[0]["ends_at"], "03:30");
    assert_eq!(schedules[0]["time_budget"]["reset"]["local_time"], "02:00");
    assert_eq!(
        schedules[0]["time_budget"]["carryover"]["mode"],
        "cap-carryover"
    );
    assert_eq!(schedules[0]["time_budget"]["carryover"]["max_minutes"], 45);
    assert_eq!(schedules[1]["starts_at"], "01:15");
    assert_eq!(schedules[1]["ends_at"], "01:45");
    assert_eq!(schedules[1]["time_budget"]["reset"]["local_time"], "01:00");
    assert_eq!(
        schedules[1]["time_budget"]["carryover"]["mode"],
        "carry-forward"
    );
    assert_eq!(schedules[2]["time_budget"]["clock_source"], "child-device");
    assert_eq!(
        schedules[2]["time_budget"]["offline_recovery"],
        "resume-remaining"
    );
    assert_eq!(
        schedules[3]["time_budget"]["clock_source"],
        "manual-required"
    );
    assert_eq!(
        schedules[3]["time_budget"]["offline_recovery"],
        "manual-required"
    );

    let round_trip: DomainCompiledPolicyArtifact = test_ok!(
        serde_json::from_value(payload.clone()),
        "round trip compiled artifact"
    );
    assert_eq!(round_trip, artifact);
    assert_eq!(
        payload["rules"][0]["schedule_id"],
        "schedule-dst-spring-forward"
    );
    assert_eq!(payload["rules"][1]["schedule_id"], "schedule-dst-fall-back");
    assert_eq!(
        payload["rules"][2]["schedule_id"],
        "schedule-child-device-clock"
    );
    assert_eq!(
        payload["rules"][3]["schedule_id"],
        "schedule-manual-clock-review"
    );
    Ok(())
}

#[test]
fn screen_compiler_serialization_preserves_status_strings() -> TestResult {
    let source = sample_screen_status_source_document(5)?;
    let artifact = test_ok!(
        compile_screen_policy(&source, source.policy_version),
        "screen policy artifact"
    );
    let payload = test_ok!(serde_json::to_value(&artifact), "compiled artifact payload");

    assert_eq!(payload["domain"], "screen");
    assert_eq!(
        payload["rules"]
            .as_array()
            .ok_or_else(|| std::io::Error::other("serialized rules"))?
            .len(),
        4
    );
    assert_eq!(payload["rules"][0]["capability_state"], "manual-required");
    assert_eq!(payload["rules"][0]["status"], "manual-required");
    assert_eq!(
        payload["rules"][0]["reason_code"],
        policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET
    );
    assert_eq!(payload["rules"][1]["status"], "manual-required");
    assert_eq!(
        payload["rules"][1]["reason_code"],
        policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET
    );
    assert_eq!(payload["rules"][2]["status"], "ready");
    assert!(payload["rules"][2]["reason_code"].is_null());
    assert_eq!(payload["rules"][3]["status"], "ready");
    assert!(payload["rules"][3]["reason_code"].is_null());
    assert_eq!(
        payload["evidence_custody_requirements"]["sync_allowed"],
        false
    );
    assert_eq!(
        payload["no_claim_labels"][2],
        policy_control::compiler::NO_CLAIM_ENFORCEMENT
    );
    Ok(())
}

#[test]
fn domain_override_serialization_preserves_rule_capability_reason_pairs() -> TestResult {
    let source = sample_screen_status_source_document(5)?;
    let enforcement = test_ok!(
        compile_enforcement_policy_hints(&source, source.policy_version),
        "enforcement policy artifact"
    );
    let enforcement_payload = test_ok!(
        serde_json::to_value(&enforcement),
        "enforcement policy payload"
    );

    assert_eq!(
        enforcement_payload["support_matrix"]["rows"][0]["capability_state"],
        "supported"
    );
    assert_eq!(
        enforcement_payload["rules"][0]["capability_state"],
        "manual-required"
    );
    assert_eq!(enforcement_payload["rules"][0]["status"], "manual-required");
    assert_eq!(
        enforcement_payload["rules"][0]["reason_code"],
        policy_control::compiler::REASON_ENFORCEMENT_HANDOFF_REQUIRED
    );
    assert_eq!(
        enforcement_payload["rules"][3]["capability_state"],
        "manual-required"
    );
    assert_eq!(enforcement_payload["rules"][3]["status"], "manual-required");
    assert_eq!(
        enforcement_payload["rules"][3]["reason_code"],
        policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET
    );

    let notification = test_ok!(
        compile_notification_ask_parent_policy(&source, source.policy_version),
        "notification/ask-parent policy artifact"
    );
    let notification_payload = test_ok!(
        serde_json::to_value(&notification),
        "notification/ask-parent policy payload"
    );

    assert_eq!(
        notification_payload["support_matrix"]["rows"][0]["capability_state"],
        "supported"
    );
    assert_eq!(
        notification_payload["rules"][0]["capability_state"],
        "manual-required"
    );
    assert_eq!(
        notification_payload["rules"][0]["status"],
        "manual-required"
    );
    assert_eq!(
        notification_payload["rules"][0]["reason_code"],
        policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET
    );
    assert_eq!(
        notification_payload["rules"][3]["capability_state"],
        "supported"
    );
    assert_eq!(notification_payload["rules"][3]["status"], "ready");
    assert!(notification_payload["rules"][3]["reason_code"].is_null());
    Ok(())
}

#[test]
fn compiler_rejects_consumer_version_mismatch() -> TestResult {
    let source = sample_policy_source_document(5)?;
    let stale_consumer_version = test_ok!(PolicyVersion::new(4), "policy version");

    let error = test_err!(
        compile_browser_policy(&source, stale_consumer_version),
        "consumer version mismatch must be rejected"
    );
    assert!(error.to_string().contains("source 5 != consumer 4"));
    Ok(())
}

#[test]
fn compiled_artifact_round_trips_explicit_support_matrix_payload() -> TestResult {
    let source = sample_screen_status_source_document(5)?;
    let artifact = test_ok!(
        compile_domain_policy_with_support_matrix(
            &source,
            source.policy_version,
            PolicyCompilerDomain::Browser,
            sample_browser_support_matrix(),
        ),
        "browser policy artifact with explicit support matrix"
    );
    let payload = test_ok!(serde_json::to_value(&artifact), "compiled artifact payload");

    assert_eq!(
        payload["support_matrix"]["rows"][0]["target_kind"],
        "child-profile"
    );
    assert_eq!(
        payload["support_matrix"]["rows"][2]["capability_state"],
        "supported"
    );
    assert_eq!(payload["rules"][0]["capability_state"], "supported");
    assert_eq!(payload["rules"][1]["capability_state"], "manual-required");
    assert_eq!(payload["rules"][2]["capability_state"], "unsupported");

    let round_trip: DomainCompiledPolicyArtifact = test_ok!(
        serde_json::from_value(payload),
        "round trip compiled artifact"
    );
    assert_eq!(round_trip, artifact);
    Ok(())
}
