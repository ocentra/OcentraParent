use ocentra_app_game_core::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyBonusApprovalRef, AppGamePolicyDurationSource,
    AppGamePolicyRuntimeAdapterDispatchState, AppGamePolicyRuntimeDecisionReason,
    AppGamePolicyRuntimeDecisionState, AppGamePolicyRuntimeTimerRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::compile_app_game_policy_target;
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef,
    AppGamePolicyCompileRequestId, AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId,
    AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef, AppGamePolicyRuleRef,
    AppGamePolicyScheduleRef, AppGamePolicyTargetRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityEvidence,
    AppGamePolicyCompilerAuthorityState, AppGamePolicyCompilerCapabilityEvidence,
    AppGamePolicyCompilerCapabilityState, AppGamePolicyCompilerContext,
    AppGamePolicyCompilerEvidence, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerProofKind, AppGamePolicyCompilerRequestedAction,
    AppGamePolicyCompilerTarget, AppGamePolicyTargetKind,
};
use ocentra_app_game_core::app_game_time_budget::evaluate_app_game_time_budget;
use ocentra_app_game_core::app_game_time_budget_types::{
    AppGameTimeBudgetBonus, AppGameTimeBudgetDurationMode, AppGameTimeBudgetInput,
    AppGameTimeBudgetPeriod, AppGameTimeBudgetSchedule, AppGameTimeBudgetTimer,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::app_game::AppGameSessionSummary;

#[test]
fn stored_summary_running_and_foreground_modes_stay_distinct() -> Result<(), EventingError> {
    let mut running = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    running.budget_seconds = 60;
    running.stored_sessions = vec![session("session-mode", 90_000, 30_000)];
    running.duration_mode = AppGameTimeBudgetDurationMode::Running;
    let running_decision = evaluate_app_game_time_budget(running)?;
    assert_eq!(running_decision.runtime_decision.consumed_seconds, 90);
    assert_eq!(
        running_decision.runtime_decision.reason,
        AppGamePolicyRuntimeDecisionReason::BudgetExceeded
    );

    let mut foreground = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    foreground.budget_seconds = 60;
    foreground.stored_sessions = vec![session("session-mode", 90_000, 30_000)];
    foreground.duration_mode = AppGameTimeBudgetDurationMode::Foreground;
    let foreground_decision = evaluate_app_game_time_budget(foreground)?;
    assert_eq!(foreground_decision.runtime_decision.consumed_seconds, 30);
    assert_eq!(
        foreground_decision.runtime_decision.reason,
        AppGamePolicyRuntimeDecisionReason::WithinBudget
    );
    Ok(())
}

#[test]
fn weekly_bonus_extends_budget_and_preserves_approval_audit_refs() -> Result<(), EventingError> {
    let mut request = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    request.period = AppGameTimeBudgetPeriod::Weekly;
    request.budget_seconds = 80;
    request.stored_sessions = vec![session("session-weekly", 100_000, 100_000)];
    request.bonus = AppGameTimeBudgetBonus::Approved {
        additional_seconds: 30,
        approval_ref: bonus_ref("bonus-weekly"),
        approval_audit_ref: audit_ref("audit-bonus-weekly"),
    };
    let decision = evaluate_app_game_time_budget(request)?;
    assert_eq!(decision.period, AppGameTimeBudgetPeriod::Weekly);
    assert_eq!(decision.runtime_decision.effective_budget_seconds, 110);
    assert_eq!(
        decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::ApprovedBonusObserve
    );
    assert_eq!(
        decision.runtime_decision.bonus_approval_ref,
        Some(bonus_ref("bonus-weekly"))
    );
    assert_eq!(
        decision.bonus_audit_ref,
        Some(audit_ref("audit-bonus-weekly"))
    );
    Ok(())
}

#[test]
fn pending_bonus_requests_parent_and_preserves_request_audit_ref() -> Result<(), EventingError> {
    let mut request = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    request.bonus = AppGameTimeBudgetBonus::Pending {
        request_audit_ref: audit_ref("audit-bonus-request"),
    };
    let decision = evaluate_app_game_time_budget(request)?;
    assert_eq!(
        decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::AskParent
    );
    assert_eq!(
        decision.bonus_audit_ref,
        Some(audit_ref("audit-bonus-request"))
    );
    Ok(())
}

#[test]
fn schedule_evidence_is_bound_and_stale_schedule_fails_closed() -> Result<(), EventingError> {
    let schedule_ref = schedule_ref("schedule-school-night");
    let schedule_evidence_ref = evidence_ref("evidence-schedule");
    let mut request = input(
        AppGamePolicyCompilerRequestedAction::Warn,
        Some(schedule_ref.clone()),
    );
    request.schedule = AppGameTimeBudgetSchedule::Stale {
        schedule_ref: schedule_ref.clone(),
        evidence_refs: vec![schedule_evidence_ref.clone()],
    };
    let decision = evaluate_app_game_time_budget(request)?;
    assert_eq!(decision.schedule_ref, Some(schedule_ref));
    assert_eq!(decision.schedule_evidence_refs, vec![schedule_evidence_ref]);
    assert_eq!(
        decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        decision.runtime_decision.reason,
        AppGamePolicyRuntimeDecisionReason::StaleSchedule
    );
    Ok(())
}

#[test]
fn active_and_recovered_timers_remain_dry_run_only() -> Result<(), EventingError> {
    let mut active = input(AppGamePolicyCompilerRequestedAction::TimeLimit, None);
    active.budget_seconds = 10;
    active.timer = AppGameTimeBudgetTimer::Active {
        timer_ref: timer_ref("timer-active"),
    };
    let active_decision = evaluate_app_game_time_budget(active)?;
    assert_eq!(
        active_decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::DryRunTimeLimit
    );
    assert_eq!(active_decision.recovered_timer_ref, None);

    let mut recovered = input(AppGamePolicyCompilerRequestedAction::TimeLimit, None);
    recovered.budget_seconds = 10;
    recovered.duration_source = AppGamePolicyDurationSource::RecoveredJournal;
    recovered.timer = AppGameTimeBudgetTimer::Recovered {
        timer_ref: timer_ref("timer-recovered"),
    };
    let recovered_decision = evaluate_app_game_time_budget(recovered)?;
    assert_eq!(
        recovered_decision.runtime_decision.timer_ref,
        Some(timer_ref("timer-recovered"))
    );
    assert_eq!(
        recovered_decision.recovered_timer_ref,
        Some(timer_ref("timer-recovered"))
    );
    assert_eq!(
        recovered_decision.runtime_decision.adapter_dispatch_state,
        AppGamePolicyRuntimeAdapterDispatchState::NotDispatched
    );
    Ok(())
}

#[test]
fn invalid_summary_schedule_and_timer_inputs_fail_closed() {
    let mut duplicate = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    duplicate.stored_sessions = vec![
        session("duplicate", 10_000, 5_000),
        session("duplicate", 10_000, 5_000),
    ];
    assert!(matches!(
        evaluate_app_game_time_budget(duplicate),
        Err(EventingError::InvalidValue {
            field: "app_game.time_budget.session_ref",
            ..
        })
    ));

    let mut incoherent = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    incoherent.stored_sessions = vec![session("incoherent", 5_000, 6_000)];
    assert!(matches!(
        evaluate_app_game_time_budget(incoherent),
        Err(EventingError::InvalidValue {
            field: "app_game.time_budget.session_duration",
            ..
        })
    ));

    let mut manual = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    manual.duration_source = AppGamePolicyDurationSource::ManualEstimate;
    assert!(matches!(
        evaluate_app_game_time_budget(manual),
        Err(EventingError::InvalidValue {
            field: "app_game.time_budget.duration_source",
            ..
        })
    ));

    let mut timer = input(AppGamePolicyCompilerRequestedAction::Warn, None);
    timer.timer = AppGameTimeBudgetTimer::Active {
        timer_ref: timer_ref("timer-wrong-action"),
    };
    assert!(matches!(
        evaluate_app_game_time_budget(timer),
        Err(EventingError::InvalidValue {
            field: "app_game.time_budget.timer",
            ..
        })
    ));
}

fn input(
    action: AppGamePolicyCompilerRequestedAction,
    schedule_ref: Option<AppGamePolicyScheduleRef>,
) -> AppGameTimeBudgetInput {
    AppGameTimeBudgetInput {
        compilation: compile_app_game_policy_target(
            compile_request(action, schedule_ref),
            compiler_context(),
        ),
        evaluation_audit_ref: audit_ref("audit-time-budget"),
        period: AppGameTimeBudgetPeriod::Daily,
        budget_seconds: 120,
        warning_threshold_seconds: 20,
        stored_sessions: vec![session("session-default", 30_000, 20_000)],
        duration_mode: AppGameTimeBudgetDurationMode::Running,
        duration_source: AppGamePolicyDurationSource::AuthoritativeSession,
        schedule: AppGameTimeBudgetSchedule::NotRequired,
        bonus: AppGameTimeBudgetBonus::None,
        timer: AppGameTimeBudgetTimer::NotRequired,
    }
}

fn session(
    session_id: &str,
    running_duration_ms: u64,
    foreground_duration_ms: u64,
) -> AppGameSessionSummary {
    AppGameSessionSummary {
        schema_version: 1,
        session_id: session_id.to_string(),
        primary_process_identity: String::from("process-ref"),
        display_name: String::from("redacted-app"),
        classification_state: String::from("known-app"),
        catalog_ready_state: String::from("ready"),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        started_at: String::from("2026-08-15T00:00:00Z"),
        last_observed_at: String::from("2026-08-15T00:02:00Z"),
        ended_at: Some(String::from("2026-08-15T00:02:00Z")),
        end_reason: Some(String::from("process-exit")),
        running_duration_ms,
        foreground_duration_ms,
        background_duration_ms: running_duration_ms.saturating_sub(foreground_duration_ms),
        last_foreground_at: None,
        last_background_at: None,
        observation_gap_ms: 0,
        observation_count: 1,
        evidence_count: 0,
        evidence: Vec::new(),
        ai_digest_ref: None,
        confidence: 1.0,
    }
}

fn compile_request(
    action: AppGamePolicyCompilerRequestedAction,
    schedule_ref: Option<AppGamePolicyScheduleRef>,
) -> AppGamePolicyCompileRequest {
    let common_evidence_ref = evidence_ref("evidence-time-budget");
    let mut evidence = vec![
        compiler_evidence(
            common_evidence_ref.clone(),
            AppGamePolicyCompilerProofKind::CategoryProof,
        ),
        compiler_evidence(
            common_evidence_ref.clone(),
            AppGamePolicyCompilerProofKind::CapabilityProof,
        ),
        compiler_evidence(
            common_evidence_ref.clone(),
            AppGamePolicyCompilerProofKind::AuthorityProof,
        ),
    ];
    if schedule_ref.is_some() {
        evidence.push(compiler_evidence(
            evidence_ref("evidence-schedule"),
            AppGamePolicyCompilerProofKind::ScheduleProof,
        ));
    }
    AppGamePolicyCompileRequest {
        compile_request_id: AppGamePolicyCompileRequestId::parse("compile-time-budget")
            .expect_value("compile id"),
        rule_ref: AppGamePolicyRuleRef::parse("rule-time-budget").expect_value("rule ref"),
        device_id: AppGamePolicyDeviceId::parse("device-time-budget").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-time-budget")
            .expect_value("user ref"),
        target: AppGamePolicyCompilerTarget {
            target_kind: AppGamePolicyTargetKind::AppCategory,
            target_ref: Some(
                AppGamePolicyTargetRef::parse("target-time-budget").expect_value("target ref"),
            ),
        },
        requested_action: action,
        schedule_ref,
        evidence,
        capability_refs: vec![AppGamePolicyCompilerCapabilityEvidence {
            capability_ref: AppGamePolicyCapabilityRef::parse("capability-time-budget")
                .expect_value("capability ref"),
            capability_state: AppGamePolicyCompilerCapabilityState::Supported,
            evidence_refs: vec![common_evidence_ref.clone()],
        }],
        authority_refs: vec![AppGamePolicyCompilerAuthorityEvidence {
            authority_ref: AppGamePolicyAuthorityRef::parse("authority-time-budget")
                .expect_value("authority ref"),
            authority_state: AppGamePolicyCompilerAuthorityState::Proved,
            evidence_refs: vec![common_evidence_ref],
        }],
    }
}

fn compiler_evidence(
    evidence_ref: AppGamePolicyEvidenceRef,
    proof_kind: AppGamePolicyCompilerProofKind,
) -> AppGamePolicyCompilerEvidence {
    AppGamePolicyCompilerEvidence {
        evidence_ref,
        proof_kind,
        evidence_state: AppGamePolicyCompilerEvidenceState::Active,
        device_id: AppGamePolicyDeviceId::parse("device-time-budget").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-time-budget")
            .expect_value("user ref"),
    }
}

fn compiler_context() -> AppGamePolicyCompilerContext {
    AppGamePolicyCompilerContext {
        compiled_decision_id: AppGamePolicyCompiledDecisionId::parse("decision-time-budget")
            .expect_value("decision id"),
        audit_ref: audit_ref("audit-compile-time-budget"),
    }
}

fn evidence_ref(value: &str) -> AppGamePolicyEvidenceRef {
    AppGamePolicyEvidenceRef::parse(value).expect_value("evidence ref")
}

fn schedule_ref(value: &str) -> AppGamePolicyScheduleRef {
    AppGamePolicyScheduleRef::parse(value).expect_value("schedule ref")
}

fn timer_ref(value: &str) -> AppGamePolicyRuntimeTimerRef {
    AppGamePolicyRuntimeTimerRef::parse(value).expect_value("timer ref")
}

fn bonus_ref(value: &str) -> AppGamePolicyBonusApprovalRef {
    AppGamePolicyBonusApprovalRef::parse(value).expect_value("bonus ref")
}

fn audit_ref(value: &str) -> AppGamePolicyAuditRef {
    AppGamePolicyAuditRef::parse(value).expect_value("audit ref")
}
