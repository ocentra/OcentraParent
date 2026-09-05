use ocentra_app_game_core::app_game_policy_evaluator_runtime::evaluate_app_game_policy_runtime;
use ocentra_app_game_core::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyBonusApprovalRef, AppGamePolicyBonusState, AppGamePolicyDurationSource,
    AppGamePolicyEvaluatorInput, AppGamePolicyRuntimeAdapterDispatchState,
    AppGamePolicyRuntimeDecisionReason, AppGamePolicyRuntimeDecisionState,
    AppGamePolicyRuntimeSession, AppGamePolicyRuntimeSessionRef, AppGamePolicyRuntimeTimerRef,
    AppGamePolicyScheduleState, AppGamePolicySessionAccounting,
};
use ocentra_app_game_core::app_game_policy_target_compiler::compile_app_game_policy_target;
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef,
    AppGamePolicyCompileRequestId, AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId,
    AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef, AppGamePolicyRuleRef,
    AppGamePolicyTargetRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityEvidence,
    AppGamePolicyCompilerAuthorityState, AppGamePolicyCompilerCapabilityEvidence,
    AppGamePolicyCompilerCapabilityState, AppGamePolicyCompilerContext,
    AppGamePolicyCompilerEvidence, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerOutcomeState, AppGamePolicyCompilerProofKind,
    AppGamePolicyCompilerRequestedAction, AppGamePolicyCompilerTarget, AppGamePolicyTargetKind,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn policy_evaluator_runtime_counts_sessions_and_preserves_exclusions() {
    let input = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    let decision = evaluate_app_game_policy_runtime(&input);

    assert_eq!(decision.state, AppGamePolicyRuntimeDecisionState::WarnOnly);
    assert_eq!(
        decision.reason,
        AppGamePolicyRuntimeDecisionReason::WarningThresholdReached
    );
    assert_eq!(decision.consumed_seconds, 90);
    assert_eq!(decision.remaining_seconds, 30);
    assert_eq!(
        decision.counted_session_refs,
        vec![session_ref("session-counted")]
    );
    assert_eq!(
        decision.excluded_session_refs,
        vec![session_ref("session-excluded")]
    );
}

#[test]
fn policy_evaluator_runtime_emits_dry_run_time_limit_only_with_timer_ref() {
    let mut input = base_input(AppGamePolicyCompilerRequestedAction::TimeLimit);
    input.budget_seconds = 80;
    input.timer_ref = Some(timer_ref("timer-budget-1"));
    let decision = evaluate_app_game_policy_runtime(&input);

    assert_eq!(
        decision.state,
        AppGamePolicyRuntimeDecisionState::DryRunTimeLimit
    );
    assert_eq!(decision.timer_ref, Some(timer_ref("timer-budget-1")));
    assert_eq!(
        decision.adapter_dispatch_state,
        AppGamePolicyRuntimeAdapterDispatchState::NotDispatched
    );

    input.timer_ref = None;
    let missing = evaluate_app_game_policy_runtime(&input);
    assert_eq!(
        missing.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        missing.reason,
        AppGamePolicyRuntimeDecisionReason::MissingTimerReference
    );
    assert_eq!(missing.timer_ref, None);
}

#[test]
fn policy_evaluator_runtime_keeps_bonus_pending_and_approved_states_explicit() {
    let mut input = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    input.bonus_state = AppGamePolicyBonusState::Pending;
    let pending = evaluate_app_game_policy_runtime(&input);
    assert_eq!(pending.state, AppGamePolicyRuntimeDecisionState::AskParent);
    assert_eq!(
        pending.reason,
        AppGamePolicyRuntimeDecisionReason::BonusApprovalPending
    );

    input.bonus_state = AppGamePolicyBonusState::Approved {
        additional_seconds: 60,
        approval_ref: bonus_ref("bonus-approved-1"),
    };
    let approved = evaluate_app_game_policy_runtime(&input);
    assert_eq!(
        approved.state,
        AppGamePolicyRuntimeDecisionState::ApprovedBonusObserve
    );
    assert_eq!(approved.effective_budget_seconds, 180);
    assert_eq!(
        approved.bonus_approval_ref,
        Some(bonus_ref("bonus-approved-1"))
    );
}

#[test]
fn policy_evaluator_runtime_fails_closed_for_stale_schedule_and_manual_duration() {
    let mut input = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    input.schedule_state = AppGamePolicyScheduleState::Stale;
    let stale = evaluate_app_game_policy_runtime(&input);
    assert_eq!(
        stale.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        stale.reason,
        AppGamePolicyRuntimeDecisionReason::StaleSchedule
    );

    input.schedule_state = AppGamePolicyScheduleState::Active;
    input.duration_source = AppGamePolicyDurationSource::ManualEstimate;
    let manual = evaluate_app_game_policy_runtime(&input);
    assert_eq!(
        manual.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        manual.reason,
        AppGamePolicyRuntimeDecisionReason::UntrustedDurationSource
    );
}

#[test]
fn policy_evaluator_runtime_rejects_compiler_failure_and_duration_overflow() {
    let mut compiler_failed = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    compiler_failed.compilation.decision.outcome_state =
        AppGamePolicyCompilerOutcomeState::Rejected;
    let rejected = evaluate_app_game_policy_runtime(&compiler_failed);
    assert_eq!(rejected.state, AppGamePolicyRuntimeDecisionState::Rejected);
    assert_eq!(
        rejected.reason,
        AppGamePolicyRuntimeDecisionReason::CompilerRejected
    );

    let mut overflow = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    overflow.sessions[0].duration_seconds = u64::MAX;
    overflow.sessions.push(AppGamePolicyRuntimeSession {
        session_ref: session_ref("session-overflow"),
        duration_seconds: 1,
        accounting: AppGamePolicySessionAccounting::Counted,
    });
    let overflowed = evaluate_app_game_policy_runtime(&overflow);
    assert_eq!(
        overflowed.state,
        AppGamePolicyRuntimeDecisionState::Rejected
    );
    assert_eq!(
        overflowed.reason,
        AppGamePolicyRuntimeDecisionReason::DurationOverflow
    );
}

#[test]
fn policy_evaluator_runtime_preserves_compiler_manual_required_terminal_state() {
    let mut input = base_input(AppGamePolicyCompilerRequestedAction::Warn);
    input.compilation.decision.outcome_state = AppGamePolicyCompilerOutcomeState::ManualRequired;

    let decision = evaluate_app_game_policy_runtime(&input);

    assert_eq!(
        decision.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        decision.reason,
        AppGamePolicyRuntimeDecisionReason::CompilerManualRequired
    );
    assert_eq!(decision.consumed_seconds, 0);
    assert_eq!(decision.effective_budget_seconds, 0);
    assert_eq!(
        decision.adapter_dispatch_state,
        AppGamePolicyRuntimeAdapterDispatchState::NotDispatched
    );
}

#[test]
fn policy_evaluator_runtime_observes_outside_schedule_without_budget_action() {
    let mut input = base_input(AppGamePolicyCompilerRequestedAction::TimeLimit);
    input.schedule_state = AppGamePolicyScheduleState::OutsideWindow;
    input.timer_ref = Some(timer_ref("timer-outside-window"));

    let decision = evaluate_app_game_policy_runtime(&input);

    assert_eq!(decision.state, AppGamePolicyRuntimeDecisionState::Observe);
    assert_eq!(
        decision.reason,
        AppGamePolicyRuntimeDecisionReason::OutsideSchedule
    );
    assert_eq!(decision.consumed_seconds, 90);
    assert_eq!(decision.effective_budget_seconds, 120);
    assert_eq!(decision.timer_ref, None);
}

#[test]
fn policy_evaluator_runtime_keeps_unsupported_exceeded_action_manual_required() {
    let input = base_input(AppGamePolicyCompilerRequestedAction::BlockLaunch);
    let decision = evaluate_app_game_policy_runtime(&input);

    assert_eq!(
        decision.state,
        AppGamePolicyRuntimeDecisionState::ManualRequired
    );
    assert_eq!(
        decision.reason,
        AppGamePolicyRuntimeDecisionReason::CompilerManualRequired
    );
    assert_eq!(
        decision.adapter_dispatch_state,
        AppGamePolicyRuntimeAdapterDispatchState::NotDispatched
    );
}

fn base_input(action: AppGamePolicyCompilerRequestedAction) -> AppGamePolicyEvaluatorInput {
    AppGamePolicyEvaluatorInput {
        compilation: compile_app_game_policy_target(compile_request(action), compiler_context()),
        evaluation_audit_ref: AppGamePolicyAuditRef::parse("audit-runtime-evaluation-1")
            .expect_value("audit ref"),
        budget_seconds: 120,
        warning_threshold_seconds: 30,
        sessions: vec![
            AppGamePolicyRuntimeSession {
                session_ref: session_ref("session-counted"),
                duration_seconds: 90,
                accounting: AppGamePolicySessionAccounting::Counted,
            },
            AppGamePolicyRuntimeSession {
                session_ref: session_ref("session-excluded"),
                duration_seconds: 300,
                accounting: AppGamePolicySessionAccounting::Excluded,
            },
        ],
        duration_source: AppGamePolicyDurationSource::AuthoritativeSession,
        schedule_state: AppGamePolicyScheduleState::Active,
        bonus_state: AppGamePolicyBonusState::None,
        timer_ref: None,
    }
}

fn compile_request(action: AppGamePolicyCompilerRequestedAction) -> AppGamePolicyCompileRequest {
    let evidence_ref =
        AppGamePolicyEvidenceRef::parse("evidence-runtime-1").expect_value("evidence ref");
    AppGamePolicyCompileRequest {
        compile_request_id: AppGamePolicyCompileRequestId::parse("compile-runtime-1")
            .expect_value("compile id"),
        rule_ref: AppGamePolicyRuleRef::parse("rule-runtime-1").expect_value("rule ref"),
        device_id: AppGamePolicyDeviceId::parse("device-runtime-1").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-runtime-1").expect_value("user ref"),
        target: AppGamePolicyCompilerTarget {
            target_kind: AppGamePolicyTargetKind::AppCategory,
            target_ref: Some(
                AppGamePolicyTargetRef::parse("target-runtime-1").expect_value("target ref"),
            ),
        },
        requested_action: action,
        schedule_ref: None,
        evidence: vec![
            AppGamePolicyCompilerEvidence {
                evidence_ref: evidence_ref.clone(),
                proof_kind: AppGamePolicyCompilerProofKind::CategoryProof,
                evidence_state: AppGamePolicyCompilerEvidenceState::Active,
                device_id: AppGamePolicyDeviceId::parse("device-runtime-1")
                    .expect_value("device id"),
                local_user_ref: AppGamePolicyLocalUserRef::parse("user-runtime-1")
                    .expect_value("user ref"),
            },
            AppGamePolicyCompilerEvidence {
                evidence_ref: evidence_ref.clone(),
                proof_kind: AppGamePolicyCompilerProofKind::CapabilityProof,
                evidence_state: AppGamePolicyCompilerEvidenceState::Active,
                device_id: AppGamePolicyDeviceId::parse("device-runtime-1")
                    .expect_value("device id"),
                local_user_ref: AppGamePolicyLocalUserRef::parse("user-runtime-1")
                    .expect_value("user ref"),
            },
            AppGamePolicyCompilerEvidence {
                evidence_ref: evidence_ref.clone(),
                proof_kind: AppGamePolicyCompilerProofKind::AuthorityProof,
                evidence_state: AppGamePolicyCompilerEvidenceState::Active,
                device_id: AppGamePolicyDeviceId::parse("device-runtime-1")
                    .expect_value("device id"),
                local_user_ref: AppGamePolicyLocalUserRef::parse("user-runtime-1")
                    .expect_value("user ref"),
            },
        ],
        capability_refs: vec![AppGamePolicyCompilerCapabilityEvidence {
            capability_ref: AppGamePolicyCapabilityRef::parse("capability-runtime-1")
                .expect_value("capability ref"),
            capability_state: AppGamePolicyCompilerCapabilityState::Supported,
            evidence_refs: vec![evidence_ref.clone()],
        }],
        authority_refs: vec![AppGamePolicyCompilerAuthorityEvidence {
            authority_ref: AppGamePolicyAuthorityRef::parse("authority-runtime-1")
                .expect_value("authority ref"),
            authority_state: AppGamePolicyCompilerAuthorityState::Proved,
            evidence_refs: vec![evidence_ref],
        }],
    }
}

fn compiler_context() -> AppGamePolicyCompilerContext {
    AppGamePolicyCompilerContext {
        compiled_decision_id: AppGamePolicyCompiledDecisionId::parse("decision-runtime-1")
            .expect_value("decision id"),
        audit_ref: AppGamePolicyAuditRef::parse("audit-compile-runtime-1")
            .expect_value("audit ref"),
    }
}

fn session_ref(value: &str) -> AppGamePolicyRuntimeSessionRef {
    AppGamePolicyRuntimeSessionRef::parse(value).expect_value("session ref")
}

fn timer_ref(value: &str) -> AppGamePolicyRuntimeTimerRef {
    AppGamePolicyRuntimeTimerRef::parse(value).expect_value("timer ref")
}

fn bonus_ref(value: &str) -> AppGamePolicyBonusApprovalRef {
    AppGamePolicyBonusApprovalRef::parse(value).expect_value("bonus ref")
}
