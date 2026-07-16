use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;

use crate::policy_control_notification::PolicyControlNotificationState;

use super::policy_control_notification_validation::invalid_request_status_error;

pub fn request_state_for(
    status: PolicyRequestStatus,
) -> Result<PolicyControlNotificationState, EventingError> {
    match status {
        PolicyRequestStatus::PreviewOnly => Ok(PolicyControlNotificationState::PreviewOnly),
        PolicyRequestStatus::PendingParentReview => {
            Ok(PolicyControlNotificationState::PendingParentReview)
        }
        PolicyRequestStatus::Approved => Ok(PolicyControlNotificationState::Approved),
        PolicyRequestStatus::Modified => Ok(PolicyControlNotificationState::Modified),
        PolicyRequestStatus::Denied => Ok(PolicyControlNotificationState::Denied),
        PolicyRequestStatus::Expired => Ok(PolicyControlNotificationState::ExpiredRequest),
        PolicyRequestStatus::ReplayRejected => Err(invalid_request_status_error(
            "policy_request.status",
            status,
        )),
    }
}
