use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};

use fs2::FileExt;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope, LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;
use serde_json::Value;

use super::controller_lease::LanControllerLeaseMutation;
use super::TrustedDeviceRegistry;

mod device_state;
mod intent;
mod replay_history;

impl TrustedDeviceRegistry {
    pub fn select_pairing_for_intent(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> Result<LanSelectedRouteTarget, LanPairingRejectionReason> {
        self.apply_intent(intent, origin, observed_at, false, |candidate| {
            ensure_controller_lease_if_required(candidate, intent, observed_at)?;
            let selected = candidate.select_pairing(
                &intent.pairing_id,
                &intent.target_child_device_id,
                &intent.route_id,
                &intent.expires_at,
            )?;
            let _ = candidate.clear_selected_route_reachability();
            Ok(selected)
        })
    }

    pub fn select_pairing_for_intent_persisted(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> io::Result<Result<LanSelectedRouteTarget, LanPairingRejectionReason>> {
        let intent_for_effect = intent.clone();
        let observed_at_for_effect = observed_at.to_owned();
        self.apply_intent_persisted(
            registry_path,
            intent,
            origin,
            observed_at,
            false,
            move |candidate| {
                ensure_controller_lease_if_required(
                    candidate,
                    &intent_for_effect,
                    &observed_at_for_effect,
                )?;
                let selected = candidate.select_pairing(
                    &intent_for_effect.pairing_id,
                    &intent_for_effect.target_child_device_id,
                    &intent_for_effect.route_id,
                    &intent_for_effect.expires_at,
                )?;
                let _ = candidate.clear_selected_route_reachability();
                Ok(selected)
            },
        )
    }

    pub fn revoke_pairing_for_intent(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        self.apply_intent(intent, origin, observed_at, false, |candidate| {
            ensure_controller_lease_if_required(candidate, intent, observed_at)?;
            revoke_pairing_for_intent(candidate, &intent.pairing_id, observed_at)
        })
    }

    pub fn revoke_pairing_for_intent_persisted(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> io::Result<Result<(), LanPairingRejectionReason>> {
        let intent_for_effect = intent.clone();
        let observed_at_for_effect = observed_at.to_owned();
        self.apply_intent_persisted(
            registry_path,
            intent,
            origin,
            observed_at,
            false,
            move |candidate| {
                ensure_controller_lease_if_required(
                    candidate,
                    &intent_for_effect,
                    &observed_at_for_effect,
                )?;
                revoke_pairing_for_intent(
                    candidate,
                    &intent_for_effect.pairing_id,
                    &observed_at_for_effect,
                )
            },
        )
    }

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

    pub fn apply_household_device_decision_for_intent(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        decision: LanHouseholdDeviceDecision,
    ) -> Result<(), LanPairingRejectionReason> {
        self.apply_intent(intent, origin, observed_at, true, move |candidate| {
            ensure_controller_lease_if_required(candidate, intent, observed_at)?;
            if candidate.has_household_device_decision(decision.action_id.as_str()) {
                return Err(LanPairingRejectionReason::Replayed);
            }
            let _changed = candidate.apply_household_device_decision(decision);
            Ok(())
        })
    }

    pub fn apply_household_device_decision_for_intent_persisted(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        decision: LanHouseholdDeviceDecision,
    ) -> io::Result<Result<(), LanPairingRejectionReason>> {
        let intent_for_effect = intent.clone();
        let observed_at_for_effect = observed_at.to_owned();
        self.apply_intent_persisted(
            registry_path,
            intent,
            origin,
            observed_at,
            true,
            move |candidate| {
                ensure_controller_lease_if_required(
                    candidate,
                    &intent_for_effect,
                    &observed_at_for_effect,
                )?;
                if candidate.has_household_device_decision(decision.action_id.as_str()) {
                    return Err(LanPairingRejectionReason::Replayed);
                }
                let _changed = candidate.apply_household_device_decision(decision);
                Ok(())
            },
        )
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
        persisted.merge_accepted_intent_ids(self.accepted_intent_ids.iter().cloned());
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

fn ensure_controller_lease_if_required(
    registry: &mut TrustedDeviceRegistry,
    intent: &LanParentIntentEnvelope,
    observed_at: &str,
) -> Result<(), LanPairingRejectionReason> {
    if intent.parent_authority == LanPairingParentAuthority::ActiveController {
        registry.apply_controller_lease(intent, observed_at, LanControllerLeaseMutation::Ensure)?;
    }
    Ok(())
}

fn revoke_pairing_for_intent(
    registry: &mut TrustedDeviceRegistry,
    pairing_id: &str,
    revoked_at: &str,
) -> Result<(), LanPairingRejectionReason> {
    if !registry
        .entries
        .iter()
        .any(|entry| entry.pairing_id == pairing_id)
    {
        return Err(LanPairingRejectionReason::Anonymous);
    }
    let generation = registry
        .next_authority_generation(pairing_id)
        .map_err(|_error| LanPairingRejectionReason::Malformed)?;
    if !registry.revoke_pairing(pairing_id, revoked_at) {
        return Err(LanPairingRejectionReason::Anonymous);
    }
    registry
        .signer_anchor_generations
        .insert(pairing_id.to_string(), generation);
    Ok(())
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
