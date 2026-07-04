use crate::billing_subscription::{
    BillingCollectionRecoveryState, BillingDisputeLifecycleState, BillingProviderDeadLetterState,
    BillingProviderWebhookEvent, BillingRefundLifecycleState, BillingSubscriptionStatus,
    BillingTestLiveBoundaryState,
};

pub(crate) fn billing_provider_manual_review_required(
    accepted: bool,
    event: &BillingProviderWebhookEvent,
    dead_letter_state: BillingProviderDeadLetterState,
) -> bool {
    match accepted {
        false => true,
        true => {
            matches!(
                event.collection_recovery_state,
                BillingCollectionRecoveryState::SupportRequired
            ) || matches!(
                event.refund_state,
                BillingRefundLifecycleState::RefundIssued
            ) || matches!(
                event.dispute_state,
                BillingDisputeLifecycleState::DisputeOpened
            ) || matches!(
                dead_letter_state,
                BillingProviderDeadLetterState::ManualRequired
            ) || matches!(
                event.subscription_status,
                BillingSubscriptionStatus::Unknown | BillingSubscriptionStatus::Unavailable
            ) || matches!(
                event.test_live_boundary_state,
                BillingTestLiveBoundaryState::MixedBlocked
            )
        }
    }
}
