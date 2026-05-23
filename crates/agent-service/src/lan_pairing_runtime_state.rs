use std::sync::{Arc, Mutex};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::LanSelectedRouteTarget;

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
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
