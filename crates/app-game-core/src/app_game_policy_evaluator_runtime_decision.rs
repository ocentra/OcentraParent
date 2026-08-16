use super::types::{
    AppGamePolicyBonusState, AppGamePolicyEvaluatorInput, AppGamePolicyRuntimeAdapterDispatchState,
    AppGamePolicyRuntimeDecision, AppGamePolicyRuntimeDecisionReason,
    AppGamePolicyRuntimeDecisionState, AppGamePolicyRuntimeSessionRef,
    AppGamePolicyRuntimeTimerRef, AppGamePolicySessionAccounting,
};

pub(super) fn evaluate_ready_input(
    input: &AppGamePolicyEvaluatorInput,
) -> AppGamePolicyRuntimeDecision {
    let Some(consumed_seconds) =
        super::app_game_policy_evaluator_runtime_preflight::counted_duration(input)
    else {
        return build_decision(
            input,
            AppGamePolicyRuntimeDecisionState::Rejected,
            AppGamePolicyRuntimeDecisionReason::DurationOverflow,
            0,
            0,
            None,
        );
    };
    let Some(effective_budget_seconds) =
        super::app_game_policy_evaluator_runtime_preflight::effective_budget(input)
    else {
        return build_decision(
            input,
            AppGamePolicyRuntimeDecisionState::Rejected,
            AppGamePolicyRuntimeDecisionReason::DurationOverflow,
            consumed_seconds,
            0,
            None,
        );
    };
    if let Some((state, reason)) =
        super::app_game_policy_evaluator_runtime_preflight::preflight_state(input)
    {
        return build_decision(
            input,
            state,
            reason,
            consumed_seconds,
            effective_budget_seconds,
            None,
        );
    }
    super::app_game_policy_evaluator_runtime_budget::decide_budget_state(
        input,
        consumed_seconds,
        effective_budget_seconds,
    )
}

pub(super) fn build_decision(
    input: &AppGamePolicyEvaluatorInput,
    state: AppGamePolicyRuntimeDecisionState,
    reason: AppGamePolicyRuntimeDecisionReason,
    consumed_seconds: u64,
    effective_budget_seconds: u64,
    timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
) -> AppGamePolicyRuntimeDecision {
    AppGamePolicyRuntimeDecision {
        state,
        reason,
        consumed_seconds,
        effective_budget_seconds,
        remaining_seconds: effective_budget_seconds.saturating_sub(consumed_seconds),
        counted_session_refs: session_refs(input, AppGamePolicySessionAccounting::Counted),
        excluded_session_refs: session_refs(input, AppGamePolicySessionAccounting::Excluded),
        timer_ref,
        bonus_approval_ref: bonus_approval_ref(input),
        audit_ref: input.evaluation_audit_ref.clone(),
        adapter_dispatch_state: AppGamePolicyRuntimeAdapterDispatchState::NotDispatched,
    }
}

fn session_refs(
    input: &AppGamePolicyEvaluatorInput,
    accounting: AppGamePolicySessionAccounting,
) -> Vec<AppGamePolicyRuntimeSessionRef> {
    input
        .sessions
        .iter()
        .filter(|session| session.accounting == accounting)
        .map(|session| session.session_ref.clone())
        .collect()
}

fn bonus_approval_ref(
    input: &AppGamePolicyEvaluatorInput,
) -> Option<super::types::AppGamePolicyBonusApprovalRef> {
    match &input.bonus_state {
        AppGamePolicyBonusState::Approved { approval_ref, .. } => Some(approval_ref.clone()),
        AppGamePolicyBonusState::None | AppGamePolicyBonusState::Pending => None,
    }
}
