use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::value_support::compact_identifier;

const CHILD_PROFILE_DEVICE_PREFIX: &str = "lan-child-profile-";

pub(in crate::read_model_builder::canonical_household_device_spine) fn canonical_child_profile_device_id(
    child_profile_id: &str,
) -> String {
    let mut id = String::from(CHILD_PROFILE_DEVICE_PREFIX);
    id.push_str(child_profile_id);
    id
}

pub(in crate::read_model_builder::canonical_household_device_spine) fn child_profile_device_id(
    device: &LanPairingDeviceRef,
) -> Option<String> {
    device
        .child_profile_id
        .as_deref()
        .map(str::trim)
        .filter(|child_profile_id| !child_profile_id.is_empty())
        .map(compact_identifier)
}

pub(in crate::read_model_builder::canonical_household_device_spine) fn child_profile_identity_from_canonical(
    canonical_device_id: &str,
) -> Option<&str> {
    canonical_device_id.strip_prefix(CHILD_PROFILE_DEVICE_PREFIX)
}
