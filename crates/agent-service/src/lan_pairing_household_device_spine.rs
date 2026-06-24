use ocentra_lan_core::read_model_builder;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;

use crate::time::timestamp_now;

pub(crate) fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanCanonicalHouseholdDevice> {
    read_model_builder::canonical_household_devices(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
        &timestamp_now(),
    )
}
