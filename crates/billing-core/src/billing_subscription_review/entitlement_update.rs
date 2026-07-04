use crate::billing_subscription::{
    BillingProviderWebhookEvent, BillingRefundLifecycleState, BillingSubscriptionStatus,
};

pub(crate) fn billing_provider_entitlement_update_required(
    accepted: bool,
    event: &BillingProviderWebhookEvent,
) -> bool {
    accepted
        && !matches!(
            event.refund_state,
            BillingRefundLifecycleState::RefundIssued
        )
        && !matches!(
            event.subscription_status,
            BillingSubscriptionStatus::Unknown | BillingSubscriptionStatus::Unavailable
        )
}
