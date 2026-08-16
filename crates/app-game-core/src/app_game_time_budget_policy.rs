use ocentra_eventing::error::EventingError;

use crate::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyBonusState, AppGamePolicyDurationSource, AppGamePolicyRuntimeTimerRef,
};
use crate::app_game_policy_target_compiler::references::AppGamePolicyAuditRef;
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;
use crate::app_game_time_budget_types::{
    AppGameTimeBudgetBonus, AppGameTimeBudgetInput, AppGameTimeBudgetTimer,
};

pub(super) struct AppGameTimeBudgetPolicyParts {
    pub(super) bonus_state: AppGamePolicyBonusState,
    pub(super) bonus_audit_ref: Option<AppGamePolicyAuditRef>,
    pub(super) timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
    pub(super) recovered_timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
}

pub(super) fn runtime_policy_parts(
    input: &AppGameTimeBudgetInput,
) -> Result<AppGameTimeBudgetPolicyParts, EventingError> {
    validate_duration_source(input.duration_source)?;
    let (bonus_state, bonus_audit_ref) = bonus_parts(&input.bonus);
    let (timer_ref, recovered_timer_ref) = timer_parts(input)?;
    Ok(AppGameTimeBudgetPolicyParts {
        bonus_state,
        bonus_audit_ref,
        timer_ref,
        recovered_timer_ref,
    })
}

fn validate_duration_source(source: AppGamePolicyDurationSource) -> Result<(), EventingError> {
    if source == AppGamePolicyDurationSource::ManualEstimate {
        return Err(EventingError::InvalidValue {
            field: "app_game.time_budget.duration_source",
            value: String::from("stored session summaries cannot be manual estimates"),
        });
    }
    Ok(())
}

fn bonus_parts(
    bonus: &AppGameTimeBudgetBonus,
) -> (AppGamePolicyBonusState, Option<AppGamePolicyAuditRef>) {
    match bonus {
        AppGameTimeBudgetBonus::None => (AppGamePolicyBonusState::None, None),
        AppGameTimeBudgetBonus::Pending { request_audit_ref } => (
            AppGamePolicyBonusState::Pending,
            Some(request_audit_ref.clone()),
        ),
        AppGameTimeBudgetBonus::Approved {
            additional_seconds,
            approval_ref,
            approval_audit_ref,
        } => (
            AppGamePolicyBonusState::Approved {
                additional_seconds: *additional_seconds,
                approval_ref: approval_ref.clone(),
            },
            Some(approval_audit_ref.clone()),
        ),
    }
}

fn timer_parts(
    input: &AppGameTimeBudgetInput,
) -> Result<
    (
        Option<AppGamePolicyRuntimeTimerRef>,
        Option<AppGamePolicyRuntimeTimerRef>,
    ),
    EventingError,
> {
    let action = input.compilation.decision.request.requested_action;
    match &input.timer {
        AppGameTimeBudgetTimer::NotRequired => Ok((None, None)),
        AppGameTimeBudgetTimer::Active { timer_ref } => {
            validate_timer_action(action)?;
            Ok((Some(timer_ref.clone()), None))
        }
        AppGameTimeBudgetTimer::Recovered { timer_ref } => {
            validate_timer_action(action)?;
            Ok((Some(timer_ref.clone()), Some(timer_ref.clone())))
        }
    }
}

fn validate_timer_action(
    action: AppGamePolicyCompilerRequestedAction,
) -> Result<(), EventingError> {
    if action != AppGamePolicyCompilerRequestedAction::TimeLimit {
        return Err(EventingError::InvalidValue {
            field: "app_game.time_budget.timer",
            value: String::from("timer state requires a time-limit compiler action"),
        });
    }
    Ok(())
}
