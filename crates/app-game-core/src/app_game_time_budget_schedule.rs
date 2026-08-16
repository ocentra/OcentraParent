use ocentra_eventing::error::EventingError;

use crate::app_game_policy_evaluator_runtime::types::AppGamePolicyScheduleState;
use crate::app_game_policy_target_compiler::references::{
    AppGamePolicyEvidenceRef, AppGamePolicyScheduleRef,
};
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompilerEvidenceState, AppGamePolicyCompilerProofKind,
};
use crate::app_game_time_budget_types::{AppGameTimeBudgetInput, AppGameTimeBudgetSchedule};

pub(super) struct AppGameTimeBudgetScheduleParts {
    pub(super) state: AppGamePolicyScheduleState,
    pub(super) schedule_ref: Option<AppGamePolicyScheduleRef>,
    pub(super) evidence_refs: Vec<AppGamePolicyEvidenceRef>,
}

pub(super) fn schedule_parts(
    input: &AppGameTimeBudgetInput,
) -> Result<AppGameTimeBudgetScheduleParts, EventingError> {
    match &input.schedule {
        AppGameTimeBudgetSchedule::NotRequired => not_required_parts(input),
        AppGameTimeBudgetSchedule::Active {
            schedule_ref,
            evidence_refs,
        } => required_parts(
            input,
            schedule_ref,
            evidence_refs,
            AppGamePolicyScheduleState::Active,
        ),
        AppGameTimeBudgetSchedule::OutsideWindow {
            schedule_ref,
            evidence_refs,
        } => required_parts(
            input,
            schedule_ref,
            evidence_refs,
            AppGamePolicyScheduleState::OutsideWindow,
        ),
        AppGameTimeBudgetSchedule::Stale {
            schedule_ref,
            evidence_refs,
        } => required_parts(
            input,
            schedule_ref,
            evidence_refs,
            AppGamePolicyScheduleState::Stale,
        ),
    }
}

fn not_required_parts(
    input: &AppGameTimeBudgetInput,
) -> Result<AppGameTimeBudgetScheduleParts, EventingError> {
    if input.compilation.decision.request.schedule_ref.is_some() {
        return Err(invalid_schedule(
            "compiler carries an unexpected schedule ref",
        ));
    }
    Ok(AppGameTimeBudgetScheduleParts {
        state: AppGamePolicyScheduleState::NotRequired,
        schedule_ref: None,
        evidence_refs: Vec::new(),
    })
}

fn required_parts(
    input: &AppGameTimeBudgetInput,
    schedule_ref: &AppGamePolicyScheduleRef,
    evidence_refs: &[AppGamePolicyEvidenceRef],
    state: AppGamePolicyScheduleState,
) -> Result<AppGameTimeBudgetScheduleParts, EventingError> {
    if input.compilation.decision.request.schedule_ref.as_ref() != Some(schedule_ref) {
        return Err(invalid_schedule(
            "runtime schedule ref does not match compiler input",
        ));
    }
    if evidence_refs.is_empty()
        || !evidence_refs.iter().all(|evidence_ref| {
            input
                .compilation
                .decision
                .request
                .evidence
                .iter()
                .any(|evidence| {
                    evidence.evidence_ref == *evidence_ref
                        && evidence.proof_kind == AppGamePolicyCompilerProofKind::ScheduleProof
                        && evidence.evidence_state == AppGamePolicyCompilerEvidenceState::Active
                })
        })
    {
        return Err(invalid_schedule(
            "schedule evidence is missing or not active",
        ));
    }
    Ok(AppGameTimeBudgetScheduleParts {
        state,
        schedule_ref: Some(schedule_ref.clone()),
        evidence_refs: evidence_refs.to_vec(),
    })
}

fn invalid_schedule(value: &'static str) -> EventingError {
    EventingError::InvalidValue {
        field: "app_game.time_budget.schedule",
        value: String::from(value),
    }
}
