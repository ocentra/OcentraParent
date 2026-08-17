use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

pub(super) fn select_pairing_result(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::Malformed)
        .and_then(|mut registry| match &runtime.persistence {
            LanPairingRegistryPersistence::InMemory => {
                let selected = registry.select_pairing(
                    &intent.pairing_id,
                    &intent.target_child_device_id,
                    &intent.route_id,
                    &intent.expires_at,
                );
                if selected.is_ok() {
                    let _ = registry.clear_selected_route_reachability();
                }
                selected
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .select_pairing_persisted(
                    path.as_path(),
                    &intent.pairing_id,
                    &intent.target_child_device_id,
                    &intent.route_id,
                    &intent.expires_at,
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
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let revoked_at: String = timestamp_now();
    runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::Malformed)
        .and_then(|mut registry| {
            let revoked = match &runtime.persistence {
                LanPairingRegistryPersistence::InMemory => {
                    registry.revoke_pairing(&intent.pairing_id, &revoked_at)
                }
                LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                    .revoke_pairing_persisted(path.as_path(), &intent.pairing_id, &revoked_at)
                    .map_err(|_error| {
                        LanPairingRejectionReason::SignedChildAgentContextUnavailable
                    })?,
                LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                    return Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable);
                }
            };
            if revoked {
                Ok(())
            } else {
                Err(LanPairingRejectionReason::Anonymous)
            }
        })
}
