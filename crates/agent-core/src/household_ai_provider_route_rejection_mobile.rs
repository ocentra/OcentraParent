use crate::household_ai_provider_route::{HouseholdAiProviderCandidate, HouseholdAiRouteRequest};
use crate::household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiProviderResourceState, HouseholdAiRouteRejectionReason,
    HouseholdAiWorkClass,
};

pub(crate) fn mobile_rejection_reason(
    request: &HouseholdAiRouteRequest,
    candidate: &HouseholdAiProviderCandidate,
    desktop_or_laptop_available: bool,
) -> Option<HouseholdAiRouteRejectionReason> {
    if candidate.provider_class != HouseholdAiProviderClass::MobileDormant {
        return None;
    }
    if desktop_or_laptop_available {
        return Some(HouseholdAiRouteRejectionReason::MobileDormantDesktopAvailable);
    }
    if !candidate_supports_work(candidate, request.work_class) {
        return Some(HouseholdAiRouteRejectionReason::UnsupportedCapability);
    }
    if candidate.resource_state != HouseholdAiProviderResourceState::Ready {
        return Some(HouseholdAiRouteRejectionReason::ResourceDegraded);
    }
    let policy = candidate.resource_policy;
    if request.allow_mobile_fallback
        && policy.fallback_policy_allows_mobile
        && policy.battery_ok
        && policy.thermal_ok
    {
        None
    } else {
        Some(HouseholdAiRouteRejectionReason::MobileFallbackDenied)
    }
}

pub(crate) fn candidate_supports_work(
    candidate: &HouseholdAiProviderCandidate,
    work_class: HouseholdAiWorkClass,
) -> bool {
    match work_class {
        HouseholdAiWorkClass::HeavyScreenVision => candidate.supports_heavy_screen_vision,
        HouseholdAiWorkClass::LightText => candidate.supports_light_text,
    }
}
