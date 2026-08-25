use crate::household_authority::HouseholdAuthorityAction;

pub(super) fn requires_capability(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::StartRemoteView | HouseholdAuthorityAction::StartRemoteControl
    )
}

pub(super) fn requires_controller_lease(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::StartRemoteView | HouseholdAuthorityAction::StartRemoteControl
    )
}
