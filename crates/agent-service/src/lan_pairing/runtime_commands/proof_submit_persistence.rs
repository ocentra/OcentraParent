use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason, LanPairingText,
};

use super::super::{LanPairingRegistryPersistence, LanPairingRuntime};

pub(super) fn persist_accepted_pairing_proof(
    runtime: &LanPairingRuntime,
    proof: &LanPairingProof,
    child_device: LanPairingDeviceRef,
    parent_device: LanPairingDeviceRef,
    trusted_at: LanPairingText,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::Malformed)
        .and_then(|mut registry| match &runtime.persistence {
            LanPairingRegistryPersistence::InMemory => {
                registry.accept_pairing_proof(
                    proof,
                    child_device,
                    parent_device,
                    trusted_at.0.as_str(),
                );
                Ok(())
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .accept_pairing_proof_persisted(
                    path.as_path(),
                    proof,
                    child_device,
                    parent_device,
                    trusted_at.0.as_str(),
                )
                .map(|_entry| ())
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        })
}
