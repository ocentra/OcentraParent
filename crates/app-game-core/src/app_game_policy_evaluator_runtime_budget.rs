use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;

use super::app_game_policy_evaluator_runtime_decision::build_decision;
use super::types::{
    AppGamePolicyBonusState, AppGamePolicyEvaluatorInput, AppGamePolicyRuntimeDecision,
    AppGamePolicyRuntimeDecisionReason, AppGamePolicyRuntimeDecisionState,
};

pub(super) fn decide_budget_state(
    input: &AppGamePolicyEvaluatorInput,
    consumed_seconds: u64,
    effective_budget_seconds: u64,
) -> AppGamePolicyRuntimeDecision {
    let remaining_seconds = effective_budget_seconds.saturating_sub(consumed_seconds);
    if consumed_seconds >= effective_budget_seconds {
        return exceeded_decision(input, consumed_seconds, effective_budget_seconds);
    }
    if matches!(input.bonus_state, AppGamePolicyBonusState::Approved { .. }) {
        return build_decision(
            input,
            AppGamePolicyRuntimeDecisionState::ApprovedBonusObserve,
            AppGamePolicyRuntimeDecisionReason::ApprovedBonusActive,
            consumed_seconds,
            effective_budget_seconds,
            None,
        );
    }
    if remaining_seconds <= input.warning_threshold_seconds {
        return build_decision(
            input,
            AppGamePolicyRuntimeDecisionState::WarnOnly,
            AppGamePolicyRuntimeDecisionReason::WarningThresholdReached,
            consumed_seconds,
            effective_budget_seconds,
            None,
        );
    }
    build_decision(
        input,
        AppGamePolicyRuntimeDecisionState::Observe,
        AppGamePolicyRuntimeDecisionReason::WithinBudget,
        consumed_seconds,
        effective_budget_seconds,
        None,
    )
}

fn exceeded_decision(
    input: &AppGamePolicyEvaluatorInput,
    consumed_seconds: u64,
    effective_budget_seconds: u64,
) -> AppGamePolicyRuntimeDecision {
    let action = input.compilation.decision.request.requested_action;
    let (state, timer_ref) = match action {
        AppGamePolicyCompilerRequestedAction::Warn => {
            (AppGamePolicyRuntimeDecisionState::WarnOnly, None)
        }
        AppGamePolicyCompilerRequestedAction::AskParent => {
            (AppGamePolicyRuntimeDecisionState::AskParent, None)
        }
        AppGamePolicyCompilerRequestedAction::TimeLimit => match input.timer_ref.clone() {
            Some(timer_ref) => (
                AppGamePolicyRuntimeDecisionState::DryRunTimeLimit,
                Some(timer_ref),
            ),
            None => {
                return build_decision(
                    input,
                    AppGamePolicyRuntimeDecisionState::ManualRequired,
                    AppGamePolicyRuntimeDecisionReason::MissingTimerReference,
                    consumed_seconds,
                    effective_budget_seconds,
                    None,
                );
            }
        },
        AppGamePolicyCompilerRequestedAction::Observe => {
            (AppGamePolicyRuntimeDecisionState::Observe, None)
        }
        _ => (AppGamePolicyRuntimeDecisionState::ManualRequired, None),
    };
    build_decision(
        input,
        state,
        AppGamePolicyRuntimeDecisionReason::BudgetExceeded,
        consumed_seconds,
        effective_budget_seconds,
        timer_ref,
    )
}
