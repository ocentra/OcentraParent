use crate::billing_subscription::{
    BillingAccountMatchState, BillingProviderIdempotencyState, BillingProviderOrderingState,
    BillingProviderPayloadParseState, BillingProviderReplayState, BillingProviderSignatureState,
    BillingProviderWebhookEvent, BillingTestLiveBoundaryState,
};

pub(crate) fn billing_provider_webhook_is_accepted(event: &BillingProviderWebhookEvent) -> bool {
    matches!(
        (
            event.signature_state,
            event.payload_parse_state,
            event.idempotency_state,
            event.replay_state,
            event.ordering_state,
            event.account_match_state,
            event.test_live_boundary_state,
        ),
        (
            BillingProviderSignatureState::Verified,
            BillingProviderPayloadParseState::Parsed,
            BillingProviderIdempotencyState::Fresh,
            BillingProviderReplayState::Fresh,
            BillingProviderOrderingState::InOrder,
            BillingAccountMatchState::Matched,
            BillingTestLiveBoundaryState::Isolated,
        )
    )
}
