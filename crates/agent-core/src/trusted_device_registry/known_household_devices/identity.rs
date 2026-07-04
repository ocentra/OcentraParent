use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

pub(super) fn same_known_household_device(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing.canonical_device_id == incoming.canonical_device_id
        || existing
            .network_identity
            .mac_address
            .as_deref()
            .zip(incoming.network_identity.mac_address.as_deref())
            .map(|(left, right)| left.eq_ignore_ascii_case(right))
            .unwrap_or(false)
}
