use crate::billing_subscription::{
    BillingCollectionRecoveryState, BillingProviderEventKind, BillingProviderRetryState,
    BillingProviderWebhookEvent,
};

pub(crate) fn billing_provider_retry_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderRetryState {
    match event.event_kind {
        BillingProviderEventKind::InvoicePaymentFailed
        | BillingProviderEventKind::PaymentIntentFailed => BillingProviderRetryState::QueueRequired,
        _ => match event.collection_recovery_state {
            BillingCollectionRecoveryState::PastDue | BillingCollectionRecoveryState::Unpaid => {
                BillingProviderRetryState::QueueRequired
            }
            _ => BillingProviderRetryState::NotRequired,
        },
    }
}
