use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason, LanParentIntentEnvelope,
};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

pub(super) fn select_pairing_result(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let observed_at: String = timestamp_now();
    runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::Malformed)
        .and_then(|mut registry| match &runtime.persistence {
            LanPairingRegistryPersistence::InMemory => {
                registry.select_pairing_for_intent(intent, origin.0.as_deref(), &observed_at)
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .select_pairing_for_intent_persisted(
                    path.as_path(),
                    intent,
                    origin.0.as_deref(),
                    &observed_at,
                )
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)?,
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        })
        .map(|_target| ())
}

pub(super) fn revoke_pairing(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let revoked_at: String = timestamp_now();
    runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::Malformed)
        .and_then(|mut registry| match &runtime.persistence {
            LanPairingRegistryPersistence::InMemory => {
                registry.revoke_pairing_for_intent(intent, origin.0.as_deref(), &revoked_at)
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .revoke_pairing_for_intent_persisted(
                    path.as_path(),
                    intent,
                    origin.0.as_deref(),
                    &revoked_at,
                )
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)?,
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        })
}
