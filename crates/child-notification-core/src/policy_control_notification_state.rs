use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord;
use ocentra_policy_control_core::policy_request::ChildPolicyRequest;

use crate::policy_control_notification::PolicyControlNotificationState;

use super::policy_control_notification_delivery_state::delivery_state_for;
use super::policy_control_notification_request_state::request_state_for;

pub fn notification_state_for(
    request: &ChildPolicyRequest,
    delivery: Option<&PolicyDeliveryRecord>,
) -> Result<PolicyControlNotificationState, EventingError> {
    delivery.map_or_else(
        || request_state_for(request.status),
        |delivery| Ok(delivery_state_for(delivery.parent_visible_state())),
    )
}
