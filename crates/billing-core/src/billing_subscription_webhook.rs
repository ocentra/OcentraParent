use crate::billing_subscription::{
    BillingEntitlementUpdateRequirement, BillingManualReviewRequirement,
    BillingProviderWebhookDecision, BillingProviderWebhookEvent,
};

pub(crate) fn decide_billing_provider_webhook(
    event: BillingProviderWebhookEvent,
) -> BillingProviderWebhookDecision {
    let dead_letter_state =
        crate::billing_subscription_review::billing_provider_dead_letter_state(&event);
    let retry_state = crate::billing_subscription_review::billing_provider_retry_state(&event);
    let reconciliation_state =
        crate::billing_subscription_review::billing_provider_reconciliation_state(&event);
    let accepted = crate::billing_subscription_review::billing_provider_webhook_is_accepted(&event);
    let manual_review_required =
        crate::billing_subscription_review::billing_provider_manual_review_required(
            accepted,
            &event,
            dead_letter_state,
        );
    let entitlement_update_required =
        crate::billing_subscription_review::billing_provider_entitlement_update_required(
            accepted, &event,
        );

    BillingProviderWebhookDecision {
        event_id: event.event_id,
        provider: event.provider,
        mode: event.mode,
        decision_state: crate::billing_subscription_review::billing_provider_event_decision_state(
            accepted,
        ),
        payload_parse_state: event.payload_parse_state,
        idempotency_state: event.idempotency_state,
        replay_state: event.replay_state,
        ordering_state: event.ordering_state,
        retry_state,
        dead_letter_state,
        reconciliation_state,
        test_live_boundary_state: event.test_live_boundary_state,
        subscription_status: event.subscription_status,
        collection_recovery_state: event.collection_recovery_state,
        refund_state: event.refund_state,
        dispute_state: event.dispute_state,
        entitlement_update_requirement: if entitlement_update_required {
            BillingEntitlementUpdateRequirement::Required
        } else {
            BillingEntitlementUpdateRequirement::NotRequired
        },
        manual_review_requirement: if manual_review_required {
            BillingManualReviewRequirement::Required
        } else {
            BillingManualReviewRequirement::NotRequired
        },
    }
}
