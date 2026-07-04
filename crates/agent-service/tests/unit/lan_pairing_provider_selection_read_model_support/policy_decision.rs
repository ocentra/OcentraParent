use super::rejection_reason::selected_route_rejection_reason;
use super::{
    LanAiProviderRoutingState, LanProviderSelectionPolicyDecision, LanSelectedRouteTarget,
};
use std::string::String as TestString;

pub(super) fn selected_provider_route_id(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<TestString> {
    selected
        .filter(|target| route_is_selectable(target, routing_state))
        .map(|target| target.route_id.clone())
}

pub(super) fn policy_decision_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> LanProviderSelectionPolicyDecision {
    match (
        selected,
        selected_route_rejection_reason(selected, routing_state),
        routing_state,
    ) {
        (None, _, _) => LanProviderSelectionPolicyDecision::RefuseUnpairedProvider,
        (_, Some(_), _) => LanProviderSelectionPolicyDecision::RefuseRouteBlockedProvider,
        (_, None, LanAiProviderRoutingState::AuthorizedResult) => {
            LanProviderSelectionPolicyDecision::SelectAuthorizedProvider
        }
        (_, None, LanAiProviderRoutingState::Busy) => {
            LanProviderSelectionPolicyDecision::DegradeBusyProvider
        }
        (_, None, LanAiProviderRoutingState::Degraded | LanAiProviderRoutingState::Unavailable) => {
            LanProviderSelectionPolicyDecision::DegradeProviderUnavailable
        }
        (_, None, LanAiProviderRoutingState::UnsupportedCapability) => {
            LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability
        }
    }
}

fn route_is_selectable(
    selected: &LanSelectedRouteTarget,
    routing_state: &LanAiProviderRoutingState,
) -> bool {
    *routing_state == LanAiProviderRoutingState::AuthorizedResult
        && selected_route_rejection_reason(Some(selected), routing_state).is_none()
}
