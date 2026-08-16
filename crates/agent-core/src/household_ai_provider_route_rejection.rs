use crate::household_ai_provider_route::{HouseholdAiProviderCandidate, HouseholdAiRouteRequest};
use crate::household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiProviderTrustState, HouseholdAiRouteRejectionReason,
};

pub(crate) fn candidate_rejection_reason(
    request: &HouseholdAiRouteRequest,
    candidate: &HouseholdAiProviderCandidate,
    desktop_or_laptop_available: bool,
) -> Option<HouseholdAiRouteRejectionReason> {
    if let Some(reason) = trust_rejection_reason(candidate.trust_state) {
        return Some(reason);
    }
    if candidate.custody_label != request.required_custody_label {
        return Some(HouseholdAiRouteRejectionReason::CustodyMismatch);
    }
    if candidate.provider_class == HouseholdAiProviderClass::MobileDormant
        && desktop_or_laptop_available
    {
        return Some(HouseholdAiRouteRejectionReason::MobileDormantDesktopAvailable);
    }
    super::household_ai_provider_route_rejection_mobile::mobile_rejection_reason(
        request,
        candidate,
        desktop_or_laptop_available,
    )
}

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
