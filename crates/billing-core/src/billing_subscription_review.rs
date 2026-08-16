use crate::billing_subscription::{
    BillingProviderDeadLetterState, BillingProviderEventDecisionState,
    BillingProviderReconciliationState, BillingProviderRetryState, BillingProviderWebhookEvent,
};

mod dead_letter;
mod entitlement_update;
mod event_decision;
mod manual_review;
mod reconciliation;
mod retry;
mod webhook_acceptance;

pub(crate) fn billing_provider_manual_review_required(
    accepted: bool,
    event: &BillingProviderWebhookEvent,
    dead_letter_state: BillingProviderDeadLetterState,
) -> bool {
    manual_review::billing_provider_manual_review_required(accepted, event, dead_letter_state)
}

pub(crate) fn billing_provider_entitlement_update_required(
    accepted: bool,
    event: &BillingProviderWebhookEvent,
) -> bool {
    entitlement_update::billing_provider_entitlement_update_required(accepted, event)
}

pub(crate) fn billing_provider_event_decision_state(
    accepted: bool,
) -> BillingProviderEventDecisionState {
    event_decision::billing_provider_event_decision_state(accepted)
}

pub(crate) fn billing_provider_dead_letter_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderDeadLetterState {
    dead_letter::billing_provider_dead_letter_state(event)
}

pub(crate) fn billing_provider_retry_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderRetryState {
    retry::billing_provider_retry_state(event)
}

pub(crate) fn billing_provider_reconciliation_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderReconciliationState {
    reconciliation::billing_provider_reconciliation_state(event)
}

pub(crate) fn billing_provider_webhook_is_accepted(event: &BillingProviderWebhookEvent) -> bool {
    webhook_acceptance::billing_provider_webhook_is_accepted(event)
}
