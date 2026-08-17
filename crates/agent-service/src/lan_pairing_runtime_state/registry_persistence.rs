use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    constants,
    lan_pairing::{LanPairingRejectionReason, LanPairingText},
    lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision,
};

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};

mod known_device_merge;

impl LanPairingRuntime {
    pub(crate) fn persistence_mode(&self) -> LanPairingText {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory
            | LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED.into()
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY.into()
            }
        }
    }

    pub(crate) fn restart_behavior(&self) -> LanPairingText {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory
            | LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                constants::value::LAN_RESTART_FAIL_CLOSED_UNPAIRED.into()
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_)
                if self.selected_target().is_some() =>
            {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_SELECTED_ROUTE.into()
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED.into()
            }
        }
    }

    pub(crate) fn apply_household_device_decision(
        &self,
        registry: &mut TrustedDeviceRegistry,
        decision: LanHouseholdDeviceDecision,
    ) -> Result<(), LanPairingRejectionReason> {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                let _ = registry.apply_household_device_decision(decision);
                Ok(())
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .apply_household_device_decision_persisted(path.as_path(), decision)
                .map(|_changed| ())
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        }
    }
}
