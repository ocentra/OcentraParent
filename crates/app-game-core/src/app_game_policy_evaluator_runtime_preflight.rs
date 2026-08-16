use super::types::{
    AppGamePolicyBonusState, AppGamePolicyDurationSource, AppGamePolicyEvaluatorInput,
    AppGamePolicyRuntimeDecisionReason, AppGamePolicyRuntimeDecisionState,
    AppGamePolicyScheduleState, AppGamePolicySessionAccounting,
};

pub(super) fn preflight_state(
    input: &AppGamePolicyEvaluatorInput,
) -> Option<(
    AppGamePolicyRuntimeDecisionState,
    AppGamePolicyRuntimeDecisionReason,
)> {
    if input.duration_source == AppGamePolicyDurationSource::ManualEstimate {
        return Some((
            AppGamePolicyRuntimeDecisionState::ManualRequired,
            AppGamePolicyRuntimeDecisionReason::UntrustedDurationSource,
        ));
    }
    match input.schedule_state {
        AppGamePolicyScheduleState::Stale => Some((
            AppGamePolicyRuntimeDecisionState::ManualRequired,
            AppGamePolicyRuntimeDecisionReason::StaleSchedule,
        )),
        AppGamePolicyScheduleState::OutsideWindow => Some((
            AppGamePolicyRuntimeDecisionState::Observe,
            AppGamePolicyRuntimeDecisionReason::OutsideSchedule,
        )),
        AppGamePolicyScheduleState::NotRequired | AppGamePolicyScheduleState::Active => {
            matches!(input.bonus_state, AppGamePolicyBonusState::Pending).then_some((
                AppGamePolicyRuntimeDecisionState::AskParent,
                AppGamePolicyRuntimeDecisionReason::BonusApprovalPending,
            ))
        }
    }
}

pub(super) fn counted_duration(input: &AppGamePolicyEvaluatorInput) -> Option<u64> {
    input
        .sessions
        .iter()
        .filter(|session| session.accounting == AppGamePolicySessionAccounting::Counted)
        .try_fold(0_u64, |total, session| {
            total.checked_add(session.duration_seconds)
        })
}

pub(super) fn effective_budget(input: &AppGamePolicyEvaluatorInput) -> Option<u64> {
    match &input.bonus_state {
        AppGamePolicyBonusState::Approved {
            additional_seconds, ..
        } => input.budget_seconds.checked_add(*additional_seconds),
        AppGamePolicyBonusState::None | AppGamePolicyBonusState::Pending => {
            Some(input.budget_seconds)
        }
    }
}
