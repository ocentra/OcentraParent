use ocentra_eventing::error::EventingError;

use crate::app_game_native_game_budget_types::{
    AppGameNativeGameBudgetSession, AppGameNativeGameCandidateApprovalState,
    AppGameNativeGameSessionKind,
};
use crate::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyRuntimeSession, AppGamePolicySessionAccounting,
};

pub(super) fn runtime_session(
    session: &AppGameNativeGameBudgetSession,
) -> Result<AppGamePolicyRuntimeSession, EventingError> {
    validate_session(session)?;
    Ok(AppGamePolicyRuntimeSession {
        session_ref: session.session_ref.clone(),
        duration_seconds: session.duration_seconds,
        accounting: accounting_for(session),
    })
}

pub(super) fn is_counted_known_game(session: &AppGameNativeGameBudgetSession) -> bool {
    session.kind == AppGameNativeGameSessionKind::KnownGame
}

pub(super) fn is_counted_candidate(session: &AppGameNativeGameBudgetSession) -> bool {
    session.kind == AppGameNativeGameSessionKind::LauncherGameCandidate
        && session.candidate_approval_state
            == AppGameNativeGameCandidateApprovalState::ParentApproved
}

pub(super) fn is_excluded_launcher(session: &AppGameNativeGameBudgetSession) -> bool {
    session.kind == AppGameNativeGameSessionKind::LauncherOnly
}

pub(super) fn is_excluded_candidate(session: &AppGameNativeGameBudgetSession) -> bool {
    session.kind == AppGameNativeGameSessionKind::LauncherGameCandidate
        && session.candidate_approval_state
            != AppGameNativeGameCandidateApprovalState::ParentApproved
}

fn accounting_for(session: &AppGameNativeGameBudgetSession) -> AppGamePolicySessionAccounting {
    if is_counted_known_game(session) || is_counted_candidate(session) {
        AppGamePolicySessionAccounting::Counted
    } else {
        AppGamePolicySessionAccounting::Excluded
    }
}

fn validate_session(session: &AppGameNativeGameBudgetSession) -> Result<(), EventingError> {
    let coherent = matches!(
        (session.kind, session.candidate_approval_state),
        (
            AppGameNativeGameSessionKind::KnownGame | AppGameNativeGameSessionKind::LauncherOnly,
            AppGameNativeGameCandidateApprovalState::NotRequired
        ) | (
            AppGameNativeGameSessionKind::LauncherGameCandidate,
            AppGameNativeGameCandidateApprovalState::ParentApproved
                | AppGameNativeGameCandidateApprovalState::Pending
                | AppGameNativeGameCandidateApprovalState::Denied
                | AppGameNativeGameCandidateApprovalState::Expired
        )
    );
    if !coherent {
        return Err(EventingError::InvalidValue {
            field: "app_game.native_game.candidate_approval_state",
            value: String::from("session kind and candidate approval state disagree"),
        });
    }
    Ok(())
}
