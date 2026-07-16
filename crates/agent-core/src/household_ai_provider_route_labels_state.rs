use crate::household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
};

pub(crate) fn route_state_for_rejection(
    provider_class: HouseholdAiProviderClass,
    rejection_reason: Option<HouseholdAiRouteRejectionReason>,
) -> HouseholdAiRouteDecisionState {
    match rejection_reason {
        None => HouseholdAiRouteDecisionState::Selected,
        Some(HouseholdAiRouteRejectionReason::MobileDormantDesktopAvailable) => {
            HouseholdAiRouteDecisionState::Dormant
        }
        Some(HouseholdAiRouteRejectionReason::MobileFallbackDenied)
            if provider_class == HouseholdAiProviderClass::MobileDormant =>
        {
            HouseholdAiRouteDecisionState::Dormant
        }
        Some(_) => HouseholdAiRouteDecisionState::Rejected,
    }
}
