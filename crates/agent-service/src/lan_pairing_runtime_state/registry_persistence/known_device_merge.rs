use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    lan_pairing::LanPairingRejectionReason,
    lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
};

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};

impl LanPairingRuntime {
    pub(crate) fn merge_known_household_devices(
        &self,
        registry: &mut TrustedDeviceRegistry,
        devices: Vec<LanCanonicalHouseholdDevice>,
    ) -> Result<bool, LanPairingRejectionReason> {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                Ok(registry.merge_known_household_devices(devices))
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .merge_known_household_devices_persisted(path.as_path(), devices)
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        }
    }
}
