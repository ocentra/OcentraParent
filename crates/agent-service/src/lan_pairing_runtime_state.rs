use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{constants, LanSelectedRouteTarget};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id: None,
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
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id,
        }
    }

    pub fn persistent_json_with_local_child_device_id(
        path: &Path,
        local_child_device_id: Option<String>,
    ) -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::load_json(path))),
            persistence: LanPairingRegistryPersistence::LocalJsonRegistry(path.to_path_buf()),
            local_child_device_id,
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

fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}
