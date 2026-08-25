use super::{CurrentChildDeviceTrustBinding, VerifiedAccountIdentityAuthority};

pub(super) fn matches(
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    household_id: &str,
    account_id: &str,
    parent_device_id: &str,
    child_profile_id: &str,
    child_device_id: &str,
    installation_id: &str,
    pairing_id: &str,
    route_id: &str,
) -> bool {
    let account_binding = authority.current_binding();
    household_id == authority.household_id().to_string()
        && account_id == authority.account_id().to_string()
        && parent_device_id == authority.device_id().as_str()
        && child_profile_id == authority.child_profile_id().to_string()
        && child_device_id == authority.child_device_id().as_str()
        && installation_id == account_binding.installation_id.as_str()
        && pairing_id == account_binding.pairing_id.as_str()
        && route_id == account_binding.selected_route_id.as_str()
        && device_binding.family_id() == household_id
        && device_binding.parent_device_id() == parent_device_id
        && device_binding.child_device_id() == child_device_id
        && device_binding.installation_id() == installation_id
        && device_binding.authority_generation() == authority.authority_generation()
}
