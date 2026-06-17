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
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
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

fn sample_policy_source_document(version: u64) -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-compiler")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(version).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-site-block").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-youtube").expect("target ref"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 100,
            reason_code: PolicyReasonCode::parse("bedtime").expect("reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
            starts_at: "21:00".to_string(),
            ends_at: "07:00".to_string(),
            time_budget: sample_policy_schedule_time_budget(),
        }],
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-compiler-source").expect("audit ref")
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

fn sample_screen_status_source_document(version: u64) -> ParentPolicySourceDocument {
    let mut source = sample_policy_source_document(version);
    source.rules = vec![
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-screen-app-review").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::App,
                reference_id: PolicyTargetReferenceId::parse("app-minecraft").expect("target ref"),
            },
            action: PolicyRuleAction::TimeLimit,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 100,
            reason_code: PolicyReasonCode::parse("screen-app-review").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-screen-site-review").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-youtube").expect("target ref"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 90,
            reason_code: PolicyReasonCode::parse("screen-site-review").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-screen-device-ready").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Device,
                reference_id: PolicyTargetReferenceId::parse("device-laptop").expect("target ref"),
            },
            action: PolicyRuleAction::Warn,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 80,
            reason_code: PolicyReasonCode::parse("screen-device-ready").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-screen-category-ready").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-social")
                    .expect("target ref"),
            },
            action: PolicyRuleAction::AskParent,
            schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
            priority: 70,
            reason_code: PolicyReasonCode::parse("screen-category-ready").expect("reason code"),
            enabled: true,
        },
    ];
    source
}

fn sample_time_boundary_policy_source_document(version: u64) -> ParentPolicySourceDocument {
    let mut source = sample_policy_source_document(version);
    source.rules = vec![
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-dst-spring-forward").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-bedtime-spring-forward")
                    .expect("target ref"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-dst-spring-forward").expect("schedule"),
            ),
            priority: 120,
            reason_code: PolicyReasonCode::parse("dst-spring-forward").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-dst-fall-back").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-bedtime-fall-back")
                    .expect("target ref"),
            },
            action: PolicyRuleAction::TimeLimit,
            schedule_id: Some(PolicyScheduleId::parse("schedule-dst-fall-back").expect("schedule")),
            priority: 110,
            reason_code: PolicyReasonCode::parse("dst-fall-back").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-child-device-clock").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-child-device-clock")
                    .expect("target ref"),
            },
            action: PolicyRuleAction::Warn,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-child-device-clock").expect("schedule"),
            ),
            priority: 100,
            reason_code: PolicyReasonCode::parse("child-device-clock").expect("reason code"),
            enabled: true,
        },
        ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-manual-clock-review").expect("rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site-manual-clock-review")
                    .expect("target ref"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-manual-clock-review").expect("schedule"),
            ),
            priority: 90,
            reason_code: PolicyReasonCode::parse("manual-clock-review").expect("reason code"),
            enabled: true,
        },
    ];
    source.schedules = vec![
        PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-dst-spring-forward").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
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
            schedule_id: PolicyScheduleId::parse("schedule-dst-fall-back").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto").expect("timezone"),
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
            schedule_id: PolicyScheduleId::parse("schedule-child-device-clock").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Los_Angeles").expect("timezone"),
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
            schedule_id: PolicyScheduleId::parse("schedule-manual-clock-review").expect("schedule"),
            timezone_name: PolicyTimezoneName::parse("America/Winnipeg").expect("timezone"),
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
    source
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
fn policy_compiler_schema_version_is_nonzero() {
    let version = policy_compiler_schema_version().expect("compiler schema version");
    assert_eq!(version.value(), 1);
}

#[test]
fn compiled_artifact_serialization_preserves_schedule_payload() {
    let source = sample_policy_source_document(5);
    let artifact =
        compile_browser_policy(&source, source.policy_version).expect("browser policy artifact");
    let payload = serde_json::to_value(&artifact).expect("compiled artifact payload");
    let schedules = payload
        .get("schedules")
        .and_then(Value::as_array)
        .expect("serialized schedules array");

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
}

#[test]
fn compiled_artifact_round_trips_wp07_time_boundary_schedule_payload() {
    let source = sample_time_boundary_policy_source_document(5);
    let artifact =
        compile_browser_policy(&source, source.policy_version).expect("browser policy artifact");
    let payload = serde_json::to_value(&artifact).expect("compiled artifact payload");
    let schedules = payload
        .get("schedules")
        .and_then(Value::as_array)
        .expect("serialized schedules array");

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

    let round_trip: DomainCompiledPolicyArtifact =
        serde_json::from_value(payload.clone()).expect("round trip compiled artifact");
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
}

#[test]
fn screen_compiler_serialization_preserves_status_strings() {
    let source = sample_screen_status_source_document(5);
    let artifact =
        compile_screen_policy(&source, source.policy_version).expect("screen policy artifact");
    let payload = serde_json::to_value(&artifact).expect("compiled artifact payload");

    assert_eq!(payload["domain"], "screen");
    assert_eq!(
        payload["rules"].as_array().expect("serialized rules").len(),
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
}

#[test]
fn domain_override_serialization_preserves_rule_capability_reason_pairs() {
    let source = sample_screen_status_source_document(5);
    let enforcement = compile_enforcement_policy_hints(&source, source.policy_version)
        .expect("enforcement policy artifact");
    let enforcement_payload =
        serde_json::to_value(&enforcement).expect("enforcement policy payload");

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

    let notification = compile_notification_ask_parent_policy(&source, source.policy_version)
        .expect("notification/ask-parent policy artifact");
    let notification_payload =
        serde_json::to_value(&notification).expect("notification/ask-parent policy payload");

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
}

#[test]
fn compiler_rejects_consumer_version_mismatch() {
    let source = sample_policy_source_document(5);
    let stale_consumer_version = PolicyVersion::new(4).expect("policy version");

    let error = compile_browser_policy(&source, stale_consumer_version)
        .expect_err("consumer version mismatch must be rejected");
    assert!(error.to_string().contains("source 5 != consumer 4"));
}

#[test]
fn compiled_artifact_round_trips_explicit_support_matrix_payload() {
    let source = sample_screen_status_source_document(5);
    let artifact = compile_domain_policy_with_support_matrix(
        &source,
        source.policy_version,
        PolicyCompilerDomain::Browser,
        sample_browser_support_matrix(),
    )
    .expect("browser policy artifact with explicit support matrix");
    let payload = serde_json::to_value(&artifact).expect("compiled artifact payload");

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

    let round_trip: DomainCompiledPolicyArtifact =
        serde_json::from_value(payload).expect("round trip compiled artifact");
    assert_eq!(round_trip, artifact);
}
