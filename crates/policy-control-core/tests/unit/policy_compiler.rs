use ocentra_parent_agent_protocol::constants::policy_control;
use ocentra_policy_control_core::policy_compiler::{
    compile_ai_policy_context, compile_app_game_policy, compile_browser_policy,
    compile_domain_policy_with_support_matrix,
    compile_enforcement_policy_hints, compile_network_policy,
    compile_notification_ask_parent_policy, compile_screen_policy, compile_tracking_policy,
    DomainCompiledPolicyArtifact, PolicyCompilerCapabilityState, PolicyCompilerDomain,
    PolicyCompilerRuleStatus, PolicyCompilerSupportMatrix, PolicyCompilerSupportMatrixRow,
};
use ocentra_policy_control_core::policy_source::{
    parent_policy_source_schema_version, rollback_parent_policy_source_document,
    supersede_parent_policy_source_document, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode,
    PolicyRetentionMetadata, PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

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
        document_id: ParentPolicyDocumentId::parse("policy-source-compiler")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(5).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("device id")],
        rules: vec![
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-app-limit").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::App,
                    reference_id: PolicyTargetReferenceId::parse("app-minecraft")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::TimeLimit,
                schedule_id: Some(
                    PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
                ),
                priority: 100,
                reason_code: PolicyReasonCode::parse("school-night").expect("reason code"),
                enabled: true,
            },
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-site-block").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::Site,
                    reference_id: PolicyTargetReferenceId::parse("site-youtube")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::Block,
                schedule_id: Some(
                    PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
                ),
                priority: 90,
                reason_code: PolicyReasonCode::parse("bedtime").expect("reason code"),
                enabled: true,
            },
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-device-curfew").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::Device,
                    reference_id: PolicyTargetReferenceId::parse("device-laptop")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::Warn,
                schedule_id: Some(
                    PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
                ),
                priority: 80,
                reason_code: PolicyReasonCode::parse("device-curfew").expect("reason code"),
                enabled: true,
            },
            ParentPolicyRule {
                rule_id: PolicyRuleId::parse("rule-parent-review").expect("rule id"),
                target: PolicyRuleTarget {
                    kind: PolicyTargetKind::Category,
                    reference_id: PolicyTargetReferenceId::parse("category-social")
                        .expect("target ref"),
                },
                action: PolicyRuleAction::AskParent,
                schedule_id: Some(
                    PolicyScheduleId::parse("schedule-school-night").expect("schedule"),
                ),
                priority: 70,
                reason_code: PolicyReasonCode::parse("social-review").expect("reason code"),
                enabled: true,
            },
        ],
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

fn sample_time_boundary_policy_source_document() -> ParentPolicySourceDocument {
    let mut source = sample_policy_source_document();
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

fn compiler_reason(value: &str) -> PolicyReasonCode {
    PolicyReasonCode::parse(value).expect("compiler reason code")
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
            support_matrix_row(PolicyTargetKind::App, PolicyCompilerCapabilityState::Supported),
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

fn assert_schedule_context_preserved(
    artifact: &DomainCompiledPolicyArtifact,
    source: &ParentPolicySourceDocument,
) {
    assert_eq!(artifact.schedules, source.schedules);
    assert_eq!(artifact.rules.len(), source.rules.len());
    assert_eq!(
        artifact.delivery_target.child_profile_ids,
        source.child_profile_ids
    );
    assert_eq!(artifact.delivery_target.device_ids, source.device_ids);
    assert_eq!(artifact.delivery_target.domain, artifact.domain);
    assert_eq!(artifact.evidence_custody_requirements, source.retention);
    assert_eq!(
        artifact.no_claim_labels,
        vec![
            policy_control::compiler::NO_CLAIM_COMPILED_ARTIFACT_NOT_SOURCE_TRUTH.to_string(),
            policy_control::compiler::NO_CLAIM_RUNTIME_MUTATION.to_string(),
            policy_control::compiler::NO_CLAIM_ENFORCEMENT.to_string(),
            policy_control::compiler::NO_CLAIM_UI_DELIVERY.to_string(),
            policy_control::compiler::NO_CLAIM_PLATFORM_SUPPORT.to_string(),
        ]
    );

    for (compiled_rule, source_rule) in artifact.rules.iter().zip(source.rules.iter()) {
        assert_eq!(compiled_rule.schedule_id, source_rule.schedule_id);
    }
}

#[test]
fn compiler_preserves_wp07_time_boundary_schedule_shapes_verbatim() {
    let source = sample_time_boundary_policy_source_document();

    let artifact =
        compile_browser_policy(&source, source.policy_version).expect("browser policy artifact");

    assert_schedule_context_preserved(&artifact, &source);
    assert_eq!(artifact.schedules.len(), 4);
    assert_eq!(artifact.schedules[0].starts_at, "02:15");
    assert_eq!(artifact.schedules[0].ends_at, "03:30");
    assert_eq!(artifact.schedules[0].time_budget.reset.local_time, "02:00");
    assert_eq!(
        artifact.schedules[0].time_budget.carryover.mode,
        PolicyScheduleBudgetCarryoverMode::CapCarryover
    );
    assert_eq!(
        artifact.schedules[0].time_budget.carryover.max_minutes,
        Some(45)
    );
    assert_eq!(artifact.schedules[1].starts_at, "01:15");
    assert_eq!(artifact.schedules[1].ends_at, "01:45");
    assert_eq!(artifact.schedules[1].time_budget.reset.local_time, "01:00");
    assert_eq!(
        artifact.schedules[1].time_budget.carryover.mode,
        PolicyScheduleBudgetCarryoverMode::CarryForward
    );
    assert_eq!(
        artifact.schedules[2].time_budget.clock_source,
        PolicyScheduleClockSource::ChildDevice
    );
    assert_eq!(
        artifact.schedules[2].time_budget.offline_recovery,
        PolicyScheduleOfflineRecovery::ResumeRemaining
    );
    assert_eq!(
        artifact.schedules[3].time_budget.clock_source,
        PolicyScheduleClockSource::ManualRequired
    );
    assert_eq!(
        artifact.schedules[3].time_budget.offline_recovery,
        PolicyScheduleOfflineRecovery::ManualRequired
    );
    assert_eq!(
        artifact
            .rules
            .iter()
            .map(|rule| {
                rule.schedule_id
                    .as_ref()
                    .expect("rule schedule id")
                    .as_str()
            })
            .collect::<Vec<_>>(),
        vec![
            "schedule-dst-spring-forward",
            "schedule-dst-fall-back",
            "schedule-child-device-clock",
            "schedule-manual-clock-review",
        ]
    );
}

#[test]
fn app_game_compiler_keeps_app_targets_ready_and_browser_targets_explicit() {
    let source = sample_policy_source_document();

    let artifact =
        compile_app_game_policy(&source, source.policy_version).expect("app/game policy artifact");

    assert_eq!(artifact.domain, PolicyCompilerDomain::AppGame);
    assert_schedule_context_preserved(&artifact, &source);
    assert_eq!(artifact.rules.len(), 4);
    assert_eq!(
        artifact.rules[0].capability_state,
        PolicyCompilerCapabilityState::Supported
    );
    assert_eq!(artifact.rules[0].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(
        artifact.rules[1].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[1].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[2].capability_state,
        PolicyCompilerCapabilityState::Unsupported
    );
    assert_eq!(
        artifact.rules[2].status,
        PolicyCompilerRuleStatus::Unsupported
    );
}

#[test]
fn browser_compiler_keeps_site_targets_ready_and_app_targets_unsupported() {
    let source = sample_policy_source_document();

    let artifact =
        compile_browser_policy(&source, source.policy_version).expect("browser policy artifact");

    assert_eq!(artifact.domain, PolicyCompilerDomain::Browser);
    assert_schedule_context_preserved(&artifact, &source);
    assert_eq!(
        artifact.rules[0].capability_state,
        PolicyCompilerCapabilityState::Unsupported
    );
    assert_eq!(
        artifact.rules[0].status,
        PolicyCompilerRuleStatus::Unsupported
    );
    assert_eq!(
        artifact.rules[1].capability_state,
        PolicyCompilerCapabilityState::Supported
    );
    assert_eq!(artifact.rules[1].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(
        artifact.rules[2].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[2].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
}

#[test]
fn network_and_tracking_compilers_keep_geofence_and_location_targets_explicit() {
    let mut source = sample_policy_source_document();
    source.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-geofence-school").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Resource,
            reference_id: PolicyTargetReferenceId::parse("geofence-school").expect("target ref"),
        },
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
        priority: 60,
        reason_code: PolicyReasonCode::parse("geofence-review").expect("reason code"),
        enabled: true,
    });
    source.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-location-sharing").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::ChildProfile,
            reference_id: PolicyTargetReferenceId::parse("child-primary-location")
                .expect("target ref"),
        },
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
        priority: 50,
        reason_code: PolicyReasonCode::parse("location-sharing").expect("reason code"),
        enabled: true,
    });

    let network =
        compile_network_policy(&source, source.policy_version).expect("network policy artifact");
    let tracking =
        compile_tracking_policy(&source, source.policy_version).expect("tracking policy artifact");

    assert_eq!(network.domain, PolicyCompilerDomain::Network);
    assert_schedule_context_preserved(&network, &source);
    assert_eq!(network.rules[1].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(
        network.rules[3].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(network.rules[4].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(network.rules[4].reason_code, None);
    assert_eq!(
        network.rules[5].status,
        PolicyCompilerRuleStatus::Unsupported
    );
    assert_eq!(
        network.rules[5].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_UNSUPPORTED_TARGET,
        ))
    );

    assert_eq!(tracking.domain, PolicyCompilerDomain::Tracking);
    assert_schedule_context_preserved(&tracking, &source);
    assert_eq!(tracking.rules[2].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(tracking.rules[4].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(tracking.rules[4].reason_code, None);
    assert_eq!(tracking.rules[4].rule_id, source.rules[4].rule_id);
    assert_eq!(tracking.rules[5].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(tracking.rules[5].reason_code, None);
    assert_eq!(tracking.rules[5].rule_id, source.rules[5].rule_id);
    assert_eq!(
        tracking.rules[1].status,
        PolicyCompilerRuleStatus::Unsupported
    );
}

#[test]
fn screen_compiler_keeps_manual_required_and_unsupported_targets_explicit() {
    let mut source = sample_policy_source_document();
    source.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-child-profile-review").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::ChildProfile,
            reference_id: PolicyTargetReferenceId::parse("child-profile-review")
                .expect("target ref"),
        },
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
        priority: 60,
        reason_code: PolicyReasonCode::parse("child-profile-review").expect("reason code"),
        enabled: true,
    });

    let artifact =
        compile_screen_policy(&source, source.policy_version).expect("screen policy artifact");

    assert_eq!(artifact.domain, PolicyCompilerDomain::Screen);
    assert_schedule_context_preserved(&artifact, &source);
    assert_eq!(artifact.rules.len(), source.rules.len());

    assert_eq!(
        artifact.rules[0].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[0].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(
        artifact.rules[1].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[1].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(artifact.rules[2].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(artifact.rules[2].reason_code, None);
    assert_eq!(artifact.rules[3].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(artifact.rules[3].reason_code, None);
    assert_eq!(
        artifact.rules[4].status,
        PolicyCompilerRuleStatus::Unsupported
    );
    assert_eq!(artifact.rules[4].rule_id, source.rules[4].rule_id);
    assert_eq!(
        artifact.rules[4].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_UNSUPPORTED_TARGET,
        ))
    );
}

#[test]
fn compiler_support_matrix_input_can_override_default_browser_capability_states() {
    let source = sample_policy_source_document();
    let support_matrix = sample_browser_support_matrix();

    let artifact = compile_domain_policy_with_support_matrix(
        &source,
        source.policy_version,
        PolicyCompilerDomain::Browser,
        support_matrix.clone(),
    )
    .expect("browser policy artifact with explicit support matrix");

    assert_eq!(artifact.support_matrix, support_matrix);
    assert_eq!(
        artifact.rules[0].capability_state,
        PolicyCompilerCapabilityState::Supported
    );
    assert_eq!(artifact.rules[0].status, PolicyCompilerRuleStatus::Ready);
    assert_eq!(artifact.rules[0].reason_code, None);
    assert_eq!(
        artifact.rules[1].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[1].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(
        artifact.rules[2].capability_state,
        PolicyCompilerCapabilityState::Unsupported
    );
    assert_eq!(
        artifact.rules[2].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_UNSUPPORTED_TARGET,
        ))
    );
}

#[test]
fn ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required() {
    let source = sample_policy_source_document();

    let ai = compile_ai_policy_context(&source, source.policy_version).expect("ai policy artifact");
    let enforcement = compile_enforcement_policy_hints(&source, source.policy_version)
        .expect("enforcement policy hints");

    assert_schedule_context_preserved(&ai, &source);
    assert_schedule_context_preserved(&enforcement, &source);
    assert!(ai
        .rules
        .iter()
        .all(|rule| rule.status == PolicyCompilerRuleStatus::Ready));
    assert!(ai.rules.iter().all(|rule| {
        rule.capability_state == PolicyCompilerCapabilityState::Supported
            && rule.reason_code.is_none()
    }));
    assert!(enforcement
        .support_matrix
        .rows
        .iter()
        .all(|row| row.capability_state == PolicyCompilerCapabilityState::Supported));
    assert!(enforcement
        .rules
        .iter()
        .all(|rule| rule.status == PolicyCompilerRuleStatus::ManualRequired));
    assert!(enforcement.rules.iter().all(|rule| {
        rule.capability_state == PolicyCompilerCapabilityState::ManualRequired
    }));
    assert!(enforcement.rules[..3].iter().all(|rule| {
        rule.reason_code
            == Some(compiler_reason(
                policy_control::compiler::REASON_ENFORCEMENT_HANDOFF_REQUIRED,
            ))
    }));
    assert_eq!(
        enforcement.rules[3].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
}

#[test]
fn notification_ask_parent_compiler_keeps_review_rules_ready_and_stays_deterministic() {
    let source = sample_policy_source_document();

    let artifact = compile_notification_ask_parent_policy(&source, source.policy_version)
        .expect("notification/ask-parent policy artifact");
    let repeated = compile_notification_ask_parent_policy(&source, source.policy_version)
        .expect("repeated notification/ask-parent policy artifact");

    assert_eq!(artifact, repeated);
    assert_eq!(artifact.domain, PolicyCompilerDomain::NotificationAskParent);
    assert_schedule_context_preserved(&artifact, &source);
    assert!(artifact
        .support_matrix
        .rows
        .iter()
        .all(|row| row.capability_state == PolicyCompilerCapabilityState::Supported));

    assert_eq!(
        artifact.rules[0].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[0].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[0].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(
        artifact.rules[1].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[1].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[1].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(
        artifact.rules[2].capability_state,
        PolicyCompilerCapabilityState::ManualRequired
    );
    assert_eq!(
        artifact.rules[2].status,
        PolicyCompilerRuleStatus::ManualRequired
    );
    assert_eq!(
        artifact.rules[2].reason_code,
        Some(compiler_reason(
            policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        ))
    );
    assert_eq!(
        artifact.rules[3].capability_state,
        PolicyCompilerCapabilityState::Supported
    );
    assert_eq!(artifact.rules[3].status, PolicyCompilerRuleStatus::Ready);
    assert!(artifact.rules[3].reason_code.is_none());
}

#[test]
fn compiler_outputs_stay_deterministic_across_domain_matrix() {
    type CompilerFn = fn(
        &ParentPolicySourceDocument,
        PolicyVersion,
    )
        -> Result<DomainCompiledPolicyArtifact, ocentra_eventing::EventingError>;

    let mut source = sample_policy_source_document();
    source.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-geofence-school").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Resource,
            reference_id: PolicyTargetReferenceId::parse("geofence-school").expect("target ref"),
        },
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
        priority: 60,
        reason_code: PolicyReasonCode::parse("geofence-review").expect("reason code"),
        enabled: true,
    });
    source.rules.push(ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-location-sharing").expect("rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::ChildProfile,
            reference_id: PolicyTargetReferenceId::parse("child-primary-location")
                .expect("target ref"),
        },
        action: PolicyRuleAction::Warn,
        schedule_id: Some(PolicyScheduleId::parse("schedule-school-night").expect("schedule")),
        priority: 50,
        reason_code: PolicyReasonCode::parse("location-sharing").expect("reason code"),
        enabled: true,
    });

    let compilers: [(&str, PolicyCompilerDomain, CompilerFn); 8] = [
        (
            "app/game",
            PolicyCompilerDomain::AppGame,
            compile_app_game_policy,
        ),
        (
            "browser",
            PolicyCompilerDomain::Browser,
            compile_browser_policy,
        ),
        (
            "network",
            PolicyCompilerDomain::Network,
            compile_network_policy,
        ),
        (
            "tracking",
            PolicyCompilerDomain::Tracking,
            compile_tracking_policy,
        ),
        (
            "screen",
            PolicyCompilerDomain::Screen,
            compile_screen_policy,
        ),
        ("ai", PolicyCompilerDomain::Ai, compile_ai_policy_context),
        (
            "enforcement",
            PolicyCompilerDomain::Enforcement,
            compile_enforcement_policy_hints,
        ),
        (
            "notification/ask-parent",
            PolicyCompilerDomain::NotificationAskParent,
            compile_notification_ask_parent_policy,
        ),
    ];

    for (label, domain, compile) in compilers {
        let first = compile(&source, source.policy_version)
            .unwrap_or_else(|error| panic!("{label} first compile failed: {error}"));
        let second = compile(&source, source.policy_version)
            .unwrap_or_else(|error| panic!("{label} second compile failed: {error}"));

        assert_eq!(
            first.domain, domain,
            "{label} compiler returned wrong domain"
        );
        assert_eq!(
            first, second,
            "{label} compiler output changed for identical input"
        );
    }
}

#[test]
fn compiler_rejects_draft_and_preview_source_documents_before_release_candidate_stage() {
    for status in [
        PolicySourceDocumentStatus::Draft,
        PolicySourceDocumentStatus::Preview,
    ] {
        let mut source = sample_policy_source_document();
        source.status = status;

        let error = compile_browser_policy(&source, source.policy_version)
            .expect_err("draft and preview source documents cannot compile release candidates");
        assert!(error.to_string().contains("policy_compiler.source_status"));
    }
}

#[test]
fn compiler_rejects_domain_cache_source_documents_as_non_canonical_source_truth() {
    let mut source = sample_policy_source_document();
    source.source_surface = PolicySourceWriteSurface::DomainCache;

    let error = compile_browser_policy(&source, source.policy_version)
        .expect_err("domain-cache source documents cannot be recompiled as source truth");
    assert!(error
        .to_string()
        .contains(policy_control::source::FIELD_SOURCE_SURFACE));
    assert!(error
        .to_string()
        .contains(policy_control::source::SURFACE_DOMAIN_CACHE));
}

#[test]
fn compiler_rejects_support_matrix_domain_mismatches_and_missing_target_coverage() {
    let source = sample_policy_source_document();

    let mismatched_domain = compile_domain_policy_with_support_matrix(
        &source,
        source.policy_version,
        PolicyCompilerDomain::Browser,
        PolicyCompilerSupportMatrix {
            domain: PolicyCompilerDomain::Tracking,
            rows: sample_browser_support_matrix().rows,
        },
    )
    .expect_err("support matrix domain mismatch must fail");
    assert!(mismatched_domain
        .to_string()
        .contains(policy_control::compiler::FIELD_SUPPORT_MATRIX_DOMAIN));

    let missing_target = compile_domain_policy_with_support_matrix(
        &source,
        source.policy_version,
        PolicyCompilerDomain::Browser,
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
                    PolicyTargetKind::Category,
                    PolicyCompilerCapabilityState::Supported,
                ),
            ],
        },
    )
    .expect_err("support matrix must classify every target kind exactly once");
    assert!(missing_target
        .to_string()
        .contains(policy_control::compiler::FIELD_SUPPORT_MATRIX_TARGET_KIND));
}

#[test]
fn compiler_artifact_has_deterministic_id_and_explicit_delivery_scope() {
    let source = sample_policy_source_document();

    let artifact =
        compile_browser_policy(&source, source.policy_version).expect("browser policy artifact");

    assert_eq!(
        artifact.compiled_artifact_id.as_str(),
        "policy-compiler:browser:policy-source-compiler:5"
    );
    assert_eq!(artifact.delivery_target.child_profile_ids.len(), 1);
    assert_eq!(
        artifact.delivery_target.child_profile_ids[0].as_str(),
        "child-primary"
    );
    assert_eq!(artifact.delivery_target.device_ids.len(), 1);
    assert_eq!(
        artifact.delivery_target.device_ids[0].as_str(),
        "device-laptop"
    );
    assert_eq!(
        artifact.delivery_target.domain,
        PolicyCompilerDomain::Browser
    );
}

#[test]
fn compiler_artifact_preserves_audit_and_lifecycle_refs_from_source_documents() {
    let source = sample_policy_source_document();
    let superseded = supersede_parent_policy_source_document(
        &source,
        PolicyVersion::new(6).expect("policy version"),
        PolicyAuditReferenceId::parse("audit-compiler-superseded").expect("audit ref"),
    )
    .expect("superseded source document");
    let browser_artifact =
        compile_browser_policy(&superseded, superseded.policy_version).expect("browser policy");

    assert_schedule_context_preserved(&browser_artifact, &superseded);
    assert_eq!(browser_artifact.audit_reference_ids.len(), 2);
    assert_eq!(
        browser_artifact
            .superseded_by_policy_version
            .expect("replacement policy version")
            .value(),
        6
    );
    assert!(browser_artifact.rollback_ref.is_none());

    let rollback_ref = PolicyRollbackRef {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        rolled_back_document_id: ParentPolicyDocumentId::parse("policy-source-compiler")
            .expect("policy source document id"),
        rolled_back_policy_version: PolicyVersion::new(5).expect("policy version"),
        restored_document_id: ParentPolicyDocumentId::parse("policy-source-compiler-previous")
            .expect("policy source document id"),
        restored_policy_version: PolicyVersion::new(4).expect("policy version"),
    };

    let rolled_back = rollback_parent_policy_source_document(
        &source,
        &rollback_ref,
        PolicyAuditReferenceId::parse("audit-compiler-rolled-back").expect("audit ref"),
    )
    .expect("rolled-back source document");
    let tracking_artifact =
        compile_tracking_policy(&rolled_back, rolled_back.policy_version).expect("tracking policy");

    assert_schedule_context_preserved(&tracking_artifact, &rolled_back);
    assert_eq!(tracking_artifact.audit_reference_ids.len(), 2);
    assert!(tracking_artifact.superseded_by_policy_version.is_none());
    assert_eq!(
        tracking_artifact
            .rollback_ref
            .as_ref()
            .expect("rollback ref")
            .restored_policy_version
            .value(),
        4
    );
}
