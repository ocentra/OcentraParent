use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};

use fs2::FileExt;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason, LanSelectedRouteTarget,
    LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
use serde_json::Value;

use super::TrustedDeviceRegistry;

mod replay_history;

impl TrustedDeviceRegistry {
    pub fn record_challenge_request_persisted(
        &mut self,
        registry_path: &Path,
        challenge_id: &str,
    ) -> io::Result<bool> {
        let challenge_id = challenge_id.to_string();
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.record_challenge_request(challenge_id.as_str()))
        })
    }

    pub fn apply_household_device_decision_persisted(
        &mut self,
        registry_path: &Path,
        decision: LanHouseholdDeviceDecision,
    ) -> io::Result<bool> {
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.apply_household_device_decision(decision))
        })
    }

    pub fn merge_known_household_devices_persisted(
        &mut self,
        registry_path: &Path,
        devices: Vec<LanCanonicalHouseholdDevice>,
    ) -> io::Result<bool> {
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.merge_known_household_devices(devices))
        })
    }

    pub fn accept_pairing_proof_persisted(
        &mut self,
        registry_path: &Path,
        proof: &LanPairingProof,
        child_device: LanPairingDeviceRef,
        parent_device: LanPairingDeviceRef,
        trusted_at: &str,
    ) -> io::Result<LanTrustedDeviceRegistryEntry> {
        self.mutate_persisted_registry(registry_path, move |candidate| {
            let generation = candidate.next_authority_generation(proof.pairing_id.as_str())?;
            let entry =
                candidate.accept_pairing_proof(proof, child_device, parent_device, trusted_at);
            candidate
                .signer_anchor_generations
                .insert(proof.pairing_id.clone(), generation);
            Ok(entry)
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

    fn mutate_persisted_registry<T>(
        &mut self,
        registry_path: &Path,
        mutation: impl FnOnce(&mut Self) -> io::Result<T>,
    ) -> io::Result<T> {
        let lock_path = registry_lock_path(registry_path);
        let lock_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(lock_path)?;
        FileExt::lock_exclusive(&lock_file)?;

        let mut persisted = Self::load_json_strict(registry_path)?;
        if !same_durable_registry_state(&persisted, self) {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "trusted device registry changed before mutation",
            ));
        }
        persisted
            .accepted_intent_ids
            .extend(self.accepted_intent_ids.iter().cloned());
        while persisted.accepted_intent_ids.len()
            > constants::lan_pairing::LAN_PAIRING_MAX_ACCEPTED_INTENT_HISTORY
        {
            if let Some(oldest) = persisted.accepted_intent_ids.iter().next().cloned() {
                persisted.accepted_intent_ids.remove(&oldest);
            }
        }
        replay_history::merge_challenge_ids(self, &mut persisted);
        let result = mutation(&mut persisted)?;
        persisted.save_json(registry_path)?;

        let mut verified = Self::load_json_strict(registry_path)?;
        if verified.to_json_value() != persisted.to_json_value() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "trusted device registry write verification failed",
            ));
        }
        verified.accepted_intent_ids = persisted.accepted_intent_ids;
        verified.accepted_challenge_ids = persisted.accepted_challenge_ids;
        *self = verified;
        Ok(result)
    }

    fn next_authority_generation(&self, pairing_id: &str) -> io::Result<u64> {
        match self.signer_anchor_generations.get(pairing_id) {
            None => Ok(1),
            Some(generation) => generation.checked_add(1).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "trusted device registry authority generation exhausted",
                )
            }),
        }
    }
}

fn same_durable_registry_state(
    persisted: &TrustedDeviceRegistry,
    current: &TrustedDeviceRegistry,
) -> bool {
    let mut persisted_value = persisted.to_json_value();
    let mut current_value = current.to_json_value();
    remove_replay_history(&mut persisted_value);
    remove_replay_history(&mut current_value);
    persisted_value == current_value
}

fn remove_replay_history(value: &mut Value) {
    if let Some(object) = value.as_object_mut() {
        object.remove(super::json_persistence::ACCEPTED_INTENT_IDS_KEY);
        object.remove(super::json_persistence::ACCEPTED_CHALLENGE_IDS_KEY);
    }
}

fn registry_lock_path(registry_path: &Path) -> PathBuf {
    let mut lock_path = registry_path.as_os_str().to_os_string();
    lock_path.push(".lock");
    PathBuf::from(lock_path)
}
