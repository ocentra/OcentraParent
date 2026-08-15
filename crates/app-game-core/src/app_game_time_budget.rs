use ocentra_eventing::error::EventingError;

use crate::app_game_policy_evaluator_runtime::evaluate_app_game_policy_runtime;
use crate::app_game_policy_evaluator_runtime::types::AppGamePolicyEvaluatorInput;
use crate::app_game_time_budget_policy::runtime_policy_parts;
use crate::app_game_time_budget_schedule::schedule_parts;
use crate::app_game_time_budget_sessions::runtime_sessions;
use crate::app_game_time_budget_types::{AppGameTimeBudgetDecision, AppGameTimeBudgetInput};

pub fn evaluate_app_game_time_budget(
    input: AppGameTimeBudgetInput,
) -> Result<AppGameTimeBudgetDecision, EventingError> {
    let sessions = runtime_sessions(&input.stored_sessions, input.duration_mode)?;
    let schedule = schedule_parts(&input)?;
    let policy = runtime_policy_parts(&input)?;
    let stored_session_refs = sessions
        .iter()
        .map(|session| session.session_ref.clone())
        .collect();
    let evaluator_input = AppGamePolicyEvaluatorInput {
        compilation: input.compilation,
        evaluation_audit_ref: input.evaluation_audit_ref,
        budget_seconds: input.budget_seconds,
        warning_threshold_seconds: input.warning_threshold_seconds,
        sessions,
        duration_source: input.duration_source,
        schedule_state: schedule.state,
        bonus_state: policy.bonus_state,
        timer_ref: policy.timer_ref,
    };
    let runtime_decision = evaluate_app_game_policy_runtime(&evaluator_input);
    Ok(AppGameTimeBudgetDecision {
        runtime_decision,
        period: input.period,
        duration_mode: input.duration_mode,
        stored_session_refs,
        schedule_ref: schedule.schedule_ref,
        schedule_evidence_refs: schedule.evidence_refs,
        bonus_audit_ref: policy.bonus_audit_ref,
        recovered_timer_ref: policy.recovered_timer_ref,
    })
}
