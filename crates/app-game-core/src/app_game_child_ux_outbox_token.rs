use crate::app_game_child_ux_types::{AppGameChildUxNotice, AppGameChildUxTextToken};

pub(super) fn text_token(notice: &AppGameChildUxNotice) -> &'static str {
    match notice.text_token {
        AppGameChildUxTextToken::NoNotice => "no-notice",
        AppGameChildUxTextToken::FamilyRuleAppLimit => "family-rule-app-limit",
        AppGameChildUxTextToken::FamilyRuleNewAppApproval => "family-rule-new-app-approval",
        AppGameChildUxTextToken::FamilyRuleNewGameApproval => "family-rule-new-game-approval",
        AppGameChildUxTextToken::FamilyRuleGameTimeAlmostFinished => {
            "family-rule-game-time-almost-finished"
        }
        AppGameChildUxTextToken::FamilyRuleRequestSubmitted => "family-rule-request-submitted",
        AppGameChildUxTextToken::FamilyRuleRequestApproved => "family-rule-request-approved",
        AppGameChildUxTextToken::FamilyRuleRequestDenied => "family-rule-request-denied",
        AppGameChildUxTextToken::FamilyRuleNeedsHelp => "family-rule-needs-help",
        AppGameChildUxTextToken::FamilyRuleUnavailable => "family-rule-unavailable",
    }
}
