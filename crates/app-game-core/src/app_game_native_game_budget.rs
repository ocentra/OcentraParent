use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;

use crate::app_game_native_game_budget_accounting::{
    is_counted_candidate, is_counted_known_game, is_excluded_candidate, is_excluded_launcher,
    runtime_session,
};
use crate::app_game_native_game_budget_types::{
    AppGameNativeGameAdvisorySignal, AppGameNativeGameBudgetDecision, AppGameNativeGameBudgetInput,
    AppGameNativeGameBudgetSession,
};
use crate::app_game_policy_evaluator_runtime::evaluate_app_game_policy_runtime;
use crate::app_game_policy_target_compiler::types::AppGamePolicyTargetKind;

pub fn evaluate_app_game_native_game_budget(
    mut input: AppGameNativeGameBudgetInput,
) -> Result<AppGameNativeGameBudgetDecision, EventingError> {
    validate_input(&input)?;
    input.evaluator_input.sessions = input
        .sessions
        .iter()
        .map(runtime_session)
        .collect::<Result<Vec<_>, _>>()?;
    let runtime_decision = evaluate_app_game_policy_runtime(&input.evaluator_input);
    Ok(AppGameNativeGameBudgetDecision {
        runtime_decision,
        counted_known_game_session_refs: refs_matching(&input.sessions, is_counted_known_game),
        counted_parent_approved_candidate_session_refs: refs_matching(
            &input.sessions,
            is_counted_candidate,
        ),
        excluded_launcher_only_session_refs: refs_matching(&input.sessions, is_excluded_launcher),
        excluded_unapproved_candidate_session_refs: refs_matching(
            &input.sessions,
            is_excluded_candidate,
        ),
        advisory_signals: advisory_signals(&input.sessions),
    })
}

fn validate_input(input: &AppGameNativeGameBudgetInput) -> Result<(), EventingError> {
    if !input.evaluator_input.sessions.is_empty() {
        return Err(EventingError::InvalidValue {
            field: "app_game.native_game.evaluator_sessions",
            value: String::from("native game composition owns evaluator sessions"),
        });
    }
    let target_kind = input
        .evaluator_input
        .compilation
        .decision
        .request
        .target
        .target_kind;
    if !is_game_target(target_kind) {
        return Err(EventingError::InvalidValue {
            field: "app_game.native_game.target_kind",
            value: format!("{target_kind:?}"),
        });
    }
    let mut refs = BTreeSet::new();
    if input
        .sessions
        .iter()
        .any(|session| !refs.insert(session.session_ref.as_str()))
    {
        return Err(EventingError::InvalidValue {
            field: "app_game.native_game.session_ref",
            value: String::from("duplicate session ref"),
        });
    }
    Ok(())
}

fn is_game_target(target_kind: AppGamePolicyTargetKind) -> bool {
    matches!(
        target_kind,
        AppGamePolicyTargetKind::SpecificGame
            | AppGamePolicyTargetKind::LauncherGameId
            | AppGamePolicyTargetKind::StoreGameId
            | AppGamePolicyTargetKind::GameCategory
            | AppGamePolicyTargetKind::UnknownGame
            | AppGamePolicyTargetKind::NewGame
            | AppGamePolicyTargetKind::LauncherGameCandidate
            | AppGamePolicyTargetKind::MultiplayerGame
            | AppGamePolicyTargetKind::UgcGame
            | AppGamePolicyTargetKind::PurchaseCapableGame
            | AppGamePolicyTargetKind::MatureGame
            | AppGamePolicyTargetKind::AllGames
    )
}

fn refs_matching(
    sessions: &[AppGameNativeGameBudgetSession],
    predicate: fn(&AppGameNativeGameBudgetSession) -> bool,
) -> Vec<crate::app_game_policy_evaluator_runtime::types::AppGamePolicyRuntimeSessionRef> {
    sessions
        .iter()
        .filter(|session| predicate(session))
        .map(|session| session.session_ref.clone())
        .collect()
}

fn advisory_signals(
    sessions: &[AppGameNativeGameBudgetSession],
) -> Vec<AppGameNativeGameAdvisorySignal> {
    sessions
        .iter()
        .flat_map(|session| session.advisory_signals.iter().cloned())
        .collect()
}
