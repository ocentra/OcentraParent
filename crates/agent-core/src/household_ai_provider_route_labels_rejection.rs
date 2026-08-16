use ocentra_parent_agent_protocol::constants;

use crate::household_ai_provider_route_state::HouseholdAiRouteRejectionReason;

pub(crate) fn rejection_reason_label(reason: HouseholdAiRouteRejectionReason) -> &'static str {
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
