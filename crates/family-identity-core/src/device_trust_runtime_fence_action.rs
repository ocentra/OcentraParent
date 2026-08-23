use crate::household_authority::HouseholdAuthorityAction;

pub(super) fn code(action: HouseholdAuthorityAction) -> i64 {
    match action {
        HouseholdAuthorityAction::SealParentDeviceTrust => 0,
        HouseholdAuthorityAction::PairChildDevice => 1,
        HouseholdAuthorityAction::RegisterLanSignerAnchor => 2,
        HouseholdAuthorityAction::RevokeChildDevice => 3,
        HouseholdAuthorityAction::ViewChildStatus => 4,
        HouseholdAuthorityAction::ChangePolicy => 5,
        HouseholdAuthorityAction::StartRemoteView => 6,
        HouseholdAuthorityAction::StartRemoteControl => 7,
        HouseholdAuthorityAction::ExportDeleteData => 8,
        HouseholdAuthorityAction::ImportRestoreData => 9,
        HouseholdAuthorityAction::ManageBilling => 10,
    }
}
