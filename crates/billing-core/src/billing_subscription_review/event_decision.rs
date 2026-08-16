use crate::billing_subscription::BillingProviderEventDecisionState;

pub(crate) fn billing_provider_event_decision_state(
    accepted: bool,
) -> BillingProviderEventDecisionState {
    if accepted {
        BillingProviderEventDecisionState::Accepted
    } else {
        BillingProviderEventDecisionState::Rejected
    }
}
