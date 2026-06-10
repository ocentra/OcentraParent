use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    constants, LanPairingProof, LanPairingRejectionReason, LanSelectedRouteTarget,
};

use crate::{
    lan_pairing::{LanPairingChallengeState, LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

mod device_roles;
mod job_leases;
mod provider_heartbeat;
mod provider_routing;
use device_roles::{
    default_device_role_read_model, device_role_read_model_from_env,
    lan_ai_provider_capabilities_from_env, non_empty_env,
};
pub(crate) use job_leases::{LanAiJobLeaseState, LanAiJobLeaseTransition};
pub(crate) use provider_heartbeat::LanAiProviderHeartbeatState;

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
            challenges: Arc::new(Mutex::new(Vec::new())),
            controller_lease: Arc::new(Mutex::new(None)),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id: None,
            device_roles: default_device_role_read_model(None),
            lan_ai_provider_capabilities: Vec::new(),
        }
    }

    pub fn from_env() -> Self {
        let local_child_device_id =
            non_empty_env(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV);
        match std::env::var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH)
            .ok()
            .filter(|path| !path.is_empty())
        {
            Some(path) => Self::persistent_json_with_local_child_device_id(
                Path::new(&path),
                local_child_device_id,
            ),
            None => Self::empty_with_local_child_device_id(local_child_device_id),
        }
    }

    #[cfg(test)]
    pub fn persistent_json(path: &Path) -> Self {
        Self::persistent_json_with_local_child_device_id(path, None)
    }

    pub fn empty_with_local_child_device_id(local_child_device_id: Option<String>) -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
            challenges: Arc::new(Mutex::new(Vec::new())),
            controller_lease: Arc::new(Mutex::new(None)),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id,
            device_roles: device_role_read_model_from_env(),
            lan_ai_provider_capabilities: lan_ai_provider_capabilities_from_env(),
        }
    }

    pub fn persistent_json_with_local_child_device_id(
        path: &Path,
        local_child_device_id: Option<String>,
    ) -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::load_json(path))),
            challenges: Arc::new(Mutex::new(Vec::new())),
            controller_lease: Arc::new(Mutex::new(None)),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            persistence: LanPairingRegistryPersistence::LocalJsonRegistry(path.to_path_buf()),
            local_child_device_id,
            device_roles: device_role_read_model_from_env(),
            lan_ai_provider_capabilities: lan_ai_provider_capabilities_from_env(),
        }
    }

    pub fn trusted_device_count(&self) -> usize {
        self.registry
            .lock()
            .map(|registry| registry.trusted_device_count())
            .unwrap_or(0)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        let observed_at = timestamp_now();
        self.registry
            .lock()
            .ok()
            .and_then(|registry| registry.selected_target_at(&observed_at))
    }

    pub fn trusted_device_ids(&self) -> Vec<String> {
        self.registry
            .lock()
            .map(|registry| registry.trusted_device_ids())
            .unwrap_or_default()
    }

    pub fn revoked_device_ids(&self) -> Vec<String> {
        self.registry
            .lock()
            .map(|registry| registry.revoked_device_ids())
            .unwrap_or_default()
    }

    pub fn has_revoked_pairing(&self) -> bool {
        self.registry
            .lock()
            .map(|registry| registry.has_revoked_pairing())
            .unwrap_or(false)
    }

    pub(crate) fn remember_challenge(&self, challenge: LanPairingChallengeState) {
        if let Ok(mut challenges) = self.challenges.lock() {
            challenges.retain(|candidate| candidate.challenge_id != challenge.challenge_id);
            challenges.push(challenge);
        }
    }

    pub(crate) fn validate_challenge_proof(
        &self,
        proof: &LanPairingProof,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let mut challenges = self
            .challenges
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        if challenges.is_empty() {
            return Ok(());
        }

        let challenge = challenges
            .iter_mut()
            .find(|candidate| candidate.challenge_id == proof.challenge_id)
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if challenge.accepted {
            return Err(LanPairingRejectionReason::Replayed);
        }
        if challenge.child_device_id != proof.child_device_id {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if challenge.parent_device_id != proof.parent_device_id {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if challenge.route_id != proof.route_id {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        if challenge.origin != proof.origin {
            return Err(LanPairingRejectionReason::WrongOrigin);
        }
        if challenge.proof_digest != proof.proof_digest {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if observed_at > challenge.expires_at.as_str() || observed_at > proof.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Stale);
        }

        challenge.accepted = true;
        Ok(())
    }

    pub(crate) fn persistence_mode(&self) -> &'static str {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY
            }
        }
    }

    pub(crate) fn restart_behavior(&self) -> &'static str {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                constants::value::LAN_RESTART_FAIL_CLOSED_UNPAIRED
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_)
                if self.selected_target().is_some() =>
            {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_SELECTED_ROUTE
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED
            }
        }
    }

    pub(crate) fn persist_registry(&self, registry: &TrustedDeviceRegistry) -> bool {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => true,
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => {
                registry.save_json(path.as_path()).is_ok()
            }
        }
    }

    #[cfg(test)]
    pub fn mark_selected_offline_for_test(&self, offline_at: &str) -> bool {
        self.registry
            .lock()
            .map(|mut registry| registry.mark_selected_offline(offline_at))
            .unwrap_or(false)
    }

    #[cfg(test)]
    pub fn mark_selected_stale_for_test(&self, stale_at: &str) -> bool {
        self.registry
            .lock()
            .map(|mut registry| registry.mark_selected_stale(stale_at))
            .unwrap_or(false)
    }
}
