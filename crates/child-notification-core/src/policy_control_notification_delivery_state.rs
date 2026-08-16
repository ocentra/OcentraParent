use ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState;

use crate::policy_control_notification::PolicyControlNotificationState;

pub fn delivery_state_for(
    state: PolicyDeliveryParentVisibleState,
) -> PolicyControlNotificationState {
    match state {
        PolicyDeliveryParentVisibleState::Pending => {
            PolicyControlNotificationState::DeliveryPending
        }
        PolicyDeliveryParentVisibleState::Applied => {
            PolicyControlNotificationState::DeliveryApplied
        }
        PolicyDeliveryParentVisibleState::Degraded => {
            PolicyControlNotificationState::DeliveryDegraded
        }
        PolicyDeliveryParentVisibleState::ManualRequired => {
            PolicyControlNotificationState::DeliveryManualRequired
        }
        PolicyDeliveryParentVisibleState::Superseded => {
            PolicyControlNotificationState::DeliverySuperseded
        }
    }
}
