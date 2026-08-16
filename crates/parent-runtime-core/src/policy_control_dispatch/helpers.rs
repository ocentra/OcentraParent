use super::*;

pub(super) fn parent_visible_state_while_dispatching(
    state: PolicyDeliveryState,
) -> PolicyDeliveryParentVisibleState {
    match state {
        PolicyDeliveryState::Degraded | PolicyDeliveryState::Offline => {
            PolicyDeliveryParentVisibleState::Degraded
        }
        PolicyDeliveryState::Superseded => PolicyDeliveryParentVisibleState::Superseded,
        _ => PolicyDeliveryParentVisibleState::Pending,
    }
}

pub(super) fn parent_visible_state_when_dispatch_is_blocked(
    state: PolicyDeliveryState,
    current_parent_visible_state: PolicyDeliveryParentVisibleState,
) -> PolicyDeliveryParentVisibleState {
    match state {
        PolicyDeliveryState::Degraded | PolicyDeliveryState::Offline => {
            PolicyDeliveryParentVisibleState::Degraded
        }
        PolicyDeliveryState::Superseded => PolicyDeliveryParentVisibleState::Superseded,
        _ => match current_parent_visible_state {
            PolicyDeliveryParentVisibleState::Superseded => {
                PolicyDeliveryParentVisibleState::Superseded
            }
            PolicyDeliveryParentVisibleState::Degraded => {
                PolicyDeliveryParentVisibleState::Degraded
            }
            _ => PolicyDeliveryParentVisibleState::ManualRequired,
        },
    }
}

pub(super) fn parent_runtime_policy_control_dispatch_ref(
    delivery: &PolicyDeliveryRecord,
    decision_event: &PolicyDecisionResolvedEvent,
) -> String {
    let mut value = String::from(PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_PREFIX);
    value.push_str(delivery.delivery_id.as_str());
    value.push_str(PARENT_RUNTIME_POLICY_CONTROL_IDEMPOTENCY_SEPARATOR);
    value.push_str(decision_event.source_request_id.as_str());
    value
}
