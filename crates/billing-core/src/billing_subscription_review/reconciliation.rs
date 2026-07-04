use crate::billing_subscription::{
    BillingDisputeLifecycleState, BillingProviderOrderingState, BillingProviderReconciliationState,
    BillingProviderReplayState, BillingProviderWebhookEvent, BillingRefundLifecycleState,
};

pub(crate) fn billing_provider_reconciliation_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderReconciliationState {
    match (
        event.replay_state,
        event.ordering_state,
        event.refund_state,
        event.dispute_state,
    ) {
        (BillingProviderReplayState::Replayed, _, _, _)
        | (_, BillingProviderOrderingState::OutOfOrder, _, _)
        | (_, _, BillingRefundLifecycleState::RefundIssued, _)
        | (_, _, _, BillingDisputeLifecycleState::DisputeOpened) => {
            BillingProviderReconciliationState::QueueRequired
        }
        _ => BillingProviderReconciliationState::NotRequired,
    }
}
