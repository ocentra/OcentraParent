use super::{CurrentChildDeviceTrustBinding, VerifiedAccountIdentityAuthority};

pub(super) fn matches(
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    household_id: &str,
    account_id: &str,
    parent_device_id: &str,
    child_profile_id: &str,
    child_device_id: &str,
) -> bool {
    household_id == authority.household_id().to_string()
        && account_id == authority.account_id().to_string()
        && parent_device_id == authority.device_id().as_str()
        && child_profile_id == authority.child_profile_id().to_string()
        && child_device_id == authority.child_device_id().as_str()
        && device_binding.family_id() == household_id
        && device_binding.parent_device_id() == parent_device_id
        && device_binding.child_device_id() == child_device_id
        && device_binding.authority_generation() == authority.authority_generation()
}
