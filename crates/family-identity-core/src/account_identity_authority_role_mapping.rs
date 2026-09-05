use ocentra_schema::account_identity_authority::AccountIdentityRole;

use crate::family_identity::{DeviceOwnershipScope, HouseholdRole};

pub(super) fn map_role(role: AccountIdentityRole) -> HouseholdRole {
    match role {
        AccountIdentityRole::ParentOwner => HouseholdRole::ParentOwner,
        AccountIdentityRole::CoParentGuardian => HouseholdRole::CoParentGuardian,
        AccountIdentityRole::Observer => HouseholdRole::Observer,
        AccountIdentityRole::ChildProfile => HouseholdRole::ChildProfile,
        AccountIdentityRole::ChildDeviceAgent => HouseholdRole::ChildDeviceAgent,
        AccountIdentityRole::SupportAdmin => HouseholdRole::SupportAdmin,
    }
}

pub(super) fn map_device_scope(role: AccountIdentityRole) -> DeviceOwnershipScope {
    match role {
        AccountIdentityRole::ChildProfile | AccountIdentityRole::ChildDeviceAgent => {
            DeviceOwnershipScope::ChildProfileDevice
        }
        AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian => {
            DeviceOwnershipScope::ParentControllerDevice
        }
        AccountIdentityRole::Observer => DeviceOwnershipScope::ParentObserverDevice,
        AccountIdentityRole::SupportAdmin => DeviceOwnershipScope::OtherDevice,
    }
}
