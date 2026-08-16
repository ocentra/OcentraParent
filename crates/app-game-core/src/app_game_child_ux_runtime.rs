use crate::app_game_child_ux_types::{
    AppGameChildUxAction, AppGameChildUxInput, AppGameChildUxNoticeState,
    AppGameChildUxSubjectKind, AppGameChildUxTextToken,
};
use crate::app_game_policy_evaluator_runtime::types::AppGamePolicyRuntimeDecisionState;

pub(super) type NoticeParts = (
    AppGameChildUxNoticeState,
    AppGameChildUxTextToken,
    AppGameChildUxAction,
);

pub(super) fn resolve_runtime(input: &AppGameChildUxInput) -> NoticeParts {
    match input.runtime_decision.state {
        AppGamePolicyRuntimeDecisionState::AskParent => approval_needed(input.subject_kind),
        AppGamePolicyRuntimeDecisionState::WarnOnly
        | AppGamePolicyRuntimeDecisionState::DryRunTimeLimit => warning(input.subject_kind),
        AppGamePolicyRuntimeDecisionState::ManualRequired => manual_required(),
        AppGamePolicyRuntimeDecisionState::Rejected => unavailable(),
        AppGamePolicyRuntimeDecisionState::Observe
        | AppGamePolicyRuntimeDecisionState::ApprovedBonusObserve => fixed(
            AppGameChildUxNoticeState::NoNotice,
            AppGameChildUxTextToken::NoNotice,
        ),
    }
}

pub(super) fn approval_needed(subject: AppGameChildUxSubjectKind) -> NoticeParts {
    let (state, token) = match subject {
        AppGameChildUxSubjectKind::App => (
            AppGameChildUxNoticeState::NewAppNeedsApproval,
            AppGameChildUxTextToken::FamilyRuleNewAppApproval,
        ),
        AppGameChildUxSubjectKind::Game => (
            AppGameChildUxNoticeState::NewGameNeedsApproval,
            AppGameChildUxTextToken::FamilyRuleNewGameApproval,
        ),
    };
    (state, token, AppGameChildUxAction::AskParent)
}

fn warning(subject: AppGameChildUxSubjectKind) -> NoticeParts {
    match subject {
        AppGameChildUxSubjectKind::App => fixed(
            AppGameChildUxNoticeState::AppLimited,
            AppGameChildUxTextToken::FamilyRuleAppLimit,
        ),
        AppGameChildUxSubjectKind::Game => fixed(
            AppGameChildUxNoticeState::GameTimeAlmostFinished,
            AppGameChildUxTextToken::FamilyRuleGameTimeAlmostFinished,
        ),
    }
}

pub(super) fn manual_required() -> NoticeParts {
    fixed(
        AppGameChildUxNoticeState::ManualRequired,
        AppGameChildUxTextToken::FamilyRuleNeedsHelp,
    )
}

pub(super) fn unavailable() -> NoticeParts {
    fixed(
        AppGameChildUxNoticeState::Unavailable,
        AppGameChildUxTextToken::FamilyRuleUnavailable,
    )
}

pub(super) fn fixed(
    state: AppGameChildUxNoticeState,
    token: AppGameChildUxTextToken,
) -> NoticeParts {
    (state, token, AppGameChildUxAction::None)
}
