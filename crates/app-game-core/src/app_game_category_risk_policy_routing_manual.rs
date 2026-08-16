use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;

use super::types::{
    AppGameCategoryRiskCandidateSource, AppGameCategoryRiskRouteReason,
    AppGameCategoryRiskRouteRequest,
};

pub(super) fn manual_reason(
    request: &AppGameCategoryRiskRouteRequest,
) -> Option<AppGameCategoryRiskRouteReason> {
    if request.candidate.candidate_source == AppGameCategoryRiskCandidateSource::ParentManualReview
    {
        return Some(AppGameCategoryRiskRouteReason::CandidateRequiresManualReview);
    }
    is_hard_action(request.candidate.requested_action)
        .then_some(AppGameCategoryRiskRouteReason::HardActionRequiresManualReview)
}

fn is_hard_action(action: AppGamePolicyCompilerRequestedAction) -> bool {
    matches!(
        action,
        AppGamePolicyCompilerRequestedAction::TerminateRunning
            | AppGamePolicyCompilerRequestedAction::BlockLaunch
            | AppGamePolicyCompilerRequestedAction::HideApp
            | AppGamePolicyCompilerRequestedAction::SuspendApp
            | AppGamePolicyCompilerRequestedAction::ShieldApp
    )
}
