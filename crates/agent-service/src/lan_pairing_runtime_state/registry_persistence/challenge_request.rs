use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingRejectionReason, LanPairingText};

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};

impl LanPairingRuntime {
    pub(crate) fn record_challenge_request(
        &self,
        registry: &mut TrustedDeviceRegistry,
        challenge_id: &LanPairingText,
    ) -> Result<bool, LanPairingRejectionReason> {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                Ok(registry.record_challenge_request(challenge_id.0.as_str()))
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .record_challenge_request_persisted(path.as_path(), challenge_id.0.as_str())
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        }
    }
}
