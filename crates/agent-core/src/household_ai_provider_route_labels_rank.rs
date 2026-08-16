use crate::household_ai_provider_route_state::HouseholdAiProviderClass;

pub(crate) fn route_rank(provider_class: HouseholdAiProviderClass) -> u8 {
    match provider_class {
        HouseholdAiProviderClass::DesktopPreferred => 0,
        HouseholdAiProviderClass::LaptopPreferred => 1,
        HouseholdAiProviderClass::ChildDesktopLocal => 2,
        HouseholdAiProviderClass::MobileDormant => 3,
    }
}
