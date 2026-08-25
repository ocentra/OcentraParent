use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use ocentra_lan_core::lan_pairing::LanSignedChildAgentReplayGuard;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

use super::device_roles::{
    default_device_role_read_model, device_role_read_model_from_env,
    lan_ai_provider_capabilities_from_env, non_empty_env,
};

#[derive(Clone)]
struct LanPairingRuntimeContext {
    local_child_device_id: Option<LanPairingText>,
    signed_child_agent_parent_device_id: Option<LanPairingText>,
    signed_child_agent_family_hash: Option<LanPairingText>,
    signed_child_agent_route_id: LanPairingText,
}

struct LanPairingRegistryPath(PathBuf);

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
            challenges: Arc::new(Mutex::new(Vec::new())),
            signed_child_agent_replay_guard: Arc::new(Mutex::new(
                LanSignedChildAgentReplayGuard::new(),
            )),
            passive_discovery_listener_state: Arc::new(Mutex::new(
                LanPassiveDiscoveryListenerState::running(timestamp_now()),
            )),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            browser_discovery_scan_worker: Arc::new(Mutex::new(None)),
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id: None,
            signed_child_agent_parent_device_id: None,
            signed_child_agent_family_hash: None,
            signed_child_agent_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            device_roles: default_device_role_read_model(None),
            lan_ai_provider_capabilities: Vec::new(),
        }
    }

    pub fn from_env() -> Self {
        let runtime_context = LanPairingRuntimeContext {
            local_child_device_id: non_empty_env(LanPairingText(
                constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV.to_string(),
            )),
            signed_child_agent_parent_device_id: non_empty_env(LanPairingText(
                constants::lan_pairing::SIGNED_CHILD_AGENT_PARENT_DEVICE_ID_ENV.to_string(),
            )),
            signed_child_agent_family_hash: non_empty_env(LanPairingText(
                constants::lan_pairing::SIGNED_CHILD_AGENT_FAMILY_HASH_ENV.to_string(),
            )),
            signed_child_agent_route_id: non_empty_env(LanPairingText(
                constants::lan_pairing::SIGNED_CHILD_AGENT_ROUTE_ID_ENV.to_string(),
            ))
            .unwrap_or_else(|| {
                constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
                    .to_string()
                    .into()
            }),
        };
        let registry_path = lan_pairing_registry_path_from_env();
        Self::persistent_json_with_context(registry_path.as_ref(), runtime_context)
    }

    fn persistent_json_with_context(
        path: Option<&LanPairingRegistryPath>,
        runtime_context: LanPairingRuntimeContext,
    ) -> Self {
        let (registry, persistence) = load_registry_with_owner_path(path);
        Self {
            registry: Arc::new(Mutex::new(registry)),
            challenges: Arc::new(Mutex::new(Vec::new())),
            signed_child_agent_replay_guard: Arc::new(Mutex::new(
                LanSignedChildAgentReplayGuard::new(),
            )),
            passive_discovery_listener_state: Arc::new(Mutex::new(
                LanPassiveDiscoveryListenerState::running(timestamp_now()),
            )),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            browser_discovery_scan_worker: Arc::new(Mutex::new(None)),
            persistence,
            local_child_device_id: runtime_context.local_child_device_id.map(|value| value.0),
            signed_child_agent_parent_device_id: runtime_context
                .signed_child_agent_parent_device_id
                .map(|value| value.0),
            signed_child_agent_family_hash: runtime_context
                .signed_child_agent_family_hash
                .map(|value| value.0),
            signed_child_agent_route_id: runtime_context.signed_child_agent_route_id.0,
            device_roles: device_role_read_model_from_env(),
            lan_ai_provider_capabilities: lan_ai_provider_capabilities_from_env()
                .into_iter()
                .map(|value| value.0)
                .collect(),
        }
    }
}

fn load_registry_with_owner_path(
    path: Option<&LanPairingRegistryPath>,
) -> (TrustedDeviceRegistry, LanPairingRegistryPersistence) {
    let Some(path) = path else {
        return (
            TrustedDeviceRegistry::empty(),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry,
        );
    };
    match TrustedDeviceRegistry::load_or_initialize_json_strict(path.0.as_path()) {
        Ok(registry) => (
            registry,
            LanPairingRegistryPersistence::LocalJsonRegistry(path.0.clone()),
        ),
        Err(_error) => (
            TrustedDeviceRegistry::empty(),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry,
        ),
    }
}

fn lan_pairing_registry_path_from_env() -> Option<LanPairingRegistryPath> {
    std::env::var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH)
        .ok()
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
        .map(LanPairingRegistryPath)
}
