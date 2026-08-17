use ocentra_schema::account_identity_authority::{
    AccountIdentityAccountState, AccountIdentityMembershipState,
};

use crate::family_identity::{ActorAccountState, HouseholdMembershipState};

pub(super) fn map_account_state(state: AccountIdentityAccountState) -> ActorAccountState {
    match state {
        AccountIdentityAccountState::Active => ActorAccountState::Active,
        AccountIdentityAccountState::Suspended => ActorAccountState::Suspended,
        AccountIdentityAccountState::Disabled => ActorAccountState::Disabled,
    }
}

pub(super) fn map_membership_state(
    state: AccountIdentityMembershipState,
) -> HouseholdMembershipState {
    match state {
        AccountIdentityMembershipState::Invited => HouseholdMembershipState::Invited,
        AccountIdentityMembershipState::Pending => HouseholdMembershipState::Pending,
        AccountIdentityMembershipState::Active => HouseholdMembershipState::Active,
        AccountIdentityMembershipState::Revoked => HouseholdMembershipState::Revoked,
        AccountIdentityMembershipState::Disabled => HouseholdMembershipState::Disabled,
    }
}
