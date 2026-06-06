use ocentra_parent_agent_protocol::constants;

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

pub(crate) fn route_reason_label(
    provider_class: HouseholdAiProviderClass,
    rejection_reason: Option<HouseholdAiRouteRejectionReason>,
) -> &'static str {
    match rejection_reason {
        Some(reason) => rejection_reason_label(reason),
        None => selected_reason_label(provider_class),
    }
}

pub(crate) fn route_rank(provider_class: HouseholdAiProviderClass) -> u8 {
    match provider_class {
        HouseholdAiProviderClass::DesktopPreferred => 0,
        HouseholdAiProviderClass::LaptopPreferred => 1,
        HouseholdAiProviderClass::ChildDesktopLocal => 2,
        HouseholdAiProviderClass::MobileDormant => 3,
    }
}

fn selected_reason_label(provider_class: HouseholdAiProviderClass) -> &'static str {
    match provider_class {
        HouseholdAiProviderClass::DesktopPreferred => {
            constants::household_mesh::ROUTE_REASON_SELECTED_DESKTOP
        }
        HouseholdAiProviderClass::LaptopPreferred => {
            constants::household_mesh::ROUTE_REASON_SELECTED_LAPTOP
        }
        HouseholdAiProviderClass::ChildDesktopLocal => {
            constants::household_mesh::ROUTE_REASON_SELECTED_CHILD_DESKTOP
        }
        HouseholdAiProviderClass::MobileDormant => {
            constants::household_mesh::ROUTE_REASON_MOBILE_FALLBACK_ALLOWED
        }
    }
}

fn rejection_reason_label(reason: HouseholdAiRouteRejectionReason) -> &'static str {
    match reason {
        HouseholdAiRouteRejectionReason::StaleProvider => {
            constants::household_mesh::ROUTE_REASON_STALE_PROVIDER
        }
        HouseholdAiRouteRejectionReason::OfflineProvider => {
            constants::household_mesh::ROUTE_REASON_OFFLINE_PROVIDER
        }
        HouseholdAiRouteRejectionReason::RevokedProvider => {
            constants::household_mesh::ROUTE_REASON_REVOKED_PROVIDER
        }
        HouseholdAiRouteRejectionReason::CustodyMismatch => {
            constants::household_mesh::ROUTE_REASON_CUSTODY_MISMATCH
        }
        HouseholdAiRouteRejectionReason::UnsupportedCapability => {
            constants::household_mesh::ROUTE_REASON_UNSUPPORTED_CAPABILITY
        }
        HouseholdAiRouteRejectionReason::ResourceDegraded => {
            constants::household_mesh::ROUTE_REASON_PROVIDER_DEGRADED
        }
        HouseholdAiRouteRejectionReason::MobileDormantDesktopAvailable => {
            constants::household_mesh::ROUTE_REASON_MOBILE_DORMANT_DESKTOP_AVAILABLE
        }
        HouseholdAiRouteRejectionReason::MobileFallbackDenied => {
            constants::household_mesh::ROUTE_REASON_MOBILE_FALLBACK_DENIED
        }
        HouseholdAiRouteRejectionReason::NoProvider => {
            constants::household_mesh::ROUTE_REASON_NO_PROVIDER
        }
    }
}
