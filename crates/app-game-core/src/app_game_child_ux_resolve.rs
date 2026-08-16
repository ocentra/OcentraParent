use crate::app_game_child_ux_request::resolve_available;
use crate::app_game_child_ux_runtime::{manual_required, unavailable, NoticeParts};
use crate::app_game_child_ux_types::{AppGameChildUxCapabilityState, AppGameChildUxInput};

pub(super) fn resolve_notice(input: &AppGameChildUxInput) -> NoticeParts {
    match input.capability_state {
        AppGameChildUxCapabilityState::ManualRequired => manual_required(),
        AppGameChildUxCapabilityState::Unavailable => unavailable(),
        AppGameChildUxCapabilityState::Available => resolve_available(input),
    }
}
