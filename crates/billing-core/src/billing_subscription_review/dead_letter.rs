use crate::billing_subscription::{
    BillingAccountMatchState, BillingProviderDeadLetterState, BillingProviderPayloadParseState,
    BillingProviderSignatureState, BillingProviderWebhookEvent, BillingTestLiveBoundaryState,
};

pub(crate) fn billing_provider_dead_letter_state(
    event: &BillingProviderWebhookEvent,
) -> BillingProviderDeadLetterState {
    match (
        event.signature_state,
        event.payload_parse_state,
        event.account_match_state,
        event.test_live_boundary_state,
    ) {
        (
            BillingProviderSignatureState::Verified,
            BillingProviderPayloadParseState::Parsed,
            BillingAccountMatchState::Matched,
            BillingTestLiveBoundaryState::Isolated,
        ) => BillingProviderDeadLetterState::NotRequired,
        _ => BillingProviderDeadLetterState::ManualRequired,
    }
}
