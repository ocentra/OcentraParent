use crate::app_game_child_ux_runtime::{approval_needed, fixed, NoticeParts};
use crate::app_game_child_ux_types::{
    AppGameChildUxInput, AppGameChildUxNoticeState, AppGameChildUxRequestState,
    AppGameChildUxTextToken,
};

pub(super) fn resolve_available(input: &AppGameChildUxInput) -> NoticeParts {
    match input.request_state {
        AppGameChildUxRequestState::ApprovalNeeded => approval_needed(input.subject_kind),
        AppGameChildUxRequestState::Submitted => fixed(
            AppGameChildUxNoticeState::RequestSubmitted,
            AppGameChildUxTextToken::FamilyRuleRequestSubmitted,
        ),
        AppGameChildUxRequestState::Approved => fixed(
            AppGameChildUxNoticeState::RequestApproved,
            AppGameChildUxTextToken::FamilyRuleRequestApproved,
        ),
        AppGameChildUxRequestState::Denied => fixed(
            AppGameChildUxNoticeState::RequestDenied,
            AppGameChildUxTextToken::FamilyRuleRequestDenied,
        ),
        AppGameChildUxRequestState::None => {
            crate::app_game_child_ux_runtime::resolve_runtime(input)
        }
    }
}
