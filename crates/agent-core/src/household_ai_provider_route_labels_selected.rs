use ocentra_parent_agent_protocol::constants;

use crate::household_ai_provider_route_state::HouseholdAiProviderClass;

pub(crate) fn selected_reason_label(provider_class: HouseholdAiProviderClass) -> &'static str {
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
