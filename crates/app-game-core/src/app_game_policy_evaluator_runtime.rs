#[path = "app_game_policy_evaluator_runtime_budget.rs"]
mod app_game_policy_evaluator_runtime_budget;
#[path = "app_game_policy_evaluator_runtime_decision.rs"]
mod app_game_policy_evaluator_runtime_decision;
#[path = "app_game_policy_evaluator_runtime_preflight.rs"]
mod app_game_policy_evaluator_runtime_preflight;
#[path = "app_game_policy_evaluator_runtime_types.rs"]
pub mod types;

use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerOutcomeState;
use types::{
    AppGamePolicyEvaluatorInput, AppGamePolicyRuntimeDecision, AppGamePolicyRuntimeDecisionReason,
    AppGamePolicyRuntimeDecisionState,
};

pub fn evaluate_app_game_policy_runtime(
    input: &AppGamePolicyEvaluatorInput,
) -> AppGamePolicyRuntimeDecision {
    match input.compilation.decision.outcome_state {
        AppGamePolicyCompilerOutcomeState::Rejected => terminal_decision(
            input,
            AppGamePolicyRuntimeDecisionState::Rejected,
            AppGamePolicyRuntimeDecisionReason::CompilerRejected,
        ),
        AppGamePolicyCompilerOutcomeState::ManualRequired => terminal_decision(
            input,
            AppGamePolicyRuntimeDecisionState::ManualRequired,
            AppGamePolicyRuntimeDecisionReason::CompilerManualRequired,
        ),
        AppGamePolicyCompilerOutcomeState::DryRunReady => {
            app_game_policy_evaluator_runtime_decision::evaluate_ready_input(input)
        }
    }
}

pub(super) fn terminal_decision(
    input: &AppGamePolicyEvaluatorInput,
    state: AppGamePolicyRuntimeDecisionState,
    reason: AppGamePolicyRuntimeDecisionReason,
) -> AppGamePolicyRuntimeDecision {
    app_game_policy_evaluator_runtime_decision::build_decision(input, state, reason, 0, 0, None)
}
