use ocentra_parent_agent_core::trusted_device_registry::controller_lease::LanControllerLeaseMutation;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason, LanPairingText, LanParentIntentEnvelope,
};

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};

struct LanPairingRegistryPathRef<'a>(&'a std::path::Path);

impl LanPairingRuntime {
    pub(crate) fn apply_controller_lease_intent(
        &self,
        origin: &LanPairingOptionalText,
        intent: &LanParentIntentEnvelope,
        observed_at: &LanPairingText,
        mutation: LanControllerLeaseMutation,
    ) -> Result<(), LanPairingRejectionReason> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_error| LanPairingRejectionReason::Malformed)?;
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                apply_ephemeral(&mut registry, origin, intent, observed_at, mutation)
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => apply_persisted(
                &mut registry,
                &LanPairingRegistryPathRef(path.as_path()),
                origin,
                intent,
                observed_at,
                mutation,
            )?,
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        }
    }
}

fn apply_ephemeral(
    registry: &mut ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
    observed_at: &LanPairingText,
    mutation: LanControllerLeaseMutation,
) -> Result<(), LanPairingRejectionReason> {
    registry.apply_intent(
        intent,
        origin.0.as_deref(),
        observed_at.0.as_str(),
        false,
        |candidate| candidate.apply_controller_lease(intent, observed_at.0.as_str(), mutation),
    )
}

fn apply_persisted(
    registry: &mut ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry,
    path: &LanPairingRegistryPathRef<'_>,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
    observed_at: &LanPairingText,
    mutation: LanControllerLeaseMutation,
) -> Result<Result<(), LanPairingRejectionReason>, LanPairingRejectionReason> {
    registry
        .apply_intent_persisted(
            path.0,
            intent,
            origin.0.as_deref(),
            observed_at.0.as_str(),
            false,
            |candidate| candidate.apply_controller_lease(intent, observed_at.0.as_str(), mutation),
        )
        .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)
}
