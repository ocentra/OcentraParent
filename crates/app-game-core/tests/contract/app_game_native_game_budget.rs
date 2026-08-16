use ocentra_app_game_core::app_game_native_game_budget::evaluate_app_game_native_game_budget;
use ocentra_app_game_core::app_game_native_game_budget_types::{
    AppGameNativeGameAdvisorySignal, AppGameNativeGameAdvisorySignalKind,
    AppGameNativeGameBudgetInput, AppGameNativeGameBudgetSession,
    AppGameNativeGameCandidateApprovalState, AppGameNativeGameSessionKind,
};
use ocentra_app_game_core::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyBonusState, AppGamePolicyDurationSource, AppGamePolicyEvaluatorInput,
    AppGamePolicyRuntimeAdapterDispatchState, AppGamePolicyRuntimeDecisionState,
    AppGamePolicyRuntimeSession, AppGamePolicyRuntimeSessionRef, AppGamePolicyScheduleState,
    AppGamePolicySessionAccounting,
};
use ocentra_app_game_core::app_game_policy_target_compiler::compile_app_game_policy_target;
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef,
    AppGamePolicyCompileRequestId, AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId,
    AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef, AppGamePolicyRuleRef,
};
use ocentra_app_game_core::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityEvidence,
    AppGamePolicyCompilerAuthorityState, AppGamePolicyCompilerCapabilityEvidence,
    AppGamePolicyCompilerCapabilityState, AppGamePolicyCompilerContext,
    AppGamePolicyCompilerEvidence, AppGamePolicyCompilerEvidenceState,
    AppGamePolicyCompilerProofKind, AppGamePolicyCompilerRequestedAction,
    AppGamePolicyCompilerTarget, AppGamePolicyTargetKind,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn known_games_count_and_launcher_only_sessions_are_excluded_by_default(
) -> Result<(), EventingError> {
    let input = budget_input(vec![
        session(
            "known-game-1",
            80,
            AppGameNativeGameSessionKind::KnownGame,
            AppGameNativeGameCandidateApprovalState::NotRequired,
        ),
        session(
            "launcher-1",
            600,
            AppGameNativeGameSessionKind::LauncherOnly,
            AppGameNativeGameCandidateApprovalState::NotRequired,
        ),
    ]);
    let decision = evaluate_app_game_native_game_budget(input)?;
    assert_eq!(decision.runtime_decision.consumed_seconds, 80);
    assert_eq!(
        decision.counted_known_game_session_refs,
        vec![session_ref("known-game-1")]
    );
    assert_eq!(
        decision.excluded_launcher_only_session_refs,
        vec![session_ref("launcher-1")]
    );
    assert_eq!(
        decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::WarnOnly
    );
    Ok(())
}

#[test]
fn launcher_game_candidates_count_only_after_parent_approval() -> Result<(), EventingError> {
    let input = budget_input(vec![
        session(
            "candidate-approved",
            40,
            AppGameNativeGameSessionKind::LauncherGameCandidate,
            AppGameNativeGameCandidateApprovalState::ParentApproved,
        ),
        session(
            "candidate-pending",
            500,
            AppGameNativeGameSessionKind::LauncherGameCandidate,
            AppGameNativeGameCandidateApprovalState::Pending,
        ),
    ]);
    let decision = evaluate_app_game_native_game_budget(input)?;
    assert_eq!(decision.runtime_decision.consumed_seconds, 40);
    assert_eq!(
        decision.counted_parent_approved_candidate_session_refs,
        vec![session_ref("candidate-approved")]
    );
    assert_eq!(
        decision.excluded_unapproved_candidate_session_refs,
        vec![session_ref("candidate-pending")]
    );
    Ok(())
}

#[test]
fn rating_ugc_multiplayer_and_purchase_signals_remain_advisory() -> Result<(), EventingError> {
    let mut game = session(
        "known-game-signals",
        20,
        AppGameNativeGameSessionKind::KnownGame,
        AppGameNativeGameCandidateApprovalState::NotRequired,
    );
    game.advisory_signals = [
        AppGameNativeGameAdvisorySignalKind::Rating,
        AppGameNativeGameAdvisorySignalKind::UserGeneratedContent,
        AppGameNativeGameAdvisorySignalKind::Multiplayer,
        AppGameNativeGameAdvisorySignalKind::PurchaseCapable,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, kind)| AppGameNativeGameAdvisorySignal {
        kind,
        evidence_ref: evidence_ref(&format!("signal-{index}")),
    })
    .collect();
    let decision = evaluate_app_game_native_game_budget(budget_input(vec![game]))?;
    assert_eq!(decision.advisory_signals.len(), 4);
    assert_eq!(decision.runtime_decision.consumed_seconds, 20);
    assert_eq!(
        decision.runtime_decision.adapter_dispatch_state,
        AppGamePolicyRuntimeAdapterDispatchState::NotDispatched
    );
    assert_eq!(
        decision.runtime_decision.state,
        AppGamePolicyRuntimeDecisionState::Observe
    );
    Ok(())
}

#[test]
fn native_game_composition_rejects_non_game_targets_and_prepopulated_sessions() {
    let mut wrong_target = budget_input(Vec::new());
    wrong_target
        .evaluator_input
        .compilation
        .decision
        .request
        .target
        .target_kind = AppGamePolicyTargetKind::AppCategory;
    assert!(matches!(
        evaluate_app_game_native_game_budget(wrong_target),
        Err(EventingError::InvalidValue {
            field: "app_game.native_game.target_kind",
            ..
        })
    ));

    let mut bypass = budget_input(Vec::new());
    bypass
        .evaluator_input
        .sessions
        .push(AppGamePolicyRuntimeSession {
            session_ref: session_ref("bypass-session"),
            duration_seconds: 10,
            accounting: AppGamePolicySessionAccounting::Counted,
        });
    assert!(matches!(
        evaluate_app_game_native_game_budget(bypass),
        Err(EventingError::InvalidValue {
            field: "app_game.native_game.evaluator_sessions",
            ..
        })
    ));
}

#[test]
fn native_game_composition_rejects_duplicate_and_incoherent_session_state() {
    let duplicate = session(
        "duplicate-session",
        10,
        AppGameNativeGameSessionKind::KnownGame,
        AppGameNativeGameCandidateApprovalState::NotRequired,
    );
    assert!(matches!(
        evaluate_app_game_native_game_budget(budget_input(vec![duplicate.clone(), duplicate])),
        Err(EventingError::InvalidValue {
            field: "app_game.native_game.session_ref",
            ..
        })
    ));

    let incoherent = session(
        "incoherent-session",
        10,
        AppGameNativeGameSessionKind::LauncherOnly,
        AppGameNativeGameCandidateApprovalState::ParentApproved,
    );
    assert!(matches!(
        evaluate_app_game_native_game_budget(budget_input(vec![incoherent])),
        Err(EventingError::InvalidValue {
            field: "app_game.native_game.candidate_approval_state",
            ..
        })
    ));
}

fn budget_input(sessions: Vec<AppGameNativeGameBudgetSession>) -> AppGameNativeGameBudgetInput {
    AppGameNativeGameBudgetInput {
        evaluator_input: AppGamePolicyEvaluatorInput {
            compilation: compile_app_game_policy_target(compile_request(), compiler_context()),
            evaluation_audit_ref: AppGamePolicyAuditRef::parse("audit-native-budget")
                .expect_value("audit ref"),
            budget_seconds: 100,
            warning_threshold_seconds: 20,
            sessions: Vec::new(),
            duration_source: AppGamePolicyDurationSource::AuthoritativeSession,
            schedule_state: AppGamePolicyScheduleState::Active,
            bonus_state: AppGamePolicyBonusState::None,
            timer_ref: None,
        },
        sessions,
    }
}

fn session(
    value: &str,
    duration_seconds: u64,
    kind: AppGameNativeGameSessionKind,
    candidate_approval_state: AppGameNativeGameCandidateApprovalState,
) -> AppGameNativeGameBudgetSession {
    AppGameNativeGameBudgetSession {
        session_ref: session_ref(value),
        duration_seconds,
        kind,
        candidate_approval_state,
        advisory_signals: Vec::new(),
    }
}

fn compile_request() -> AppGamePolicyCompileRequest {
    let evidence_ref = evidence_ref("evidence-native-budget");
    AppGamePolicyCompileRequest {
        compile_request_id: AppGamePolicyCompileRequestId::parse("compile-native-budget")
            .expect_value("compile request id"),
        rule_ref: AppGamePolicyRuleRef::parse("rule-native-budget").expect_value("rule ref"),
        device_id: AppGamePolicyDeviceId::parse("device-native-budget").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-native-budget")
            .expect_value("local user ref"),
        target: AppGamePolicyCompilerTarget {
            target_kind: AppGamePolicyTargetKind::AllGames,
            target_ref: None,
        },
        requested_action: AppGamePolicyCompilerRequestedAction::Warn,
        schedule_ref: None,
        evidence: vec![
            compiler_evidence(
                evidence_ref.clone(),
                AppGamePolicyCompilerProofKind::CapabilityProof,
            ),
            compiler_evidence(
                evidence_ref.clone(),
                AppGamePolicyCompilerProofKind::AuthorityProof,
            ),
        ],
        capability_refs: vec![AppGamePolicyCompilerCapabilityEvidence {
            capability_ref: AppGamePolicyCapabilityRef::parse("capability-native-budget")
                .expect_value("capability ref"),
            capability_state: AppGamePolicyCompilerCapabilityState::Supported,
            evidence_refs: vec![evidence_ref.clone()],
        }],
        authority_refs: vec![AppGamePolicyCompilerAuthorityEvidence {
            authority_ref: AppGamePolicyAuthorityRef::parse("authority-native-budget")
                .expect_value("authority ref"),
            authority_state: AppGamePolicyCompilerAuthorityState::Proved,
            evidence_refs: vec![evidence_ref],
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
        device_id: AppGamePolicyDeviceId::parse("device-native-budget").expect_value("device id"),
        local_user_ref: AppGamePolicyLocalUserRef::parse("user-native-budget")
            .expect_value("local user ref"),
    }
}

fn compiler_context() -> AppGamePolicyCompilerContext {
    AppGamePolicyCompilerContext {
        compiled_decision_id: AppGamePolicyCompiledDecisionId::parse("decision-native-budget")
            .expect_value("decision id"),
        audit_ref: AppGamePolicyAuditRef::parse("audit-compile-native-budget")
            .expect_value("audit ref"),
    }
}

fn evidence_ref(value: &str) -> AppGamePolicyEvidenceRef {
    AppGamePolicyEvidenceRef::parse(value).expect_value("evidence ref")
}

fn session_ref(value: &str) -> AppGamePolicyRuntimeSessionRef {
    AppGamePolicyRuntimeSessionRef::parse(value).expect_value("session ref")
}
