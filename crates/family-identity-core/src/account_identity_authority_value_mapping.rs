use ocentra_schema::account_identity_authority::{
    AccountIdentityAccountState, AccountIdentityDeviceTrustState,
    AccountIdentityHouseholdChildDeviceBinding, AccountIdentityMembershipState,
    AccountIdentityRole, AccountIdentitySessionFreshnessState,
};

use crate::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};

#[path = "account_identity_authority_device_mapping.rs"]
mod device;
#[path = "account_identity_authority_role_mapping.rs"]
mod role;
#[path = "account_identity_authority_state_mapping.rs"]
mod state;

pub(super) fn map_role(role: AccountIdentityRole) -> HouseholdRole {
    role::map_role(role)
}

pub(super) fn map_account_state(state: AccountIdentityAccountState) -> ActorAccountState {
    state::map_account_state(state)
}

pub(super) fn map_membership_state(
    state: AccountIdentityMembershipState,
) -> HouseholdMembershipState {
    state::map_membership_state(state)
}

pub(super) fn map_binding_state(
    binding: &AccountIdentityHouseholdChildDeviceBinding,
) -> ChildProfileBindingState {
    device::map_binding_state(binding)
}

pub(super) fn map_device_scope(role: AccountIdentityRole) -> DeviceOwnershipScope {
    role::map_device_scope(role)
}

pub(super) fn map_device_trust(state: AccountIdentityDeviceTrustState) -> DeviceTrustState {
    device::map_device_trust(state)
}

pub(super) fn map_session_freshness(
    state: AccountIdentitySessionFreshnessState,
) -> SessionFreshnessState {
    device::map_session_freshness(state)
}
