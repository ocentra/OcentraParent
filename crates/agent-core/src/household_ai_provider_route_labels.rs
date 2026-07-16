use crate::household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
};

#[path = "household_ai_provider_route_labels_rank.rs"]
mod household_ai_provider_route_labels_rank;
#[path = "household_ai_provider_route_labels_rejection.rs"]
mod household_ai_provider_route_labels_rejection;
#[path = "household_ai_provider_route_labels_selected.rs"]
mod household_ai_provider_route_labels_selected;
#[path = "household_ai_provider_route_labels_state.rs"]
mod household_ai_provider_route_labels_state;

pub(crate) fn route_state_for_rejection(
    provider_class: HouseholdAiProviderClass,
    rejection_reason: Option<HouseholdAiRouteRejectionReason>,
) -> HouseholdAiRouteDecisionState {
    household_ai_provider_route_labels_state::route_state_for_rejection(
        provider_class,
        rejection_reason,
    )
}

pub(crate) fn route_reason_label(
    provider_class: HouseholdAiProviderClass,
    rejection_reason: Option<HouseholdAiRouteRejectionReason>,
) -> &'static str {
    if let Some(reason) = rejection_reason {
        household_ai_provider_route_labels_rejection::rejection_reason_label(reason)
    } else {
        household_ai_provider_route_labels_selected::selected_reason_label(provider_class)
    }
}

pub(crate) fn route_rank(provider_class: HouseholdAiProviderClass) -> u8 {
    household_ai_provider_route_labels_rank::route_rank(provider_class)
}
