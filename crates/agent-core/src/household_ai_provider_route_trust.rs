use crate::household_ai_provider_route_state::{
    HouseholdAiProviderTrustState, HouseholdAiRouteRejectionReason,
};

pub(crate) fn trust_rejection_reason(
    trust_state: HouseholdAiProviderTrustState,
) -> Option<HouseholdAiRouteRejectionReason> {
    match trust_state {
        HouseholdAiProviderTrustState::Trusted => None,
        HouseholdAiProviderTrustState::Stale => {
            Some(HouseholdAiRouteRejectionReason::StaleProvider)
        }
        HouseholdAiProviderTrustState::Offline => {
            Some(HouseholdAiRouteRejectionReason::OfflineProvider)
        }
        HouseholdAiProviderTrustState::Revoked => {
            Some(HouseholdAiRouteRejectionReason::RevokedProvider)
        }
    }
}
