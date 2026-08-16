use crate::app_game_unknown_approval_types::{
    AppGameUnknownAdapterCapabilityState, AppGameUnknownApprovalStatus,
    AppGameUnknownParentResponse,
};

pub(crate) fn response_status(
    response: AppGameUnknownParentResponse,
    capability_state: AppGameUnknownAdapterCapabilityState,
) -> AppGameUnknownApprovalStatus {
    match response {
        AppGameUnknownParentResponse::AllowOnce => AppGameUnknownApprovalStatus::AllowedOnce,
        AppGameUnknownParentResponse::AllowTarget => AppGameUnknownApprovalStatus::AllowedTarget,
        AppGameUnknownParentResponse::AllowCategory => {
            AppGameUnknownApprovalStatus::AllowedCategory
        }
        AppGameUnknownParentResponse::AskChildWhy => {
            AppGameUnknownApprovalStatus::AwaitingChildReason
        }
        AppGameUnknownParentResponse::Deny => AppGameUnknownApprovalStatus::Denied,
        AppGameUnknownParentResponse::BlockIfSupported => block_status(capability_state),
        AppGameUnknownParentResponse::ReportOnly => AppGameUnknownApprovalStatus::ReportOnly,
        AppGameUnknownParentResponse::Override => AppGameUnknownApprovalStatus::Overridden,
    }
}

fn block_status(
    capability_state: AppGameUnknownAdapterCapabilityState,
) -> AppGameUnknownApprovalStatus {
    match capability_state {
        AppGameUnknownAdapterCapabilityState::Supported => {
            AppGameUnknownApprovalStatus::BlockApproved
        }
        AppGameUnknownAdapterCapabilityState::Unproven => {
            AppGameUnknownApprovalStatus::ManualRequired
        }
    }
}
