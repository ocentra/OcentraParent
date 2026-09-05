use std::{io, path::Path};

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::super::TrustedDeviceRegistry;

impl TrustedDeviceRegistry {
    pub fn merge_known_household_devices_persisted(
        &mut self,
        registry_path: &Path,
        devices: Vec<LanCanonicalHouseholdDevice>,
    ) -> io::Result<bool> {
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.merge_known_household_devices(devices))
        })
    }

    pub fn select_pairing_persisted(
        &mut self,
        registry_path: &Path,
        pairing_id: &str,
        target_child_device_id: &str,
        route_id: &str,
        stale_at: &str,
    ) -> io::Result<Result<LanSelectedRouteTarget, LanPairingRejectionReason>> {
        self.mutate_persisted_registry(registry_path, |candidate| {
            let selected =
                candidate.select_pairing(pairing_id, target_child_device_id, route_id, stale_at);
            if selected.is_ok() {
                let _ = candidate.clear_selected_route_reachability();
            }
            Ok(selected)
        })
    }

    pub fn revoke_pairing_persisted(
        &mut self,
        registry_path: &Path,
        pairing_id: &str,
        revoked_at: &str,
    ) -> io::Result<bool> {
        self.mutate_persisted_registry(registry_path, |candidate| {
            if !candidate
                .entries
                .iter()
                .any(|entry| entry.pairing_id == pairing_id)
            {
                return Ok(false);
            }
            let generation = candidate.next_authority_generation(pairing_id)?;
            let revoked = candidate.revoke_pairing(pairing_id, revoked_at);
            if revoked {
                candidate
                    .signer_anchor_generations
                    .insert(pairing_id.to_string(), generation);
            }
            Ok(revoked)
        })
    }
}
