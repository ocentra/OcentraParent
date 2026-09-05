use super::{CurrentChildDeviceTrustBinding, VerifiedAccountIdentityAuthority};

pub(super) struct HouseholdAuthorityRuntimeBinding<'a> {
    pub(super) household_id: &'a str,
    pub(super) account_id: &'a str,
    pub(super) parent_device_id: &'a str,
    pub(super) child_profile_id: &'a str,
    pub(super) child_device_id: &'a str,
    pub(super) installation_id: &'a str,
    pub(super) pairing_id: &'a str,
    pub(super) route_id: &'a str,
}

pub(super) fn matches(
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    binding: &HouseholdAuthorityRuntimeBinding<'_>,
) -> bool {
    let account_binding = authority.current_binding();
    binding.household_id == authority.household_id().to_string()
        && binding.account_id == authority.account_id().to_string()
        && binding.parent_device_id == authority.device_id().as_str()
        && binding.child_profile_id == authority.child_profile_id().to_string()
        && binding.child_device_id == authority.child_device_id().as_str()
        && binding.installation_id == account_binding.installation_id.as_str()
        && binding.pairing_id == account_binding.pairing_id.as_str()
        && binding.route_id == account_binding.selected_route_id.as_str()
        && device_binding.family_id() == binding.household_id
        && device_binding.parent_device_id() == binding.parent_device_id
        && device_binding.child_device_id() == binding.child_device_id
        && device_binding.installation_id() == binding.installation_id
        && device_binding.authority_generation() == authority.authority_generation()
}
